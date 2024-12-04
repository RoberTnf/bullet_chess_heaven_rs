use bevy::prelude::*;
use rand::seq::IteratorRandom;

use crate::{
    board::position::BoardPosition,
    globals::SPRITESHEET_WIDTH,
    pieces::{
        attack::AttackPieceEvent,
        common::{Piece, Team},
    },
};

use super::SideEffect;

pub fn apply_side_effect(
    mut side_effect_event: EventReader<SideEffect>,
    mut attack_writer: EventWriter<AttackPieceEvent>,
    pieces: Query<(&BoardPosition, &Team, Entity), With<Piece>>,
) {
    for side_effect in side_effect_event.read() {
        if let SideEffect::AttackRandomTarget {
            damage,
            generator_event,
        } = side_effect
        {
            let (_, attacker_team, _) = pieces.get(generator_event.attacker).unwrap();
            let enemies = pieces.iter().filter(|(_, &team, _)| team != *attacker_team);
            let (target_pos, _, target_entity) = enemies.choose(&mut rand::thread_rng()).unwrap();

            attack_writer.send(AttackPieceEvent {
                attacker: generator_event.attacker,
                destination: *target_pos,
                target: target_entity,
                damage: *damage,
                sprite_index: Some(
                    generator_event.movement_type.sprite_index() + SPRITESHEET_WIDTH,
                ),
                movement_type: generator_event.movement_type.clone(),
                delay: generator_event.delay,
                origin: generator_event.origin,
                upgrades_applied: true,
            });
        }
    }
}
