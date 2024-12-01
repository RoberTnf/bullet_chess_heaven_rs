use crate::{
    globals::SPRITESHEET_WIDTH,
    pieces::common::{Piece, Team},
};

use super::{immortal::Immortal, SideEffect};
use bevy::prelude::*;

pub fn apply_side_effect(
    mut side_effect_event: EventReader<SideEffect>,
    mut commands: Commands,
    mut pieces: Query<(&Team, &mut Sprite), (With<Piece>, Without<Converted>)>,
) {
    for side_effect in side_effect_event.read() {
        if let SideEffect::ConvertPiece {
            turns_to_convert,
            team,
            entity,
        } = side_effect
        {
            if let Ok((original_team, mut sprite)) = pieces.get_mut(*entity) {
                commands.entity(*entity).insert(*team);
                commands.entity(*entity).insert(Converted {
                    turns_to_convert: *turns_to_convert,
                    original_team: *original_team,
                    original_sprite_index: sprite.texture_atlas.as_mut().unwrap().index,
                });
                commands
                    .entity(*entity)
                    .insert(Immortal { turns_remaining: 1 });
                if *original_team == Team::Enemy && *team == Team::Player {
                    sprite.texture_atlas.as_mut().unwrap().index += SPRITESHEET_WIDTH;
                }
            }
        }
    }
}

#[derive(Component)]
pub struct Converted {
    turns_to_convert: usize,
    original_team: Team,
    original_sprite_index: usize,
}

pub fn decrement_turns_to_convert(
    mut converted_query: Query<(Entity, &mut Converted, &mut Sprite, &Team)>,
    mut commands: Commands,
) {
    for (entity, mut converted, mut sprite, _) in converted_query.iter_mut() {
        converted.turns_to_convert -= 1;
        if converted.turns_to_convert == 0 {
            commands.entity(entity).insert(converted.original_team);
            commands.entity(entity).remove::<Converted>();
            sprite.texture_atlas.as_mut().unwrap().index = converted.original_sprite_index;
        } else {
            sprite.texture_atlas.as_mut().unwrap().index += SPRITESHEET_WIDTH;
        }
    }
}
