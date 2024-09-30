use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

use crate::board::position::BoardPosition;

#[derive(Resource)]
pub struct PossibleMovesCache {
    pub movement_tiles: HashMap<BoardPosition, HashSet<BoardPosition>>,
}

impl PossibleMovesCache {
    pub fn get_movement_tiles(&self, position: &BoardPosition) -> HashSet<BoardPosition> {
        self.movement_tiles
            .get(position)
            .cloned()
            .unwrap_or_default()
    }

    pub fn update_movement_tiles(
        &mut self,
        position: &BoardPosition,
        old_position: &BoardPosition,
        possible_moves: HashSet<BoardPosition>,
    ) {
        self.movement_tiles.remove(old_position);
        self.movement_tiles.insert(*position, possible_moves);
    }

    pub fn new() -> Self {
        let movement_tiles = HashMap::new();
        Self { movement_tiles }
    }
}
