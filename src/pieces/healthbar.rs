use bevy::prelude::*;

use crate::globals::{
    EMPTY_HEALTHBAR_ATLAS_INDEX, EMPTY_HEALTHBAR_Z_INDEX, FULL_HEALTHBAR_ATLAS_INDEX,
    HEALTHBAR_Z_INDEX,
};

use super::health::Health;

#[derive(Component)]
pub struct Healthbar {
    pub fraction: f32,
}

#[derive(Component)]
pub struct EmptyHealthbar;

#[derive(Bundle)]
pub struct HealthbarBundle {
    pub healthbar: Healthbar,
    pub sprite: SpriteBundle,
    pub atlas: TextureAtlas,
}

#[derive(Bundle)]
pub struct EmptyHealthbarBundle {
    pub sprite: SpriteBundle,
    pub atlas: TextureAtlas,
    pub empty_healthbar: EmptyHealthbar,
}

pub fn update_healthbars(
    mut query: Query<(Entity, &mut Healthbar, &mut Transform, &Parent)>,
    parent_query: Query<(Entity, Option<&Health>)>,
) {
    for (entity, mut healthbar, mut transform, parent) in query.iter_mut() {
        match parent_query.get(parent.get()) {
            Ok((_, Some(parent_health))) => {
                let new_fraction =
                    parent_health.value as f32 / parent_health.max_value.upgraded_value;
                if new_fraction != healthbar.fraction {
                    healthbar.fraction = new_fraction;
                    transform.scale = Vec3::new(new_fraction, 1.0, 1.0);
                }
            }
            Ok((parent_entity, None)) => {
                warn!("Parent entity {:?} exists but doesn't have a Health component. Healthbar entity: {:?}", parent_entity, entity);
            }
            Err(e) => {
                warn!(
                    "Failed to get parent entity for healthbar {:?}: {:?}",
                    entity, e
                );
            }
        }
    }
}

pub fn spawn_healthbar(
    commands: &mut Commands,
    asset_server: &AssetServer,
    atlas_layout: &Handle<TextureAtlasLayout>,
) -> [Entity; 2] {
    let id1 = commands
        .spawn((
            HealthbarBundle {
                healthbar: Healthbar { fraction: 1.0 },
                sprite: SpriteBundle {
                    texture: asset_server.load("custom/spritesheet.png"),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, HEALTHBAR_Z_INDEX)),
                    ..default()
                },
                atlas: TextureAtlas {
                    layout: atlas_layout.clone(),
                    index: FULL_HEALTHBAR_ATLAS_INDEX,
                },
            },
            Name::new("Healthbar"),
        ))
        .id();
    let id2 = commands
        .spawn((
            EmptyHealthbarBundle {
                sprite: SpriteBundle {
                    texture: asset_server.load("custom/spritesheet.png"),
                    transform: Transform::from_translation(Vec3::new(
                        0.0,
                        0.0,
                        EMPTY_HEALTHBAR_Z_INDEX,
                    )),
                    ..default()
                },
                atlas: TextureAtlas {
                    layout: atlas_layout.clone(),
                    index: EMPTY_HEALTHBAR_ATLAS_INDEX,
                },
                empty_healthbar: EmptyHealthbar,
            },
            Name::new("Empty Healthbar"),
        ))
        .id();
    [id1, id2]
}
