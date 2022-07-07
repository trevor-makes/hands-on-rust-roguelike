use crate::prelude::*;

#[system]
#[read_component(WantsToMove)]
#[read_component(Point)]
#[read_component(Player)]
pub fn movement(
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
) {
    let &player_entity = <Entity>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();

    <(Entity, &WantsToMove)>::query()
        .iter(ecs)
        .for_each(|(&entity, &WantsToMove(destination))| {
            if map.can_enter_tile(destination) {
                let mut blocked = false;
                for (&other_entity, &other_pos) in <(Entity, &Point)>::query().iter(ecs) {
                    if other_entity == entity {
                        break;
                    }
                    if let Ok(&WantsToMove(other_dest)) = ecs
                        .entry_ref(other_entity)
                        .unwrap()
                        .get_component() {
                        if other_dest == destination {
                            // blocked by an earlier WantsToMove
                            blocked = true;
                            break;
                        }
                    } else if other_pos == destination {
                        // blocked by non-moving entity
                        blocked = true;
                        break;
                    }
                }
                if !blocked {
                    commands.add_component(entity, destination);
                    if entity == player_entity {
                        camera.on_player_move(destination);
                    }
                }
            }
            commands.remove_component::<WantsToMove>(entity);
        });
}
