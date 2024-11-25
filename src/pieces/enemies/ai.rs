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

use super::spawn::AIControlled;

pub fn ai_system_enemy(
    pieces: Query<
        (
            &BoardPosition,
            &Upgrades,
            &Team,
            &Attack,
            Entity,
            &mut PieceState,
        ),
        (With<Piece>, Without<DeathAnimation>, With<AIControlled>),
    >,
    non_ai_pieces: Query<
        (
            &BoardPosition,
            &Upgrades,
            &Team,
            &Attack,
            Entity,
            &mut PieceState,
        ),
        (With<Piece>, Without<DeathAnimation>, Without<AIControlled>),
    >,
    move_events: EventWriter<MovePieceEvent>,
    mut turn_state: ResMut<NextState<TurnState>>,
    attack_events: EventWriter<AttackPieceEvent>,
) {
    execute_ai(
        pieces,
        attack_events,
        move_events,
        non_ai_pieces,
        Team::Enemy,
        Team::Player,
    );
    turn_state.set(TurnState::EnemyAnimation);
}

pub fn ai_system_player(
    pieces: Query<
        (
            &BoardPosition,
            &Upgrades,
            &Team,
            &Attack,
            Entity,
            &mut PieceState,
        ),
        (With<Piece>, Without<DeathAnimation>, With<AIControlled>),
    >,
    non_ai_pieces: Query<
        (
            &BoardPosition,
            &Upgrades,
            &Team,
            &Attack,
            Entity,
            &mut PieceState,
        ),
        (With<Piece>, Without<DeathAnimation>, Without<AIControlled>),
    >,
    move_events: EventWriter<MovePieceEvent>,
    mut turn_state: ResMut<NextState<TurnState>>,
    attack_events: EventWriter<AttackPieceEvent>,
) {
    execute_ai(
        pieces,
        attack_events,
        move_events,
        non_ai_pieces,
        Team::Player,
        Team::Enemy,
    );
    turn_state.set(TurnState::PlayerAnimationAI);
}

fn execute_ai(
    mut ai_pieces: Query<
        (
            &BoardPosition,
            &Upgrades,
            &Team,
            &Attack,
            Entity,
            &mut PieceState,
        ),
        (With<Piece>, Without<DeathAnimation>, With<AIControlled>),
    >,
    mut attack_events: EventWriter<'_, AttackPieceEvent>,
    mut move_events: EventWriter<'_, MovePieceEvent>,
    non_ai_pieces: Query<
        (
            &BoardPosition,
            &Upgrades,
            &Team,
            &Attack,
            Entity,
            &mut PieceState,
        ),
        (With<Piece>, Without<DeathAnimation>, Without<AIControlled>),
    >,
    allied_team: Team,
    enemy_team: Team,
) {
    let all_pieces: Vec<_> = ai_pieces.iter().chain(non_ai_pieces.iter()).collect();
    let mut all_pieces_positions =
        HashSet::from_iter(all_pieces.iter().map(|(&pos, _, _, _, _, _)| pos));

    let enemy_pieces_positions_and_entities = HashSet::from_iter(
        all_pieces
            .iter()
            .filter(|(_, _, &team, _, _, _)| team == enemy_team)
            .map(|(&pos, _, _, _, entity, _)| (*entity, pos)),
    );

    let allied_pieces = ai_pieces
        .iter_mut()
        .filter(|(_, _, &team, _, _, _)| team == allied_team);
    let enemy_pieces_positions = HashSet::from_iter(
        enemy_pieces_positions_and_entities
            .iter()
            .map(|(_, pos)| *pos),
    );
    for (ally_pos, ally_upgrades, _, ally_damage, ally_entity, mut ally_state) in allied_pieces {
        let mut has_attacked = false;
        let enemy_movement_types = ally_upgrades.get_movement_types_set();
        // we make the assumption that there will be enemies with more than one movement type
        let mut moves = HashSet::new();
        for movement_type in enemy_movement_types.iter() {
            let response = movement_type.get_valid_moves(
                ally_pos,
                &all_pieces_positions,
                &enemy_pieces_positions,
            );
            moves.extend(response.valid_moves);
            for attack in response.valid_attacks {
                attack_events.send(AttackPieceEvent {
                    attacker: ally_entity,
                    target: enemy_pieces_positions_and_entities
                        .iter()
                        .find(|(_, pos)| *pos == attack)
                        .unwrap()
                        .0,
                    damage: ally_damage.0.upgraded_value,
                    destination: attack,
                    sprite_index: None,
                    delay: None,
                    movement_type: movement_type.clone(),
                    // for now, we don't want to apply unique upgrades to the AI
                    with_unique_upgrade: false,
                    origin: *ally_pos,
                });
                has_attacked = true;
            }
        }

        if !has_attacked {
            if moves.is_empty() {
                *ally_state = PieceState::AttackEnded;
                continue;
            }
            // no attacks, so we move
            // we select the move that enables a potential attack next turn or minimizes distance
            let best_move = moves
                .iter()
                .min_by_key(|pos| {
                    let enables_attack = enemy_movement_types.iter().any(|movement_type| {
                        !movement_type
                            .get_valid_moves(pos, &all_pieces_positions, &enemy_pieces_positions)
                            .valid_attacks
                            .is_empty()
                    });

                    if enables_attack {
                        0
                    } else {
                        enemy_pieces_positions
                            .iter()
                            .map(|player_pos| pos.distance(*player_pos))
                            .min()
                            .unwrap()
                    }
                })
                .expect("There should be at least one valid move");

            // mark origin as available, remove destination
            all_pieces_positions.insert(*best_move);
            all_pieces_positions.remove(ally_pos);
            move_events.send(MovePieceEvent {
                entity: ally_entity,
                destination: *best_move,
            });
        }
    }
}
