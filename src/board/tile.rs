use bevy::prelude::*;

use crate::{
    board::position::BoardPosition, globals, graphics::spritesheet::SpriteSheetAtlas,
    states::game_state::GameState,
};

#[derive(Bundle)]
pub struct TileBundle {
    pub sprite: SpriteBundle,
    pub atlas: TextureAtlas,
    pub tile: Tile,
}

#[derive(Component)]
pub struct Tile;

pub fn spawn_board(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    atlas_layout: Res<SpriteSheetAtlas>,
) {
    debug!("Spawning board");
    for x in 0..globals::BOARD_SIZE {
        for y in 0..globals::BOARD_SIZE {
            let tile_position = BoardPosition::new(x, y).unwrap();
            let global_position = tile_position
                .as_global_position()
                .extend(globals::BOARD_Z_INDEX);
            commands.spawn((
                Name::new(format!("Tile ({}, {})", x, y)),
                StateScoped(GameState::Game),
                TileBundle {
                    sprite: SpriteBundle {
                        texture: asset_server.load("custom/spritesheet.png"),
                        transform: Transform::from_translation(global_position),
                        ..default()
                    },
                    atlas: TextureAtlas {
                        layout: atlas_layout.handle.clone(),
                        index: if tile_position.is_white() { 2 } else { 1 },
                    },
                    tile: Tile,
                },
                tile_position,
            ));
        }
    }
}
