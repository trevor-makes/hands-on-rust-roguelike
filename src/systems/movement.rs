use crate::prelude::*;

#[system]
#[read_component(WantsToMove)]
#[read_component(Player)]
pub fn movement(
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
) {
    let player_entity = <Entity>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next().unwrap();

    <(Entity, &WantsToMove)>::query()
        .iter(ecs)
        .for_each(|(entity, want_move)| {
            if map.can_enter_tile(want_move.destination) {
                commands.add_component(want_move.entity, want_move.destination);
                if want_move.entity == *player_entity {
                    camera.on_player_move(want_move.destination);
                }
            }
            commands.remove(*entity);
        });
}
