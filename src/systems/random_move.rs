use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(AIState)]
#[read_component(Player)]
pub fn random_move(ecs: &SubWorld, commands: &mut CommandBuffer) {
    let &player = <Entity>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next().unwrap();

    <(Entity, &Point, &AIState)>::query()
        .iter(ecs)
        .filter(|(_, _, &state)| state == AIState::MovingRandomly)
        .for_each(|(entity, pos, _)| {
            let mut rng = RandomNumberGenerator::new();
            let delta = rng.random_slice_entry(&[(-1, 0), (1, 0), (0, -1), (0, 1)])
                .copied().map(|(x, y)| Point::new(x, y)).unwrap();
            let destination = *pos + delta;

            let mut attacked = false;
            <(Entity, &Point)>::query()
                .iter(ecs)
                .filter(|(&victim, &target_pos)|
                    victim == player && target_pos == destination)
                .for_each(|(victim, _)| {
                    attacked = true;
                    commands.add_component(*entity, WantsToAttack(*victim));
                });
            if !attacked {
                commands.add_component(*entity, WantsToMove(destination));
            }
        });
}
