use bevy::{prelude::*, window::WindowResized};

use crate::globals::{TARGET_PIXEL_HEIGHT, TARGET_PIXEL_WIDTH};

#[derive(Resource)]
struct WindowScaleTimer(Timer);

fn update_window_scale_factor(
    mut windows: Query<&mut Window>,
    mut resize_event: EventReader<WindowResized>,
    mut timer: ResMut<WindowScaleTimer>,
    time: Res<Time>,
) {
    for event in resize_event.read() {
        timer.0.tick(time.delta());

        if !timer.0.finished() {
            return;
        }

        if let Ok(mut window) = windows.get_single_mut() {
            let target_aspect_ratio = TARGET_PIXEL_WIDTH as f32 / TARGET_PIXEL_HEIGHT as f32;
            let width = event.width;
            let height = event.height;
            let aspect_ratio = width / height;

            if width < 100.0 || width > 32000.0 {
                // non sensical window size, ignore
                continue;
            }

            let scale_factor = if aspect_ratio > target_aspect_ratio {
                // Wide screen, calculate aspect ratio based on height
                Some(height / TARGET_PIXEL_HEIGHT)
            } else {
                // Tall screen
                Some(width / TARGET_PIXEL_WIDTH)
            };

            window.resolution.set_scale_factor_override(scale_factor);
        }
    }
}

pub struct ResolutionPlugin;

impl Plugin for ResolutionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_window_scale_factor)
            .insert_resource(WindowScaleTimer(Timer::from_seconds(3.0, TimerMode::Once)));
    }
}
