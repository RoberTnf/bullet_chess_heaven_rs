use bevy::prelude::*;
use bevy::utils::HashSet;

use crate::{
    board::{
        movement_types::{cache::RefreshCacheEvent, MovementType, MovementTypes},
        position::BoardPosition,
    },
    events::update_position::UpdatePositionEvent,
};

use super::{EnemyType, PieceConfig};

pub fn get_white_pawn_config() -> PieceConfig {
    PieceConfig {
        sprite_tile_id: 4,
        movement_set: MovementTypes(HashSet::from([MovementType::PawnWhite])),
    }
}

pub fn get_black_pawn_config() -> PieceConfig {
    PieceConfig {
        sprite_tile_id: 5,
        movement_set: MovementTypes(HashSet::from([MovementType::PawnBlack])),
    }
}

pub fn promote_pawn(
    mut commands: Commands,
    mut enemies: Query<(&EnemyType, &BoardPosition, &mut TextureAtlas, Entity)>,
) {
    for (enemy_type, position, mut texture_atlas, entity) in enemies.iter_mut() {
        match enemy_type {
            EnemyType::WhitePawn => {
                let config = get_black_pawn_config();
                if position.y == 7 {
                    debug!("Promoting white pawn into black pawn");
                    commands
                        .entity(entity)
                        .insert(EnemyType::BlackPawn)
                        .insert(config.movement_set);
                    texture_atlas.index = config.sprite_tile_id; // Update the sprite index
                }
            }
            EnemyType::BlackPawn => {
                let config = get_white_pawn_config();
                if position.y == 0 {
                    debug!("Promoting black pawn into white pawn");
                    commands
                        .entity(entity)
                        .insert(EnemyType::WhitePawn)
                        .insert(config.movement_set);
                    texture_atlas.index = config.sprite_tile_id; // Update the sprite index
                }
            }
        }
    }
}
