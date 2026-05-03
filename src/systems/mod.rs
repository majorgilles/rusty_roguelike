use crate::prelude::*;
mod player_inputs;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_inputs::player_input_system())
        .build()
}
