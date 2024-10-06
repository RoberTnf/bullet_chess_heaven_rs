use crate::board::{board_map::BoardMap, position::BoardPosition};

use super::{from_directions_short, MovementType, MovementTypesResponse};

pub fn get_movement_tiles(position: &BoardPosition, board_map: &BoardMap) -> MovementTypesResponse {
    let directions: Vec<(i32, i32)> = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    from_directions_short(directions, position, board_map, MovementType::King)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_king_movement_center() {
        let board_map = BoardMap::new();
        let position = BoardPosition::new(4, 4);
        let response = get_movement_tiles(&position, &board_map);

        assert_eq!(response.movement_tiles.len(), 8);
        assert!(response.movement_tiles.contains(&BoardPosition::new(3, 3)));
        assert!(response.movement_tiles.contains(&BoardPosition::new(3, 4)));
        assert!(response.movement_tiles.contains(&BoardPosition::new(3, 5)));
        assert!(response.movement_tiles.contains(&BoardPosition::new(4, 3)));
        assert!(response.movement_tiles.contains(&BoardPosition::new(4, 5)));
        assert!(response.movement_tiles.contains(&BoardPosition::new(5, 3)));
        assert!(response.movement_tiles.contains(&BoardPosition::new(5, 4)));
        assert!(response.movement_tiles.contains(&BoardPosition::new(5, 5)));
    }

    #[test]
    fn test_king_movement_corner() {
        let board_map = BoardMap::new();
        let position = BoardPosition::new(0, 0);
        let response = get_movement_tiles(&position, &board_map);

        assert_eq!(response.movement_tiles.len(), 3);
        assert!(response.movement_tiles.contains(&BoardPosition::new(0, 1)));
        assert!(response.movement_tiles.contains(&BoardPosition::new(1, 0)));
        assert!(response.movement_tiles.contains(&BoardPosition::new(1, 1)));
    }

    #[test]
    fn test_king_movement_edge() {
        let board_map = BoardMap::new();
        let position = BoardPosition::new(0, 4);
        let response = get_movement_tiles(&position, &board_map);

        assert_eq!(response.movement_tiles.len(), 5);
        assert!(response.movement_tiles.contains(&BoardPosition::new(0, 3)));
        assert!(response.movement_tiles.contains(&BoardPosition::new(0, 5)));
        assert!(response.movement_tiles.contains(&BoardPosition::new(1, 3)));
        assert!(response.movement_tiles.contains(&BoardPosition::new(1, 4)));
        assert!(response.movement_tiles.contains(&BoardPosition::new(1, 5)));
    }
}
