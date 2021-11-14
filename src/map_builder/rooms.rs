use super::MapArchitect;
use crate::prelude::*;

pub struct RoomsArchitect {}

impl MapArchitect for RoomsArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };

        mb.fill(TileType::Wall);
        self.build_random_rooms(&mut mb.map, &mut mb.rooms, rng);
        self.build_corridors(&mut mb.map, &mb.rooms, rng);
        mb.player_start = mb.rooms[0].center();
        mb.amulet_start = mb.find_most_distant();

        for room in mb.rooms.iter().skip(1) {
            mb.monster_spawns.push(room.center());
        }

        mb
    }
}

impl RoomsArchitect {
    fn build_random_rooms(
        &mut self,
        map: &mut Map,
        rooms: &mut Vec<Rect>,
        rng: &mut RandomNumberGenerator,
    ) {
        const NUM_ROOMS: usize = 20;

        while rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );

            let mut overlap = false;
            for r in rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                    break;
                }
            }

            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        map.tiles[idx] = TileType::Floor;
                    }
                });

                rooms.push(room);
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, map: &mut Map, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = map.try_idx(Point::new(x, y)) {
                map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, map: &mut Map, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = map.try_idx(Point::new(x, y)) {
                map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn build_corridors(
        &mut self,
        map: &mut Map,
        rooms: &Vec<Rect>,
        rng: &mut RandomNumberGenerator,
    ) {
        let mut room_clone = rooms.clone();
        room_clone.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in room_clone.iter().enumerate().skip(1) {
            let prev = room_clone[i - 1].center();
            let new = room.center();

            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(map, prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(map, prev.y, new.y, new.x);
            } else {
                self.apply_horizontal_tunnel(map, prev.x, new.x, new.y);
                self.apply_vertical_tunnel(map, prev.y, new.y, prev.x);
            }
        }
    }
}
