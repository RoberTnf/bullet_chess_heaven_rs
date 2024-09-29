use bevy::prelude::*;

use crate::{
    game_state::{GamePauseState, GameState},
    input::player_movement,
};
pub mod click_tile;
mod update_pos;

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_movement::mouse_input,
                click_tile::tile_clicked,
                update_pos::update_position,
            )
                .chain()
                .run_if(in_state(GameState::Game))
                .run_if(in_state(GamePauseState::Play)),
        )
        .add_event::<click_tile::TileClickedEvent>()
        .add_event::<update_pos::UpdatePositionEvent>();
    }
}
