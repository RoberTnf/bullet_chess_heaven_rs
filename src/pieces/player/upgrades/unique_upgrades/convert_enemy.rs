use crate::{
    globals::SPRITESHEET_WIDTH,
    pieces::common::{Piece, Team},
};

use super::SideEffect;
use bevy::prelude::*;

pub fn apply_side_effect(
    mut side_effect_event: EventReader<SideEffect>,
    mut commands: Commands,
    mut pieces: Query<(&Team, &mut TextureAtlas), With<Piece>>,
) {
    for side_effect in side_effect_event.read() {
        if let SideEffect::ConvertPiece {
            turns_to_convert,
            team,
            entity,
        } = side_effect
        {
            let (original_team, mut atlas) = pieces.get_mut(*entity).unwrap();
            commands.entity(*entity).insert(*team);
            commands.entity(*entity).insert(Converted {
                turns_to_convert: *turns_to_convert,
                original_team: *original_team,
            });
            if *original_team == Team::Enemy && *team == Team::Player {
                atlas.index += SPRITESHEET_WIDTH;
            }
        }
    }
}

#[derive(Component)]
pub struct Converted {
    turns_to_convert: usize,
    original_team: Team,
}

pub fn decrement_turns_to_convert(
    mut converted_query: Query<(Entity, &mut Converted, &mut TextureAtlas, &Team)>,
    mut commands: Commands,
) {
    for (entity, mut converted, mut atlas, current_team) in converted_query.iter_mut() {
        converted.turns_to_convert -= 1;
        if converted.turns_to_convert == 0 {
            commands.entity(entity).insert(converted.original_team);
            commands.entity(entity).remove::<Converted>();
            if converted.original_team == Team::Enemy && *current_team == Team::Player {
                atlas.index -= SPRITESHEET_WIDTH;
            }
        }
    }
}
