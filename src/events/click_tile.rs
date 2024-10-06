use bevy::prelude::*;

use crate::{
    board::{board_map::BoardMap, movement_types::MovementTypes, position::BoardPosition},
    game_state::TurnState,
    pieces::player::Player,
};

use super::update_position::UpdatePositionEvent;

#[derive(Event)]
pub struct TileClickedEvent {
    pub tile_pos: BoardPosition,
}

pub fn tile_clicked(
    mut player: Query<(Entity, &BoardPosition, &MovementTypes), With<Player>>,
    mut board_map: ResMut<BoardMap>,
    mut events: EventReader<TileClickedEvent>,
    mut events_writer: EventWriter<UpdatePositionEvent>,
    mut turn_state: ResMut<NextState<TurnState>>,
) {
    let (entity, board_position, movement_types) =
        player.get_single_mut().expect("0 or 2+ players");

    for event in events.read() {
        let movement_tiles = board_map
            .get_possible_moves(&entity, movement_types, board_position)
            .movement_tiles;

        if movement_tiles.contains(&event.tile_pos) {
            events_writer.send(UpdatePositionEvent {
                tile_pos: event.tile_pos,
                old_tile_pos: *board_position,
                piece: entity,
            });

            turn_state.set(TurnState::Enemy);
        } else {
            debug!("Tried to move to tile that is not in movement tiles");
        }
    }
}
