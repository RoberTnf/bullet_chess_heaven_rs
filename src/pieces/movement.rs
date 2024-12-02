use bevy::prelude::*;

use crate::{
    board::{highlight::HighlightCache, position::BoardPosition},
    globals::TWEEN_MOVE_ANIMATION_SPEED,
    states::turn_state::TurnState,
};

use super::{
    common::{Piece, PieceState, Team},
    enemies::spawn::AIControlled,
    player::spawn::Player,
};

#[derive(Event)]
pub struct MovePieceEvent {
    pub destination: BoardPosition,
    pub entity: Entity,
}

pub fn move_piece(
    mut move_piece_events: EventReader<MovePieceEvent>,
    mut pieces: Query<(&Transform, &mut BoardPosition), With<Piece>>,
    mut commands: Commands,
    mut highlight_cache: ResMut<HighlightCache>,
) {
    for event in move_piece_events.read() {
        let MovePieceEvent {
            destination,
            entity,
        } = event;

        if let Ok((transform, mut board_position)) = pieces.get_mut(*entity) {
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
        }
    }
}

#[derive(Event)]
pub struct MovePieceAnimationEndEvent {
    pub entity: Entity,
    pub origin: BoardPosition,
    pub destination: BoardPosition,
}

pub fn move_pieces_animation(
    mut pieces: Query<(&mut Transform, &mut PieceState, Entity)>,
    time: Res<Time>,
    mut move_piece_animation_end_events: EventWriter<MovePieceAnimationEndEvent>,
) {
    for (mut transform, mut state, entity) in pieces.iter_mut() {
        if let PieceState::Moving {
            origin,
            destination,
        } = state.as_mut()
        {
            let current_position = transform.translation;
            let lerp_value = TWEEN_MOVE_ANIMATION_SPEED * time.delta_secs();
            let distance = destination.distance_squared(current_position);

            // if less than 1 pixel away, snap to destination
            if distance < 1.0 {
                transform.translation = *destination;
                move_piece_animation_end_events.send(MovePieceAnimationEndEvent {
                    entity,
                    origin: BoardPosition::from_world_position(origin.truncate())
                        .expect("Invalid origin position"),
                    destination: BoardPosition::from_world_position(destination.truncate())
                        .expect("Invalid destination position"),
                });
                *state = PieceState::MoveEnded;
            } else {
                transform.translation = transform.translation.lerp(*destination, lerp_value);
            }
        }
    }
}

pub fn all_enemies_idle(
    mut moves: Query<(&mut PieceState, &Team), With<AIControlled>>,
    mut turn_state: ResMut<NextState<TurnState>>,
) {
    if moves
        .iter()
        .filter(|(_, &team)| team == Team::Enemy)
        .all(|(state, _)| matches!(state, PieceState::AttackEnded))
    {
        turn_state.set(TurnState::EnemySpawn);
        moves.iter_mut().for_each(|(mut state, _)| {
            *state = PieceState::Idle;
        });
    }
}

pub fn all_player_ai_idle(
    mut moves: Query<(&mut PieceState, &Team), With<AIControlled>>,
    mut turn_state: ResMut<NextState<TurnState>>,
) {
    if moves
        .iter()
        .filter(|(_, &team)| team == Team::Player)
        .all(|(state, _)| matches!(state, PieceState::AttackEnded))
    {
        turn_state.set(TurnState::EnemyAI);
        moves.iter_mut().for_each(|(mut state, _)| {
            *state = PieceState::Idle;
        });
    }
}

pub fn is_player_idle(
    mut moves: Query<&mut PieceState, With<Player>>,
    mut turn_state: ResMut<NextState<TurnState>>,
) {
    if moves
        .iter()
        .all(|state| matches!(state, PieceState::AttackEnded))
    {
        turn_state.set(TurnState::PlayerAI);
        moves.iter_mut().for_each(|mut state| {
            *state = PieceState::Idle;
        });
    }
}
