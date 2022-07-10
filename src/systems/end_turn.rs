use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(AmuletOfYala)]
#[read_component(Point)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    // Get amulet position, if it exists
    // TODO probably better to pick it up and check if is in inventory
    let amulet_pos = <&Point>::query()
        .filter(component::<AmuletOfYala>())
        .iter(ecs)
        .cloned()
        .next();

    // Check if player is dead or stepped on amulet
    let (is_player_dead, has_amulet) = <(&Health, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .map(|(hp, pos)| (hp.current < 1, Some(*pos) == amulet_pos))
        .next().unwrap();

    // Advance state machine to next state
    *turn_state = match (is_player_dead, has_amulet, *turn_state) {
        (true, _, _) => TurnState::GameOver,
        (_, true, _) => TurnState::Victory,
        (_, _, TurnState::PlayerTurn) => TurnState::MonsterTurn,
        (_, _, TurnState::MonsterTurn) => TurnState::AwaitingInput,
        (_, _, _) => return,
    };
}
