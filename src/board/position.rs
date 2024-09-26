use bevy::prelude::*;

#[derive(Component, PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
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
}
