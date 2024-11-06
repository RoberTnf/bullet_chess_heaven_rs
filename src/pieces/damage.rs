use bevy::prelude::*;

#[derive(Component)]
pub struct Damage {
    pub value: usize,
}

impl Damage {
    pub fn new(value: usize) -> Self {
        Damage { value }
    }
}
