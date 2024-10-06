use crate::board::{board_map::BoardMap, position::BoardPosition};
use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

pub mod cache;
pub mod king;
pub mod pawn;

#[derive(Component, Debug)]
pub struct MovementTypes(pub HashSet<MovementType>);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MovementType {
    King,
    PawnWhite,
    PawnBlack,
}

pub type AttackTiles = HashMap<(BoardPosition, MovementType), Entity>;

#[derive(Debug, Clone)]
pub struct MovementTypesResponse {
    pub movement_tiles: HashSet<BoardPosition>,
    pub attack_tiles: AttackTiles,
}

impl MovementTypes {
    pub fn get_movement_tiles(
        &self,
        position: &BoardPosition,
        board_map: &BoardMap,
    ) -> MovementTypesResponse {
        let mut movement_tiles = HashSet::new();
        let mut attack_tiles = HashMap::new();

        for movement_type in &self.0 {
            match movement_type {
                MovementType::King => {
                    let king_response = king::get_movement_tiles(position, board_map);
                    movement_tiles.extend(king_response.movement_tiles);
                    attack_tiles.extend(king_response.attack_tiles);
                }
                MovementType::PawnWhite => {
                    let pawn_response = pawn::get_movement_tiles_white(position, board_map);
                    movement_tiles.extend(pawn_response.movement_tiles);
                    attack_tiles.extend(pawn_response.attack_tiles);
                }
                MovementType::PawnBlack => {
                    let pawn_response = pawn::get_movement_tiles_black(position, board_map);
                    movement_tiles.extend(pawn_response.movement_tiles);
                    attack_tiles.extend(pawn_response.attack_tiles);
                }
            }
        }

        MovementTypesResponse {
            movement_tiles,
            attack_tiles,
        }
    }
}

pub fn from_directions_short(
    directions: Vec<(i32, i32)>,
    position: &BoardPosition,
    board_map: &BoardMap,
    movement_type: MovementType,
) -> MovementTypesResponse {
    let mut movement_tiles = HashSet::new();
    let mut attack_tiles = HashMap::new();

    for direction in directions {
        let new_position = BoardPosition::new(position.x + direction.0, position.y + direction.1);
        if board_map.is_movable(new_position) {
            movement_tiles.insert(new_position);
        } else if let Some(entity) = board_map.get_entity_at(new_position) {
            attack_tiles.insert((new_position, movement_type), *entity);
        }
    }

    MovementTypesResponse {
        movement_tiles,
        attack_tiles,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movement_types_single() {
        let board_map = BoardMap::new();
        let position = BoardPosition::new(4, 4);
        let movement_types = MovementTypes(vec![MovementType::King].into_iter().collect());

        let response = movement_types.get_movement_tiles(&position, &board_map);

        // We expect 8 moves for a king in the center
        assert_eq!(response.movement_tiles.len(), 8);
    }

    #[test]
    fn test_movement_types_empty() {
        let board_map = BoardMap::new();
        let position = BoardPosition::new(4, 4);
        let movement_types = MovementTypes(HashSet::new());

        let response = movement_types.get_movement_tiles(&position, &board_map);

        assert_eq!(response.movement_tiles.len(), 0);
    }

    #[test]
    fn test_from_directions_short() {
        let board_map = BoardMap::new();
        let position = BoardPosition::new(4, 4);
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

        let response = from_directions_short(directions, &position, &board_map, MovementType::King);

        assert_eq!(response.movement_tiles.len(), 4);
        assert!(response.movement_tiles.contains(&BoardPosition::new(4, 5)));
        assert!(response.movement_tiles.contains(&BoardPosition::new(5, 4)));
        assert!(response.movement_tiles.contains(&BoardPosition::new(4, 3)));
        assert!(response.movement_tiles.contains(&BoardPosition::new(3, 4)));
    }

    #[test]
    fn test_from_directions_short_edge() {
        let board_map = BoardMap::new();
        let position = BoardPosition::new(0, 0);
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

        let response = from_directions_short(directions, &position, &board_map, MovementType::King);

        assert_eq!(response.movement_tiles.len(), 2);
        assert!(response.movement_tiles.contains(&BoardPosition::new(0, 1)));
        assert!(response.movement_tiles.contains(&BoardPosition::new(1, 0)));
    }
}
