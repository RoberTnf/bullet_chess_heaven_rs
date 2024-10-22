use bevy::prelude::*;

use crate::{
    board::{highlight::HighlightCache, position::BoardPosition},
    globals::TWEEN_MOVE_ANIMATION_SPEED,
    states::turn_state::TurnState,
};

use super::{
    common::{Piece, PieceState},
    enemies::Enemy,
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
            next_turn_state.set(TurnState::EnemyAI);
        } else {
            transform.translation = transform.translation.lerp(*destination, lerp_value);
        }
    } else {
        next_turn_state.set(TurnState::EnemyAI);
    }
}

pub fn move_enemies_animation(
    mut enemies: Query<(&mut Transform, &PieceState, Entity), With<Enemy>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut transform, state, entity) in enemies.iter_mut() {
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
            } else {
                transform.translation = transform.translation.lerp(*destination, lerp_value);
            }
        }
    }
}

pub fn all_enemies_moved(
    enemies: Query<&PieceState, With<Enemy>>,
    mut turn_state: ResMut<NextState<TurnState>>,
) {
    if enemies
        .iter()
        .all(|state| matches!(state, PieceState::Idle))
    {
        turn_state.set(TurnState::EnemySpawn);
    }
}
