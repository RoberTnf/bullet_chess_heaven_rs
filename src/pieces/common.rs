use bevy::prelude::*;

use crate::board::position::BoardPosition;

use super::{
    attack::AttackPieceAnimationState,
    damage::Attack,
    health::{Health, PieceHealthChangeEvent},
    player::upgrades::data::Upgrades,
};

#[derive(Component, Default)]
pub struct BlocksMovement;

#[derive(Component, Default)]
pub struct Piece;

#[derive(Component, Clone)]
pub enum PieceState {
    Idle,
    Moving {
        origin: Vec3,
        destination: Vec3,
    },
    MoveEnded,
    AttackEnded,
    Attacking {
        destination: BoardPosition,
        origin: BoardPosition,
        animation_state: AttackPieceAnimationState,
        event: PieceHealthChangeEvent,
    },
    AttackingWithNewSprite,
}

#[derive(Component, Eq, PartialEq, Copy, Clone, Hash)]
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
    pub damage: Attack,
    pub blocks_movement: BlocksMovement,
    pub state: PieceState,
    pub upgrades: Upgrades,
    pub team: Team,
}
