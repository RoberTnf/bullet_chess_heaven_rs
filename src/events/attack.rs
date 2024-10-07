use crate::{
    board::{movement_types::AttackTiles, position::BoardPosition},
    pieces::{damage::Damage, health::Health},
};
use bevy::prelude::*;

#[derive(Event)]
pub struct AttackEvent {
    pub tile_pos: BoardPosition,
    pub attacker: Entity,
    pub attacks: AttackTiles,
}

pub fn attack_system(
    mut attack_events: EventReader<AttackEvent>,
    mut health_query: Query<(&mut Health, &Name)>,
    damage_query: Query<&Damage>,
) {
    for event in attack_events.read() {
        let attack_tiles = event.attacks.clone();
        let attacker = event.attacker;

        let damage = damage_query
            .get(attacker)
            .expect("Attacker does not have damage component");

        for ((_pos, movement_type), attacked_entity) in attack_tiles.iter() {
            let (mut health, name) = health_query.get_mut(*attacked_entity).unwrap();
            health.take_damage(damage.value);
            // TODO: Add attack animation
            debug!(
                "Attacking entity {:?} with movement type {:?}, damage {:?}, health left {:?}",
                name, movement_type, damage.value, health.value
            );
        }
    }
}
