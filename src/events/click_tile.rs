use bevy::prelude::*;

use crate::{board::position::BoardPosition, pieces::player::Player};

use super::update_pos::UpdatePositionEvent;

#[derive(Event)]
pub struct TileClickedEvent {
    pub tile_pos: BoardPosition,
}

pub fn tile_clicked(
    mut player: Query<Entity, With<Player>>,
    mut events: EventReader<TileClickedEvent>,
    mut events_writer: EventWriter<UpdatePositionEvent>,
) {
    let entity = player.get_single_mut().expect("0 or 2+ players");

    for event in events.read() {
        events_writer.send(UpdatePositionEvent {
            tile_pos: event.tile_pos,
            piece: entity,
        });
    }
}
