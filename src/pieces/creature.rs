use bevy::prelude::*;

#[derive(Bundle)]
pub struct Creature {
    pub sprite: SpriteBundle,
    pub atlas: TextureAtlas,
}
