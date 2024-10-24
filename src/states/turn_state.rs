use std::fmt;

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum TurnState {
    PlayerInput,
    PlayerAnimation,
    EnemyAI,
    EnemyAnimation,
    EnemySpawn,
}

impl fmt::Display for TurnState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TurnState::PlayerInput => write!(f, "PlayerInput"),
            TurnState::PlayerAnimation => write!(f, "PlayerAnimation"),
            TurnState::EnemyAI => write!(f, "EnemyAI"),
            TurnState::EnemyAnimation => write!(f, "EnemyAnimation"),
            TurnState::EnemySpawn => write!(f, "EnemySpawn"),
        }
    }
}

impl TurnState {
    pub fn get_color(&self) -> Color {
        match self {
            TurnState::PlayerInput => Color::srgb(1.0, 0.0, 0.0),
            TurnState::PlayerAnimation => Color::srgb(0.0, 1.0, 0.0),
            TurnState::EnemyAI => Color::srgb(0.0, 0.0, 1.0),
            TurnState::EnemyAnimation => Color::srgb(1.0, 1.0, 0.0),
            TurnState::EnemySpawn => Color::srgb(0.0, 1.0, 1.0),
        }
    }
}

#[derive(Resource, Default)]
pub struct TurnInfo {
    pub number: u32,
}

fn increment_turn(mut turn_info: ResMut<TurnInfo>) {
    turn_info.number += 1;
}

pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(TurnState::PlayerInput), increment_turn);
    }
}

pub fn reset_turn(mut turn_info: ResMut<TurnInfo>) {
    turn_info.number = 1;
}
