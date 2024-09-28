use bevy::{prelude::*, utils::HashSet};

use crate::{
    board::{
        movement_types::{MovementType, MovementTypes},
        position::BoardPosition,
    },
    globals,
    graphics::spritesheet::SpriteSheetAtlas,
};

use super::creature::{BlocksMovement, Creature, CreatureBundle};

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
            movement_types: MovementTypes(HashSet::from([MovementType::King])),
            blocks_movement: BlocksMovement,
            creature: Creature,
            board_position: BoardPosition::new(4, 4),
        },
        Player,
    ));
}
