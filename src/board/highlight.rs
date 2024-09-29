use bevy::{prelude::*, utils::HashSet};

use crate::{
    game_state::{GamePauseState, GameState},
    globals,
    graphics::spritesheet::SpriteSheetAtlas,
    pieces::player::Player,
};

use super::{
    board_map::BoardMap,
    movement_types::MovementTypes,
    position::BoardPosition,
    tile::{Tile, TileBundle},
};

#[derive(Component)]
pub struct Highlight;

#[derive(Component)]
pub struct HighlightPlayerPosition {
    pos: BoardPosition,
}

pub fn highlight_player_movable_positions(
    mut commands: Commands,
    board_map: Res<BoardMap>,
    player_position: Query<(&BoardPosition, &MovementTypes), With<Player>>,
    asset_server: Res<AssetServer>,
    atlas_layout: Res<SpriteSheetAtlas>,
    query_highlights: Query<(Entity, &BoardPosition), With<Highlight>>, // Query to find existing highlights
    old_highlight_player_position: Query<(Entity, &HighlightPlayerPosition)>,
) {
    if let Ok((entity, old_highlight_player_position)) = old_highlight_player_position.get_single()
    {
        let player_position = player_position.get_single().unwrap().0;
        if old_highlight_player_position.pos == *player_position {
            return; // Only highlight if the player has moved
        }
        commands.entity(entity).despawn_recursive();
    }

    if let Ok((player_position, movement_types)) = player_position.get_single() {
        let player_movable_positions =
            movement_types.get_movement_tiles(player_position, &board_map);

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
                            transform: Transform::from_xyz(0.0, 0.0, globals::HIGHLIGHT_Z_INDEX),
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

        commands.spawn(HighlightPlayerPosition {
            pos: player_position.clone(),
        });
    }
}
