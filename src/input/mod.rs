use bevy::prelude::*;

mod player_movement;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement::mouse_input);
    }
}
