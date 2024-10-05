use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

use crate::board::position::BoardPosition;

#[derive(Resource, Debug)]
pub struct PossibleMovesCache {
    pub movement_tiles: HashMap<Entity, HashSet<BoardPosition>>,
}

impl PossibleMovesCache {
    pub fn get_movement_tiles(&self, entity: &Entity) -> Option<HashSet<BoardPosition>> {
        self.movement_tiles.get(entity).cloned()
    }

    pub fn refresh_cache(&mut self) {
        self.movement_tiles.clear();
    }

    pub fn add_entity(&mut self, entity: &Entity, possible_moves: HashSet<BoardPosition>) {
        self.movement_tiles.insert(*entity, possible_moves);
    }

    pub fn new() -> Self {
        let movement_tiles = HashMap::new();
        Self { movement_tiles }
    }

    // TODO: Use it to remove entities from the cache when they are removed from the board
    #[allow(dead_code)]
    pub fn remove_entity(&mut self, entity: &Entity) {
        if self.movement_tiles.contains_key(entity) {
            self.movement_tiles.remove(entity);
        }
    }
}
