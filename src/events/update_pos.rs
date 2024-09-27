use bevy::prelude::*;

use crate::{
    board::{board_map::BoardMap, position::BoardPosition},
    pieces::player::Movable,
};

#[derive(Event)]
pub struct UpdatePositionEvent {
    pub tile_pos: BoardPosition,
    pub piece: Entity,
}

pub fn update_position(
    mut events: EventReader<UpdatePositionEvent>,
    mut board_positions: Query<&mut BoardPosition, With<Movable>>,
    mut board_map: ResMut<BoardMap>,
) {
    for event in events.read() {
        if board_map.is_movable(event.tile_pos) {
            let mut piece = board_positions.get_mut(event.piece).unwrap();
            board_map.remove_entity(BoardPosition::new(piece.x, piece.y));
            piece.update(event.tile_pos.x, event.tile_pos.y);
            board_map.add_entity(event.tile_pos, event.piece);
        }
    }
}
