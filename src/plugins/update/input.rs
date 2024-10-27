use bevy::prelude::*;

use crate::{
    input::click_tile::{click_tile_update_player_position, update_hovered_tile, HoveredTile},
    states::{game_state::GameState, pause_state::GamePauseState, turn_state::TurnState},
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (click_tile_update_player_position, update_hovered_tile)
                .run_if(in_state(GameState::Game))
                .run_if(in_state(TurnState::PlayerInput))
                .run_if(in_state(GamePauseState::Playing)),
        );
        app.insert_resource(HoveredTile(None));
    }
}
