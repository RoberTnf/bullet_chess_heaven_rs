use bevy::{prelude::*, utils::HashMap};

use crate::{globals, pieces::creature::BlocksMovement};

use super::position::BoardPosition;

#[derive(Resource)]
pub struct BoardMap {
    pub creatures: HashMap<BoardPosition, Entity>,
}

impl BoardMap {
    pub fn get_entity_at(&self, pos: BoardPosition) -> Option<&Entity> {
        self.creatures.get(&pos)
    }

    pub fn is_movable(&self, pos: BoardPosition) -> bool {
        let is_empty = self.get_entity_at(pos).is_none();
        let is_off_limits = pos.x >= globals::BOARD_SIZE || pos.y >= globals::BOARD_SIZE;
        is_empty && !is_off_limits
    }

    pub fn remove_entity(&mut self, pos: BoardPosition) -> Option<Entity> {
        self.creatures.remove(&pos)
    }

    pub fn add_entity(&mut self, pos: BoardPosition, entity: Entity) {
        self.creatures.insert(pos, entity);
    }

    pub fn new() -> Self {
        Self {
            creatures: HashMap::new(),
        }
    }
}

pub fn register_new_movement_blockers(
    mut board_map: ResMut<BoardMap>,
    new_creatures: Query<(&BoardPosition, Entity), With<BlocksMovement>>,
) {
    for (position, entity) in new_creatures.iter() {
        board_map.add_entity(BoardPosition::new(position.x, position.y), entity);
    }
}
