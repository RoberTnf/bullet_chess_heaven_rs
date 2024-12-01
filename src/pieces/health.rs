use std::time::Duration;

use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    board::highlight::HighlightCache,
    globals::{
        DEATH_ANIMATION_DURATION, HEALTH_CHANGE_TEXT_ANIMATION_DURATION,
        HEALTH_CHANGE_TEXT_ANIMATION_SPEED, HEALTH_CHANGE_TEXT_FONT_SIZE,
        HEALTH_CHANGE_TEXT_Z_INDEX, PRIMARY_COLOR, UI_FONT,
    },
    states::game_state::GameState,
};

use super::{
    common::Team,
    player::{
        spawn::Player,
        upgrades::{
            stats::{Stat, StatVariant},
            unique_upgrades::{block::Block, immortal::Immortal},
        },
    },
};

#[derive(Component, Default, Debug)]
pub struct Health {
    pub value: f32,
    pub changes: Vec<f32>,
    pub max_value: Stat,
}

#[derive(Event)]
pub struct PieceDeathEvent {
    pub entity: Entity,
}

impl Health {
    pub fn new(value: f32) -> Self {
        Health {
            value,
            changes: vec![],
            max_value: Stat {
                base_value: value,
                stat_variant: StatVariant::MaxHealth,
                upgraded_value: value,
            },
        }
    }

    pub fn take_damage(&mut self, damage: f32) {
        self.value -= damage;
        self.value = self.value.clamp(0.0, self.max_value.upgraded_value);
        self.changes.push(-damage);
    }

    pub fn is_dead(&self) -> bool {
        self.value <= 0.0
    }

    pub fn heal(&mut self, amount: f32) {
        self.value += amount;
        self.value = self.value.clamp(0.0, self.max_value.upgraded_value);
        self.changes.push(amount);
    }

    pub fn set_health(&mut self, value: f32) {
        self.changes.push(value - self.value);
        self.value = value;
    }

    pub fn clear_changes(&mut self) {
        self.changes.clear();
    }
}

#[derive(Component)]
pub struct TextAnimation {
    pub timer: Timer,
    pub direction: Vec2,
    pub speed: f32,
}

impl Default for TextAnimation {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        TextAnimation {
            timer: Timer::new(Duration::from_secs(1), TimerMode::Once),
            direction: Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize(),
            speed: 0.0,
        }
    }
}

#[derive(Component)]
pub struct DeathAnimation {
    pub timer: Timer,
}

#[derive(Event, Copy, Clone)]
pub struct PieceHealthChangeEvent {
    pub entity: Entity,
    pub change: f32,
}

pub fn spawn_health_change_text(
    mut health_query: Query<(&mut Health, &Transform, &Team)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (mut health, transform, team) in health_query.iter_mut() {
        for change in health.changes.iter() {
            let color = if *change < 0.0 {
                if *team == Team::Player {
                    Color::srgba(1.0, 0.0, 0.0, 1.0)
                } else {
                    PRIMARY_COLOR
                }
            } else if *change == 0.0 {
                Color::srgba(0.7, 0.7, 0.7, 1.0)
            } else if *team == Team::Player {
                Color::srgba(0.0, 1.0, 0.0, 1.0)
            } else {
                PRIMARY_COLOR
            };

            commands.spawn((
                Text2d(format!("{}", change)),
                Transform {
                    translation: Vec3::new(
                        transform.translation.x,
                        transform.translation.y,
                        HEALTH_CHANGE_TEXT_Z_INDEX,
                    ),
                    ..default()
                },
                TextFont {
                    font: asset_server.load(UI_FONT),
                    font_size: HEALTH_CHANGE_TEXT_FONT_SIZE,
                    ..default()
                },
                TextColor(color),
                TextAnimation {
                    timer: Timer::from_seconds(
                        HEALTH_CHANGE_TEXT_ANIMATION_DURATION,
                        TimerMode::Once,
                    ),
                    speed: HEALTH_CHANGE_TEXT_ANIMATION_SPEED,
                    ..default()
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
    mut health_query: Query<(&mut Health, &mut Block)>,
) {
    for event in health_change_event_reader.read() {
        if event.change < 0.0 {
            if let Ok((mut health, mut block)) = health_query.get_mut(event.entity) {
                if block.amount > 0 {
                    block.amount -= 1;
                    health.take_damage(0.0);
                    return;
                }
                health.take_damage(-event.change);
            }
        } else if let Ok((mut health, _)) = health_query.get_mut(event.entity) {
            health.heal(event.change);
        }
    }
}

// death of player is handled by game logic
pub fn death_system(
    health_query: Query<
        (&Health, Entity, &Name),
        (Without<DeathAnimation>, Without<Player>, Without<Immortal>),
    >,
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
    mut health_change_text_query: Query<(&mut Transform, &mut TextAnimation, Entity)>,
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
                * time.delta_secs();
        }
    }
}
