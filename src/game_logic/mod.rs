use crate::states::game_state::GameState;
use bevy::prelude::*;
use defeat::check_defeat;

pub mod defeat;
pub mod score;

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Defeat), defeat::reset_game)
            .add_systems(Update, check_defeat);
    }
}
