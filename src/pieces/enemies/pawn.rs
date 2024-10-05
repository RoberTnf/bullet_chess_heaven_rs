use bevy::prelude::*;
use bevy::utils::HashSet;

use crate::{
    board::movement_types::{MovementType, MovementTypes},
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
    mut update_position_event: EventReader<UpdatePositionEvent>,
    mut enemies: Query<(&EnemyType, &mut TextureAtlas)>,
) {
    for event in update_position_event.read() {
        if let Ok((enemy_type, mut texture_atlas)) = enemies.get_mut(event.piece) {
            match enemy_type {
                EnemyType::WhitePawn => {
                    let config = get_black_pawn_config();
                    if event.tile_pos.y == 7 {
                        debug!("Promoting white pawn into black pawn");
                        commands
                            .entity(event.piece)
                            .insert(EnemyType::BlackPawn)
                            .insert(config.movement_set);
                        texture_atlas.index = config.sprite_tile_id; // Update the sprite index
                    }
                }
                EnemyType::BlackPawn => {
                    let config = get_white_pawn_config();
                    if event.tile_pos.y == 0 {
                        debug!("Promoting black pawn into white pawn");
                        commands
                            .entity(event.piece)
                            .insert(EnemyType::WhitePawn)
                            .insert(config.movement_set);
                        texture_atlas.index = config.sprite_tile_id; // Update the sprite index
                    }
                }
            }
        }
    }
}
