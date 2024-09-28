use crate::board::{board_map::BoardMap, position::BoardPosition};
use bevy::{prelude::Component, utils::HashSet};

pub mod king;

#[derive(Component)]
pub struct MovementTypes(pub HashSet<MovementType>);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MovementType {
    King,
}

impl MovementTypes {
    pub fn get_movement_tiles(
        &self,
        position: &BoardPosition,
        board_map: &BoardMap,
    ) -> HashSet<BoardPosition> {
        let mut movement_tiles = HashSet::new();

        for movement_type in &self.0 {
            match movement_type {
                MovementType::King => {
                    movement_tiles.extend(king::get_movement_tiles(position, board_map));
                }
            }
        }

        movement_tiles
    }
}

pub fn from_directions_short(
    directions: Vec<(i32, i32)>,
    position: &BoardPosition,
    board_map: &BoardMap,
) -> HashSet<BoardPosition> {
    let mut movement_tiles = HashSet::new();

    for direction in directions {
        let new_position = BoardPosition::new(position.x + direction.0, position.y + direction.1);
        if board_map.is_movable(new_position) {
            movement_tiles.insert(new_position);
        }
    }

    movement_tiles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movement_types_single() {
        let board_map = BoardMap::new();
        let position = BoardPosition::new(4, 4);
        let movement_types = MovementTypes(vec![MovementType::King].into_iter().collect());

        let moves = movement_types.get_movement_tiles(&position, &board_map);

        // We expect 8 moves for a king in the center
        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn test_movement_types_empty() {
        let board_map = BoardMap::new();
        let position = BoardPosition::new(4, 4);
        let movement_types = MovementTypes(HashSet::new());

        let moves = movement_types.get_movement_tiles(&position, &board_map);

        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn test_from_directions_short() {
        let board_map = BoardMap::new();
        let position = BoardPosition::new(4, 4);
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

        let moves = from_directions_short(directions, &position, &board_map);

        assert_eq!(moves.len(), 4);
        assert!(moves.contains(&BoardPosition::new(4, 5)));
        assert!(moves.contains(&BoardPosition::new(5, 4)));
        assert!(moves.contains(&BoardPosition::new(4, 3)));
        assert!(moves.contains(&BoardPosition::new(3, 4)));
    }

    #[test]
    fn test_from_directions_short_edge() {
        let board_map = BoardMap::new();
        let position = BoardPosition::new(0, 0);
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

        let moves = from_directions_short(directions, &position, &board_map);

        assert_eq!(moves.len(), 2);
        assert!(moves.contains(&BoardPosition::new(0, 1)));
        assert!(moves.contains(&BoardPosition::new(1, 0)));
    }
}
