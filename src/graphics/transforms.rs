use bevy::prelude::*;

use crate::{
    board::{board_map::BoardMap, position::BoardPosition, tile::Tile},
    globals::TWEEN_MOVE_ANIMATION_SPEED,
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
    mut creatures: Query<(&mut Transform, &CreatureState, &BoardPosition, Entity)>,
    board_map: Res<BoardMap>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (mut creature_transform, state, board_position, entity) in creatures.iter_mut() {
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

                commands.entity(entity).insert(CreatureState::Moving {
                    origin,
                    destination,
                });
            }
            CreatureState::Initializing => {}
            CreatureState::Moving {
                origin,
                destination,
            } => {
                let current_pos = creature_transform.translation;
                let lerp_value = TWEEN_MOVE_ANIMATION_SPEED * time.delta_seconds();
                let distance = destination.distance(current_pos);
                let original_distance = origin.distance(*destination);
                let left_to_finish = distance / original_distance;

                if left_to_finish <= 0.01 {
                    commands.entity(entity).insert(CreatureState::Idle);
                    creature_transform.translation = *destination;
                } else {
                    let new_pos = current_pos.lerp(*destination, lerp_value);
                    creature_transform.translation = new_pos;
                }
            }
        }
    }
}
