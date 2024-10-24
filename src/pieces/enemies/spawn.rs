use crate::{
    board::position::{BoardPosition, PositionAvailable},
    globals::{ENEMY_Z_INDEX, PER_TURN_ENEMY_SPAWN_COUNT, TARGET_NUM_ENEMIES},
    graphics::spritesheet::SpriteSheetAtlas,
    pieces::{
        common::{BlocksMovement, Piece, PieceBundle, PieceState, Team},
        damage::Damage,
        enemies::{
            bishop::{BLACK_BISHOP_INFO, WHITE_BISHOP_INFO},
            king::{BLACK_KING_INFO, WHITE_KING_INFO},
            knight::{BLACK_KNIGHT_INFO, WHITE_KNIGHT_INFO},
            queen::{BLACK_QUEEN_INFO, WHITE_QUEEN_INFO},
            rook::{BLACK_ROOK_INFO, WHITE_ROOK_INFO},
        },
        experience::PieceValue,
        health::Health,
        healthbar::spawn_healthbar,
        movement_type::MovementType,
    },
    states::{
        game_state::GameState,
        turn_state::{TurnInfo, TurnState},
    },
};
use bevy::{prelude::*, utils::HashSet};
use rand::prelude::*;

use super::{
    pawn::{BLACK_PAWN_INFO, WHITE_PAWN_INFO},
    Enemy, PieceInfo,
};

fn get_random_piece_info(turn_info: &Res<TurnInfo>) -> PieceInfo {
    let pieces = [
        WHITE_PAWN_INFO.clone(),
        BLACK_PAWN_INFO.clone(),
        WHITE_KING_INFO.clone(),
        BLACK_KING_INFO.clone(),
        WHITE_QUEEN_INFO.clone(),
        BLACK_QUEEN_INFO.clone(),
        WHITE_ROOK_INFO.clone(),
        BLACK_ROOK_INFO.clone(),
        WHITE_BISHOP_INFO.clone(),
        BLACK_BISHOP_INFO.clone(),
        WHITE_KNIGHT_INFO.clone(),
        BLACK_KNIGHT_INFO.clone(),
    ];

    let spawnable_pieces = pieces
        .iter()
        .filter(|p| turn_info.number >= p.spawn_turn)
        .collect::<Vec<_>>();

    let total_weight = spawnable_pieces.iter().map(|p| p.spawn_weight).sum::<f64>();
    let mut random_value = rand::thread_rng().gen_range(0.0..total_weight);

    for &piece in spawnable_pieces.iter() {
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
    turn_info: Res<TurnInfo>,
) {
    debug!("Spawning enemies");
    let num_enemies = enemies.iter().count();
    let enemies_to_spawn = (TARGET_NUM_ENEMIES - num_enemies).clamp(0, PER_TURN_ENEMY_SPAWN_COUNT);
    let mut occupied_positions = HashSet::from_iter(piece_position_query.iter().copied());
    let all_positions = vec![
        PositionAvailable::Top,
        PositionAvailable::Bottom,
        PositionAvailable::Left,
        PositionAvailable::Right,
    ];
    for _ in 0..enemies_to_spawn {
        let piece_info = get_random_piece_info(&turn_info);
        let tile_pos = get_spawn_position(&piece_info, &occupied_positions, &all_positions);

        occupied_positions.insert(tile_pos);
        let global_position = tile_pos.as_global_position().extend(ENEMY_Z_INDEX);
        let enemy = commands
            .spawn((
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
                PieceValue {
                    value: piece_info.value,
                },
            ))
            .id();

        let healthbars = spawn_healthbar(&mut commands, &asset_server, &atlas_layout.handle);
        commands.entity(enemy).push_children(&healthbars);
    }
    next_turn_state.set(TurnState::PlayerInput);
}

fn get_spawn_position(
    piece_info: &PieceInfo,
    occupied_positions: &HashSet<BoardPosition>,
    all_positions: &[PositionAvailable],
) -> BoardPosition {
    if piece_info
        .movement_types
        .0
        .contains(&MovementType::WhitePawn)
    {
        BoardPosition::get_random_position_limited(occupied_positions, &[PositionAvailable::Bottom])
    } else if piece_info
        .movement_types
        .0
        .contains(&MovementType::BlackPawn)
    {
        BoardPosition::get_random_position_limited(occupied_positions, &[PositionAvailable::Top])
    } else {
        BoardPosition::get_random_position_limited(occupied_positions, all_positions)
    }
}
