use bevy::prelude::*;

use crate::{
    graphics,
    states::{game_state::GameState, pause_state::GamePauseState},
};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                graphics::animations::pulse::animate_pulse_scale,
                graphics::animations::death::animate_death,
            )
                .run_if(in_state(GameState::Game))
                .run_if(in_state(GamePauseState::Playing)),
        );
    }
}
