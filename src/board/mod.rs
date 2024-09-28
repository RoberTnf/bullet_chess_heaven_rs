use bevy::prelude::*;

pub mod board_map;
pub mod movement_types;
pub mod position;
pub mod tile;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, board_map::register_new_movement_blockers)
            .insert_resource(board_map::BoardMap::new());
    }
}
