use crate::{
    globals::{self},
    pieces::movement_type::MovementType,
};
use once_cell::sync::Lazy;

use super::PieceInfo;

pub static WHITE_ROOK_INFO: Lazy<PieceInfo> = Lazy::new(|| PieceInfo {
    health: globals::ENEMY_BASE_HEALTH,
    damage: globals::ENEMY_BASE_DAMAGE,
    sprite_index: 15,
    movement_type: MovementType::Rook,
    spawn_weight: globals::ROOK_SPAWN_WEIGHT,
    spawn_turn: globals::ROOK_SPAWN_TURN,
    value: 5,
    name: "Rook".to_string(),
});

pub static BLACK_ROOK_INFO: Lazy<PieceInfo> = Lazy::new(|| PieceInfo {
    health: globals::ENEMY_BASE_HEALTH,
    damage: globals::ENEMY_BASE_DAMAGE,
    sprite_index: 16,
    movement_type: MovementType::Rook,
    spawn_weight: globals::ROOK_SPAWN_WEIGHT,
    spawn_turn: globals::ROOK_SPAWN_TURN,
    value: 5,
    name: "Rook".to_string(),
});
