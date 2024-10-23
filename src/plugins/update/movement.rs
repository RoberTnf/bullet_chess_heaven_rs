use bevy::prelude::*;

use crate::{
    pieces::movement::{all_enemies_idle, move_piece, move_pieces_animation, player_idle},
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
                (
                    move_pieces_animation,
                    all_enemies_idle.run_if(in_state(TurnState::EnemyAnimation)),
                    player_idle.run_if(in_state(TurnState::PlayerAnimation)),
                )
                    .after(move_piece)
                    .run_if(in_state(GameState::Game))
                    .run_if(in_state(GamePauseState::Playing)),
            ),
        );
    }
}
