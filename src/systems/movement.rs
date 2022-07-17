use crate::prelude::*;

use std::collections::HashSet;

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
        .filter(!component::<Item>())
        .iter(ecs)
        .for_each(|(&entity_a, &loc_a)| {
            // If A is attempting to move...
            if let Ok(&WantsToMove(move_a)) = ecs
                .entry_ref(entity_a).unwrap()
                .get_component() {
                // NOTE the "can enter tile" state could change dynamically during system processing
                // (door is closed, trap is triggered), so an attempted move by player/AI could be blocked
                if map.can_enter_tile(move_a) {
                    for (&entity_b, &loc_b) in <(Entity, &Point)>::query()
                        .filter(!component::<Item>())
                        .iter(ecs) {
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
                    .filter(component::<Point>() & !component::<Item>())
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


fn is_entity_moving(ecs: &SubWorld, entity: Entity, entry: &EntryRef) -> Option<Point> {
    fn recursive_impl(ecs: &SubWorld, entity: Entity, entry: &EntryRef, visited: &mut HashSet::<Entity>) -> Option<Point> {
        // TODO the ECS is making this traversal a bit difficult; maybe implement the traversal graph outside of ECS
        // TODO if we could set a mut flag when we already computed this, we'd have dynamic programming
        visited.insert(entity);
        if let Ok(&WantsToMove(move_to)) = entry.get_component() {
            if let Ok(&BlockedBy(blocker)) = entry.get_component() {
                // Break cycles in the blocking graph by checking for already visited nodes
                if visited.contains(&entity) {
                    // TODO in some cases movement should be permitted:
                    // - 2 entites swapping places?
                    // - 4 entities chasing each other in a circle? but a 5th entity trying to jump in SHOULD be blocked?
                    None
                } else {
                    let blocker_ref = ecs.entry_ref(blocker).unwrap();
                    recursive_impl(ecs, blocker, &blocker_ref, visited).map(|_| move_to)
                }
            } else {
                Some(move_to)
            }
        } else {
            None
        }
    }
    let mut visited = HashSet::<Entity>::new();
    recursive_impl(ecs, entity, entry, &mut visited)
}

#[system]
#[read_component(WantsToMove)]
#[read_component(BlockedBy)]
#[read_component(Player)]
#[read_component(FieldOfView)]
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
            let entry = ecs.entry_ref(entity).unwrap();
            if let Some(move_to) = is_entity_moving(ecs, entity, &entry) {
                if let Ok(fov) = entry.get_component::<FieldOfView>() {
                    commands.add_component(entity, fov.clone_dirty());
                }
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
