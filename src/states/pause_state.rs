use bevy::prelude::*;
use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum GamePauseState {
    Playing,
    PausedLevelUpReward,
}

impl fmt::Display for GamePauseState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GamePauseState::Playing => write!(f, "Playing"),
            GamePauseState::PausedLevelUpReward => write!(f, "PausedLevelUpReward"),
        }
    }
}

impl GamePauseState {
    pub fn get_color(&self) -> Color {
        match self {
            GamePauseState::Playing => Color::srgb(0.0, 1.0, 0.0),
            GamePauseState::PausedLevelUpReward => Color::srgb(1.0, 0.0, 0.0),
        }
    }
}
