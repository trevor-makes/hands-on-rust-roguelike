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
    let (&player, &player_pos) = <(Entity, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
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
            let mut attacked = false;
            <(Entity, &Point)>::query()
                .filter(component::<Enemy>())
                .iter(ecs)
                .filter(|(_, &pos)| pos == destination)
                .for_each(|(&entity, _)| {
                    attacked = true;
                    commands.add_component(player, WantsToAttack(entity));
                });
            if !attacked {
                commands.add_component(player, WantsToMove(destination));
            }
        } else if let Ok(mut health) = ecs.entry_mut(player).unwrap().get_component_mut::<Health>() {
            // TODO this heal is too frequent; maybe add a cooldown so it only fires every N frames
            health.current = i32::min(health.max, health.current + 1);
        }
        *turn_state = TurnState::PlayerTurn;
    }
}
