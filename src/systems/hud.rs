use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
#[read_component(Weapon)]
pub fn hud(ecs: &SubWorld) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.print_centered(1, "Explore the dungeon. Cursor keys to move.");

    // Player health
    {
        let mut health_query = <&Health>::query().filter(component::<Player>());
        let player_health = health_query.iter(ecs).nth(0).unwrap();

        draw_batch.bar_horizontal(
            Point::zero(),
            SCREEN_WIDTH * 2,
            player_health.current,
            player_health.max,
            ColorPair::new(RED, BLACK),
        );
        draw_batch.print_color_centered(
            0,
            format!("Health: {} / {}", player_health.current, player_health.max),
            ColorPair::new(WHITE, RED),
        );
    }

    let (player, map_level) = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, player)| Some((*entity, player.map_level)))
        .unwrap();

    // Map level
    {
        draw_batch.print_color_right(
            Point::new(SCREEN_WIDTH * 2, 1),
            format!("Dungeon level: {}", map_level + 1),
            ColorPair::new(YELLOW, BLACK),
        );
    }

    // Player weapon
    {
        let item = <(&Weapon, &Item, &Name, &Carried)>::query()
            .iter(ecs)
            .filter(|(_weapon, _item, _name, carried)| carried.0 == player)
            .find_map(|(_weapon, _item, name, _carried)| Some(name));

        let weapon_name = if let Some(weapon) = item {
            &weapon.0
        } else {
            "None"
        };

        draw_batch.print_color(Point::new(3, 2), "Weapon:", ColorPair::new(YELLOW, BLACK));
        draw_batch.print(Point::new(11, 2), weapon_name);
    }

    // Player items
    {
        let mut has_items = false;

        <(Entity, &Item, &Name, &Carried)>::query()
            .iter(ecs)
            .filter(|(entity, _item, _name, carried)| {
                let mut is_weapon = false;
                if let Ok(e) = ecs.entry_ref(**entity) {
                    is_weapon = e.get_component::<Weapon>().is_ok();
                }
                !is_weapon && carried.0 == player
            })
            .enumerate()
            .for_each(|(i, (_entity, _item, name, _carried))| {
                draw_batch.print(Point::new(3, 5 + i), format!("{} : {}", i + 1, &name.0));
                has_items = true;
            });

        if has_items {
            draw_batch.print_color(
                Point::new(3, 4),
                "Items carried",
                ColorPair::new(YELLOW, BLACK),
            );
        }
    }

    draw_batch.submit(10000).expect("Batch error");
}
