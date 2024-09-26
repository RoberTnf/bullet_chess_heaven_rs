use bevy::prelude::*;

use crate::{board::position::Position, resources::spritesheet::SpriteSheetAtlas};

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
    for x in 0..8 {
        for y in 0..8 {
            let position = Position::new(x, y);
            commands.spawn((
                TileBundle {
                    sprite: SpriteBundle {
                        texture: asset_server.load("custom/spritesheet.png"),
                        transform: Transform::from_xyz(0.0, 0.0, 0.0),
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
