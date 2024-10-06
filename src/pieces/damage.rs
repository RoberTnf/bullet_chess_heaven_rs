use bevy::prelude::*;

#[derive(Component)]
pub struct Damage {
    pub value: u64,
}

impl Damage {
    pub fn new(value: u64) -> Self {
        Damage { value }
    }
}
