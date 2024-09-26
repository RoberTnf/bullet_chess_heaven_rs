use bevy::prelude::*;

use crate::{board::position::Position, resources::spritesheet::SpriteSheetAtlas};

use super::creature::Creature;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    atlas_layout: Res<SpriteSheetAtlas>,
) {
    commands.spawn((
        Creature {
            sprite: SpriteBundle {
                texture: asset_server.load("custom/spritesheet.png"),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            atlas: TextureAtlas {
                layout: atlas_layout.handle.clone(),
                index: 0,
            },
        },
        Player,
        Position::new(4, 4),
    ));
}
