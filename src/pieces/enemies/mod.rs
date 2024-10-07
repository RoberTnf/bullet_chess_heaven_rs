use std::cmp::min;

use bevy::prelude::*;
use pawn::get_white_pawn_config;

pub mod movement;
pub mod pawn;

use crate::{
    board::{board_map::BoardMap, movement_types::MovementTypes, position::BoardPosition},
    events::update_position::UpdatePositionEvent,
    game_state::{GameState, TurnState},
    globals,
    graphics::spritesheet::SpriteSheetAtlas,
};

use super::{
    creature::{BlocksMovement, Creature, CreatureBundle, CreatureState},
    damage::Damage,
    health::{DeathAnimation, Health},
};

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

pub fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    atlas_layout: Res<SpriteSheetAtlas>,
    mut update_position_event: EventWriter<UpdatePositionEvent>,
    enemies: Query<Entity, (With<Enemy>, Without<DeathAnimation>)>,
    board_map: Res<BoardMap>,
    mut turn_state: ResMut<NextState<TurnState>>,
) {
    let piece_config = get_white_pawn_config();

    let enemies_left_to_spawn = globals::TARGET_NUM_ENEMIES.saturating_sub(enemies.iter().count());
    let enemies_to_spawn = min(globals::PER_TURN_ENEMY_SPAWN_COUNT, enemies_left_to_spawn);

    let empty_tiles = board_map.get_n_random_empty_tiles(enemies_to_spawn);

    for pos in empty_tiles {
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
                movement_types: piece_config.movement_set.clone(),
                blocks_movement: BlocksMovement,
                creature: Creature,
                board_position: pos,
                creature_state: CreatureState::Initializing,
                health: Health::new(globals::PAWN_HEALTH),
                damage: Damage::new(globals::PAWN_DAMAGE),
            },
            Enemy,
            Name::new("Enemy"),
            StateScoped(GameState::Game),
            EnemyType::WhitePawn,
        ));

        // update_position_event.send(UpdatePositionEvent {
        //     tile_pos: pos,
        //     old_tile_pos: BoardPosition::new(-1, -1),
        //     piece: entity.id(),
        // });
    }

    turn_state.set(TurnState::Player);
}
