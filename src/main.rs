use bevy::{log::LogPlugin, prelude::*, window::WindowResolution};
use game_state::{GamePauseState, GameState};
mod board;
mod events;
mod game_state;
mod globals;
mod graphics;
mod input;
mod pieces;
mod startup;
mod update;
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bullet Chess Heaven".to_string(),
                        resolution: WindowResolution::new(
                            globals::WINDOW_WIDTH,
                            globals::WINDOW_HEIGHT,
                        )
                        // TODO: dynamic scaling
                        .with_scale_factor_override(
                            globals::WINDOW_WIDTH / globals::TARGET_PIXEL_WIDTH,
                        ),
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    filter: "bullet_chess_heaven_rs=debug".into(),
                    level: bevy::log::Level::WARN,
                    ..default()
                }),
        )
        .add_plugins((startup::StartupPlugin, update::UpdatePlugin))
        .insert_resource(Msaa::Off)
        .enable_state_scoped_entities::<GameState>()
        .enable_state_scoped_entities::<GamePauseState>()
        .run();
}
