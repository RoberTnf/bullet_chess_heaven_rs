use bevy::prelude::*;

use crate::board;
use crate::game_state::{GamePauseState, GameState};
use crate::pieces;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (board::tile::spawn_board, pieces::player::spawn_player),
        )
        .add_systems(Startup, board::tile::spawn_board)
        .insert_state(GamePauseState::Play)
        .insert_state(GameState::Game);
    }
}
