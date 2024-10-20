use bevy::{prelude::*, utils::HashSet};

use crate::{
    board::{highlight::HighlightCache, position::BoardPosition},
    globals::TWEEN_MOVE_ANIMATION_SPEED,
    states::turn_state::TurnState,
};

use super::{
    common::{MovementTypes, Piece, PieceState, Team},
    player::spawn::Player,
};

#[derive(Event)]
pub struct MovePiece {
    pub destination: BoardPosition,
    pub entity: Entity,
    pub is_player: bool,
}

pub fn move_piece(
    mut move_piece_events: EventReader<MovePiece>,
    mut pieces: Query<
        (
            &Transform,
            &mut BoardPosition,
            &MovementTypes,
            &Team,
            Entity,
        ),
        With<Piece>,
    >,
    mut commands: Commands,
    mut highlight_cache: ResMut<HighlightCache>,
    mut next_turn_state: ResMut<NextState<TurnState>>,
) {
    for event in move_piece_events.read() {
        let MovePiece {
            destination,
            entity,
            is_player,
        } = event;

        let all_positions: Vec<_> = pieces
            .iter()
            .map(|(_, board_position, _, team, e)| (*board_position, *team, e))
            .collect();

        let other_pieces_positions: Vec<_> = all_positions
            .iter()
            .filter(|(_, _, e)| *e != *entity)
            .map(|(pos, _, _)| *pos)
            .collect();

        if let Ok((transform, mut board_position, movement_types, team, _)) =
            pieces.get_mut(*entity)
        {
            let enemies_positions: Vec<_> = all_positions
                .iter()
                .filter(|(_, t, _)| t != team)
                .map(|(pos, _, _)| *pos)
                .collect();

            // Check if the destination is a valid move
            let is_valid_move = movement_types.0.iter().any(|movement_type| {
                let valid_moves = movement_type.get_valid_moves(
                    &board_position,
                    &HashSet::from_iter(other_pieces_positions.clone()),
                    &HashSet::from_iter(enemies_positions.clone()),
                );
                valid_moves.valid_moves.contains(destination)
            });

            if is_valid_move {
                if *is_player {
                    next_turn_state.set(TurnState::PlayerAnimation);
                }
                highlight_cache.invalidate();
                debug!(
                    "Valid move from {:?} to {:?}, for piece {:?}",
                    board_position, destination, *entity
                );
                // Change state to moving for piece
                commands.entity(*entity).insert(PieceState::Moving {
                    origin: transform.translation,
                    destination: destination
                        .as_global_position()
                        .extend(transform.translation.z),
                });
                board_position.update(destination.x, destination.y);
            } else {
                debug!(
                    "Invalid move from {:?} to {:?}, for piece {:?}",
                    board_position, destination, *entity
                );
            }
        }
    }
}

pub fn move_player_animation(
    mut player: Query<(&mut Transform, &PieceState, Entity), With<Player>>,
    time: Res<Time>,
    mut commands: Commands,
    mut next_turn_state: ResMut<NextState<TurnState>>,
) {
    let (mut transform, state, entity) = player.single_mut();
    if let PieceState::Moving {
        origin: _,
        destination,
    } = state
    {
        let current_position = transform.translation;
        let lerp_value = TWEEN_MOVE_ANIMATION_SPEED * time.delta_seconds();
        let distance = destination.distance_squared(current_position);

        // if less than 1 pixel away, snap to destination
        if distance < 1.0 {
            transform.translation = *destination;
            commands.entity(entity).insert(PieceState::Idle);
            // TODO: Change to AI Turn
            next_turn_state.set(TurnState::PlayerInput);
        } else {
            transform.translation = transform.translation.lerp(*destination, lerp_value);
        }
    }
}
