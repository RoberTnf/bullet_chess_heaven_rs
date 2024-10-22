use bevy::prelude::*;

use super::{
    attack::attack_piece_system,
    enemies::EnemyPlugin,
    health::{death_animation, death_system, health_change_system},
};

pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnemyPlugin).add_systems(
            Update,
            (
                attack_piece_system,
                death_system,
                death_animation,
                health_change_system,
            ),
        );
    }
}
