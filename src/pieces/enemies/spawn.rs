use crate::{
    board::position::BoardPosition,
    globals::{ENEMY_Z_INDEX, PER_TURN_ENEMY_SPAWN_COUNT, TARGET_NUM_ENEMIES},
    graphics::spritesheet::SpriteSheetAtlas,
    pieces::{
        common::{BlocksMovement, Piece, PieceBundle, PieceState, Team},
        damage::Damage,
        health::Health,
    },
    states::{game_state::GameState, turn_state::TurnState},
};
use bevy::{prelude::*, utils::HashSet};
use rand::prelude::*;

use super::{
    pawn::{BLACK_PAWN_INFO, WHITE_PAWN_INFO},
    Enemy, PieceInfo,
};

fn get_random_piece_info() -> PieceInfo {
    let pieces = [WHITE_PAWN_INFO.clone(), BLACK_PAWN_INFO.clone()];
    let total_weight = pieces.iter().map(|p| p.spawn_weight).sum::<f64>();
    let mut random_value = rand::thread_rng().gen_range(0.0..total_weight);

    for piece in pieces.iter() {
        if random_value < piece.spawn_weight {
            return piece.clone();
        }
        random_value -= piece.spawn_weight;
    }

    // This should never happen if the weights are positive
    warn!("Logic error: no piece selected randomly, defaulting to last piece");
    pieces.last().unwrap().clone()
}

pub fn spawn_enemies(
    mut commands: Commands,
    enemies: Query<Entity, With<Enemy>>,
    asset_server: Res<AssetServer>,
    atlas_layout: Res<SpriteSheetAtlas>,
    piece_position_query: Query<&BoardPosition, With<Piece>>,
    mut next_turn_state: ResMut<NextState<TurnState>>,
) {
    debug!("Spawning enemies");
    let num_enemies = enemies.iter().count();
    let enemies_to_spawn = (TARGET_NUM_ENEMIES - num_enemies).clamp(0, PER_TURN_ENEMY_SPAWN_COUNT);
    let mut occupied_positions = HashSet::from_iter(piece_position_query.iter().copied());

    for _ in 0..enemies_to_spawn {
        let tile_pos = BoardPosition::get_random_empty_position(&occupied_positions);
        occupied_positions.insert(tile_pos);

        let global_position = tile_pos.as_global_position().extend(ENEMY_Z_INDEX);
        let piece_info = get_random_piece_info();

        commands.spawn((
            PieceBundle {
                sprite: SpriteBundle {
                    texture: asset_server.load("custom/spritesheet.png"),
                    transform: Transform::from_translation(global_position),
                    ..default()
                },
                atlas: TextureAtlas {
                    layout: atlas_layout.handle.clone(),
                    index: piece_info.sprite_index,
                },
                blocks_movement: BlocksMovement,
                creature: Piece,
                board_position: tile_pos,
                health: Health::new(piece_info.health),
                damage: Damage::new(piece_info.damage),
                state: PieceState::Idle,
                movement_types: piece_info.movement_types,
                team: Team::Enemy,
            },
            Enemy,
            Name::new("Enemy"),
            StateScoped(GameState::Game),
        ));
    }
    next_turn_state.set(TurnState::PlayerInput);
}
