use bevy::{prelude::*, utils::HashSet};
use std::time::Duration;

use bevy_tweening::{lens::TransformScaleLens, Animator, EaseFunction, RepeatCount, Tween};

use crate::{
    board::{
        movement_types::{MovementType, MovementTypes},
        position::BoardPosition,
    },
    events::update_pos::UpdatePositionEvent,
    game_state::GameState,
    globals,
    graphics::spritesheet::SpriteSheetAtlas,
};

use super::creature::{BlocksMovement, Creature, CreatureBundle, CreatureState};

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    atlas_layout: Res<SpriteSheetAtlas>,
    mut update_position_event: EventWriter<UpdatePositionEvent>,
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

    let player_start_pos = BoardPosition::new(4, 4);

    let entity = commands.spawn((
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
            board_position: player_start_pos,
            creature_state: CreatureState::Initializing,
        },
        Player,
        Animator::new(tween),
        Name::new("Player"),
        StateScoped(GameState::Game),
    ));

    update_position_event.send(UpdatePositionEvent {
        tile_pos: player_start_pos,
        old_tile_pos: player_start_pos,
        piece: entity.id(),
    });
}
