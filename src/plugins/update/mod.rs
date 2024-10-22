use bevy::prelude::*;

use crate::{board::highlight, pieces::plugin::PiecePlugin, states::turn_state};
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
        ));
    }
}
