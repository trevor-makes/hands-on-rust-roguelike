use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let player = <Entity>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .copied()
        .next()
        .unwrap();
    let victims = <(Entity, &WantsToAttack)>::query()
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim))
        .collect::<Vec<_>>();
    victims.iter().for_each(|(message, victim)| {
        if let Ok(mut health) = ecs.entry_mut(*victim).unwrap().get_component_mut::<Health>() {
            health.current -= 1;
            if health.current < 1 && *victim != player {
                commands.remove(*victim);
            }
        }
        commands.remove(*message);
    });
}
