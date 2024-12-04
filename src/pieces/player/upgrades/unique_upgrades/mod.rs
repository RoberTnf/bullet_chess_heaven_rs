use bevy::prelude::*;
use limit::apply_movement_type_limit;

use crate::{
    globals::{
        ATTACK_ANIMATION_DURATION, CONVERT_ENEMY_TURNS_TO_CONVERT, QUEEN_UNIQUE_CHANCE,
        UNIQUE_ABILITY_UNLOCK_UPGRADE_NUMBER, UNIQUE_UPGRADE_DAMAGE_MULTIPLIER,
    },
    pieces::{attack::AttackPieceEvent, common::Team, movement_type::MovementType},
    states::{game_state::GameState, turn_state::TurnState},
    ui::shop::ApplyUpgrades,
};

use super::data::Upgrades;
mod attack_random_target;
pub mod block;
mod chain;
pub mod convert_enemy;
pub mod immortal;
pub mod limit;
mod pierce;

#[derive(Event)]
pub enum SideEffect {
    AttackRandomTarget {
        damage: f32,
        generator_event: AttackPieceEvent,
    },
    Chain {
        damage: f32,
        generator_event: AttackPieceEvent,
    },
    ConvertPiece {
        turns_to_convert: usize,
        team: Team,
        entity: Entity,
    },
    Pierce {
        pierce_count: Option<usize>,
        damage: f32,
        generator_event: AttackPieceEvent,
    },
    Block {
        amount: usize,
        entity: Entity,
    },
    Nothing,
}

pub fn apply_unique_upgrades(
    attack: &mut AttackPieceEvent,
    upgrades: &Upgrades,
    side_effect_event_writter: &mut EventWriter<SideEffect>,
    commands: &mut Commands,
) {
    let movement_upgrades = upgrades.get_movement_types_count();
    if let Some(&count) = movement_upgrades.get(&attack.movement_type) {
        debug!(
            "Applying unique upgrade for movement type: {:?}, count: {}",
            attack.movement_type, count
        );
        if !attack.upgrades_applied {
            attack.damage += UNIQUE_UPGRADE_DAMAGE_MULTIPLIER * (count - 1) as f32;
        }

        if count >= UNIQUE_ABILITY_UNLOCK_UPGRADE_NUMBER {
            let side_effect = fetch_side_effect(attack);
            side_effect_event_writter.send(side_effect);
        }
    }

    // Queen unique ability is a bit different,
    // it has a chance to repeat any attack
    if let Some(queen_count) = movement_upgrades.get(&MovementType::Queen) {
        if *queen_count >= UNIQUE_ABILITY_UNLOCK_UPGRADE_NUMBER {
            let chance = QUEEN_UNIQUE_CHANCE;
            let random_value = rand::random::<f32>();
            let mut new_event = attack.clone();

            if let Some(delay) = new_event.delay {
                new_event.delay = Some(delay + ATTACK_ANIMATION_DURATION);
            } else {
                new_event.delay = Some(ATTACK_ANIMATION_DURATION);
            }
            new_event.upgrades_applied = true;

            if random_value < chance {
                commands.queue(move |world: &mut World| {
                    world.send_event(new_event);
                });
            }
        }
    }
}

fn fetch_side_effect(attack: &AttackPieceEvent) -> SideEffect {
    match attack.movement_type {
        MovementType::Knight => SideEffect::Chain {
            damage: attack.damage,
            generator_event: attack.clone(),
        },
        MovementType::BlackPawn => SideEffect::ConvertPiece {
            turns_to_convert: CONVERT_ENEMY_TURNS_TO_CONVERT,
            team: Team::Player,
            entity: attack.target,
        },
        MovementType::WhitePawn => SideEffect::ConvertPiece {
            turns_to_convert: CONVERT_ENEMY_TURNS_TO_CONVERT,
            team: Team::Player,
            entity: attack.target,
        },
        MovementType::Bishop => SideEffect::Pierce {
            pierce_count: None,
            damage: attack.damage,
            generator_event: attack.clone(),
        },
        MovementType::Rook => SideEffect::Block {
            amount: 1,
            entity: attack.attacker,
        },
        MovementType::King => SideEffect::Nothing,
        MovementType::Queen => SideEffect::Nothing,
    }
}

pub struct UniqueUpgradesPlugin;

impl Plugin for UniqueUpgradesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SideEffect>();
        app.add_systems(
            Update,
            (
                attack_random_target::apply_side_effect,
                chain::apply_side_effect,
                pierce::apply_side_effect,
                convert_enemy::apply_side_effect,
                block::apply_side_effect,
            )
                .run_if(in_state(GameState::Game))
                .run_if(on_event::<SideEffect>),
        );
        app.add_systems(
            OnEnter(TurnState::PlayerInput),
            (
                convert_enemy::decrement_turns_to_convert,
                immortal::decrement_turns_remaining,
            )
                .run_if(in_state(GameState::Game)),
        );
        app.add_systems(
            Update,
            apply_movement_type_limit
                .run_if(in_state(GameState::Game))
                .run_if(on_event::<ApplyUpgrades>),
        );
    }
}
