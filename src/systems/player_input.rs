use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let (player_entity, player_pos) = <(Entity, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .map(|(entity, pos)| (*entity, *pos))
        .next().unwrap();
    if let Some(key) = key {
        if let Some(delta) = match key {
            VirtualKeyCode::Left => Some(Point::new(-1, 0)),
            VirtualKeyCode::Right => Some(Point::new(1, 0)),
            VirtualKeyCode::Up => Some(Point::new(0, -1)),
            VirtualKeyCode::Down => Some(Point::new(0, 1)),
            _ => None,
        } {
            let destination = player_pos + delta;
            let mut blocked = false;
            <(Entity, &Point)>::query()
                .filter(component::<Enemy>())
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    blocked = true;
                    commands.push(((), WantsToAttack {
                        attacker: player_entity,
                        victim: *entity,
                    }));
                });
            if !blocked {
                commands.add_component(player_entity, WantsToMove(destination));
            }
        } else if let Ok(mut health) = ecs.entry_mut(player_entity).unwrap().get_component_mut::<Health>() {
            // TODO this heal is too frequent; maybe add a cooldown so it only fires every N frames
            health.current = i32::min(health.max, health.current + 1);
        }
        *turn_state = TurnState::PlayerTurn;
    }
}