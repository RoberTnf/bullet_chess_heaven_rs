use bevy::prelude::*;
use pawn::get_white_pawn_config;

pub mod movement;
pub mod pawn;

use crate::{
    board::{movement_types::MovementTypes, position::BoardPosition},
    events::update_position::UpdatePositionEvent,
    game_state::GameState,
    globals,
    graphics::spritesheet::SpriteSheetAtlas,
};

use super::creature::{BlocksMovement, Creature, CreatureBundle, CreatureState};

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub enum EnemyType {
    WhitePawn,
    BlackPawn,
}

pub struct PieceConfig {
    sprite_tile_id: usize,
    movement_set: MovementTypes,
}

pub fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    atlas_layout: Res<SpriteSheetAtlas>,
    mut update_position_event: EventWriter<UpdatePositionEvent>,
) {
    let start_pos = BoardPosition::new(0, 6);
    let piece_config = get_white_pawn_config();

    let entity = commands.spawn((
        CreatureBundle {
            sprite: SpriteBundle {
                texture: asset_server.load("custom/spritesheet.png"),
                transform: Transform::from_xyz(0.0, 0.0, globals::PLAYER_Z_INDEX),
                ..default()
            },
            atlas: TextureAtlas {
                layout: atlas_layout.handle.clone(),
                index: piece_config.sprite_tile_id,
            },
            movement_types: piece_config.movement_set,
            blocks_movement: BlocksMovement,
            creature: Creature,
            board_position: start_pos,
            creature_state: CreatureState::Initializing,
        },
        Enemy,
        Name::new("Enemy"),
        StateScoped(GameState::Game),
        EnemyType::WhitePawn,
    ));

    update_position_event.send(UpdatePositionEvent {
        tile_pos: start_pos,
        old_tile_pos: BoardPosition::new(-1, -1),
        piece: entity.id(),
    });
}
