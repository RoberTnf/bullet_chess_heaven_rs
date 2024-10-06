use bevy::{prelude::*, utils::HashSet};
pub mod animation;

use crate::{
    board::{
        movement_types::{MovementType, MovementTypes},
        position::BoardPosition,
    },
    events::update_position::UpdatePositionEvent,
    game_state::GameState,
    globals,
    graphics::spritesheet::SpriteSheetAtlas,
};

use super::{
    creature::{BlocksMovement, Creature, CreatureBundle, CreatureState},
    damage::Damage,
    health::Health,
};

#[derive(Component)]
pub struct PulseSize {
    pub start_size: f32,
    pub final_size: f32,
    pub progress: f32,
    pub speed: f32,
}

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    atlas_layout: Res<SpriteSheetAtlas>,
    mut update_position_event: EventWriter<UpdatePositionEvent>,
) {
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
            health: Health::new(globals::PLAYER_HEALTH),
            damage: Damage::new(globals::PLAYER_DAMAGE),
        },
        Player,
        Name::new("Player"),
        StateScoped(GameState::Game),
        PulseSize {
            start_size: 1.0,
            final_size: 1.1,
            progress: 0.0,
            speed: globals::PULSE_ANIMATION_SPEED,
        },
    ));

    update_position_event.send(UpdatePositionEvent {
        tile_pos: player_start_pos,
        old_tile_pos: BoardPosition::new(-1, -1),
        piece: entity.id(),
    });
}
