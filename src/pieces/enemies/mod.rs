use bevy::prelude::*;
use pawn::promotion::promotion_system;

use crate::states::{game_state::GameState, pause_state::GamePauseState, turn_state::TurnState};

use super::common::MovementTypes;
pub mod ai;
pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;
pub mod spawn;

#[derive(Component)]
pub struct Enemy;

#[derive(Clone)]
pub struct PieceInfo {
    pub health: usize,
    pub damage: usize,
    pub sprite_index: usize,
    pub movement_types: MovementTypes,
    pub spawn_weight: f32,
    pub spawn_turn: usize,
    pub value: usize,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn::spawn_enemies, promotion_system)
                .run_if(in_state(GameState::Game))
                .run_if(in_state(TurnState::EnemySpawn))
                .run_if(in_state(GamePauseState::Playing)),
        );
        app.add_systems(
            Update,
            ai::ai_system
                .run_if(in_state(TurnState::EnemyAI))
                .run_if(in_state(GamePauseState::Playing))
                .run_if(in_state(GameState::Game)),
        );
    }
}
