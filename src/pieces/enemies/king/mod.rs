use crate::{
    globals::{self, KING_SPAWN_TURN},
    pieces::{common::MovementTypes, movement_type::MovementType},
};
use once_cell::sync::Lazy;

use super::PieceInfo;

pub static WHITE_KING_INFO: Lazy<PieceInfo> = Lazy::new(|| PieceInfo {
    health: globals::ENEMY_BASE_HEALTH,
    damage: globals::ENEMY_BASE_DAMAGE,
    sprite_index: 9,
    movement_types: MovementTypes(vec![MovementType::King]),
    spawn_weight: 1.0,
    spawn_turn: KING_SPAWN_TURN,
});

pub static BLACK_KING_INFO: Lazy<PieceInfo> = Lazy::new(|| PieceInfo {
    health: globals::ENEMY_BASE_HEALTH,
    damage: globals::ENEMY_BASE_DAMAGE,
    sprite_index: 10,
    movement_types: MovementTypes(vec![MovementType::King]),
    spawn_weight: 1.0,
    spawn_turn: KING_SPAWN_TURN,
});