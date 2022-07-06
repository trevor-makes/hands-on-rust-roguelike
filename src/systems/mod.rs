mod player_input;
mod map_render;
mod entity_render;
mod hud;
mod tooltips;
mod random_move;
mod chasing;
mod movement;
mod combat;
mod end_turn;

use crate::prelude::*;
use player_input::player_input_system;
use map_render::map_render_system;
use entity_render::entity_render_system;
use hud::hud_system;
use tooltips::tooltips_system;
use random_move::random_move_system;
use chasing::chasing_system;
use movement::movement_system;
use combat::combat_system;
use end_turn::end_turn_system;

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input_system())
        .flush()
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .add_system(hud_system())
        .add_system(tooltips_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(combat_system())
        .flush()
        .add_system(movement_system())
        .flush()
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .add_system(hud_system())
        .add_system(end_turn_system())
        .build()
}

pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_move_system())
        .add_system(chasing_system())
        .flush()
        .add_system(combat_system())
        .flush()
        .add_system(movement_system())
        .flush()
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .add_system(hud_system())
        .add_system(end_turn_system())
        .build()
}
