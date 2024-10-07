use bevy::prelude::*;

use crate::{
    board::position::BoardPosition,
    globals,
    graphics::spritesheet::SpriteSheetAtlas,
    pieces::{
        common::{BlocksMovement, Piece, PieceBundle, PieceState},
        damage::Damage,
        health::Health,
    },
    states::game_state::GameState,
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
) {
    let tile_pos = BoardPosition::new(4, 4);
    let global_position = tile_pos
        .as_global_position()
        .extend(globals::PLAYER_Z_INDEX);

    commands.spawn((
        PieceBundle {
            sprite: SpriteBundle {
                texture: asset_server.load("custom/spritesheet.png"),
                transform: Transform::from_translation(global_position),
                ..default()
            },
            atlas: TextureAtlas {
                layout: atlas_layout.handle.clone(),
                index: 0,
            },
            blocks_movement: BlocksMovement,
            creature: Piece,
            board_position: tile_pos,
            health: Health::new(globals::PLAYER_HEALTH),
            damage: Damage::new(globals::PLAYER_DAMAGE),
            state: PieceState::Idle,
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
}
