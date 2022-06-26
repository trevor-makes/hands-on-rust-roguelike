mod player_input;
mod map_render;
mod entity_render;

use crate::prelude::*;
use player_input::player_input_system;
use map_render::map_render_system;
use entity_render::entity_render_system;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input_system())
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .build()
}
