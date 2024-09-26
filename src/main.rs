use bevy::{prelude::*, window::WindowResolution};
mod board;
mod camera;
mod globals;
mod pieces;
mod resources;

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
        .init_resource::<resources::spritesheet::SpriteSheetAtlas>()
        .add_systems(Startup, pieces::player::spawn_player)
        .add_systems(Startup, camera::setup_camera)
        .add_systems(Startup, board::tile::spawn_board)
        .add_systems(Update, pieces::transforms::update_transforms)
        .run();
}
