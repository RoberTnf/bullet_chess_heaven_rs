use bevy::prelude::*;

use crate::{
    globals::MOVEMENT_TYPE_LIMITS,
    pieces::player::{
        spawn::Player,
        upgrades::data::{Effect, Upgrades},
    },
};

#[derive(Component)]
pub struct MovementTypeLimit {
    pub limit: usize,
}

pub fn apply_movement_type_limit(
    mut commands: Commands,
    query: Query<(&Upgrades, Entity), With<Player>>,
) {
    let mut n_upgrades = 0;
    let (upgrades, entity) = query.get_single().unwrap();
    for upgrade in upgrades.0.iter() {
        if let Effect::MovementType(_) = &upgrade.effect {
            n_upgrades += 1;
        }
    }

    for limit in MOVEMENT_TYPE_LIMITS.iter().rev() {
        if n_upgrades >= limit.0 {
            commands
                .entity(entity)
                .insert(MovementTypeLimit { limit: limit.1 });
            return;
        }
    }
}
