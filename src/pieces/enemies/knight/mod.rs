use crate::{
    globals::{self},
    pieces::movement_type::MovementType,
};
use once_cell::sync::Lazy;

use super::PieceInfo;

pub static WHITE_KNIGHT_INFO: Lazy<PieceInfo> = Lazy::new(|| PieceInfo {
    health: globals::ENEMY_BASE_HEALTH,
    damage: globals::ENEMY_BASE_DAMAGE,
    sprite_index: 11,
    movement_type: MovementType::Knight,
    spawn_weight: globals::KNIGHT_SPAWN_WEIGHT,
    spawn_turn: globals::KNIGHT_SPAWN_TURN,
    value: 3,
    name: "Knight".to_string(),
});

pub static BLACK_KNIGHT_INFO: Lazy<PieceInfo> = Lazy::new(|| PieceInfo {
    health: globals::ENEMY_BASE_HEALTH,
    damage: globals::ENEMY_BASE_DAMAGE,
    sprite_index: 12,
    movement_type: MovementType::Knight,
    spawn_weight: globals::KNIGHT_SPAWN_WEIGHT,
    spawn_turn: globals::KNIGHT_SPAWN_TURN,
    value: 3,
    name: "Knight".to_string(),
});
