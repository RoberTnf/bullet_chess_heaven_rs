use bevy::{prelude::*, utils::hashbrown::HashSet};

use crate::{
    board::{
        board_map::BoardMap,
        movement_types::{AttackTiles, MovementTypes},
        position::BoardPosition,
    },
    game_state::TurnState,
    pieces::player::Player,
};

use super::{attack::AttackEvent, update_position::UpdatePositionEvent};

#[derive(Event)]
pub struct TileClickedEvent {
    pub tile_pos: BoardPosition,
}

pub fn tile_clicked(
    mut player: Query<(Entity, &BoardPosition, &MovementTypes), With<Player>>,
    mut board_map: ResMut<BoardMap>,
    mut events: EventReader<TileClickedEvent>,
    mut move_event_writer: EventWriter<UpdatePositionEvent>,
    mut attack_event_writer: EventWriter<AttackEvent>,
    mut turn_state: ResMut<NextState<TurnState>>,
) {
    let (entity, board_position, movement_types) =
        player.get_single_mut().expect("0 or 2+ players");

    for event in events.read() {
        let possible_moves = board_map.get_possible_moves(&entity, movement_types, board_position);
        let moves = possible_moves.movement_tiles;
        let attacks: HashSet<BoardPosition> = possible_moves
            .attack_tiles
            .clone()
            .into_iter()
            .map(|(pos, _)| pos.0)
            .collect();

        if moves.contains(&event.tile_pos) {
            move_event_writer.send(UpdatePositionEvent {
                tile_pos: event.tile_pos,
                old_tile_pos: *board_position,
                piece: entity,
            });

            turn_state.set(TurnState::Enemy);
        } else if attacks.contains(&event.tile_pos) {
            let attacks_filtered: AttackTiles = possible_moves
                .attack_tiles
                .into_iter()
                .filter(|(pos, _)| pos.0 == event.tile_pos)
                .collect();
            attack_event_writer.send(AttackEvent {
                tile_pos: event.tile_pos,
                attacker: entity,
                attacks: attacks_filtered,
            });
            debug!("Attack event sent");
            turn_state.set(TurnState::Enemy);
        } else {
            debug!("Tried to move to tile that is not in movement tiles");
        }
    }
}
