use crate::{
    globals::{self},
    pieces::{common::MovementTypes, movement_type::MovementType},
};
use once_cell::sync::Lazy;

use super::PieceInfo;

pub static WHITE_ROOK_INFO: Lazy<PieceInfo> = Lazy::new(|| PieceInfo {
    health: globals::ENEMY_BASE_HEALTH,
    damage: globals::ENEMY_BASE_DAMAGE,
    sprite_index: 15,
    movement_types: MovementTypes(vec![MovementType::Rook]),
    spawn_weight: globals::ROOK_SPAWN_WEIGHT,
    spawn_turn: globals::ROOK_SPAWN_TURN,
});

pub static BLACK_ROOK_INFO: Lazy<PieceInfo> = Lazy::new(|| PieceInfo {
    health: globals::ENEMY_BASE_HEALTH,
    damage: globals::ENEMY_BASE_DAMAGE,
    sprite_index: 16,
    movement_types: MovementTypes(vec![MovementType::Rook]),
    spawn_weight: globals::ROOK_SPAWN_WEIGHT,
    spawn_turn: globals::ROOK_SPAWN_TURN,
});