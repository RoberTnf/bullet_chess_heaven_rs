use bevy::prelude::*;
use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum GamePauseState {
    Play,
    Paused,
}

impl fmt::Display for GamePauseState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GamePauseState::Play => write!(f, "Play"),
            GamePauseState::Paused => write!(f, "Paused"),
        }
    }
}

impl GamePauseState {
    pub fn get_color(&self) -> Color {
        match self {
            GamePauseState::Play => Color::srgb(0.0, 1.0, 0.0),
            GamePauseState::Paused => Color::srgb(1.0, 0.0, 0.0),
        }
    }
}
