use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Tween, TweenCompleted};

use crate::{
    board::{board_map::BoardMap, position::BoardPosition, tile::Tile},
    globals::{TWEEN_EVENT_MOVE_ANIMATION_FINISHED, TWEEN_MOVE_ANIMATION_DURATION},
    pieces::creature::CreatureState,
};

pub fn update_transforms(
    mut tiles: Query<(&BoardPosition, &mut Transform), With<Tile>>,
    mut creatures: Query<(&BoardPosition, &mut Transform, Entity, &CreatureState), Without<Tile>>,
    board_map: Res<BoardMap>,
    mut commands: Commands,
) {
    for (tile_pos, mut tile_transform) in tiles.iter_mut() {
        let pos = board_map.get_world_position(tile_pos);

        tile_transform.translation.x = pos.x;
        tile_transform.translation.y = pos.y;
    }

    for (creature_pos, mut creature_transform, creature_entity, creature_state) in
        creatures.iter_mut()
    {
        if let CreatureState::Initializing = creature_state {
            let pos = board_map.get_world_position(creature_pos);

            creature_transform.translation.x = pos.x;
            creature_transform.translation.y = pos.y;
            commands.entity(creature_entity).insert(CreatureState::Idle);
        }
    }
}

pub fn animate_transforms(
    creatures: Query<(&Transform, &CreatureState, &BoardPosition, Entity)>,
    board_map: Res<BoardMap>,
    mut commands: Commands,
    mut events: EventReader<TweenCompleted>,
) {
    for (creature_transform, state, board_position, entity) in creatures.iter() {
        match state {
            CreatureState::Idle => {
                let origin = Vec3::new(
                    creature_transform.translation.x,
                    creature_transform.translation.y,
                    creature_transform.translation.z,
                );
                let destination = board_map
                    .get_world_position(board_position)
                    .extend(origin.z);

                if origin == destination {
                    continue;
                }

                debug!("Tweening creature from {:?} to {:?}", origin, destination);
                let tween = Tween::new(
                    EaseFunction::SineInOut,
                    Duration::from_secs_f64(TWEEN_MOVE_ANIMATION_DURATION),
                    TransformPositionLens {
                        start: origin,
                        end: destination,
                    },
                )
                .with_completed_event(TWEEN_EVENT_MOVE_ANIMATION_FINISHED);

                let animator = Animator::new(tween);

                commands.entity(entity).insert(animator);
                commands.entity(entity).insert(CreatureState::Moving);
            }
            CreatureState::Initializing => {}
            CreatureState::Moving => {
                for ev in events.read() {
                    debug!("Animation finished for {:?}", ev.entity);
                    if ev.user_data == TWEEN_EVENT_MOVE_ANIMATION_FINISHED {
                        commands.entity(entity).insert(CreatureState::Idle);
                    }
                }
            }
        }
    }
}
