use bevy::{prelude::*, utils::HashSet};

use crate::{
    board::position::BoardPosition,
    pieces::{
        common::{MovementTypes, Piece, Team},
        movement::MovePiece,
    },
    states::turn_state::TurnState,
};

pub fn ai_system(
    pieces: Query<(&BoardPosition, &MovementTypes, &Team, Entity), With<Piece>>,
    mut move_events: EventWriter<MovePiece>,
    mut turn_state: ResMut<NextState<TurnState>>,
) {
    let enemy_pieces = pieces.iter().filter(|(_, _, &team, _)| team == Team::Enemy);
    let player_pieces_positions = HashSet::from_iter(
        pieces
            .iter()
            .filter(|(_, _, &team, _)| team == Team::Player)
            .map(|(pos, _, _, _)| *pos),
    );
    let all_pieces_positions = HashSet::from_iter(pieces.iter().map(|(pos, _, _, _)| *pos));

    for (enemy_pos, enemy_movement_types, _, enemy_entity) in enemy_pieces {
        // we make the assumption that there will be enemies with more than one movement type
        let mut moves = HashSet::new();
        let mut attacks = HashSet::new();
        for movement_type in enemy_movement_types.0.iter() {
            let response = movement_type.get_valid_moves(
                enemy_pos,
                &all_pieces_positions,
                &player_pieces_positions,
            );
            moves.extend(response.valid_moves);
            attacks.extend(response.valid_attacks);
        }

        if attacks.is_empty() {
            if moves.is_empty() {
                continue;
            }
            // no attacks, so we move
            // we select the move that minimizes distance to any player piece
            let best_move = moves
                .iter()
                .min_by_key(|pos| {
                    player_pieces_positions
                        .iter()
                        .map(|player_pos| pos.distance(*player_pos))
                        .min()
                        .unwrap()
                })
                .unwrap();

            move_events.send(MovePiece {
                entity: enemy_entity,
                is_player: false,
                destination: *best_move,
            });
        } else {
            // we attack
        }
    }
    turn_state.set(TurnState::EnemyAnimation);
}
