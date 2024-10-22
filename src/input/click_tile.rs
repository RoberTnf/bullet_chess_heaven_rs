use bevy::{prelude::*, utils::HashSet, window::PrimaryWindow};

use crate::{
    board::position::BoardPosition,
    pieces::{
        attack::AttackPieceEvent,
        common::{MovementTypes, Piece, Team},
        damage::Damage,
        movement::MovePieceEvent,
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
                let mut valid_moves = HashSet::new();
                let mut valid_attacks = HashSet::new();

                movement_types.0.iter().for_each(|movement_type| {
                    let response = movement_type.get_valid_moves(
                        player_position,
                        &all_pieces_positions,
                        &enemy_pieces_positions,
                    );
                    valid_moves.extend(response.valid_moves);
                    valid_attacks.extend(response.valid_attacks);
                });

                if valid_moves.contains(&tile_position) {
                    send_move_event(
                        &mut move_event_writer,
                        tile_position,
                        player_entity,
                        &mut next_state,
                    );
                } else if valid_attacks.contains(&tile_position) {
                    let enemy_entity = pieces_query
                        .iter()
                        .find(|(_, &pos, _)| pos == tile_position)
                        .map(|(entity, _, _)| entity)
                        .unwrap();

                    send_attack_event(
                        &mut attack_event_writer,
                        tile_position,
                        player_entity,
                        enemy_entity,
                        damage.value,
                        &mut next_state,
                    );
                }
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
    next_state: &mut ResMut<NextState<TurnState>>,
) {
    event_writer.send(MovePieceEvent {
        destination: tile_position,
        entity: player_entity,
    });
    debug!("Clicked tile: {:?}", tile_position);
    next_state.set(TurnState::PlayerAnimation);
}

fn send_attack_event(
    event_writer: &mut EventWriter<AttackPieceEvent>,
    tile_position: BoardPosition,
    player_entity: Entity,
    target_entity: Entity,
    damage: u64,
    next_state: &mut ResMut<NextState<TurnState>>,
) {
    debug!(
        "Clicked tile: {:?}, attacking target: {:?}, with damage: {}",
        tile_position, target_entity, damage
    );
    event_writer.send(AttackPieceEvent {
        destination: tile_position,
        attacker: player_entity,
        target: target_entity,
        damage,
    });
    next_state.set(TurnState::PlayerAnimation);
}
