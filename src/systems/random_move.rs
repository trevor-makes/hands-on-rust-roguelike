use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(AIState)]
#[read_component(Player)]
pub fn random_move(ecs: &SubWorld, commands: &mut CommandBuffer) {
    let (&player, &player_pos) = <(Entity, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next().unwrap();

    <(Entity, &Point, &AIState)>::query()
        .iter(ecs)
        .filter(|(_, _, &state)| state == AIState::MovingRandomly)
        .for_each(|(&entity, &pos, _)| {
            // Select an adjacent tile randomly
            let mut rng = RandomNumberGenerator::new();
            let delta = rng.random_slice_entry(&[(-1, 0), (1, 0), (0, -1), (0, 1)])
                .copied().map(|(x, y)| Point::new(x, y)).unwrap();
            let move_pos = pos + delta;

            // Attack if player is in the way
            if move_pos == player_pos {
                commands.add_component(entity, WantsToAttack(player));
            } else {
                commands.add_component(entity, WantsToMove(move_pos));
            }
        });
}
