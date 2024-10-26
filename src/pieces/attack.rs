use std::time::Duration;

use bevy::{prelude::*, utils::HashSet};

use crate::{
    board::position::BoardPosition,
    globals::{ATTACK_ANIMATION_DURATION, TILE_SIZE},
    graphics::spritesheet::SpriteSheetAtlas,
    input::click_tile::send_attack_event,
    states::{game_state::GameState, pause_state::GamePauseState, turn_state::TurnState},
};

use super::{
    common::{MovementTypes, Piece, PieceState, Team},
    damage::Damage,
    health::PieceHealthChangeEvent,
    movement::MovePieceAnimationEndEvent,
    player::spawn::Player,
};

#[derive(Event)]
pub struct AttackPieceEvent {
    pub destination: BoardPosition,
    pub attacker: Entity,
    pub target: Entity,
    pub damage: u64,
    pub sprite_index: Option<usize>,
    pub delay: Option<f32>,
}
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum AttackPieceAnimationState {
    Start,
    Backwards,
    Forwards,
    Finished,
}

#[derive(Component)]
pub struct AttackAfterMove;

#[derive(Component)]
pub struct AttackingWithNewSprite {
    pub destination: BoardPosition,
    pub origin: BoardPosition,
    pub sprite_index: usize,
    pub animation_state: AttackPieceAnimationState,
    pub timer: Timer,
    pub piece_health_change_event: PieceHealthChangeEvent,
}

pub fn attack_piece_system(
    mut attack_event_reader: EventReader<AttackPieceEvent>,
    mut pieces: Query<(&BoardPosition, &mut PieceState), With<Piece>>,
    mut commands: Commands,
) {
    for event in attack_event_reader.read() {
        let (attacker_pos, mut attacker_state) = pieces.get_mut(event.attacker).unwrap();

        if let Some(sprite_index) = event.sprite_index {
            *attacker_state = PieceState::AttackingWithNewSprite;
            let entity = commands
                .spawn((
                    AttackingWithNewSprite {
                        destination: event.destination,
                        origin: *attacker_pos,
                        sprite_index,
                        animation_state: AttackPieceAnimationState::Start,
                        timer: Timer::new(
                            Duration::from_secs_f32(event.delay.unwrap_or(0.0)),
                            TimerMode::Once,
                        ),
                        piece_health_change_event: PieceHealthChangeEvent {
                            entity: event.target,
                            change: -(event.damage as i64),
                        },
                    },
                    TransformBundle::default(),
                ))
                .id();
            commands.entity(event.attacker).add_child(entity);
            debug!("Added AttackingWithNewSprite to {}", event.attacker);
        } else {
            *attacker_state = PieceState::Attacking {
                destination: event.destination,
                origin: *attacker_pos,
                animation_state: AttackPieceAnimationState::Forwards,
                event: PieceHealthChangeEvent {
                    entity: event.target,
                    change: -(event.damage as i64),
                },
            };
        }
    }
}

pub fn attack_piece_animation_system(
    mut query: Query<(&mut Transform, &mut PieceState), (With<Piece>, Without<Player>)>,
    time: Res<Time>,
    mut event_writer: EventWriter<PieceHealthChangeEvent>,
) {
    for (mut transform, mut piece_state) in query.iter_mut() {
        if let PieceState::Attacking {
            destination,
            origin,
            animation_state,
            event,
        } = &mut *piece_state
        {
            // TODO: If this becomes slow, store this variables in the animation component
            let origin_global_position = origin.as_global_position();
            let destination_global_position = destination.as_global_position();
            let original_distance = (destination_global_position - origin_global_position).length();
            let direction = (destination_global_position - origin_global_position).normalize();
            let truncated_translation = transform.translation.truncate();
            let speed = original_distance * 2.0 / ATTACK_ANIMATION_DURATION;
            let delta = direction * speed * time.delta_seconds();

            // work in 2D except for the end
            let original_z = transform.translation.z;

            match animation_state {
                AttackPieceAnimationState::Forwards => {
                    let new_position = truncated_translation + delta;
                    let pixel_distance = new_position.distance(destination_global_position);
                    transform.translation = new_position.extend(original_z);

                    if pixel_distance < TILE_SIZE as f32 / 1.5 {
                        *animation_state = AttackPieceAnimationState::Backwards;
                        event_writer.send(*event);
                    }
                }
                AttackPieceAnimationState::Backwards => {
                    let new_position = truncated_translation - delta;
                    let progress =
                        new_position.distance(destination_global_position) / original_distance;
                    transform.translation = new_position.extend(original_z);

                    if progress > 0.98 {
                        // snap to the origin
                        transform.translation = origin_global_position.extend(original_z);
                        *piece_state = PieceState::Idle;
                    }
                }
                AttackPieceAnimationState::Start => {}
                AttackPieceAnimationState::Finished => {}
            }
        }
    }
}

#[derive(Component)]
struct AttackingSprite;

fn attacking_with_new_sprite_animation_system(
    asset_server: Res<AssetServer>,
    mut piece_query: Query<&Transform, With<PieceState>>,
    mut attacking_sprite_query: Query<(&mut AttackingWithNewSprite, &Parent, Entity)>,
    mut sprite_query: Query<&mut Transform, (With<AttackingSprite>, Without<PieceState>)>,
    mut commands: Commands,
    atlas_layout: Res<SpriteSheetAtlas>,
    time: Res<Time>,
    children_query: Query<&Children>,
    mut event_writer: EventWriter<PieceHealthChangeEvent>,
) {
    for (mut attacking_sprite, parent, entity) in attacking_sprite_query.iter_mut() {
        let Ok(piece_transform) = piece_query.get_mut(parent.get()) else {
            continue;
        };

        match attacking_sprite.animation_state {
            AttackPieceAnimationState::Start => {
                attacking_sprite.timer.tick(time.delta());
                if attacking_sprite.timer.finished() {
                    spawn_attack_sprite(
                        &mut commands,
                        entity,
                        &asset_server,
                        &atlas_layout,
                        attacking_sprite.sprite_index,
                    );
                    attacking_sprite.animation_state = AttackPieceAnimationState::Forwards;
                }
            }
            AttackPieceAnimationState::Forwards => {
                update_sprite_positions(
                    &children_query,
                    entity,
                    &mut sprite_query,
                    &mut commands,
                    piece_transform,
                    attacking_sprite.as_mut(),
                    time.delta_seconds(),
                    &mut event_writer,
                );
            }
            AttackPieceAnimationState::Backwards => (),
            AttackPieceAnimationState::Finished => {}
        }
    }
}

fn piece_idle_if_all_animations_finished(
    mut piece_query: Query<(&mut PieceState, &Children)>,
    children_query: Query<&AttackingWithNewSprite>,
    mut commands: Commands,
) {
    for (mut piece_state, children) in piece_query.iter_mut() {
        let mut finished = true;
        let mut children_to_despawn = Vec::new();
        for child in children.iter() {
            if let Ok(attacking_sprite) = children_query.get(*child) {
                if attacking_sprite.animation_state != AttackPieceAnimationState::Finished {
                    finished = false;
                }
                children_to_despawn.push(*child);
            }
        }
        if finished && !children_to_despawn.is_empty() {
            *piece_state = PieceState::Idle;
            for child in children_to_despawn.iter() {
                commands.entity(*child).despawn_recursive();
            }
        }
    }
}

fn spawn_attack_sprite(
    commands: &mut Commands,
    parent: Entity,
    asset_server: &Res<AssetServer>,
    atlas_layout: &Res<SpriteSheetAtlas>,
    sprite_index: usize,
) {
    let sprite = SpriteBundle {
        texture: asset_server.load("custom/spritesheet.png"),
        // TODO: Fix invisible
        transform: Transform::default(),
        ..default()
    };
    let atlas = TextureAtlas {
        layout: atlas_layout.handle.clone(),
        index: sprite_index,
    };
    let sprite_entity = commands.spawn((sprite, atlas, AttackingSprite)).id();
    commands.entity(parent).add_child(sprite_entity);
}

fn update_sprite_positions(
    children_query: &Query<&Children>,
    parent: Entity,
    sprite_query: &mut Query<&mut Transform, (With<AttackingSprite>, Without<PieceState>)>,
    commands: &mut Commands,
    piece_transform: &Transform,
    attacking_sprite: &mut AttackingWithNewSprite,
    delta_time: f32,
    event_writer: &mut EventWriter<PieceHealthChangeEvent>,
) {
    let Ok(children) = children_query.get(parent) else {
        return;
    };

    for sprite in children.iter() {
        let Ok(mut sprite_transform) = sprite_query.get_mut(*sprite) else {
            continue;
        };

        let destination = attacking_sprite.destination.as_global_position();
        let current_pos =
            sprite_transform.translation.truncate() + piece_transform.translation.truncate();

        let movement = (destination - attacking_sprite.origin.as_global_position()) * 2.0
            / ATTACK_ANIMATION_DURATION
            * delta_time;

        if (current_pos - destination).length() < TILE_SIZE as f32 / 2.0 {
            commands.entity(*sprite).despawn_recursive();
            attacking_sprite.animation_state = AttackPieceAnimationState::Finished;
            event_writer.send(attacking_sprite.piece_health_change_event);
        } else {
            sprite_transform.translation = (sprite_transform.translation.truncate() + movement)
                .extend(sprite_transform.translation.z);
        }
    }
}

pub fn attack_from_tile(
    movement_types: &MovementTypes,
    current_tile_position: &BoardPosition,
    all_pieces_positions: &HashSet<BoardPosition>,
    enemy_pieces_positions: &HashSet<BoardPosition>,
    pieces_query: &Query<(Entity, &BoardPosition, &Team), (With<Piece>, Without<Player>)>,
    attack_event_writer: &mut EventWriter<AttackPieceEvent>,
    player_entity: Entity,
    damage: &Damage,
    next_state: &mut ResMut<NextState<TurnState>>,
) -> bool {
    let mut attack = false;
    let mut delay = 0.0;
    movement_types.0.iter().for_each(|movement_type| {
        let valid_attacks = movement_type
            .get_valid_moves(
                current_tile_position,
                all_pieces_positions,
                enemy_pieces_positions,
            )
            .valid_attacks;

        for attack_position in valid_attacks.iter() {
            attack = true;
            let enemy_entity = pieces_query
                .iter()
                .find(|(_, &pos, _)| pos == *attack_position)
                .map(|(entity, _, _)| entity)
                .unwrap();

            send_attack_event(
                attack_event_writer,
                attack_position,
                player_entity,
                enemy_entity,
                damage.value,
                movement_type,
                Some(delay),
            );
            next_state.set(TurnState::PlayerAnimation);
        }
        if !valid_attacks.is_empty() {
            delay += ATTACK_ANIMATION_DURATION / 3.0;
        }
    });
    attack
}

pub fn on_move_animation_end_attack_system(
    mut attack_event_writer: EventWriter<AttackPieceEvent>,
    mut pieces_with_attack: Query<
        (
            Entity,
            &BoardPosition,
            &Damage,
            &MovementTypes,
            &mut PieceState,
        ),
        With<AttackAfterMove>,
    >,
    mut pieces_without_attack: Query<&mut PieceState, (With<Piece>, Without<AttackAfterMove>)>,
    pieces_query: Query<(Entity, &BoardPosition, &Team), (With<Piece>, Without<Player>)>,
    mut next_state: ResMut<NextState<TurnState>>,
    mut move_piece_animation_end_events: EventReader<MovePieceAnimationEndEvent>,
) {
    let all_pieces_positions = pieces_query.iter().map(|(_, pos, _)| *pos).collect();
    let enemy_pieces_positions = pieces_query
        .iter()
        .filter(|(_, _, &team)| team == Team::Enemy)
        .map(|(_, pos, _)| *pos)
        .collect();
    for event in move_piece_animation_end_events.read() {
        let mut to_attack = false;
        for (piece_entity, piece_position, damage, movement_types, mut piece_state) in
            pieces_with_attack.iter_mut()
        {
            if event.entity != piece_entity {
                continue;
            }

            to_attack = true;
            if !attack_from_tile(
                movement_types,
                piece_position,
                &all_pieces_positions,
                &enemy_pieces_positions,
                &pieces_query,
                &mut attack_event_writer,
                piece_entity,
                damage,
                &mut next_state,
            ) {
                *piece_state = PieceState::Idle;
            }
        }
        if !to_attack {
            if let Ok(mut piece_state) = pieces_without_attack.get_mut(event.entity) {
                *piece_state = PieceState::Idle;
            }
        }
    }
}

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                attacking_with_new_sprite_animation_system,
                attack_piece_animation_system,
            ),
        );
        app.add_systems(
            Update,
            (
                on_move_animation_end_attack_system,
                attack_piece_system,
                piece_idle_if_all_animations_finished,
            )
                .run_if(in_state(GameState::Game))
                .run_if(in_state(GamePauseState::Playing)),
        );
    }
}
