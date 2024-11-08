use bevy::{prelude::*, render::camera::ScalingMode};

use crate::globals;

pub fn setup_camera(mut commands: Commands) {
    debug!("Setting up camera");
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(
                4.0 * globals::TILE_SIZE as f32,
                4.0 * globals::TILE_SIZE as f32,
                0.0,
            ),
            projection: OrthographicProjection {
                near: -1000.0,
                far: 1000.0,
                scale: 1.0 / 5.0,
                scaling_mode: ScalingMode::AutoMax {
                    max_width: 2560.0,
                    max_height: 1440.0,
                },
                ..default()
            },
            ..default()
        },
        Name::new("Camera"),
        IsDefaultUiCamera,
    ));
}
