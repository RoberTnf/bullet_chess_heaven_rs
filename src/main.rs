use bevy::{prelude::*, window::WindowResolution};
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
                }),
        )
        .insert_resource(Msaa::Off)
        .add_plugins((
            graphics::GraphicsPlugin,
            startup::StartupPlugin,
            input::InputPlugin,
            events::EventPlugin,
            board::BoardPlugin,
        ))
        .run();
}
