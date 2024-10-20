use bevy::{prelude::*, utils::HashSet};

use crate::{
    globals,
    graphics::spritesheet::SpriteSheetAtlas,
    pieces::{
        common::{MovementTypes, Piece, Team},
        movement::MovePiece,
        player::spawn::Player,
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
    other_pieces: Query<(&BoardPosition, &Team), (With<Piece>, Without<Player>)>,
    player: Query<(&BoardPosition, &MovementTypes, &Team), (With<Piece>, With<Player>)>,
) {
    let (player_board_position, player_movement_types, player_team) = player.single();
    if highlight.player_moves.is_empty() && highlight.player_attacks.is_empty() {
        let enemies = other_pieces
            .iter()
            .filter(|(_, team)| **team == *player_team);
        let other_pieces_board_positions = HashSet::from_iter(
            other_pieces
                .iter()
                .map(|(board_position, _)| *board_position),
        );
        let enemies_board_positions =
            HashSet::from_iter(enemies.map(|(board_position, _)| *board_position));

        // fill the highlight with valid moves and attacks
        player_movement_types.0.iter().for_each(|movement_type| {
            let response = movement_type.get_valid_moves(
                player_board_position,
                &other_pieces_board_positions,
                &enemies_board_positions,
            );
            highlight.player_moves.extend(response.valid_moves);
            highlight.player_attacks.extend(response.valid_attacks);
        });
    }
}

/// System that despawns all highlight tiles
pub fn despawn_highlight_tiles(
    mut commands: Commands,
    highlight_tiles_attack: Query<Entity, With<HighlightTileAttack>>,
    highlight_tiles_move: Query<Entity, With<HighlightTileMove>>,
    mut highlight: ResMut<HighlightCache>,
) {
    highlight_tiles_attack.iter().for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
    highlight_tiles_move.iter().for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
    highlight.invalidate();
}

// invalidate cache if piece moved
pub fn invalidate_highlight_cache_on_move(
    mut highlight: ResMut<HighlightCache>,
    mut piece_moved: EventReader<MovePiece>,
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
            present_tiles_attack.insert(*board_position);
        }
    }
    for (entity, board_position) in highlight_tiles_move.iter() {
        if !highlight.player_moves.contains(board_position) {
            commands.entity(entity).despawn_recursive();
        } else {
            present_tiles_move.insert(*board_position);
        }
    }

    // spawn new highlight tiles
    for board_position in highlight.player_attacks.iter() {
        let global_position = board_position
            .as_global_position()
            .extend(globals::BOARD_Z_INDEX);

        if !present_tiles_attack.contains(board_position) {
            commands.spawn((
                Name::new(format!(
                    "Highlight Tile Attack ({}, {})",
                    board_position.x, board_position.y
                )),
                StateScoped(GameState::Game),
                SpriteBundle {
                    texture: asset_server.load("custom/spritesheet.png"),
                    transform: Transform::from_translation(global_position),
                    ..default()
                },
                TextureAtlas {
                    layout: atlas_layout.handle.clone(),
                    index: 6,
                },
                board_position.clone(),
                HighlightTileAttack,
            ));
        }
    }
    for board_position in highlight.player_moves.iter() {
        if !present_tiles_move.contains(board_position) {
            let global_position = board_position
                .as_global_position()
                .extend(globals::HIGHLIGHT_Z_INDEX);
            commands.spawn((HighlightTileMove, board_position.clone()));
            if !present_tiles_attack.contains(board_position) {
                commands.spawn((
                    Name::new(format!(
                        "Highlight Tile Attack ({}, {})",
                        board_position.x, board_position.y
                    )),
                    StateScoped(GameState::Game),
                    SpriteBundle {
                        texture: asset_server.load("custom/spritesheet.png"),
                        transform: Transform::from_translation(global_position),
                        ..default()
                    },
                    TextureAtlas {
                        layout: atlas_layout.handle.clone(),
                        index: 3,
                    },
                    board_position.clone(),
                    HighlightTileMove,
                ));
            }
        }
    }
}

pub struct HighlightPlugin;

impl Plugin for HighlightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (update_highlight_tiles, update_highlight_cache)
                    .run_if(in_state(TurnState::PlayerInput))
                    .run_if(in_state(GameState::Game))
                    .run_if(in_state(GamePauseState::Play)),
                despawn_highlight_tiles
                    .run_if(in_state(TurnState::PlayerAnimation))
                    .run_if(in_state(GameState::Game)),
                invalidate_highlight_cache_on_move,
            ),
        );
    }
}
