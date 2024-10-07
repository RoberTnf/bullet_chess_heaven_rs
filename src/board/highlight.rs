use bevy::{prelude::*, utils::HashSet};

use crate::{
    events::update_position::UpdatePositionEvent,
    game_state::{GamePauseState, GameState},
    globals,
    graphics::spritesheet::SpriteSheetAtlas,
    pieces::player::Player,
};

use super::{
    board_map::BoardMap,
    movement_types::MovementTypes,
    position::BoardPosition,
    tile::{Tile, TileBundle},
};

#[derive(Component)]
pub enum Highlight {
    Movement,
    Attack,
}

pub fn highlight_player_movable_positions(
    mut commands: Commands,
    mut board_map: ResMut<BoardMap>,
    player_query: Query<(Entity, &BoardPosition, &MovementTypes), With<Player>>,
    asset_server: Res<AssetServer>,
    atlas_layout: Res<SpriteSheetAtlas>,
    highlight_query: Query<(Entity, &BoardPosition, &Highlight)>,
) {
    debug!("Highlighting player positions");
    let (player_entity, player_pos, movement_types) = player_query.single();

    let possible_moves = board_map.get_possible_moves(&player_entity, movement_types, player_pos);
    let moves: HashSet<_> = possible_moves.movement_tiles.into_iter().collect();
    let attacks: HashSet<_> = possible_moves
        .attack_tiles
        .into_iter()
        .map(|(pos, _)| pos.0)
        .collect();

    // Remove old highlights and collect current positions
    let mut current_highlights = HashSet::new();
    for (entity, pos, highlight) in highlight_query.iter() {
        let should_keep = match highlight {
            Highlight::Attack => attacks.contains(pos),
            Highlight::Movement => moves.contains(pos),
        };

        if should_keep {
            current_highlights.insert(*pos);
        } else {
            commands.entity(entity).despawn_recursive();
        }
    }

    // Spawn new highlights
    for pos in moves.union(&attacks) {
        if !current_highlights.contains(pos) {
            spawn_highlight(
                &mut commands,
                &asset_server,
                &atlas_layout,
                pos,
                attacks.contains(pos),
            );
        }
    }
}

fn spawn_highlight(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    atlas_layout: &Res<SpriteSheetAtlas>,
    pos: &BoardPosition,
    is_attack: bool,
) {
    let (name, index, component) = if is_attack {
        (
            format!("Highlight Attack({}, {})", pos.x, pos.y),
            globals::HIGHLIGHT_ATTACK_ATLAS_INDEX,
            Highlight::Attack,
        )
    } else {
        (
            format!("Highlight ({}, {})", pos.x, pos.y),
            globals::HIGHLIGHT_ATLAS_INDEX,
            Highlight::Movement,
        )
    };

    commands.spawn((
        Name::new(name),
        StateScoped(GameState::Game),
        StateScoped(GamePauseState::Play),
        TileBundle {
            sprite: SpriteBundle {
                texture: asset_server.load("custom/spritesheet.png"),
                transform: Transform::from_xyz(0.0, 0.0, globals::HIGHLIGHT_Z_INDEX),
                ..default()
            },
            atlas: TextureAtlas {
                layout: atlas_layout.handle.clone(),
                index,
            },
            tile: Tile,
        },
        *pos,
        component,
    ));
}
