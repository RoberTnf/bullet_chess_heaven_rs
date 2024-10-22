use bevy::prelude::*;

use crate::board::position::BoardPosition;

use super::health::PieceHealthChangeEvent;

#[derive(Event)]
pub struct AttackPieceEvent {
    pub destination: BoardPosition,
    pub attacker: Entity,
    pub target: Entity,
    pub damage: u64,
}

pub fn attack_piece_system(
    mut attack_event_reader: EventReader<AttackPieceEvent>,
    mut health_event_writer: EventWriter<PieceHealthChangeEvent>,
) {
    for event in attack_event_reader.read() {
        health_event_writer.send(PieceHealthChangeEvent {
            entity: event.target,
            change: -(event.damage as i64),
        });
    }
}
