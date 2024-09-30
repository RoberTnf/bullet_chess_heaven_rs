use bevy::prelude::*;

use crate::{
    events::update_pos::UpdatePositionEvent,
    game_state::{GamePauseState, GameState},
    graphics::transforms,
};

pub mod board_map;
pub mod highlight;
pub mod movement_types;
pub mod position;
pub mod tile;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                board_map::register_new_movement_blockers,
                (
                    board_map::update_cache_on_move,
                    highlight::highlight_player_movable_positions,
                )
                    .run_if(on_event::<UpdatePositionEvent>())
                    .chain(),
                transforms::update_transforms.after(highlight::highlight_player_movable_positions),
            )
                .run_if(in_state(GameState::Game))
                .run_if(in_state(GamePauseState::Play)),
        )
        .insert_resource(board_map::BoardMap::new());
    }
}
