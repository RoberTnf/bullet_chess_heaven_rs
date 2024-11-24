use bevy::{prelude::*, utils::HashSet};

use crate::{
    board::position::BoardPosition,
    pieces::{
        attack::AttackPieceEvent,
        common::{Piece, PieceState, Team},
        damage::Attack,
        health::DeathAnimation,
        movement::MovePieceEvent,
        player::upgrades::data::Upgrades,
    },
    states::turn_state::TurnState,
};

pub fn ai_system(
    mut pieces: Query<
        (
            &BoardPosition,
            &Upgrades,
            &Team,
            &Attack,
            Entity,
            &mut PieceState,
        ),
        (With<Piece>, Without<DeathAnimation>),
    >,
    mut move_events: EventWriter<MovePieceEvent>,
    mut turn_state: ResMut<NextState<TurnState>>,
    mut attack_events: EventWriter<AttackPieceEvent>,
) {
    let player_pieces_positions_and_entities = HashSet::from_iter(
        pieces
            .iter()
            .filter(|(_, _, &team, _, _, _)| team == Team::Player)
            .map(|(pos, _, _, _, entity, _)| (entity, *pos)),
    );
    let mut all_pieces_positions =
        HashSet::from_iter(pieces.iter().map(|(pos, _, _, _, _, _)| *pos));
    let enemy_pieces = pieces
        .iter_mut()
        .filter(|(_, _, &team, _, _, _)| team == Team::Enemy);
    let player_pieces_positions = HashSet::from_iter(
        player_pieces_positions_and_entities
            .iter()
            .map(|(_, pos)| *pos),
    );
    for (enemy_pos, enemy_upgrades, _, enemy_damage, enemy_entity, mut enemy_state) in enemy_pieces
    {
        let mut has_attacked = false;
        let enemy_movement_types = enemy_upgrades.get_movement_types_set();
        // we make the assumption that there will be enemies with more than one movement type
        let mut moves = HashSet::new();
        for movement_type in enemy_movement_types.iter() {
            let response = movement_type.get_valid_moves(
                enemy_pos,
                &all_pieces_positions,
                &player_pieces_positions,
            );
            moves.extend(response.valid_moves);
            for attack in response.valid_attacks {
                attack_events.send(AttackPieceEvent {
                    attacker: enemy_entity,
                    target: player_pieces_positions_and_entities
                        .iter()
                        .find(|(_, pos)| *pos == attack)
                        .unwrap()
                        .0,
                    damage: enemy_damage.0.upgraded_value,
                    destination: attack,
                    sprite_index: None,
                    delay: None,
                    movement_type: movement_type.clone(),
                    // for now, we don't want to apply unique upgrades to the AI
                    with_unique_upgrade: false,
                });
                has_attacked = true;
            }
        }

        if !has_attacked {
            if moves.is_empty() {
                *enemy_state = PieceState::AttackEnded;
                continue;
            }
            // no attacks, so we move
            // we select the move that enables a potential attack next turn or minimizes distance
            let best_move = moves
                .iter()
                .min_by_key(|pos| {
                    let enables_attack = enemy_movement_types.iter().any(|movement_type| {
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
        }
    }
    turn_state.set(TurnState::EnemyAnimation);
}
