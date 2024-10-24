use bevy::prelude::*;

use crate::{
    board::position::BoardPosition,
    pieces::{
        common::{MovementTypes, Piece},
        enemies::king::{BLACK_KING_INFO, WHITE_KING_INFO},
        experience::PieceValue,
        movement_type::MovementType,
    },
};

pub fn promotion_system(
    mut pieces: Query<
        (
            &mut MovementTypes,
            &BoardPosition,
            &mut PieceValue,
            &mut TextureAtlas,
        ),
        With<Piece>,
    >,
) {
    for (mut movement_types, pos, mut value, mut atlas) in pieces.iter_mut() {
        if pos.y == 7 && movement_types.0 == vec![MovementType::WhitePawn] {
            debug!("Promoting white pawn to black king");
            *movement_types = MovementTypes(vec![MovementType::King]);
            atlas.index = BLACK_KING_INFO.sprite_index;
            *value = PieceValue {
                value: BLACK_KING_INFO.value,
            };
        } else if pos.y == 0 && movement_types.0 == vec![MovementType::BlackPawn] {
            debug!("Promoting black pawn to white king");
            *movement_types = MovementTypes(vec![MovementType::King]);
            atlas.index = WHITE_KING_INFO.sprite_index;
            *value = PieceValue {
                value: WHITE_KING_INFO.value,
            };
        }
    }
}
