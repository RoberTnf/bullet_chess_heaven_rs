use bevy::prelude::*;

use crate::{board::position::BoardPosition, globals::TWEEN_MOVE_ANIMATION_SPEED};

use super::common::{Piece, PieceState};

#[derive(Event)]
pub struct MovePiece {
    pub destination: BoardPosition,
    pub entity: Entity,
}

pub fn move_piece(
    mut move_piece_events: EventReader<MovePiece>,
    mut pieces: Query<(&Transform, &mut BoardPosition), With<Piece>>,
    mut commands: Commands,
) {
    for event in move_piece_events.read() {
        let MovePiece {
            destination,
            entity,
        } = event;
        let (transform, mut board_position) = pieces.get_mut(*entity).unwrap();
        board_position.update(destination.x, destination.y);

        // Change state to moving for piece
        commands.entity(*entity).insert(PieceState::Moving {
            origin: transform.translation,
            destination: destination
                .as_global_position()
                .extend(transform.translation.z),
        });
    }
}

pub fn move_piece_animation(
    mut pieces: Query<(&mut Transform, &PieceState, Entity)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut transform, state, entity) in pieces.iter_mut() {
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
