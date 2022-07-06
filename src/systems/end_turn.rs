use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    let is_player_dead = <&Health>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .any(|hp| hp.current < 1);
    *turn_state = match (is_player_dead, *turn_state) {
        (true, _) => TurnState::GameOver,
        (_, TurnState::PlayerTurn) => TurnState::MonsterTurn,
        (_, TurnState::MonsterTurn) => TurnState::AwaitingInput,
        (_, _) => return,
    };
}
