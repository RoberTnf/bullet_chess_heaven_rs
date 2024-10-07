use bevy::{input::touch::TouchPhase, prelude::*, window::PrimaryWindow};

use crate::{
    board::position::BoardPosition,
    pieces::{movement::MovePiece, player::spawn::Player},
};

/// Handles click tile events
///
/// If the user clicks on a valid tile
/// move the player to that tile
pub fn click_tile_update_player_position(
    mut event_writer: EventWriter<MovePiece>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut touch: EventReader<TouchInput>,
    player: Query<Entity, With<Player>>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera.single();
    let player_entity = player.single();

    if mouse.just_pressed(MouseButton::Left) {
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            if let Some(tile_position) = BoardPosition::from_world_position(world_position) {
                event_writer.send(MovePiece {
                    destination: tile_position,
                    entity: player_entity,
                });
                debug!("Clicked tile: {:?}", tile_position);
            }
        }
    } else {
        for ev in touch.read() {
            if ev.phase == TouchPhase::Ended {
                let cursor = camera
                    .viewport_to_world_2d(camera_transform, ev.position)
                    .unwrap();

                if let Some(tile_position) = BoardPosition::from_world_position(cursor) {
                    event_writer.send(MovePiece {
                        destination: tile_position,
                        entity: player_entity,
                    });
                    debug!("Touched tile: {:?}", tile_position);
                }
            }
        }
    }
}
