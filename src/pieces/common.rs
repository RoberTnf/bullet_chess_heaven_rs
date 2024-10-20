use bevy::prelude::*;

use crate::board::position::BoardPosition;

use super::{damage::Damage, health::Health, movement_type::MovementType};

#[derive(Component, Default)]
pub struct BlocksMovement;

#[derive(Component, Default)]
pub struct Piece;

#[derive(Component)]
pub enum PieceState {
    Idle,
    Moving { origin: Vec3, destination: Vec3 },
}

#[derive(Component)]
pub struct MovementTypes(pub Vec<MovementType>);

#[derive(Component, Eq, PartialEq, Copy, Clone)]
pub enum Team {
    Player,
    Enemy,
}

#[derive(Bundle)]
pub struct PieceBundle {
    pub sprite: SpriteBundle,
    pub atlas: TextureAtlas,
    pub creature: Piece,
    // pub movement_types: MovementTypes,
    pub board_position: BoardPosition,
    pub health: Health,
    pub damage: Damage,
    pub blocks_movement: BlocksMovement,
    pub state: PieceState,
    pub movement_types: MovementTypes,
    pub team: Team,
}
