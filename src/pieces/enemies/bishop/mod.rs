use crate::{
    globals::{self},
    pieces::{common::MovementTypes, movement_type::MovementType},
};
use once_cell::sync::Lazy;

use super::PieceInfo;

pub static WHITE_BISHOP_INFO: Lazy<PieceInfo> = Lazy::new(|| PieceInfo {
    health: globals::ENEMY_BASE_HEALTH,
    damage: globals::ENEMY_BASE_DAMAGE,
    sprite_index: 13,
    movement_types: MovementTypes(vec![MovementType::Bishop]),
    spawn_weight: globals::BISHOP_SPAWN_WEIGHT,
    spawn_turn: globals::BISHOP_SPAWN_TURN,
    value: 3,
});

pub static BLACK_BISHOP_INFO: Lazy<PieceInfo> = Lazy::new(|| PieceInfo {
    health: globals::ENEMY_BASE_HEALTH,
    damage: globals::ENEMY_BASE_DAMAGE,
    sprite_index: 14,
    movement_types: MovementTypes(vec![MovementType::Bishop]),
    spawn_weight: globals::BISHOP_SPAWN_WEIGHT,
    spawn_turn: globals::BISHOP_SPAWN_TURN,
    value: 3,
});
