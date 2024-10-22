use bevy::prelude::*;

use crate::{
    board::position::BoardPosition,
    pieces::{
        common::{MovementTypes, Piece},
        movement_type::MovementType,
    },
};

use super::{BLACK_PAWN_INFO, WHITE_PAWN_INFO};

pub fn promotion_system(
    mut pieces: Query<(&mut MovementTypes, &BoardPosition, &mut TextureAtlas), With<Piece>>,
) {
    for (mut movement_types, pos, mut atlas) in pieces.iter_mut() {
        if pos.y == 7 && movement_types.0 == vec![MovementType::WhitePawn] {
            debug!("Promoting white pawn to black pawn");
            *movement_types = MovementTypes(vec![MovementType::BlackPawn]);
            atlas.index = BLACK_PAWN_INFO.sprite_index;
        } else if pos.y == 0 && movement_types.0 == vec![MovementType::BlackPawn] {
            debug!("Promoting black pawn to white pawn");
            *movement_types = MovementTypes(vec![MovementType::WhitePawn]);
            atlas.index = WHITE_PAWN_INFO.sprite_index;
        }
    }
}
