use bevy::prelude::*;

use crate::{
    board::{board_map::BoardMap, position::BoardPosition},
    pieces::creature::Movable,
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
            let mut piece = board_positions
                .get_mut(event.piece)
                .expect("A UpdatePositionEvent was fired without a valid entity");
            board_map.move_entity(piece.clone(), event.tile_pos);
            piece.update(event.tile_pos.x, event.tile_pos.y);
        }
    }
}
