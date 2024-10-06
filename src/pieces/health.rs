use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub value: u64,
}

impl Health {
    pub fn new(value: u64) -> Self {
        Health { value }
    }

    pub fn take_damage(&mut self, damage: u64) {
        self.value = self.value.saturating_sub(damage);
    }

    pub fn is_dead(&self) -> bool {
        self.value == 0
    }

    pub fn heal(&mut self, amount: u64) {
        self.value = self.value.saturating_add(amount);
    }

    pub fn set_health(&mut self, value: u64) {
        self.value = value;
    }
}
