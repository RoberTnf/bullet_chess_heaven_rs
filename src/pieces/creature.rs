use bevy::prelude::*;

#[derive(Component, Default)]
pub struct BlocksMovement;

#[derive(Component, Default)]
pub struct Creature;

#[derive(Component, Default)]
pub struct Movable;

#[derive(Bundle, Default)]
pub struct CreatureBundle {
    pub sprite: SpriteBundle,
    pub atlas: TextureAtlas,
    pub blocks_movement: BlocksMovement,
    pub creature: Creature,
    pub movable: Movable,
}
