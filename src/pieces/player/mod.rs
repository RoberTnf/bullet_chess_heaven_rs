use bevy::prelude::*;

use crate::{board::position::BoardPosition, globals, graphics::spritesheet::SpriteSheetAtlas};

use super::creature::CreatureBundle;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    atlas_layout: Res<SpriteSheetAtlas>,
) {
    commands.spawn((
        CreatureBundle {
            sprite: SpriteBundle {
                texture: asset_server.load("custom/spritesheet.png"),
                transform: Transform::from_xyz(0.0, 0.0, globals::PLAYER_Z_INDEX),
                ..default()
            },
            atlas: TextureAtlas {
                layout: atlas_layout.handle.clone(),
                index: 0,
            },
            ..default()
        },
        Player,
        BoardPosition::new(4, 4),
    ));
}
