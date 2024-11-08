use bevy::prelude::*;

use crate::{
    board::position::BoardPosition,
    globals::{
        GOLD_ANIMATION_DURATION, GOLD_ANIMATION_SPEED, GOLD_FONT_SIZE, GOLD_Z_INDEX, STARTING_GOLD,
        UI_FONT,
    },
    pieces::health::{PieceDeathEvent, TextAnimation},
    states::game_state::GameState,
};

use super::experience::PieceValue;

#[derive(Resource)]
pub struct Gold {
    pub amount: usize,
}

#[derive(Component)]
pub struct PickedUpGold {
    pub amount: usize,
}

impl Gold {
    pub fn new(amount: usize) -> Self {
        Self { amount }
    }
}

pub fn spawn_gold(
    mut death_event: EventReader<PieceDeathEvent>,
    enemies: Query<(&PieceValue, &BoardPosition)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut gold: ResMut<Gold>,
) {
    death_event.read().for_each(|event| {
        let (enemy_value, enemy_position) = enemies.get(event.entity).unwrap();
        let gold_position = enemy_position.as_global_position().extend(GOLD_Z_INDEX);
        commands.spawn((
            Name::new("PickedUpGold"),
            StateScoped(GameState::Game),
            PickedUpGold {
                amount: enemy_value.value,
            },
            TextAnimation {
                speed: GOLD_ANIMATION_SPEED,
                timer: Timer::from_seconds(GOLD_ANIMATION_DURATION, TimerMode::Once),
                ..default()
            },
            Text2dBundle {
                text: Text::from_section(
                    format!("+{}$", enemy_value.value),
                    TextStyle {
                        font_size: GOLD_FONT_SIZE,
                        color: Color::linear_rgb(0.0, 1.0, 0.0),
                        font: asset_server.load(UI_FONT),
                    },
                ),
                transform: Transform {
                    translation: gold_position,
                    ..default()
                },
                ..default()
            },
        ));
        gold.amount += enemy_value.value;
    });
}

pub fn reset_gold(mut gold: ResMut<Gold>) {
    debug!("Resetting gold");
    gold.amount = STARTING_GOLD;
}

pub struct GoldPlugin;

impl Plugin for GoldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gold::new(STARTING_GOLD));
        app.add_systems(Update, spawn_gold.run_if(on_event::<PieceDeathEvent>()));
        app.add_systems(OnEnter(GameState::Game), reset_gold);
    }
}
