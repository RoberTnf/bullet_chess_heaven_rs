use bevy::prelude::*;

use crate::{
    globals::{UNIQUE_ABILITY_UNLOCK_UPGRADE_NUMBER, UNIQUE_UPGRADE_DAMAGE_MULTIPLIER},
    pieces::{attack::AttackPieceEvent, movement_type::MovementType},
    states::game_state::GameState,
};

use super::data::Upgrades;
mod attack_random_target;

#[derive(Event)]
pub enum SideEffect {
    AttackRandomTarget {
        limited_per_turn: Option<usize>,
        damage: f32,
        generator_event: AttackPieceEvent,
    },
    SpawnAlly {},
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
        attack.damage += UNIQUE_UPGRADE_DAMAGE_MULTIPLIER * count as f32;

        if count >= UNIQUE_ABILITY_UNLOCK_UPGRADE_NUMBER {
            let side_effect = fetch_side_effect(attack);
            side_effect_event_writter.send(side_effect);
        }
    }
}

fn fetch_side_effect(attack: &AttackPieceEvent) -> SideEffect {
    match attack.movement_type {
        MovementType::Knight => SideEffect::AttackRandomTarget {
            limited_per_turn: None,
            damage: attack.damage,
            generator_event: attack.clone(),
        },
        _ => todo!("Side effect for this movement type not implemented"),
    }
}

pub struct UniqueUpgradesPlugin;

impl Plugin for UniqueUpgradesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SideEffect>();
        app.add_systems(
            Update,
            attack_random_target::apply_side_effect
                .run_if(in_state(GameState::Game))
                .run_if(on_event::<AttackPieceEvent>()),
        );
    }
}
