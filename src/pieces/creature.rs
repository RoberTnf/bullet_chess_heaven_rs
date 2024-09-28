use bevy::prelude::*;

use crate::board::{movement_types::MovementTypes, position::BoardPosition};

#[derive(Component, Default)]
pub struct BlocksMovement;

#[derive(Component, Default)]
pub struct Creature;

#[derive(Bundle)]
pub struct CreatureBundle {
    pub sprite: SpriteBundle,
    pub atlas: TextureAtlas,
    pub blocks_movement: BlocksMovement,
    pub creature: Creature,
    pub movement_types: MovementTypes,
    pub board_position: BoardPosition,
}
