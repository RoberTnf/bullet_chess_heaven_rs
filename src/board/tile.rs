use bevy::prelude::*;

use crate::{
    board::position::BoardPosition, globals, graphics::spritesheet::SpriteSheetAtlas,
    states::game_state::GameState,
};

#[derive(Component)]
#[require(Sprite)]
pub struct Tile;

pub fn spawn_board(
    mut commands: Commands,
    atlas_layout: Res<SpriteSheetAtlas>,
    asset_server: Res<AssetServer>,
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
                Tile,
                Sprite {
                    texture_atlas: Some(TextureAtlas {
                        layout: atlas_layout.handle.clone(),
                        index: if tile_position.is_white() { 2 } else { 1 },
                    }),
                    image: asset_server.load("custom/spritesheet.png"),
                    ..default()
                },
                Transform::from_translation(global_position),
                tile_position,
            ));
        }
    }
}
