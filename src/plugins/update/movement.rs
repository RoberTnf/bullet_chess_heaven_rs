use bevy::prelude::*;

use crate::{
    pieces::movement::{move_piece, move_piece_animation},
    states::{game_state::GameState, pause_state::GamePauseState},
};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_piece, move_piece_animation)
                .run_if(in_state(GameState::Game))
                .run_if(in_state(GamePauseState::Play)),
        );
    }
}
