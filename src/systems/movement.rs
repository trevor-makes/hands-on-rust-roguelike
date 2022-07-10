use crate::prelude::*;

#[system]
#[read_component(WantsToMove)]
#[read_component(Point)]
pub fn movement_phase1(
    #[resource] map: &Map,
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
) {
    // Iterate over (Point[,WantsToMove])x(Point[,WantsToMove]) combinations
    <(Entity, &Point)>::query()
        .iter(ecs)
        .for_each(|(&entity_a, &loc_a)| {
            // If A is attempting to move...
            if let Ok(&WantsToMove(move_a)) = ecs
                .entry_ref(entity_a).unwrap()
                .get_component() {
                // NOTE the "can enter tile" state could change dynamically during system processing
                // (door is closed, trap is triggered), so an attempted move by player/AI could be blocked
                if map.can_enter_tile(move_a) {
                    for (&entity_b, &loc_b) in <(Entity, &Point)>::query().iter(ecs) {
                        // When entity_b reaches entity_a, increment entity_a and start over
                        if entity_b == entity_a {
                            break;
                        }
                        // If B is attempting to move...
                        if let Ok(&WantsToMove(move_b)) = ecs
                            .entry_ref(entity_b).unwrap()
                            .get_component() {
                            // TODO if A and B are swapping places, we may crash by creating a cycle in the blocking graph
                            // If B is leaving where A wants move...
                            if loc_b == move_a {
                                commands.add_component(entity_a, BlockedBy(entity_b));
                            }
                            // If A is leaving where B wants to move...
                            if move_b == loc_a {
                                commands.add_component(entity_b, BlockedBy(entity_a));
                            }
                            // If A and B are trying to move to the same place...
                            if move_b == move_a {
                                // For now, let the earlier ID win (will always be B, so block A's move)
                                // TODO Should there be an initiative order (based on speed, size, mass, will) to settle winner?
                                commands.remove_component::<WantsToMove>(entity_a);
                            }
                        } else if loc_b == move_a { // B is blocking A's move
                            commands.remove_component::<WantsToMove>(entity_a);
                        }
                    }
                } else { // A is blocked by map
                    commands.remove_component::<WantsToMove>(entity_a);
                }
            } else { // A is not moving
                for &entity_b in <Entity>::query()
                    .filter(component::<Point>())
                    .iter(ecs) {
                    // When entity_b reaches entity_a, increment entity_a and start over
                    if entity_b == entity_a {
                        break;
                    }
                    // If B is attempting to move...
                    if let Ok(&WantsToMove(move_b)) = ecs
                        .entry_ref(entity_b).unwrap()
                        .get_component() {
                        // If A is not moving and blocks B...
                        if move_b == loc_a {
                            commands.remove_component::<WantsToMove>(entity_b);
                        }
                    }
                    // Otherwise, don't care if neither WantsToMove
                }
            }
        });
}

fn is_entity_moving(ecs: &SubWorld, entity: Entity) -> Option<Point> {
    // TODO if we could set a mut flag when we already computed this, we'd have dynamic programming
    if let Ok(&WantsToMove(move_to)) = ecs
        .entry_ref(entity).unwrap()
        .get_component() {
        if let Ok(&BlockedBy(blocker)) = ecs
            .entry_ref(entity).unwrap()
            .get_component() {
            // TODO this could crash if a cycle forms
            is_entity_moving(ecs, blocker).map(|_| move_to)
        } else {
            Some(move_to)
        }
    } else {
        None
    }
}

#[system]
#[read_component(WantsToMove)]
#[read_component(BlockedBy)]
#[read_component(Player)]
pub fn movement_phase2(
    #[resource] camera: &mut Camera,
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
) {
    // Get player ID; no reference, just an integer
    let &player_entity = <Entity>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next().unwrap();

    // Resolve moves by traversing blocking graph
    <Entity>::query()
        .filter(component::<WantsToMove>())
        .iter(ecs)
        .for_each(|&entity| {
            if let Some(move_to) = is_entity_moving(ecs, entity) {
                commands.add_component(entity, move_to);
                if entity == player_entity {
                    camera.on_player_move(move_to);
                }
            }
            commands.remove_component::<WantsToMove>(entity);
        });

    // TODO is there a better way to remove all the BlockedBy components?
    <Entity>::query()
        .filter(component::<BlockedBy>())
        .iter(ecs)
        .for_each(|&entity| {
            commands.remove_component::<BlockedBy>(entity);
        });
}
