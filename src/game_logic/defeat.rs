use crate::pieces::health::Health;
use crate::pieces::player::spawn::Player;
use crate::states::game_state::GameState;
use bevy::prelude::*;

pub fn check_defeat(
    player_query: Query<&Health, With<Player>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(player_health) = player_query.get_single() {
        if player_health.is_dead() {
            game_state.set(GameState::Defeat);
        }
    }
}

pub fn reset_game(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Game);
}
