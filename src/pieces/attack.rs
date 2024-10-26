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
}
pub enum AttackPieceAnimationState {
    Start,
    Backwards,
    Forwards,
}

#[derive(Component)]
pub struct AttackAfterMove;

#[derive(Component)]
pub struct AttackingWithNewSprite {
    pub destination: BoardPosition,
    pub origin: BoardPosition,
    pub sprite_index: usize,
    pub animation_state: AttackPieceAnimationState,
}

pub fn attack_piece_system(
    mut attack_event_reader: EventReader<AttackPieceEvent>,
    mut health_event_writer: EventWriter<PieceHealthChangeEvent>,
    mut pieces: Query<(&BoardPosition, &mut PieceState), With<Piece>>,
    mut commands: Commands,
) {
    for event in attack_event_reader.read() {
        health_event_writer.send(PieceHealthChangeEvent {
            entity: event.target,
            change: -(event.damage as i64),
        });

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
            };
        }
    }
}

pub fn attack_piece_animation_system(
    mut query: Query<(&mut Transform, &mut PieceState), (With<Piece>, Without<Player>)>,
    time: Res<Time>,
) {
    for (mut transform, mut piece_state) in query.iter_mut() {
        if let PieceState::Attacking {
            destination,
            origin,
            animation_state,
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
            }
        }
    }
}

#[derive(Component)]
struct AttackingSprite;

fn attacking_with_new_sprite_animation_system(
    asset_server: Res<AssetServer>,
    mut piece_query: Query<(&mut PieceState, &Transform)>,
    mut attacking_sprite_query: Query<(&mut AttackingWithNewSprite, &Parent, Entity)>,
    mut sprite_query: Query<&mut Transform, (With<AttackingSprite>, Without<PieceState>)>,
    mut commands: Commands,
    atlas_layout: Res<SpriteSheetAtlas>,
    time: Res<Time>,
    children_query: Query<&Children>,
) {
    for (mut attacking_sprite, parent, entity) in attacking_sprite_query.iter_mut() {
        let (mut piece_state, piece_transform) = match piece_query.get_mut(parent.get()) {
            Ok(result) => result,
            Err(_) => continue,
        };

        match attacking_sprite.animation_state {
            AttackPieceAnimationState::Start => {
                spawn_attack_sprite(
                    &mut commands,
                    entity,
                    &asset_server,
                    &atlas_layout,
                    attacking_sprite.sprite_index,
                );
                attacking_sprite.animation_state = AttackPieceAnimationState::Forwards;
            }
            AttackPieceAnimationState::Forwards => {
                let sprites_still_active = update_sprite_positions(
                    &children_query,
                    entity,
                    &mut sprite_query,
                    &mut commands,
                    piece_transform,
                    &attacking_sprite,
                    time.delta_seconds(),
                );

                if !sprites_still_active {
                    *piece_state = PieceState::Idle;
                    commands.entity(entity).despawn_recursive();
                }
            }
            AttackPieceAnimationState::Backwards => (), // Sprite is already despawned
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
    attacking_sprite: &Mut<AttackingWithNewSprite>,
    delta_time: f32,
) -> bool {
    let Ok(children) = children_query.get(parent) else {
        return false;
    };

    let mut active_sprites = false;
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
        } else {
            sprite_transform.translation = (sprite_transform.translation.truncate() + movement)
                .extend(sprite_transform.translation.z);
            active_sprites = true;
        }
    }
    active_sprites
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
    movement_types.0.iter().for_each(|movement_type| {
        let valid_attacks = movement_type
            .get_valid_moves(
                current_tile_position,
                all_pieces_positions,
                enemy_pieces_positions,
            )
            .valid_attacks;

        for attack_position in valid_attacks {
            attack = true;
            let enemy_entity = pieces_query
                .iter()
                .find(|(_, &pos, _)| pos == attack_position)
                .map(|(entity, _, _)| entity)
                .unwrap();

            send_attack_event(
                attack_event_writer,
                attack_position,
                player_entity,
                enemy_entity,
                damage.value,
                movement_type,
            );
            next_state.set(TurnState::PlayerAnimation);
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
                attack_piece_system
                    .run_if(in_state(GameState::Game))
                    .run_if(in_state(GamePauseState::Playing)),
            ),
        );
    }
}
