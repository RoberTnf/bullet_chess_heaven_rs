use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub value: u64,
}

#[derive(Event)]
pub struct DeathEvent {
    pub entity: Entity,
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

pub fn death_system(
    health_query: Query<(&Health, Entity)>,
    mut commands: Commands,
    mut death_event_writer: EventWriter<DeathEvent>,
) {
    for (health, entity) in health_query.iter() {
        if health.is_dead() {
            commands.entity(entity).despawn_recursive();
            death_event_writer.send(DeathEvent { entity });
        }
    }
}
