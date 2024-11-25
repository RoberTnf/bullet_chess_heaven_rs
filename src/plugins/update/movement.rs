use bevy::prelude::*;

use crate::{
    pieces::movement::{
        all_enemies_idle, all_player_ai_idle, is_player_idle, move_piece, move_pieces_animation,
        MovePieceAnimationEndEvent,
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
                (
                    move_pieces_animation,
                    all_enemies_idle.run_if(in_state(TurnState::EnemyAnimation)),
                    is_player_idle.run_if(in_state(TurnState::PlayerAnimation)),
                    all_player_ai_idle.run_if(in_state(TurnState::PlayerAnimationAI)),
                )
                    .after(move_piece)
                    .run_if(in_state(GameState::Game))
                    .run_if(in_state(GamePauseState::Playing)),
            ),
        );
        app.add_event::<MovePieceAnimationEndEvent>();
    }
}
