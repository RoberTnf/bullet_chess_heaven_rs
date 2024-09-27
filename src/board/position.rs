use std::ops::Sub;

use bevy::prelude::*;

use crate::globals;

#[derive(Component, PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct BoardPosition {
    pub x: u32,
    pub y: u32,
}

impl BoardPosition {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn update(&mut self, x: u32, y: u32) {
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
            x: (x / globals::TILE_SIZE as f32) as u32,
            y: (y / globals::TILE_SIZE as f32) as u32,
        })
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
