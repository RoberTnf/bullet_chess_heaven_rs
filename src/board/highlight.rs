use bevy::{prelude::*, utils::HashSet};

use crate::{
    globals,
    graphics::spritesheet::SpriteSheetAtlas,
    input::click_tile::HoveredTile,
    pieces::{
        attack::attack_piece_system,
        common::{Piece, Team},
        health::DeathAnimation,
        movement::{move_piece, MovePieceEvent},
        player::{spawn::Player, upgrades::data::Upgrades},
    },
    states::{game_state::GameState, pause_state::GamePauseState, turn_state::TurnState},
};

use super::position::BoardPosition;

#[derive(Resource, Default)]
pub struct HighlightCache {
    pub player_attacks: HashSet<BoardPosition>,
    pub player_moves: HashSet<BoardPosition>,
}

#[derive(Component)]
pub struct HighlightTileMove;

#[derive(Component)]
pub struct HighlightTileAttack;

#[derive(Component)]
pub struct HighlightHoveredTile;

impl HighlightCache {
    pub fn new() -> Self {
        Self {
            player_attacks: HashSet::new(),
            player_moves: HashSet::new(),
        }
    }

    pub fn invalidate(&mut self) {
        self.player_attacks.clear();
        self.player_moves.clear();
    }
}

// system that updates the highlight cache
pub fn update_highlight_cache(
    mut highlight: ResMut<HighlightCache>,
    other_pieces: Query<
        (&BoardPosition, &Team),
        (With<Piece>, Without<Player>, Without<DeathAnimation>),
    >,
    player: Query<(&BoardPosition, &Upgrades, &Team), (With<Piece>, With<Player>)>,
) {
    let (player_board_position, player_upgrades, player_team) = player.single();
    if highlight.player_moves.is_empty() && highlight.player_attacks.is_empty() {
        let enemies_board_positions = HashSet::from_iter(
            other_pieces
                .iter()
                .filter(|(_, &team)| team != *player_team)
                .map(|(board_position, _)| *board_position),
        );

        let other_pieces_board_positions = HashSet::from_iter(
            other_pieces
                .iter()
                .map(|(board_position, _)| *board_position),
        );
        let movement_types = player_upgrades.get_movement_types_set();

        // fill the highlight with valid moves and attacks
        for movement_type in movement_types {
            let response = movement_type.get_valid_moves(
                player_board_position,
                &other_pieces_board_positions,
                &enemies_board_positions,
            );
            highlight.player_moves.extend(response.valid_moves);
            highlight.player_attacks.extend(response.valid_attacks);
        }
    }
}

/// System that despawns all highlight tiles
pub fn despawn_highlight_tiles(
    mut commands: Commands,
    highlight_tiles_attack: Query<Entity, With<HighlightTileAttack>>,
    highlight_tiles_move: Query<Entity, With<HighlightTileMove>>,
    hovered_tile: Query<Entity, With<HighlightHoveredTile>>,
    mut highlight: ResMut<HighlightCache>,
) {
    highlight_tiles_attack.iter().for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
    highlight_tiles_move.iter().for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
    hovered_tile.iter().for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
    highlight.invalidate();
}

// invalidate cache if piece moved
pub fn invalidate_highlight_cache_on_move(
    mut highlight: ResMut<HighlightCache>,
    mut piece_moved: EventReader<MovePieceEvent>,
) {
    piece_moved.read().for_each(|_| {
        highlight.invalidate();
    });
}

// system that spawns the highlight tiles
pub fn update_highlight_tiles(
    mut commands: Commands,
    highlight_tiles_attack: Query<(Entity, &BoardPosition), With<HighlightTileAttack>>,
    highlight_tiles_move: Query<(Entity, &BoardPosition), With<HighlightTileMove>>,
    highlight_hovered_tile: Query<Entity, With<HighlightHoveredTile>>,
    hovered_tile: Res<HoveredTile>,
    highlight: Res<HighlightCache>,
    asset_server: Res<AssetServer>,
    atlas_layout: Res<SpriteSheetAtlas>,
) {
    let mut present_tiles_attack = HashSet::new();
    let mut present_tiles_move = HashSet::new();
    // despawn highlight tiles not in the highlight cache
    for (entity, board_position) in highlight_tiles_attack.iter() {
        if !highlight.player_attacks.contains(board_position) {
            commands.entity(entity).despawn_recursive();
        } else {
            if let Some(hovered_tile) = hovered_tile.0 {
                if hovered_tile == *board_position {
                    commands.entity(entity).despawn_recursive();
                    continue;
                }
            }
            present_tiles_attack.insert(*board_position);
        }
    }
    for (entity, board_position) in highlight_tiles_move.iter() {
        if !highlight.player_moves.contains(board_position) {
            commands.entity(entity).despawn_recursive();
        } else {
            if let Some(hovered_tile) = hovered_tile.0 {
                if hovered_tile == *board_position {
                    commands.entity(entity).despawn_recursive();
                    continue;
                }
            }
            present_tiles_move.insert(*board_position);
        }
    }

    // deal with hovered tile
    if hovered_tile.is_changed() {
        highlight_hovered_tile.iter().for_each(|entity| {
            commands.entity(entity).despawn_recursive();
        });
        if let Some(hovered_tile) = hovered_tile.0 {
            commands.spawn((
                Name::new("Highlight Hovered Tile"),
                StateScoped(GameState::Game),
                HighlightHoveredTile,
                Sprite {
                    texture_atlas: Some(TextureAtlas {
                        layout: atlas_layout.handle.clone(),
                        index: 23,
                    }),
                    image: asset_server.load("custom/spritesheet.png"),
                    ..default()
                },
                Transform::from_translation(
                    hovered_tile
                        .as_global_position()
                        .extend(globals::HIGHLIGHT_Z_INDEX),
                ),
            ));
        }
    }

    // spawn new highlight tiles
    for board_position in highlight.player_attacks.iter() {
        let global_position = board_position
            .as_global_position()
            .extend(globals::BOARD_Z_INDEX);

        if let Some(hovered_tile) = hovered_tile.0 {
            if hovered_tile == *board_position {
                continue;
            }
        }

        if !present_tiles_attack.contains(board_position) {
            commands.spawn((
                Name::new(format!(
                    "Highlight Tile Attack ({}, {})",
                    board_position.x, board_position.y
                )),
                StateScoped(GameState::Game),
                HighlightTileAttack,
                Sprite {
                    texture_atlas: Some(TextureAtlas {
                        layout: atlas_layout.handle.clone(),
                        index: 6,
                    }),
                    image: asset_server.load("custom/spritesheet.png"),
                    ..default()
                },
                Transform::from_translation(global_position),
                *board_position,
            ));
        }
    }
    for board_position in highlight.player_moves.iter() {
        if let Some(hovered_tile) = hovered_tile.0 {
            if hovered_tile == *board_position {
                continue;
            }
        }
        if !present_tiles_move.contains(board_position) {
            let global_position = board_position
                .as_global_position()
                .extend(globals::HIGHLIGHT_Z_INDEX);
            commands.spawn((HighlightTileMove, *board_position));
            if !present_tiles_attack.contains(board_position) {
                commands.spawn((
                    Name::new(format!(
                        "Highlight Tile Move ({}, {})",
                        board_position.x, board_position.y
                    )),
                    StateScoped(GameState::Game),
                    HighlightTileMove,
                    Sprite {
                        texture_atlas: Some(TextureAtlas {
                            layout: atlas_layout.handle.clone(),
                            index: 3,
                        }),
                        image: asset_server.load("custom/spritesheet.png"),
                        ..default()
                    },
                    Transform::from_translation(global_position),
                    *board_position,
                ));
            }
        }
    }
}

fn restart_cleanup(
    mut commands: Commands,
    mut highlight: ResMut<HighlightCache>,
    highlight_tiles_move: Query<Entity, With<HighlightTileMove>>,
    highlight_tiles_attack: Query<Entity, With<HighlightTileAttack>>,
) {
    highlight.invalidate();
    for entity in highlight_tiles_move.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in highlight_tiles_attack.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct HighlightPlugin;

impl Plugin for HighlightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (update_highlight_cache, update_highlight_tiles)
                    .chain()
                    .before(move_piece)
                    .before(attack_piece_system)
                    .run_if(in_state(TurnState::PlayerInput))
                    .run_if(in_state(GameState::Game))
                    .run_if(in_state(GamePauseState::Playing)),
                despawn_highlight_tiles
                    .run_if(in_state(TurnState::PlayerAnimation))
                    .run_if(in_state(GameState::Game)),
                invalidate_highlight_cache_on_move,
            ),
        );
        app.add_systems(OnEnter(GameState::Game), restart_cleanup);
    }
}
