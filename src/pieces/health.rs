use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    board::highlight::HighlightCache,
    globals::{
        DEATH_ANIMATION_DURATION, HEALTH_CHANGE_TEXT_ANIMATION_DURATION,
        HEALTH_CHANGE_TEXT_ANIMATION_SPEED, HEALTH_CHANGE_TEXT_Z_INDEX, HEALTH_Z_INDEX, TILE_SIZE,
    },
    states::game_state::GameState,
};

use super::common::Team;

#[derive(Component)]
pub struct Health {
    pub value: u64,
    pub changes: Vec<i64>,
    pub max_value: u64,
}

#[derive(Event)]
pub struct PieceDeathEvent {
    pub entity: Entity,
}

impl Health {
    pub fn new(value: u64) -> Self {
        Health {
            value,
            changes: vec![],
            max_value: value,
        }
    }

    pub fn take_damage(&mut self, damage: u64) {
        self.value = self.value.saturating_sub(damage);
        self.changes.push(-(damage as i64));
    }

    pub fn is_dead(&self) -> bool {
        self.value == 0
    }

    pub fn heal(&mut self, amount: u64) {
        self.value = self.value.saturating_add(amount);
        self.changes.push(amount as i64);
    }

    pub fn set_health(&mut self, value: u64) {
        self.value = value;
        self.changes.push((value as i64) - (self.value as i64));
    }

    pub fn clear_changes(&mut self) {
        self.changes.clear();
    }
}

#[derive(Component)]
pub struct HealthChangeTextAnimation {
    timer: Timer,
    direction: Vec2,
    speed: f32,
}

#[derive(Component)]
pub struct DeathAnimation {
    pub timer: Timer,
}

#[derive(Event)]
pub struct PieceHealthChangeEvent {
    pub entity: Entity,
    pub change: i64,
}

pub fn spawn_health_change_text(
    mut health_query: Query<(&mut Health, &Transform, &Team)>,
    mut commands: Commands,
) {
    for (mut health, transform, team) in health_query.iter_mut() {
        for change in health.changes.iter() {
            let color = if *change < 0 {
                if *team == Team::Player {
                    Color::srgba(1.0, 0.0, 0.0, 1.0)
                } else {
                    Color::srgba(0.0, 1.0, 0.0, 1.0)
                }
            } else {
                if *team == Team::Player {
                    Color::srgba(0.0, 1.0, 0.0, 1.0)
                } else {
                    Color::srgba(1.0, 0.0, 0.0, 1.0)
                }
            };

            let mut rng = rand::thread_rng();
            let direction = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));

            commands.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        format!("{}", change),
                        TextStyle {
                            font_size: 14.0,
                            color: color,
                            ..default()
                        },
                    ),
                    transform: Transform {
                        translation: Vec3::new(
                            transform.translation.x,
                            transform.translation.y,
                            HEALTH_CHANGE_TEXT_Z_INDEX,
                        ),
                        ..default()
                    },
                    ..default()
                },
                HealthChangeTextAnimation {
                    timer: Timer::from_seconds(
                        HEALTH_CHANGE_TEXT_ANIMATION_DURATION,
                        TimerMode::Once,
                    ),
                    direction: direction,
                    speed: HEALTH_CHANGE_TEXT_ANIMATION_SPEED,
                },
                StateScoped(GameState::Game),
            ));
            debug!("Spawned health change text: {}", change);
        }
        health.clear_changes();
    }
}

pub fn health_change_system(
    mut health_change_event_reader: EventReader<PieceHealthChangeEvent>,
    mut health_query: Query<&mut Health>,
) {
    for event in health_change_event_reader.read() {
        if event.change < 0 {
            health_query
                .get_mut(event.entity)
                .unwrap()
                .take_damage(-event.change as u64);
        } else {
            health_query
                .get_mut(event.entity)
                .unwrap()
                .heal(event.change as u64);
        }
    }
}

pub fn death_system(
    health_query: Query<(&Health, Entity, &Name), Without<DeathAnimation>>,
    mut commands: Commands,
    mut death_event_writer: EventWriter<PieceDeathEvent>,
) {
    for (health, entity, name) in health_query.iter() {
        if health.is_dead() {
            // Add delay before despawn
            debug!("Entity {} ({}) is dead", entity, name);
            commands.entity(entity).insert(DeathAnimation {
                timer: Timer::from_seconds(DEATH_ANIMATION_DURATION, TimerMode::Once),
            });
            death_event_writer.send(PieceDeathEvent { entity });
        }
    }
}

pub fn death_animation(
    mut death_animation_query: Query<(&mut DeathAnimation, Entity, &Name)>,
    time: Res<Time>,
    mut commands: Commands,
    mut highlight_cache: ResMut<HighlightCache>,
) {
    for (mut death_animation, entity, name) in death_animation_query.iter_mut() {
        death_animation.timer.tick(time.delta());
        if death_animation.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
            highlight_cache.invalidate();
            debug!(
                "Entity {} ({}) death animation finished, despawned",
                entity, name
            );
        }
    }
}

pub fn health_change_text_animation(
    mut health_change_text_query: Query<(&mut Transform, &mut HealthChangeTextAnimation, Entity)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut transform, mut health_change_text_animation, entity) in
        health_change_text_query.iter_mut()
    {
        health_change_text_animation.timer.tick(time.delta());
        if health_change_text_animation.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        } else {
            transform.translation += Vec3::new(
                health_change_text_animation.direction.x,
                health_change_text_animation.direction.y,
                0.0,
            ) * health_change_text_animation.speed
                * time.delta_seconds();
        }
    }
}
