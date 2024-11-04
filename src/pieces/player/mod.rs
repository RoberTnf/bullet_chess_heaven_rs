use bevy::prelude::*;
use spawn::spawn_player;

use crate::states::game_state::GameState;

pub mod experience;
pub mod gold;
pub mod spawn;
pub mod upgrades;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), spawn_player);
        app.add_plugins((experience::ExperiencePlugin, gold::GoldPlugin));
    }
}
