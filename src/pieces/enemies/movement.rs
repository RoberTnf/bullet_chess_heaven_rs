use bevy::prelude::*;

use crate::{
    board::{board_map::BoardMap, movement_types::MovementTypes, position::BoardPosition},
    events::update_position::UpdatePositionEvent,
    game_state::TurnState,
    pieces::player::Player,
};

use super::Enemy;

pub fn enemy_movement(
    mut board_map: ResMut<BoardMap>,
    enemies: Query<(&BoardPosition, Entity, &MovementTypes), With<Enemy>>,
    player: Query<&BoardPosition, With<Player>>,
    mut event_writer: EventWriter<UpdatePositionEvent>,
    mut turn_state: ResMut<NextState<TurnState>>,
) {
    for (enemy_position, entity, movement_types) in &enemies {
        let possible_moves = board_map.get_possible_moves(&entity, movement_types, enemy_position);

        let player_position = player.get_single().expect("0 or 2+ player found");

        let closest_move = possible_moves
            .iter()
            .min_by_key(|pos| pos.distance_squared(player_position));

        if let Some(closest_move) = closest_move {
            event_writer.send(UpdatePositionEvent {
                tile_pos: *closest_move,
                old_tile_pos: *enemy_position,
                piece: entity,
            });
        }

        debug!("Enemy moved to {:?}", closest_move);
    }

    turn_state.set(TurnState::Player);
}
