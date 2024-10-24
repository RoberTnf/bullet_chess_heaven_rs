use crate::{
    globals::{self, KING_SPAWN_TURN, KING_SPAWN_WEIGHT},
    pieces::{common::MovementTypes, movement_type::MovementType},
};
use once_cell::sync::Lazy;

use super::PieceInfo;

pub static WHITE_KING_INFO: Lazy<PieceInfo> = Lazy::new(|| PieceInfo {
    health: globals::ENEMY_BASE_HEALTH,
    damage: globals::ENEMY_BASE_DAMAGE,
    sprite_index: 9,
    movement_types: MovementTypes(vec![MovementType::King]),
    spawn_weight: KING_SPAWN_WEIGHT,
    spawn_turn: KING_SPAWN_TURN,
    value: 3,
});

pub static BLACK_KING_INFO: Lazy<PieceInfo> = Lazy::new(|| PieceInfo {
    health: globals::ENEMY_BASE_HEALTH,
    damage: globals::ENEMY_BASE_DAMAGE,
    sprite_index: 10,
    movement_types: MovementTypes(vec![MovementType::King]),
    spawn_weight: KING_SPAWN_WEIGHT,
    spawn_turn: KING_SPAWN_TURN,
    value: 3,
});
