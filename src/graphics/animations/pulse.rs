use bevy::prelude::*;

use crate::pieces::player::spawn::PulseSize;

// Pulse animation for the player
// Uses a sine wave to create a pulsing effect
pub fn animate_pulse_scale(
    mut player_query: Query<(&mut Transform, &mut PulseSize)>,
    time: Res<Time>,
) {
    for (mut transform, mut pulse_size) in player_query.iter_mut() {
        let next_progress = pulse_size.progress + pulse_size.speed * time.delta_secs();

        let t = next_progress.sin();
        let size = pulse_size.start_size + (pulse_size.final_size - pulse_size.start_size) * t;
        transform.scale = Vec3::splat(size);
        pulse_size.progress = next_progress;
    }
}
