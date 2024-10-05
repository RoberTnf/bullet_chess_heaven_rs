use std::ops::Sub;

use bevy::prelude::*;

use crate::globals;

#[derive(Component, PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct BoardPosition {
    pub x: i32,
    pub y: i32,
}

impl BoardPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn update(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn is_white(&self) -> bool {
        (self.x + self.y) % 2 == 0
    }

    pub fn from_global_position(x: f32, y: f32) -> Option<Self> {
        if x < 0.0 || y < 0.0 {
            return None;
        }

        Some(Self {
            x: (x / globals::TILE_SIZE as f32) as i32,
            y: (y / globals::TILE_SIZE as f32) as i32,
        })
    }

    pub fn distance_squared(&self, other: &BoardPosition) -> i32 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2)
    }
}

impl Sub<BoardPosition> for BoardPosition {
    type Output = BoardPosition;

    fn sub(self, rhs: BoardPosition) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{board::position::BoardPosition, globals};

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
}
