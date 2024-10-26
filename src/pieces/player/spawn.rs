use bevy::prelude::*;

use crate::{
    board::position::BoardPosition,
    globals::{self, PLAYER_ATLAS_INDEX},
    graphics::spritesheet::SpriteSheetAtlas,
    pieces::{
        attack::AttackAfterMove,
        common::{BlocksMovement, MovementTypes, Piece, PieceBundle, PieceState, Team},
        damage::Damage,
        health::Health,
        healthbar::spawn_healthbar,
        movement_type::MovementType,
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
    let tile_pos = BoardPosition::new(4, 4).unwrap();
    let global_position = tile_pos
        .as_global_position()
        .extend(globals::PLAYER_Z_INDEX);

    let player_id = commands
        .spawn((
            PieceBundle {
                sprite: SpriteBundle {
                    texture: asset_server.load("custom/spritesheet.png"),
                    transform: Transform::from_translation(global_position),
                    ..default()
                },
                atlas: TextureAtlas {
                    layout: atlas_layout.handle.clone(),
                    index: PLAYER_ATLAS_INDEX,
                },
                blocks_movement: BlocksMovement,
                creature: Piece,
                board_position: tile_pos,
                health: Health::new(globals::PLAYER_HEALTH),
                damage: Damage::new(globals::PLAYER_DAMAGE),
                state: PieceState::Idle,
                movement_types: MovementTypes(vec![MovementType::Queen, MovementType::Knight]),
                team: Team::Player,
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
            AttackAfterMove,
        ))
        .id();

    let healthbars = spawn_healthbar(&mut commands, &asset_server, &atlas_layout.handle);
    commands.entity(player_id).push_children(&healthbars);
}
