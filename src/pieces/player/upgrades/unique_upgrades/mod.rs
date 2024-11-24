use bevy::prelude::*;

use crate::{globals::UNIQUE_UPGRADE_DAMAGE_MULTIPLIER, pieces::attack::AttackPieceEvent};

use super::data::Upgrades;

pub mod pawn;

pub struct SideEffect;

pub fn apply_unique_upgrades(
    attack: &mut AttackPieceEvent,
    upgrades: &Upgrades,
) -> Vec<SideEffect> {
    let movement_upgrades = upgrades.get_movement_types_count();
    if let Some(&count) = movement_upgrades.get(&attack.movement_type) {
        debug!(
            "Applying unique upgrade for movement type: {:?}, count: {}",
            attack.movement_type, count
        );
        attack.damage *= UNIQUE_UPGRADE_DAMAGE_MULTIPLIER * count as f32;
    }

    vec![]
}
