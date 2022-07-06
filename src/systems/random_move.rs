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
            <(Entity, &Point, &Health)>::query()
                .iter(ecs)
                .filter(|(_, target_pos, _)| **target_pos == destination)
                .for_each(|(victim, _, _)| {
                    if victim == player {
                        commands.push(((), WantsToAttack {
                            attacker: *entity,
                            victim: *victim,
                        }));
                    }
                    blocked = true;
                });
            if !blocked {
                commands.push(((), WantsToMove{ entity: *entity, destination }));
            }
        });
}
