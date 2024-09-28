// TODO: Game states with cleanup
// TOOD: All Update systems should be in a state
// TODO: Entities should have a cleanup component
// TODO: All entities should have a name

use bevy::{log::LogPlugin, prelude::*, window::WindowResolution};
use bevy_tweening::TweeningPlugin;
mod board;
mod events;
mod globals;
mod graphics;
mod input;
mod pieces;
mod startup;

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
        .add_plugins(TweeningPlugin)
        .insert_resource(Msaa::Off)
        .add_plugins((
            startup::StartupPlugin,
            board::BoardPlugin,
            events::EventPlugin,
            graphics::GraphicsPlugin,
        ))
        .run();
}
