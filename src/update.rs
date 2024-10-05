use bevy::prelude::*;

use crate::{
    board::{board_map, highlight},
    events::{
        click_tile,
        update_position::{self, UpdatePositionEvent},
    },
    game_state::{GamePauseState, GameState, TurnState},
    graphics::transforms,
    input::player_movement,
    pieces::{
        enemies::{movement, pawn},
        player::animation,
    },
};

pub struct UpdatePlugin;

impl Plugin for UpdatePlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems that run in all game states
            .add_systems(
                FixedUpdate,
                (
                    transforms::animate_transforms,
                    animation::animate_pulse_scale,
                ),
            )
            // Systems that run during active gameplay
            .add_systems(
                Update,
                (
                    // Board management
                    board_map::register_new_movement_blockers,
                    // Position updates and highlighting
                    // TODO: Remove caching as a system,
                    // Instead, have a resource CacheStatus and make it dirty on each move
                    // whenever we need a piece possible moves, check the cache status
                    (highlight::highlight_player_movable_positions,)
                        .run_if(on_event::<UpdatePositionEvent>())
                        .after(update_position::update_position),
                    // Transform updates
                    transforms::update_transforms
                        .after(highlight::highlight_player_movable_positions),
                    // Player input (only during player's turn)
                    (player_movement::mouse_input, click_tile::tile_clicked)
                        .chain()
                        .run_if(in_state(TurnState::Player)),
                    // Enemy movement (only during enemy's turn)
                    movement::enemy_movement.run_if(in_state(TurnState::Enemy)),
                    // Position update (after player input and enemy movement)
                    update_position::update_position
                        .after(click_tile::tile_clicked)
                        .after(movement::enemy_movement),
                    // Pawn promotion
                    pawn::promote_pawn.after(update_position::update_position),
                )
                    .run_if(in_state(GameState::Game))
                    .run_if(in_state(GamePauseState::Play)),
            );
    }
}
