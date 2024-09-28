use bevy::prelude::*;

use crate::board::{board_map::BoardMap, position::BoardPosition};

#[derive(Event)]
pub struct UpdatePositionEvent {
    pub tile_pos: BoardPosition,
    pub piece: Entity,
}

pub fn update_position(
    mut events: EventReader<UpdatePositionEvent>,
    mut board_positions: Query<&mut BoardPosition>,
    mut board_map: ResMut<BoardMap>,
) {
    for event in events.read() {
        if board_map.is_movable(event.tile_pos) {
            let mut piece_pos = board_positions
                .get_mut(event.piece)
                .expect("A UpdatePositionEvent was fired without a valid entity");

            board_map.move_entity(piece_pos.clone(), event.tile_pos);
            piece_pos.update(event.tile_pos.x, event.tile_pos.y);
        }
    }
}
