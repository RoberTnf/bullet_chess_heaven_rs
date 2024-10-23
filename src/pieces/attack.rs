use bevy::prelude::*;

use crate::{
    board::position::BoardPosition,
    globals::{ATTACK_ANIMATION_DURATION, TILE_SIZE},
};

use super::{
    common::{Piece, PieceState},
    health::PieceHealthChangeEvent,
};

#[derive(Event)]
pub struct AttackPieceEvent {
    pub destination: BoardPosition,
    pub attacker: Entity,
    pub target: Entity,
    pub damage: u64,
}
pub enum AttackPieceAnimationState {
    Backwards,
    Forwards,
}

pub fn attack_piece_system(
    mut attack_event_reader: EventReader<AttackPieceEvent>,
    mut health_event_writer: EventWriter<PieceHealthChangeEvent>,
    mut pieces: Query<(&BoardPosition, &mut PieceState), With<Piece>>,
) {
    for event in attack_event_reader.read() {
        health_event_writer.send(PieceHealthChangeEvent {
            entity: event.target,
            change: -(event.damage as i64),
        });

        let (attacker_pos, mut attacker_state) = pieces.get_mut(event.attacker).unwrap();

        *attacker_state = PieceState::Attacking {
            destination: event.destination,
            origin: *attacker_pos,
            animation_state: AttackPieceAnimationState::Forwards,
        };
    }
}

pub fn attack_piece_animation_system(
    mut query: Query<(&mut Transform, &mut PieceState), With<Piece>>,
    time: Res<Time>,
) {
    for (mut transform, mut piece_state) in query.iter_mut() {
        if let PieceState::Attacking {
            destination,
            origin,
            animation_state,
        } = &mut *piece_state
        {
            // TODO: If this becomes slow, store this variables in the animation component
            let origin_global_position = origin.as_global_position();
            let destination_global_position = destination.as_global_position();
            let original_distance = (destination_global_position - origin_global_position).length();
            let direction = (destination_global_position - origin_global_position).normalize();
            let truncated_translation = transform.translation.truncate();
            let speed = original_distance * 2.0 / ATTACK_ANIMATION_DURATION;
            let delta = direction * speed * time.delta_seconds();

            // work in 2D except for the end
            let original_z = transform.translation.z;

            match animation_state {
                AttackPieceAnimationState::Forwards => {
                    let new_position = truncated_translation + delta;
                    let pixel_distance = new_position.distance(destination_global_position);
                    transform.translation = new_position.extend(original_z);

                    if pixel_distance < TILE_SIZE as f32 / 1.5 {
                        *animation_state = AttackPieceAnimationState::Backwards;
                    }
                }
                AttackPieceAnimationState::Backwards => {
                    let new_position = truncated_translation - delta;
                    let progress =
                        new_position.distance(destination_global_position) / original_distance;
                    transform.translation = new_position.extend(original_z);

                    if progress > 0.98 {
                        // snap to the origin
                        transform.translation = origin_global_position.extend(original_z);
                        *piece_state = PieceState::Idle;
                    }
                }
            }
        }
    }
}
