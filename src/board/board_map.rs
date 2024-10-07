use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
use rand::Rng;

use crate::{
    globals,
    pieces::{
        creature::{BlocksMovement, CreatureState},
        health::DeathAnimation,
    },
};

use super::{
    movement_types::{
        cache::{PossibleMovesCache, RefreshCacheEvent},
        MovementTypes, MovementTypesResponse,
    },
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
        if let Some(entity) = self.creatures.remove(&pos) {
            debug!("Removing entity at {:?} from board map", pos);
            Some(entity)
        } else {
            None
        }
    }

    pub fn add_entity(&mut self, pos: BoardPosition, entity: Entity) {
        if !self.creatures.contains_key(&pos) {
            debug!("Adding entity {:?} to board map at {:?}", entity, pos);
            self.creatures.insert(pos, entity);
        }
    }

    pub fn new() -> Self {
        Self {
            creatures: HashMap::new(),
            possible_moves_cache: PossibleMovesCache::new(),
        }
    }

    pub fn move_entity(&mut self, old_pos: BoardPosition, new_pos: BoardPosition) {
        debug!("Moving entity from {:?} to {:?}", old_pos, new_pos);
        debug!("Board map: {:?}", self.creatures);
        let entity = self
            .remove_entity(old_pos)
            .expect("Move Entity: Entity not found");
        self.add_entity(new_pos, entity);
    }

    pub fn get_world_position(&self, pos: &BoardPosition) -> Vec2 {
        Vec2::new(
            (pos.x as f32 + 0.5) * globals::TILE_SIZE as f32,
            (pos.y as f32 + 0.5) * globals::TILE_SIZE as f32,
        )
    }

    pub fn get_n_random_empty_tiles(&self, n: usize) -> Vec<BoardPosition> {
        let mut empty_tiles = Vec::new();
        let mut rng = rand::thread_rng();

        while empty_tiles.len() < n {
            let x = rng.gen_range(0..globals::BOARD_SIZE);
            let y = rng.gen_range(0..globals::BOARD_SIZE);
            let pos = BoardPosition::new(x, y);
            if self.is_movable(pos) {
                empty_tiles.push(pos);
            }
        }
        empty_tiles
    }

    pub fn get_possible_moves(
        &mut self,
        entity: &Entity,
        movement_types: &MovementTypes,
        pos: &BoardPosition,
    ) -> MovementTypesResponse {
        if self
            .possible_moves_cache
            .get_movement_tiles(*entity)
            .is_none()
        {
            let possible_moves = movement_types.get_movement_tiles(pos, self);
            self.possible_moves_cache
                .add_entity(entity, possible_moves.clone());
        }
        self.possible_moves_cache
            .get_movement_tiles(*entity)
            .unwrap()
    }

    pub fn refresh_cache(&mut self, event_writer: &mut EventWriter<RefreshCacheEvent>) {
        debug!("Refreshing cache");
        self.possible_moves_cache.refresh_cache();
        event_writer.send(RefreshCacheEvent);
    }
}

pub fn register_new_movement_blockers(
    mut board_map: ResMut<BoardMap>,
    creatures: Query<
        (&BoardPosition, Entity, &CreatureState),
        (With<BlocksMovement>, Without<DeathAnimation>),
    >,
) {
    for (position, entity, creature_state) in creatures.iter() {
        if let CreatureState::Initializing = creature_state {
            // Check if the entity is already present in the board map
            board_map.add_entity(BoardPosition::new(position.x, position.y), entity);
        }
    }
}

pub fn remove_dead_entities(
    mut board_map: ResMut<BoardMap>,
    dead_entities: Query<&BoardPosition, With<DeathAnimation>>,
) {
    for position in dead_entities.iter() {
        board_map.remove_entity(*position);
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
        pieces::creature::{BlocksMovement, CreatureState},
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

        let entity1 = world
            .spawn((
                BoardPosition::new(1, 1),
                BlocksMovement,
                CreatureState::Initializing,
            ))
            .id();
        let entity2 = world
            .spawn((
                BoardPosition::new(2, 2),
                BlocksMovement,
                CreatureState::Initializing,
            ))
            .id();

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
