use bevy::prelude::*;
use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum GameState {
    MainMenu,
    Game,
    Defeat,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameState::MainMenu => write!(f, "MainMenu"),
            GameState::Game => write!(f, "Game"),
            GameState::Defeat => write!(f, "Defeat"),
        }
    }
}

impl GameState {
    pub fn get_color(&self) -> Color {
        match self {
            GameState::MainMenu => Color::srgb(1.0, 0.0, 0.0),
            GameState::Game => Color::srgb(0.0, 1.0, 0.0),
            GameState::Defeat => Color::srgb(0.0, 0.0, 1.0),
        }
    }
}
