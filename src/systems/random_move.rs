use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
#[read_component(Health)]
#[read_component(Player)]
pub fn random_move(ecs: &SubWorld, commands: &mut CommandBuffer) {
    let player = <Entity>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();
    <(Entity, &Point, &MovingRandomly)>::query().iter(ecs)
        .for_each(|(entity, pos, _)| {
            let mut rng = RandomNumberGenerator::new();
            let delta = rng.random_slice_entry(&[(-1, 0), (1, 0), (0, -1), (0, 1)])
                .copied().map(|(x, y)| Point::new(x, y)).unwrap();
            let destination = *pos + delta;
            let mut blocked = false;
            <(Entity, &Point)>::query()
                .iter(ecs)
                .filter(|(_, target_pos)| **target_pos == destination)
                .map(|(victim, _)| victim)
                .inspect(|_| blocked = true)
                .filter(|victim| *victim == player)
                .for_each(|victim| {
                    commands.push(((), WantsToAttack {
                        attacker: *entity,
                        victim: *victim,
                    }));
                });
            if !blocked {
                commands.push(((), WantsToMove{ entity: *entity, destination }));
            }
        });
}
