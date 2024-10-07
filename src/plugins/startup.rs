use bevy::prelude::*;

use crate::{board, graphics, pieces, states};

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app
            // Set up states
            .insert_state(states::game_state::GameState::Game)
            .insert_state(states::pause_state::GamePauseState::Play)
            .insert_state(states::turn_state::TurnState::PlayerInput)
            .enable_state_scoped_entities::<states::pause_state::GamePauseState>()
            .enable_state_scoped_entities::<states::turn_state::TurnState>()
            .enable_state_scoped_entities::<states::game_state::GameState>()
            // Resources
            .init_resource::<graphics::spritesheet::SpriteSheetAtlas>()
            .insert_resource(ClearColor(Color::srgb(0.063, 0.063, 0.082)))
            // Events
            .add_event::<pieces::movement::MovePiece>()
            // One off systems
            .add_systems(
                Startup,
                (
                    board::tile::spawn_board,
                    graphics::camera::setup_camera,
                    pieces::player::spawn::spawn_player,
                ),
            );
    }
}
