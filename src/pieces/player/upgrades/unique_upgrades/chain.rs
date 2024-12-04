use bevy::{prelude::*, utils::HashMap};

use crate::{
    board::position::BoardPosition,
    globals::{ATTACK_ANIMATION_DURATION, SPRITESHEET_WIDTH},
    pieces::{
        attack::AttackPieceEvent,
        common::{Piece, Team},
    },
    states::{game_state::GameState, turn_state::TurnState},
};

use super::SideEffect;

#[derive(Component)]
pub struct HasChainedTo;

pub fn apply_side_effect(
    mut side_effect_event: EventReader<SideEffect>,
    mut attack_writer: EventWriter<AttackPieceEvent>,
    pieces: Query<(&BoardPosition, &Team, Entity), (With<Piece>, Without<HasChainedTo>)>,
    mut commands: Commands,
) {
    for side_effect in side_effect_event.read() {
        if let SideEffect::Chain {
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
            let targets = generator_event
                .movement_type
                .get_valid_moves(&generator_event.destination, &other_pieces, &enemies)
                .valid_attacks;

            let previous_delay = generator_event.delay.unwrap_or(0.0);
            let delay = previous_delay + ATTACK_ANIMATION_DURATION;

            for target in targets {
                debug!("Chaining to: {:?}", target);
                let target_entity = *position_to_entity.get(&target).unwrap();
                attack_writer.send(AttackPieceEvent {
                    attacker: generator_event.attacker,
                    destination: target,
                    origin: generator_event.destination,
                    target: target_entity,
                    damage: *damage,
                    sprite_index: Some(
                        generator_event.movement_type.sprite_index() + SPRITESHEET_WIDTH,
                    ),
                    movement_type: generator_event.movement_type.clone(),
                    delay: Some(delay),
                    upgrades_applied: true,
                });
                commands.entity(target_entity).insert(HasChainedTo);
            }
        }
    }
}

pub fn remove_chain_component(mut commands: Commands, pieces: Query<Entity, With<HasChainedTo>>) {
    for entity in pieces.iter() {
        commands.entity(entity).remove::<HasChainedTo>();
    }
}

pub struct ChainPlugin;

impl Plugin for ChainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (apply_side_effect)
                .run_if(in_state(GameState::Game))
                .run_if(on_event::<SideEffect>),
        );
        app.add_systems(OnEnter(TurnState::PlayerInput), remove_chain_component);
    }
}
