use bevy::prelude::*;
use limit::apply_movement_type_limit;

use crate::{
    globals::{
        CONVERT_ENEMY_TURNS_TO_CONVERT, UNIQUE_ABILITY_UNLOCK_UPGRADE_NUMBER,
        UNIQUE_UPGRADE_DAMAGE_MULTIPLIER,
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
) {
    let movement_upgrades = upgrades.get_movement_types_count();
    if let Some(&count) = movement_upgrades.get(&attack.movement_type) {
        debug!(
            "Applying unique upgrade for movement type: {:?}, count: {}",
            attack.movement_type, count
        );
        attack.damage += UNIQUE_UPGRADE_DAMAGE_MULTIPLIER * (count - 1) as f32;

        if count >= UNIQUE_ABILITY_UNLOCK_UPGRADE_NUMBER {
            let side_effect = fetch_side_effect(attack);
            side_effect_event_writter.send(side_effect);
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
        _ => todo!("Side effect for this movement type not implemented"),
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
                .run_if(on_event::<SideEffect>()),
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
                .run_if(on_event::<ApplyUpgrades>()),
        );
    }
}
