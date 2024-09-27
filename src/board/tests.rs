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

    board_map.add_entity(pos_occupied, Entity::from_raw(1));

    assert!(board_map.is_movable(pos_empty));
    assert!(!board_map.is_movable(pos_occupied));
    assert!(!board_map.is_movable(pos_off_limits));
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

#[test]
fn test_new() {
    let pos = BoardPosition::new(3, 4);
    assert_eq!(pos.x, 3);
    assert_eq!(pos.y, 4);
}

#[test]
fn test_update() {
    let mut pos = BoardPosition::new(1, 1);
    pos.update(5, 6);
    assert_eq!(pos.x, 5);
    assert_eq!(pos.y, 6);
}

#[test]
fn test_is_white() {
    assert!(BoardPosition::new(0, 0).is_white());
    assert!(!BoardPosition::new(0, 1).is_white());
    assert!(!BoardPosition::new(1, 0).is_white());
    assert!(BoardPosition::new(1, 1).is_white());
}

#[test]
fn test_from_global_position() {
    let tile_size = globals::TILE_SIZE as f32;

    assert_eq!(
        BoardPosition::from_global_position(0.0, 0.0),
        Some(BoardPosition::new(0, 0))
    );
    assert_eq!(
        BoardPosition::from_global_position(tile_size, tile_size),
        Some(BoardPosition::new(1, 1))
    );
    assert_eq!(
        BoardPosition::from_global_position(tile_size * 2.5, tile_size * 3.5),
        Some(BoardPosition::new(2, 3))
    );
    assert_eq!(BoardPosition::from_global_position(-1.0, 0.0), None);
    assert_eq!(BoardPosition::from_global_position(0.0, -1.0), None);
}

#[test]
fn test_subtraction() {
    let pos1 = BoardPosition::new(5, 7);
    let pos2 = BoardPosition::new(2, 3);
    let result = pos1 - pos2;
    assert_eq!(result, BoardPosition::new(3, 4));
}
