use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
#[read_component(Item)]
#[read_component(Carried)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = *key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::G => pick_up_item(ecs, commands),
            VirtualKeyCode::Key1 => use_item(0, ecs, commands),
            VirtualKeyCode::Key2 => use_item(1, ecs, commands),
            VirtualKeyCode::Key3 => use_item(2, ecs, commands),
            VirtualKeyCode::Key4 => use_item(3, ecs, commands),
            VirtualKeyCode::Key5 => use_item(4, ecs, commands),
            VirtualKeyCode::Key6 => use_item(5, ecs, commands),
            VirtualKeyCode::Key7 => use_item(6, ecs, commands),
            VirtualKeyCode::Key8 => use_item(7, ecs, commands),
            VirtualKeyCode::Key9 => use_item(8, ecs, commands),
            _ => Point::zero(),
        };

        let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();

        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;

            let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                });

            if !hit_something {
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
        }

        *turn_state = TurnState::PlayerTurn;
    }
}

fn pick_up_item(ecs: &mut SubWorld, commands: &mut CommandBuffer) -> Point {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    let (player, player_pos) = players
        .iter(ecs)
        .find_map(|(entity, pos)| Some((*entity, *pos)))
        .unwrap();

    let mut items = <(Entity, &Point)>::query().filter(component::<Item>());
    items
        .iter(ecs)
        .filter(|(_, &item_pos)| item_pos == player_pos)
        .for_each(|(entity, _)| {
            commands.remove_component::<Point>(*entity);
            commands.add_component(*entity, Carried(player));
        });

    Point::zero()
}

fn use_item(n: usize, ecs: &mut SubWorld, commands: &mut CommandBuffer) -> Point {
    let player = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, _player)| Some(*entity))
        .unwrap();

    let carried_item = <(Entity, &Item, &Carried)>::query()
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player)
        .enumerate()
        .filter(|(item_count, (_, _, _))| *item_count == n)
        .find_map(|(_, (item, _, _))| Some(*item));

    if let Some(carried_item) = carried_item {
        commands.push((
            (),
            ActivateItem {
                used_by: player,
                item: carried_item,
            },
        ));
    }

    Point::zero()
}
