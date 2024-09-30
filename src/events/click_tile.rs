use bevy::prelude::*;

use crate::{
    board::{board_map::BoardMap, movement_types::MovementTypes, position::BoardPosition},
    pieces::player::Player,
};

use super::update_pos::UpdatePositionEvent;

#[derive(Event)]
pub struct TileClickedEvent {
    pub tile_pos: BoardPosition,
}

pub fn tile_clicked(
    mut player: Query<(Entity, &BoardPosition), With<Player>>,
    board_map: Res<BoardMap>,
    mut events: EventReader<TileClickedEvent>,
    mut events_writer: EventWriter<UpdatePositionEvent>,
) {
    let (entity, board_position) = player.get_single_mut().expect("0 or 2+ players");

    for event in events.read() {
        let movement_tiles = board_map
            .possible_moves_cache
            .get_movement_tiles(board_position);

        if movement_tiles.contains(&event.tile_pos) {
            events_writer.send(UpdatePositionEvent {
                tile_pos: event.tile_pos,
                old_tile_pos: *board_position,
                piece: entity,
            });
        } else {
            debug!("Tried to move to tile that is not in movement tiles");
        }
    }
}
