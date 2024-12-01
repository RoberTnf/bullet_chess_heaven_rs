use bevy::prelude::*;

use crate::board::position::BoardPosition;

use super::{
    attack::AttackPieceAnimationState,
    damage::Attack,
    health::{Health, PieceHealthChangeEvent},
    player::upgrades::{data::Upgrades, unique_upgrades::block::Block},
};

#[derive(Component, Default)]
#[require(
    Sprite,
    BoardPosition,
    Health,
    Attack,
    PieceState,
    Upgrades,
    Team,
    Block
)]
pub struct Piece;

#[derive(Component, Clone, Default)]
pub enum PieceState {
    #[default]
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

#[derive(Component, Eq, PartialEq, Copy, Clone, Hash, Default)]
pub enum Team {
    #[default]
    Player,
    Enemy,
}
