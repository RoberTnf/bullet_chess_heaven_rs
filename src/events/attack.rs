use crate::board::{movement_types::AttackTiles, position::BoardPosition};
use bevy::prelude::*;

#[derive(Event)]
pub struct AttackEvent {
    pub tile_pos: BoardPosition,
    pub attacker: Entity,
    pub attacks: AttackTiles,
}
