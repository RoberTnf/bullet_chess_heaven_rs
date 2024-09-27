use bevy::{prelude::*, window::PrimaryWindow};

use crate::board::position::BoardPosition;
use crate::events::click_tile::TileClickedEvent;

pub fn mouse_input(
    input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut events_writer: EventWriter<TileClickedEvent>,
) {
    if input.just_pressed(MouseButton::Left) {
        let window = windows.single();
        let (camera, camera_transform) = camera.single();

        if let Some(world_position) = window
            .cursor_position()
            .and_then(|p| camera.viewport_to_world_2d(camera_transform, p))
        {
            if let Some(tile_pos) =
                BoardPosition::from_global_position(world_position.x, world_position.y)
            {
                events_writer.send(TileClickedEvent { tile_pos });
            }
        }
    }
}
