use bevy::prelude::*;

use crate::pieces::common::Piece;

use super::SideEffect;

#[derive(Component)]
pub struct Block {
    pub amount: usize,
}

pub fn apply_side_effect(
    mut side_effect_event: EventReader<SideEffect>,
    pieces: Query<&Piece>,
    mut commands: Commands,
) {
    for event in side_effect_event.read() {
        if let SideEffect::Block { amount, entity } = event {
            // ensure the entity is a piece
            if let Ok(_) = pieces.get(*entity) {
                commands.entity(*entity).insert(Block { amount: *amount });
            }
        }
    }
}
