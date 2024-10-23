use bevy::prelude::*;

use crate::states::{game_state::GameState, pause_state::GamePauseState};

use super::{
    attack::{attack_piece_animation_system, attack_piece_system},
    enemies::EnemyPlugin,
    health::{
        death_animation, death_system, health_change_system, health_change_text_animation,
        spawn_health_change_text,
    },
    healthbar::update_healthbars,
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
                spawn_health_change_text,
                update_healthbars,
            )
                .run_if(in_state(GameState::Game))
                .run_if(in_state(GamePauseState::Playing)),
        );
        app.add_systems(
            FixedUpdate,
            (attack_piece_animation_system, health_change_text_animation)
                .run_if(in_state(GameState::Game))
                .run_if(in_state(GamePauseState::Playing)),
        );
    }
}
