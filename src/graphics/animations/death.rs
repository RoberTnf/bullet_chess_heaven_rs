use bevy::prelude::*;

use crate::{globals::DEATH_ANIMATION_DURATION, pieces::health::DeathAnimation};

pub fn animate_death(mut death_query: Query<(&mut Transform, &DeathAnimation)>) {
    for (mut transform, death_animation) in death_query.iter_mut() {
        let time_since_death = death_animation.timer.elapsed_secs();
        let progress = time_since_death / DEATH_ANIMATION_DURATION;
        transform.scale = Vec3::splat(1.0 - progress);
    }
}
