use bevy::prelude::*;

use crate::{board::position::BoardPosition, globals, graphics::spritesheet::SpriteSheetAtlas};

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
    for x in 0..globals::BOARD_SIZE {
        for y in 0..globals::BOARD_SIZE {
            let position = BoardPosition::new(x, y);
            commands.spawn((
                TileBundle {
                    sprite: SpriteBundle {
                        texture: asset_server.load("custom/spritesheet.png"),
                        transform: Transform::from_xyz(0.0, 0.0, globals::BOARD_Z_INDEX),
                        ..default()
                    },
                    atlas: TextureAtlas {
                        layout: atlas_layout.handle.clone(),
                        index: if position.is_white() { 2 } else { 1 },
                    },
                    tile: Tile,
                },
                position,
            ));
        }
    }
}
