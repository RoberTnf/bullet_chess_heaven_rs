use bevy::{math::I64Vec2, prelude::*, utils::HashMap};

use crate::{
    board::position::BoardPosition,
    globals::{ATTACK_ANIMATION_DURATION, SPRITESHEET_WIDTH},
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
    mut commands: Commands,
) {
    let mut new_side_effects = Vec::new();
    for side_effect in side_effect_event.read() {
        if let SideEffect::Pierce {
            pierce_count,
            damage,
            generator_event,
        } = side_effect
        {
            let (_, attacker_team, _) = pieces.get(generator_event.attacker).unwrap();
            let enemies = pieces
                .iter()
                .filter(|(_, &team, _)| team != *attacker_team)
                .map(|(pos, _, _)| *pos)
                .collect();
            let other_pieces = pieces
                .iter()
                .filter(|(_, _, entity)| *entity != generator_event.attacker)
                .map(|(pos, _, _)| *pos)
                .collect();
            let position_to_entity: HashMap<BoardPosition, Entity> = pieces
                .iter()
                .map(|(pos, _, entity)| (*pos, entity))
                .collect();

            let previous_delay = generator_event.delay.unwrap_or(0.0);
            let delay = previous_delay + ATTACK_ANIMATION_DURATION;

            let valid_targets: Vec<BoardPosition> = generator_event
                .movement_type
                .get_valid_moves(&generator_event.destination, &other_pieces, &enemies)
                .valid_attacks
                .iter()
                .filter(|target| {
                    // ensure the target is in the same direction as the attack
                    let d1 = get_direction(**target, generator_event.destination);
                    let d2 = get_direction(generator_event.destination, generator_event.origin);
                    d1 == d2
                })
                .map(|target| *target)
                .collect();

            for target in valid_targets {
                debug!("Piercing to: {:?}", target);
                let new_attack_piece_event = AttackPieceEvent {
                    attacker: generator_event.attacker,
                    destination: target,
                    origin: generator_event.destination,
                    target: *position_to_entity.get(&target).unwrap(),
                    damage: *damage,
                    sprite_index: Some(
                        generator_event.movement_type.sprite_index() + SPRITESHEET_WIDTH,
                    ),
                    movement_type: generator_event.movement_type.clone(),
                    delay: Some(delay),
                    with_unique_upgrade: false,
                };
                attack_writer.send(new_attack_piece_event.clone());
                if let Some(pierce_count) = pierce_count {
                    new_side_effects.push(SideEffect::Pierce {
                        pierce_count: Some(pierce_count - 1),
                        damage: *damage,
                        generator_event: new_attack_piece_event,
                    });
                } else {
                    // infinite pierce
                    new_side_effects.push(SideEffect::Pierce {
                        pierce_count: None,
                        damage: *damage,
                        generator_event: new_attack_piece_event,
                    });
                }
            }
        }
    }

    for side_effect in new_side_effects {
        commands.add(move |world: &mut World| {
            world.send_event(side_effect);
        });
    }
}

fn get_direction(origin: BoardPosition, destination: BoardPosition) -> I64Vec2 {
    let d = destination - origin;

    I64Vec2::new((d.x / d.x.abs()) as i64, (d.y / d.y.abs()) as i64)
}
