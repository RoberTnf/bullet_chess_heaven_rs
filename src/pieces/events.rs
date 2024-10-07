use crate::board::position::BoardPosition;
use bevy::prelude::*;

#[derive(Event)]
pub struct UpdatePositionEvent {
    pub tile_position: BoardPosition,
    pub old_tile_position: BoardPosition,
    pub piece: Entity,
}
