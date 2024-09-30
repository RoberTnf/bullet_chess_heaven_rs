mod camera;
pub mod spritesheet;
pub mod transforms;
use bevy::prelude::*;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<spritesheet::SpriteSheetAtlas>()
            .add_systems(Startup, camera::setup_camera);
    }
}
