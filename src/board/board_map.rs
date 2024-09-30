use bevy::{prelude::*, utils::HashMap};

use crate::{
    events::update_pos::UpdatePositionEvent,
    globals,
    pieces::creature::{BlocksMovement, Creature},
};

use super::{
    movement_types::{cache::PossibleMovesCache, MovementType, MovementTypes},
    position::BoardPosition,
};

#[derive(Resource)]
pub struct BoardMap {
    pub creatures: HashMap<BoardPosition, Entity>,
    pub possible_moves_cache: PossibleMovesCache,
}

impl BoardMap {
    pub fn get_entity_at(&self, pos: BoardPosition) -> Option<&Entity> {
        self.creatures.get(&pos)
    }

    pub fn is_movable(&self, pos: BoardPosition) -> bool {
        let is_empty = self.get_entity_at(pos).is_none();
        let is_off_limits =
            pos.x >= globals::BOARD_SIZE || pos.y >= globals::BOARD_SIZE || pos.x < 0 || pos.y < 0;
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
            possible_moves_cache: PossibleMovesCache::new(),
        }
    }

    pub fn move_entity(&mut self, old_pos: BoardPosition, new_pos: BoardPosition) {
        let entity = self
            .remove_entity(old_pos)
            .expect("Move Entity: Entity not found");
        self.add_entity(new_pos, entity);
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

pub fn update_cache_on_move(
    mut board_map: ResMut<BoardMap>,
    creatures: Query<(&BoardPosition, &MovementTypes, &Name)>,
    mut update_position_event: EventReader<UpdatePositionEvent>,
) {
    for event in update_position_event.read() {
        let (creature_pos, movement_types, name) =
            creatures.get(event.piece).expect("Invalid entity");
        debug!("Updating cache on for {name} at {:?}", creature_pos);
        let possible_moves = movement_types.get_movement_tiles(creature_pos, &board_map);
        board_map.possible_moves_cache.update_movement_tiles(
            &event.tile_pos,
            &event.old_tile_pos,
            possible_moves,
        );
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use crate::{
        board::{
            board_map::{register_new_movement_blockers, BoardMap},
            position::BoardPosition,
        },
        globals,
        pieces::creature::BlocksMovement,
    };

    #[test]
    fn test_new_board_map_is_empty() {
        let board_map = BoardMap::new();
        assert!(board_map.creatures.is_empty());
    }

    #[test]
    fn test_add_and_get_entity() {
        let mut board_map = BoardMap::new();
        let pos = BoardPosition::new(1, 1);
        let entity = Entity::from_raw(1);

        board_map.add_entity(pos, entity);
        assert_eq!(board_map.get_entity_at(pos), Some(&entity));
    }

    #[test]
    fn test_remove_entity() {
        let mut board_map = BoardMap::new();
        let pos = BoardPosition::new(1, 1);
        let entity = Entity::from_raw(1);

        board_map.add_entity(pos, entity);
        let removed = board_map.remove_entity(pos);
        assert_eq!(removed, Some(entity));
        assert_eq!(board_map.get_entity_at(pos), None);
    }

    #[test]
    fn test_is_movable() {
        let mut board_map = BoardMap::new();
        let pos_empty = BoardPosition::new(1, 1);
        let pos_occupied = BoardPosition::new(2, 2);
        let pos_off_limits = BoardPosition::new(globals::BOARD_SIZE, globals::BOARD_SIZE);
        let pos_off_limits_negative = BoardPosition::new(-1, -1);

        board_map.add_entity(pos_occupied, Entity::from_raw(1));

        assert!(board_map.is_movable(pos_empty));
        assert!(!board_map.is_movable(pos_occupied));
        assert!(!board_map.is_movable(pos_off_limits));
        assert!(!board_map.is_movable(pos_off_limits_negative));
    }

    #[test]
    fn test_register_new_movement_blockers() {
        let mut app = App::new();

        let board_map = BoardMap::new();
        app.insert_resource(board_map)
            .add_systems(Update, register_new_movement_blockers);

        let world = app.world_mut();

        let entity1 = world.spawn((BoardPosition::new(1, 1), BlocksMovement)).id();
        let entity2 = world.spawn((BoardPosition::new(2, 2), BlocksMovement)).id();

        app.update();

        assert_eq!(
            app.world()
                .get_resource::<BoardMap>()
                .unwrap()
                .get_entity_at(BoardPosition::new(1, 1)),
            Some(&entity1)
        );
        assert_eq!(
            app.world()
                .get_resource::<BoardMap>()
                .unwrap()
                .get_entity_at(BoardPosition::new(2, 2)),
            Some(&entity2)
        );
    }
}
