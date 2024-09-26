use bevy::prelude::*;

use crate::{
    board::{position::Position, tile::Tile},
    globals,
};

use super::player::Player;

pub fn update_transforms(
    mut tiles: Query<(&Position, &mut Transform), (With<Tile>, Without<Player>)>,
    mut player: Query<(&Position, &mut Transform), (With<Player>, Without<Tile>)>,
) {
    for (tile_pos, mut tile_transform) in tiles.iter_mut() {
        tile_transform.translation.x = tile_pos.x as f32 * globals::TILE_SIZE as f32;
        tile_transform.translation.y = tile_pos.y as f32 * globals::TILE_SIZE as f32;
    }

    for (player_pos, mut player_transform) in player.iter_mut() {
        player_transform.translation.x = player_pos.x as f32 * globals::TILE_SIZE as f32;
        player_transform.translation.y = player_pos.y as f32 * globals::TILE_SIZE as f32;
    }
}
