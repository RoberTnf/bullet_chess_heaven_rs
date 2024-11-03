use bevy::prelude::*;

use crate::{
    board, graphics, pieces,
    states::{self, game_state::GameState},
    ui::UiPlugin,
};

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app
            // Set up states
            .insert_state(states::game_state::GameState::Game)
            .insert_state(states::pause_state::GamePauseState::Playing)
            .insert_state(states::turn_state::TurnState::PlayerInput)
            .enable_state_scoped_entities::<states::pause_state::GamePauseState>()
            .enable_state_scoped_entities::<states::turn_state::TurnState>()
            .enable_state_scoped_entities::<states::game_state::GameState>()
            // Resources
            .init_resource::<graphics::spritesheet::SpriteSheetAtlas>()
            .init_resource::<board::highlight::HighlightCache>()
            .insert_resource(ClearColor(Color::srgb(0.063, 0.063, 0.082)))
            // Events
            .add_event::<pieces::movement::MovePieceEvent>()
            .add_event::<pieces::attack::AttackPieceEvent>()
            .add_event::<pieces::health::PieceHealthChangeEvent>()
            .add_event::<pieces::health::PieceDeathEvent>()
            .add_systems(Startup, graphics::camera::setup_camera)
            // One off systems
            .add_systems(OnEnter(GameState::Game), (board::tile::spawn_board,))
            .add_plugins(UiPlugin)
            .init_resource::<states::turn_state::TurnInfo>();
    }
}
