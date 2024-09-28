mod camera;
pub mod spritesheet;
mod transforms;
use bevy::prelude::*;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<spritesheet::SpriteSheetAtlas>()
            .add_systems(Update, transforms::update_transforms)
            .add_systems(Startup, camera::setup_camera);
    }
}
