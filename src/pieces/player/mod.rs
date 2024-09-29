use bevy::{prelude::*, utils::HashSet};
use std::time::Duration;

use bevy_tweening::{lens::TransformScaleLens, Animator, EaseFunction, RepeatCount, Tween};

use crate::{
    board::{
        movement_types::{MovementType, MovementTypes},
        position::BoardPosition,
    },
    game_state::GameState,
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
    let tween = Tween::new(
        EaseFunction::SineInOut,
        Duration::from_secs(1),
        TransformScaleLens {
            start: Vec3::new(0.95, 0.95, 0.95),
            end: Vec3::new(0.8, 0.8, 0.8),
        },
    )
    .with_repeat_strategy(bevy_tweening::RepeatStrategy::MirroredRepeat)
    .with_repeat_count(RepeatCount::Infinite);

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
        Animator::new(tween),
        Name::new("Player"),
        StateScoped(GameState::Game),
    ));
}
