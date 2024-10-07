use bevy::prelude::*;

use crate::{
    input::click_tile::click_tile_update_player_position,
    states::{game_state::GameState, pause_state::GamePauseState, turn_state::TurnState},
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            click_tile_update_player_position
                .run_if(in_state(GameState::Game))
                .run_if(in_state(TurnState::Player))
                .run_if(in_state(GamePauseState::Play)),
        );
    }
}
