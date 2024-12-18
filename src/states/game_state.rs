use bevy::prelude::*;
use std::fmt;

use super::{
    pause_state::GamePauseState,
    turn_state::{reset_turn, TurnState},
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum GameState {
    MainMenu,
    Game,
    Defeat,
    Restart,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameState::MainMenu => write!(f, "MainMenu"),
            GameState::Game => write!(f, "Game"),
            GameState::Defeat => write!(f, "Defeat"),
            GameState::Restart => write!(f, "Restart"),
        }
    }
}

pub fn reset_states(
    mut next_state: ResMut<NextState<TurnState>>,
    mut next_state_2: ResMut<NextState<GamePauseState>>,
) {
    debug!("Resetting states");
    next_state.set(TurnState::PlayerInput);
    next_state_2.set(GamePauseState::Playing);
}

impl GameState {
    pub fn get_color(&self) -> Color {
        match self {
            GameState::MainMenu => Color::srgb(1.0, 0.0, 0.0),
            GameState::Game => Color::srgb(0.0, 1.0, 0.0),
            GameState::Defeat => Color::srgb(0.0, 0.0, 1.0),
            GameState::Restart => Color::srgb(1.0, 1.0, 0.0),
        }
    }
}

fn restart_game(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Game);
}
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), (reset_states, reset_turn));
        app.add_systems(OnEnter(GameState::Restart), restart_game);
    }
}
