use bevy::prelude::*;

#[derive(Component)]
pub struct Immortal {
    pub turns_remaining: usize,
}

pub fn decrement_turns_remaining(
    mut immortal_query: Query<(Entity, &mut Immortal)>,
    mut commands: Commands,
) {
    for (entity, mut immortal) in immortal_query.iter_mut() {
        immortal.turns_remaining -= 1;
        if immortal.turns_remaining == 0 {
            commands.entity(entity).remove::<Immortal>();
        }
    }
}
