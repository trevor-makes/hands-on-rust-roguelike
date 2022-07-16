use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(AIState)]
#[read_component(Health)]
#[read_component(Player)]
pub fn chasing(#[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer) {
    let (&player_entity, &player_pos) = <(Entity, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next().unwrap();
    let player_idx = map_idx(player_pos.x, player_pos.y);

    let dijkstra_map = {
        let search_targets = vec![player_idx];
        DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0)
    };

    <(Entity, &Point, &AIState)>::query()
        .iter(ecs)
        .filter(|(_, _, &state)| state == AIState::ChasingPlayer)
        .for_each(|(&entity, &pos, _)| {
            let idx = map_idx(pos.x, pos.y);
            if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) {
                let distance_to_player = DistanceAlg::Pythagoras.distance2d(pos, player_pos);
                let destination = if distance_to_player > 1.2 {
                    map.index_to_point2d(destination)
                } else {
                    player_pos
                };

                let mut attacked = false;
                <(Entity, &Point)>::query()
                    .filter(component::<Health>())
                    .iter(ecs)
                    .filter(|(&victim, &target_pos)|
                        victim == player_entity && target_pos == destination)
                    .for_each(|(&victim, _)| {
                        attacked = true;
                        commands.add_component(entity, WantsToAttack(victim));
                    });
                if !attacked {
                    commands.add_component(entity, WantsToMove(destination));
                }
            }
        });
}
