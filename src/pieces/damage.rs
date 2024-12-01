use bevy::prelude::*;

use super::player::upgrades::stats::{Stat, StatVariant};

#[derive(Component, Default)]
pub struct Attack(pub Stat);

impl Attack {
    pub fn new(value: f32) -> Self {
        Attack(Stat {
            base_value: value,
            stat_variant: StatVariant::Attack,
            upgraded_value: value,
        })
    }
}
