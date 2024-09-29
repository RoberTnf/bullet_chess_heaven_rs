use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum GamePauseState {
    Play,
    // Paused,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum GameState {
    // MainMenu,
    Game,
}
