use once_cell::sync::Lazy;
pub mod promotion;
use crate::{
    globals,
    pieces::{common::MovementTypes, movement_type::MovementType},
};

use super::PieceInfo;

pub static WHITE_PAWN_INFO: Lazy<PieceInfo> = Lazy::new(|| PieceInfo {
    health: globals::PAWN_HEALTH,
    damage: globals::PAWN_DAMAGE,
    sprite_index: 4,
    movement_types: MovementTypes(vec![MovementType::WhitePawn]),
    spawn_weight: 1.0,
});

pub static BLACK_PAWN_INFO: Lazy<PieceInfo> = Lazy::new(|| PieceInfo {
    health: globals::PAWN_HEALTH,
    damage: globals::PAWN_DAMAGE,
    sprite_index: 5,
    movement_types: MovementTypes(vec![MovementType::BlackPawn]),
    spawn_weight: 1.0,
});
