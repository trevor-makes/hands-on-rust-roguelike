use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        if let Some(delta) = match key {
            VirtualKeyCode::Left => Some(Point::new(-1, 0)),
            VirtualKeyCode::Right => Some(Point::new(1, 0)),
            VirtualKeyCode::Up => Some(Point::new(0, -1)),
            VirtualKeyCode::Down => Some(Point::new(0, 1)),
            _ => None,
        } {
            let mut blocked = false;
            let (player_entity, destination) = <(Entity, &Point)>::query()
                .filter(component::<Player>())
                .iter(ecs)
                .map(|(entity, pos)| (*entity, *pos + delta))
                .next().unwrap();
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
                commands.push(((), WantsToMove {
                    entity: player_entity,
                    destination,
                }));
            }
        }
        *turn_state = TurnState::PlayerTurn;
    }
}