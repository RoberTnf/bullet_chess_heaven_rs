use bevy::{prelude::*, utils::HashSet};

use crate::{
    board::position::BoardPosition,
    pieces::{
        attack::AttackPieceEvent,
        common::{MovementTypes, Piece, Team},
        damage::Damage,
        health::DeathAnimation,
        movement::MovePieceEvent,
    },
    states::turn_state::TurnState,
};

pub fn ai_system(
    pieces: Query<
        (&BoardPosition, &MovementTypes, &Team, &Damage, Entity),
        (With<Piece>, Without<DeathAnimation>),
    >,
    mut move_events: EventWriter<MovePieceEvent>,
    mut turn_state: ResMut<NextState<TurnState>>,
    mut attack_events: EventWriter<AttackPieceEvent>,
) {
    let enemy_pieces = pieces
        .iter()
        .filter(|(_, _, &team, _, _)| team == Team::Enemy);
    let player_pieces_positions_and_entities = HashSet::from_iter(
        pieces
            .iter()
            .filter(|(_, _, &team, _, _)| team == Team::Player)
            .map(|(pos, _, _, _, entity)| (entity, *pos)),
    );
    let player_pieces_positions = HashSet::from_iter(
        player_pieces_positions_and_entities
            .iter()
            .map(|(_, pos)| *pos),
    );
    let mut all_pieces_positions = HashSet::from_iter(pieces.iter().map(|(pos, _, _, _, _)| *pos));
    for (enemy_pos, enemy_movement_types, _, enemy_damage, enemy_entity) in enemy_pieces {
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
            // we select the move that enables a potential attack next turn or minimizes distance
            let best_move = moves
                .iter()
                .min_by_key(|pos| {
                    let enables_attack = enemy_movement_types.0.iter().any(|movement_type| {
                        !movement_type
                            .get_valid_moves(pos, &all_pieces_positions, &player_pieces_positions)
                            .valid_attacks
                            .is_empty()
                    });

                    if enables_attack {
                        0
                    } else {
                        player_pieces_positions
                            .iter()
                            .map(|player_pos| pos.distance(*player_pos))
                            .min()
                            .unwrap()
                    }
                })
                .expect("There should be at least one valid move");

            // mark origin as available, remove destination
            all_pieces_positions.insert(*best_move);
            all_pieces_positions.remove(enemy_pos);
            move_events.send(MovePieceEvent {
                entity: enemy_entity,
                destination: *best_move,
            });
        } else {
            let attack_position = attacks.iter().next().unwrap();
            attack_events.send(AttackPieceEvent {
                attacker: enemy_entity,
                target: player_pieces_positions_and_entities
                    .iter()
                    .find(|(_, pos)| pos == attack_position)
                    .unwrap()
                    .0,
                damage: enemy_damage.value,
                destination: *attack_position,
                sprite_index: None,
                delay: None,
            });
        }
    }
    turn_state.set(TurnState::EnemyAnimation);
}
