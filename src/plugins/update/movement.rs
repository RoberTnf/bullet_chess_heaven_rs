use bevy::prelude::*;

use crate::{
    pieces::movement::{
        all_enemies_moved, move_enemies_animation, move_piece, move_player_animation,
    },
    states::{game_state::GameState, pause_state::GamePauseState, turn_state::TurnState},
};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                move_piece
                    .run_if(in_state(GameState::Game))
                    .run_if(in_state(GamePauseState::Playing)),
                move_player_animation
                    .run_if(in_state(TurnState::PlayerAnimation))
                    .run_if(in_state(GameState::Game))
                    .run_if(in_state(GamePauseState::Playing)),
                (move_enemies_animation, all_enemies_moved)
                    .after(move_piece)
                    .run_if(in_state(TurnState::EnemyAnimation))
                    .run_if(in_state(GameState::Game))
                    .run_if(in_state(GamePauseState::Playing)),
            ),
        );
    }
}
