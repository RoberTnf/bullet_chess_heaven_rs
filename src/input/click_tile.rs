use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    board::position::BoardPosition,
    globals::SPRITESHEET_WIDTH,
    pieces::{
        attack::{attack_from_tile, AttackPieceEvent},
        common::{MovementTypes, Piece, Team},
        damage::Damage,
        movement::MovePieceEvent,
        movement_type::MovementType,
        player::spawn::Player,
    },
    states::turn_state::TurnState,
};

/// Handles click tile events
///
/// If the user clicks on a valid tile
/// move the player to that tile
pub fn click_tile_update_player_position(
    mut move_event_writer: EventWriter<MovePieceEvent>,
    mut attack_event_writer: EventWriter<AttackPieceEvent>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mouse: Res<ButtonInput<MouseButton>>,
    player: Query<(Entity, &BoardPosition, &Damage, &MovementTypes), With<Player>>,
    touches: Res<Touches>,
    pieces_query: Query<(Entity, &BoardPosition, &Team), (With<Piece>, Without<Player>)>,
    mut next_state: ResMut<NextState<TurnState>>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera.single();
    let (player_entity, player_position, damage, movement_types) = player.single();
    let all_pieces_positions = pieces_query.iter().map(|(_, pos, _)| *pos).collect();
    let enemy_pieces_positions = pieces_query
        .iter()
        .filter(|(_, _, &team)| team == Team::Enemy)
        .map(|(_, pos, _)| *pos)
        .collect();

    if mouse.just_pressed(MouseButton::Left) {
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            if let Some(tile_position) = BoardPosition::from_world_position(world_position) {
                let mut current_tile_position = player_position;
                let mut moved = false;
                // First, move the player to the tile if possible
                movement_types.0.iter().for_each(|movement_type| {
                    let response = movement_type.get_valid_moves(
                        player_position,
                        &all_pieces_positions,
                        &enemy_pieces_positions,
                    );
                    if response.valid_moves.contains(&tile_position) {
                        send_move_event(&mut move_event_writer, tile_position, player_entity);
                        current_tile_position = &tile_position;
                        moved = true;
                    }
                });

                if moved {
                    next_state.set(TurnState::PlayerAnimation);
                    return;
                }

                // else, try attacking from current tile
                attack_from_tile(
                    movement_types,
                    current_tile_position,
                    &all_pieces_positions,
                    &enemy_pieces_positions,
                    &pieces_query,
                    &mut attack_event_writer,
                    player_entity,
                    damage,
                    &mut next_state,
                );
            }
        }
    } else {
        for _ in touches.iter_just_pressed() {}
    }
}

fn send_move_event(
    event_writer: &mut EventWriter<'_, MovePieceEvent>,
    tile_position: BoardPosition,
    player_entity: Entity,
) {
    event_writer.send(MovePieceEvent {
        destination: tile_position,
        entity: player_entity,
    });
    debug!("Clicked tile: {:?}", tile_position);
}

pub fn send_attack_event(
    event_writer: &mut EventWriter<AttackPieceEvent>,
    tile_position: &BoardPosition,
    player_entity: Entity,
    target_entity: Entity,
    damage: u64,
    movement_type: &MovementType,
    delay: Option<f32>,
) {
    debug!(
        "Clicked tile: {:?}, attacking target: {:?}, with damage: {}",
        tile_position, target_entity, damage
    );
    event_writer.send(AttackPieceEvent {
        destination: *tile_position,
        attacker: player_entity,
        target: target_entity,
        damage,
        sprite_index: Some(movement_type.sprite_index() + SPRITESHEET_WIDTH),
        delay,
    });
}
