use bevy::prelude::*;

use crate::globals;

#[derive(Resource)]
pub struct SpriteSheetAtlas {
    pub handle: Handle<TextureAtlasLayout>,
}

impl FromWorld for SpriteSheetAtlas {
    fn from_world(world: &mut World) -> Self {
        let layout =
            TextureAtlasLayout::from_grid(UVec2::splat(globals::TILE_SIZE), 10, 10, None, None);

        // Get active atlases stored by Bevy
        let mut texture_atlases = world
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .unwrap();

        // Add the new Atlas and store it in the resource
        Self {
            handle: texture_atlases.add(layout),
        }
    }
}
