use bevy::{prelude::*, utils::HashSet};

use crate::{
    events::update_pos::UpdatePositionEvent,
    game_state::{GamePauseState, GameState},
    globals,
    graphics::spritesheet::SpriteSheetAtlas,
    pieces::player::Player,
};

use super::{
    board_map::BoardMap,
    position::BoardPosition,
    tile::{Tile, TileBundle},
};

#[derive(Component)]
pub struct Highlight;

pub fn highlight_player_movable_positions(
    mut commands: Commands,
    board_map: Res<BoardMap>,
    player_position: Query<Entity, With<Player>>,
    asset_server: Res<AssetServer>,
    atlas_layout: Res<SpriteSheetAtlas>,
    query_highlights: Query<(Entity, &BoardPosition), With<Highlight>>, // Query to find existing highlights
    mut update_position_event: EventReader<UpdatePositionEvent>,
) {
    let player_entity = player_position.get_single().expect("0 or 2+ players");

    for event in update_position_event.read() {
        if event.piece == player_entity {
            debug!("Updating highlights for player at {:?}", event.tile_pos);
            let player_movable_positions = board_map
                .possible_moves_cache
                .get_movement_tiles(&event.tile_pos);

            let old_highlight_positions: HashSet<BoardPosition> =
                query_highlights.iter().map(|(_, pos)| *pos).collect();

            // Despawn old highlights that are not in the new ones
            for (entity, pos) in query_highlights.iter() {
                if !player_movable_positions.contains(pos) {
                    commands.entity(entity).despawn_recursive();
                }
            }

            // Spawn new highlights that are not in the old ones
            for pos in player_movable_positions {
                if !old_highlight_positions.contains(&pos) {
                    commands.spawn((
                        Name::new(format!("Highlight ({}, {})", pos.x, pos.y)),
                        StateScoped(GameState::Game),
                        StateScoped(GamePauseState::Play),
                        TileBundle {
                            sprite: SpriteBundle {
                                texture: asset_server.load("custom/spritesheet.png"),
                                transform: Transform::from_xyz(
                                    0.0,
                                    0.0,
                                    globals::HIGHLIGHT_Z_INDEX,
                                ),
                                ..default()
                            },
                            atlas: TextureAtlas {
                                layout: atlas_layout.handle.clone(),
                                index: 3,
                            },
                            tile: Tile,
                        },
                        pos,
                        Highlight,
                    ));
                }
            }
        }
    }
}
