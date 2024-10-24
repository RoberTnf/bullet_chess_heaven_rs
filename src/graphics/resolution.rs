use bevy::{prelude::*, window::PrimaryWindow};

use crate::globals::{TARGET_PIXEL_HEIGHT, TARGET_PIXEL_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};

#[derive(Resource)]
struct WindowScaleTimer(Timer);

#[derive(Resource)]
struct WindowSize(u32, u32);

fn update_window_scale_factor(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut timer: ResMut<WindowScaleTimer>,
    time: Res<Time>,
    mut window_size: ResMut<WindowSize>,
) {
    if !timer.0.finished() {
        timer.0.tick(time.delta());
        return;
    }

    let window = windows.get_single().unwrap();
    let width = window.physical_width();
    let height = window.physical_height();

    if width == window_size.0 && height == window_size.1 {
        return;
    }

    if let Ok(mut window) = windows.get_single_mut() {
        let target_aspect_ratio = TARGET_PIXEL_WIDTH / TARGET_PIXEL_HEIGHT;
        let aspect_ratio = width as f32 / height as f32;

        if !(100..=32000).contains(&width) || !(100..=32000).contains(&height) {
            return;
        }

        debug!("Window resized to {}x{}", width, height);
        let scale_factor = if aspect_ratio > target_aspect_ratio {
            // Wide screen, calculate aspect ratio based on height
            Some(height as f32 / TARGET_PIXEL_HEIGHT)
        } else {
            // Tall screen
            Some(width as f32 / TARGET_PIXEL_WIDTH)
        };

        window.resolution.set_scale_factor_override(scale_factor);
        window.resolution.set_physical_resolution(width, height);
        window_size.0 = width;
        window_size.1 = height;
    }
}

pub struct ResolutionPlugin;

impl Plugin for ResolutionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_window_scale_factor)
            .insert_resource(WindowScaleTimer(Timer::from_seconds(0.3, TimerMode::Once)))
            .insert_resource(WindowSize(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32));
    }
}
