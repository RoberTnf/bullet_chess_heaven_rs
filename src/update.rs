use bevy::prelude::*;

use crate::{
    board::{board_map, highlight, movement_types::cache::RefreshCacheEvent},
    events::{attack, click_tile, update_position},
    game_state::{GamePauseState, GameState, TurnState},
    graphics::transforms,
    input::player_movement,
    pieces::{
        self,
        enemies::{movement, pawn},
        health::{self},
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
                    board_map::register_new_movement_blockers.after(pieces::enemies::spawn_enemies),
                    board_map::remove_dead_entities,
                    // Position updates and highlighting
                    highlight::highlight_player_movable_positions
                        .run_if(on_event::<RefreshCacheEvent>())
                        .after(update_position::update_position),
                    // Transform updates
                    transforms::update_transforms
                        .after(highlight::highlight_player_movable_positions),
                    // Player input (only during player's turn)
                    (player_movement::mouse_input, click_tile::tile_clicked)
                        .chain()
                        .run_if(in_state(TurnState::Player)),
                    // Spawn enmies (environment turn)
                    pieces::enemies::spawn_enemies.run_if(in_state(TurnState::Environment)),
                    // Enemy movement (only during enemy's turn)
                    movement::enemy_movement.run_if(in_state(TurnState::Enemy)),
                    // Position update (after player input and enemy movement)
                    update_position::update_position
                        .after(click_tile::tile_clicked)
                        .after(movement::enemy_movement),
                    // Attack system
                    attack::attack_system
                        .after(click_tile::tile_clicked)
                        .after(movement::enemy_movement),
                    // Pawn promotion
                    pawn::promote_pawn.after(update_position::update_position),
                    // Health systems
                    health::animate_health_change,
                    health::health_change_text_animation,
                    // death systems
                    health::death_system.after(update_position::update_position),
                    health::death_animation,
                )
                    .run_if(in_state(GameState::Game))
                    .run_if(in_state(GamePauseState::Play)),
            );
    }
}
