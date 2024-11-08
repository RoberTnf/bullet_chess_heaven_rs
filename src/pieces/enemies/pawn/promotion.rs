use bevy::prelude::*;

use crate::{
    board::position::BoardPosition,
    pieces::{
        common::Piece,
        enemies::king::{BLACK_KING_INFO, WHITE_KING_INFO},
        movement_type::MovementType,
        player::{
            experience::PieceValue,
            upgrades::data::{get_movement_upgrade, Upgrades},
        },
    },
};

pub fn promotion_system(
    mut pieces: Query<
        (
            &mut Upgrades,
            &BoardPosition,
            &mut PieceValue,
            &mut TextureAtlas,
        ),
        With<Piece>,
    >,
) {
    for (mut upgrades, pos, mut value, mut atlas) in pieces.iter_mut() {
        let movement_types = upgrades.get_movement_types_set();
        let is_white_pawn =
            movement_types.contains(&MovementType::WhitePawn) && movement_types.len() == 1;
        let is_black_pawn =
            movement_types.contains(&MovementType::BlackPawn) && movement_types.len() == 1;
        if pos.y == 7 && is_white_pawn {
            debug!("Promoting white pawn to black king");
            *upgrades = Upgrades(
                upgrades
                    .0
                    .iter()
                    .map(|u| {
                        if *u == get_movement_upgrade(&MovementType::WhitePawn) {
                            get_movement_upgrade(&MovementType::King)
                        } else {
                            u.clone()
                        }
                    })
                    .collect(),
            );
            atlas.index = BLACK_KING_INFO.sprite_index;
            *value = PieceValue {
                value: BLACK_KING_INFO.value,
            };
        } else if pos.y == 0 && is_black_pawn {
            debug!("Promoting black pawn to white king");
            *upgrades = Upgrades(
                upgrades
                    .0
                    .iter()
                    .map(|u| {
                        if *u == get_movement_upgrade(&MovementType::BlackPawn) {
                            get_movement_upgrade(&MovementType::King)
                        } else {
                            u.clone()
                        }
                    })
                    .collect(),
            );
            atlas.index = WHITE_KING_INFO.sprite_index;
            *value = PieceValue {
                value: WHITE_KING_INFO.value,
            };
        }
    }
}
