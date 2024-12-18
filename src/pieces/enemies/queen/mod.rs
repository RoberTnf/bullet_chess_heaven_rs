use crate::{
    globals::{self},
    pieces::movement_type::MovementType,
};
use once_cell::sync::Lazy;

use super::PieceInfo;

pub static WHITE_QUEEN_INFO: Lazy<PieceInfo> = Lazy::new(|| PieceInfo {
    health: globals::ENEMY_BASE_HEALTH,
    damage: globals::ENEMY_BASE_DAMAGE,
    sprite_index: 17,
    movement_type: MovementType::Queen,
    spawn_weight: globals::QUEEN_SPAWN_WEIGHT,
    spawn_turn: globals::QUEEN_SPAWN_TURN,
    value: 9,
    name: "Queen".to_string(),
});

pub static BLACK_QUEEN_INFO: Lazy<PieceInfo> = Lazy::new(|| PieceInfo {
    health: globals::ENEMY_BASE_HEALTH,
    damage: globals::ENEMY_BASE_DAMAGE,
    sprite_index: 18,
    movement_type: MovementType::Queen,
    spawn_weight: globals::QUEEN_SPAWN_WEIGHT,
    spawn_turn: globals::QUEEN_SPAWN_TURN,
    value: 9,
    name: "Queen".to_string(),
});
