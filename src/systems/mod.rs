use crate::prelude::*;
mod map_render;
mod player_inputs;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_inputs::player_input_system())
        .add_system(map_render::map_render_system())
        .build()
}
