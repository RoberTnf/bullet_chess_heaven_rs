use bevy::prelude::*;

use crate::globals;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(
                4.0 * globals::TILE_SIZE as f32,
                4.0 * globals::TILE_SIZE as f32,
                0.0,
            ),
            ..default()
        },
        Name::new("Camera"),
        IsDefaultUiCamera,
    ));
}
