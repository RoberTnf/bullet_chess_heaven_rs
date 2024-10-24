use once_cell::sync::Lazy;
pub mod promotion;
use crate::{
    globals,
    pieces::{common::MovementTypes, movement_type::MovementType},
};

use super::PieceInfo;

pub static WHITE_PAWN_INFO: Lazy<PieceInfo> = Lazy::new(|| PieceInfo {
    health: globals::ENEMY_BASE_HEALTH,
    damage: globals::ENEMY_BASE_DAMAGE,
    sprite_index: 4,
    movement_types: MovementTypes(vec![MovementType::WhitePawn]),
    spawn_weight: globals::PAWN_SPAWN_WEIGHT,
    spawn_turn: 1,
    value: 1,
});

pub static BLACK_PAWN_INFO: Lazy<PieceInfo> = Lazy::new(|| PieceInfo {
    health: globals::ENEMY_BASE_HEALTH,
    damage: globals::ENEMY_BASE_DAMAGE,
    sprite_index: 5,
    movement_types: MovementTypes(vec![MovementType::BlackPawn]),
    spawn_weight: globals::PAWN_SPAWN_WEIGHT,
    spawn_turn: 1,
    value: 1,
});
