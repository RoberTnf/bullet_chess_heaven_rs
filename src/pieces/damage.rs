use bevy::prelude::*;

use super::player::upgrades::stats::Stat;

#[derive(Component)]
pub struct Attack(pub Stat);
