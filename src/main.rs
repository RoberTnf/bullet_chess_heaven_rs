use bevy::{log::LogPlugin, prelude::*, window::WindowResolution};

mod board;
mod game_logic;
mod globals;
mod graphics;
mod input;
mod pieces;
mod plugins;
mod states;
mod ui;

fn main() {
    App::new()
        // Config
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
        .insert_resource(Msaa::Off)
        // Game
        .add_plugins((
            plugins::startup::StartupPlugin,
            plugins::update::UpdatePlugin,
        ))
        .run();
}
