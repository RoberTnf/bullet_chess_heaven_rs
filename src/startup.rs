use bevy::prelude::*;

use crate::board::{self, board_map};
use crate::events::{attack, click_tile, update_position};
use crate::game_state::{GamePauseState, GameState, TurnState};
use crate::graphics::{camera, spritesheet};
use crate::pieces;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                board::tile::spawn_board,
                pieces::player::spawn_player,
                pieces::enemies::spawn_enemy,
                board::tile::spawn_board,
                camera::setup_camera,
            ),
        )
        .insert_state(GamePauseState::Play)
        .insert_state(GameState::Game)
        .insert_state(TurnState::Player)
        .init_resource::<spritesheet::SpriteSheetAtlas>()
        .insert_resource(board_map::BoardMap::new())
        .insert_resource(ClearColor(Color::srgb(0.063, 0.063, 0.082)))
        .add_event::<click_tile::TileClickedEvent>()
        .add_event::<update_position::UpdatePositionEvent>()
        .add_event::<attack::AttackEvent>();
    }
}
