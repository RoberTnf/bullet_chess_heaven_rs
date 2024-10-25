use bevy::prelude::*;

use crate::{
    board::highlight,
    game_logic::{score::GameScorePlugin, GameLogicPlugin},
    pieces::{experience::ExperiencePlugin, plugin::PiecePlugin},
    states::{game_state::GameStatePlugin, turn_state},
};
mod animation;
mod input;
pub mod movement;
pub struct UpdatePlugin;

impl Plugin for UpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            animation::AnimationPlugin,
            input::InputPlugin,
            movement::MovementPlugin,
            highlight::HighlightPlugin,
            turn_state::TurnPlugin,
            PiecePlugin,
            GameLogicPlugin,
            // ResolutionPlugin,
            ExperiencePlugin,
            GameScorePlugin,
            GameStatePlugin,
        ));
    }
}
