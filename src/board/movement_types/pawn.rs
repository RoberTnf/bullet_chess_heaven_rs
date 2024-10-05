use bevy::utils::HashSet;

use crate::board::{board_map::BoardMap, position::BoardPosition};

use super::from_directions_short;

pub fn get_movement_tiles_white(
    position: &BoardPosition,
    board_map: &BoardMap,
) -> HashSet<BoardPosition> {
    let directions: Vec<(i32, i32)> = vec![(0, 1)];

    from_directions_short(directions, position, board_map)
}

pub fn get_movement_tiles_black(
    position: &BoardPosition,
    board_map: &BoardMap,
) -> HashSet<BoardPosition> {
    let directions: Vec<(i32, i32)> = vec![(0, -1)];

    from_directions_short(directions, position, board_map)
}
