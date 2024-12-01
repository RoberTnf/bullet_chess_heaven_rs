use once_cell::sync::Lazy;
pub mod promotion;
use crate::{globals, pieces::movement_type::MovementType};

use super::PieceInfo;

pub static WHITE_PAWN_INFO: Lazy<PieceInfo> = Lazy::new(|| PieceInfo {
    health: globals::ENEMY_BASE_HEALTH,
    damage: globals::ENEMY_BASE_DAMAGE,
    sprite_index: 4,
    movement_type: MovementType::WhitePawn,
    spawn_weight: globals::PAWN_SPAWN_WEIGHT,
    spawn_turn: 1,
    value: 1,
    name: "White Pawn".to_string(),
});

pub static BLACK_PAWN_INFO: Lazy<PieceInfo> = Lazy::new(|| PieceInfo {
    health: globals::ENEMY_BASE_HEALTH,
    damage: globals::ENEMY_BASE_DAMAGE,
    sprite_index: 5,
    movement_type: MovementType::BlackPawn,
    spawn_weight: globals::PAWN_SPAWN_WEIGHT,
    spawn_turn: 1,
    value: 1,
    name: "Black Pawn".to_string(),
});
