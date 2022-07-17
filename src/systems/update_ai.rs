use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[write_component(AIState)]
#[read_component(FieldOfView)]
pub fn update_ai(ecs: &mut SubWorld) {
    let &player_pos = <&Point>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next().unwrap();
    <(&mut AIState, &FieldOfView)>::query()
        .iter_mut(ecs)
        .for_each(|(state, fov)| {
            // Chase the player if visible, otherwise move randomly
            *state = if fov.visible.contains(&player_pos) {
                AIState::ChasingPlayer
            } else {
                AIState::MovingRandomly
            };
        });
}
