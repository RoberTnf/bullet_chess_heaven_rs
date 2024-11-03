use bevy::prelude::*;

use crate::{
    globals::{
        GOLD_UI_COLOR_DURATION, SPRITESHEET_WIDTH, UI_FONT, UI_FONT_SIZE, UI_HEADER_FONT_SIZE,
    },
    graphics::spritesheet::SpriteSheetAtlas,
    pieces::{
        common::MovementTypes,
        health::Health,
        player::{experience::PlayerLevel, gold::Gold, spawn::Player},
    },
    states::game_state::GameState,
    utils::math::lerp,
};

use super::{game_info::setup_game_info, setup_ui, LeftUINode};

#[derive(Component)]
struct CharacterInfoNode;

#[derive(Component)]
struct HealthUILabel;

#[derive(Component)]
struct MovementTypesUIContainer;

#[derive(Component)]
struct LevelUILabel;

#[derive(Component)]
struct ExpUILabel;

#[derive(Component)]
struct GoldUILabel {
    timer: Timer,
}
#[derive(Component)]
struct MovementTypesUILabel {
    sprite_index: usize,
}

pub fn setup_character_info(
    mut commands: Commands,
    query: Query<Entity, With<LeftUINode>>,
    asset_server: Res<AssetServer>,
) {
    let root_node = query.single();
    commands.entity(root_node).with_children(|parent| {
        parent
            .spawn(NodeBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(2.0)),
                    row_gap: Val::Px(2.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn((
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            ..default()
                        },
                        CharacterInfoNode,
                    ))
                    .with_children(|parent| {
                        parent.spawn((TextBundle::from_section(
                            "Character",
                            TextStyle {
                                font_size: UI_HEADER_FONT_SIZE,
                                font: asset_server.load(UI_FONT),
                                ..default()
                            },
                        ),));
                        parent.spawn((
                            TextBundle::from_section(
                                "HealthPlaceholder",
                                TextStyle {
                                    font_size: UI_FONT_SIZE,
                                    font: asset_server.load(UI_FONT),
                                    ..default()
                                },
                            ),
                            HealthUILabel,
                        ));
                        parent.spawn((
                            TextBundle::from_section(
                                "LevelPlaceholder",
                                TextStyle {
                                    font_size: UI_FONT_SIZE,
                                    font: asset_server.load(UI_FONT),
                                    ..default()
                                },
                            ),
                            LevelUILabel,
                        ));
                        parent.spawn((
                            TextBundle::from_section(
                                "ExpPlaceholder",
                                TextStyle {
                                    font_size: UI_FONT_SIZE,
                                    font: asset_server.load(UI_FONT),
                                    ..default()
                                },
                            ),
                            ExpUILabel,
                        ));
                        parent.spawn((
                            TextBundle::from_section(
                                "GoldPlaceholder",
                                TextStyle {
                                    font_size: UI_FONT_SIZE,
                                    font: asset_server.load(UI_FONT),
                                    color: Color::srgb(1.0, 1.0, 1.0),
                                },
                            ),
                            GoldUILabel {
                                timer: Timer::from_seconds(GOLD_UI_COLOR_DURATION, TimerMode::Once),
                            },
                        ));
                        parent
                            .spawn((
                                NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Row,
                                        ..default()
                                    },
                                    ..default()
                                },
                                MovementTypesUIContainer,
                            ))
                            .with_children(|parent| {
                                parent.spawn((TextBundle::from_section(
                                    "Moves:",
                                    TextStyle {
                                        font_size: UI_FONT_SIZE,
                                        font: asset_server.load(UI_FONT),
                                        ..default()
                                    },
                                ),));
                            });
                    });
            });
    });
}

fn update_movement_types_information(
    movement_types_query: Query<&MovementTypes, With<Player>>,
    parent_query: Query<Entity, With<MovementTypesUIContainer>>,
    mut text_query: Query<(Entity, &mut TextureAtlas, &MovementTypesUILabel)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    atlas_layout: Res<SpriteSheetAtlas>,
) {
    let container_entity = parent_query.single();
    let movement_types = movement_types_query.single();

    if movement_types.0.len() != text_query.iter().len() {
        // Despawn all labels
        for (entity, _, _) in text_query.iter() {
            commands.entity(entity).despawn();
        }
        // Spawn all labels
        for movement_type in movement_types.0.iter() {
            let player_sprite_index = movement_type.sprite_index() + SPRITESHEET_WIDTH;
            debug!("Spawning movement type in UI: {}", player_sprite_index);
            commands.entity(container_entity).with_children(|parent| {
                parent.spawn((
                    ImageBundle {
                        style: Style {
                            width: Val::Px(12.0),
                            height: Val::Px(12.0),
                            ..default()
                        },
                        image: UiImage::new(asset_server.load("custom/spritesheet.png")),
                        ..default()
                    },
                    TextureAtlas {
                        layout: atlas_layout.handle.clone(),
                        index: player_sprite_index,
                    },
                    MovementTypesUILabel {
                        sprite_index: player_sprite_index,
                    },
                ));
            });
        }
    } else {
        for ((_, mut atlas, label), movement_type) in
            text_query.iter_mut().zip(movement_types.0.iter())
        {
            let player_sprite_index = movement_type.sprite_index() + SPRITESHEET_WIDTH;
            // Update the text if the sprite index has changed5
            if label.sprite_index != player_sprite_index {
                debug!(
                    "Updating movement type in UI: {} -> {}",
                    label.sprite_index,
                    movement_type.sprite_index()
                );
                atlas.index = player_sprite_index;
            }
        }
    }
}

fn update_health_information(
    health: Query<&Health, With<Player>>,
    mut query: Query<&mut Text, With<HealthUILabel>>,
) {
    let mut text = query.get_single_mut().unwrap();
    let health = health.single();
    text.sections[0].value = format!("Health: {} / {}", health.value, health.max_value);
}

fn update_level_information(
    mut level_query: Query<&mut Text, (With<LevelUILabel>, Without<ExpUILabel>)>,
    mut exp_query: Query<&mut Text, (With<ExpUILabel>, Without<LevelUILabel>)>,
    level: Res<PlayerLevel>,
) {
    let mut level_text = level_query.get_single_mut().unwrap();
    level_text.sections[0].value = format!("Level: {}", level.level);

    let mut exp_text = exp_query.get_single_mut().unwrap();
    exp_text.sections[0].value = format!(
        "Exp: {} / {}",
        level.experience,
        level.get_exp_to_next_level()
    );
}

fn update_gold_information(
    mut query: Query<(&mut Text, &mut GoldUILabel)>,
    gold: Res<Gold>,
    time: Res<Time>,
) {
    let time_delta = time.delta();
    let (mut text, mut gold_ui_label) = query.get_single_mut().unwrap();
    let has_changed_recently =
        !gold_ui_label.timer.tick(time.delta()).finished() && gold.amount != 0;
    if let Color::Srgba(color) = text.sections[0].style.color {
        let blue = color.blue;
        let lerped = if has_changed_recently {
            lerp(blue, 0.0, time_delta.as_secs_f32() / GOLD_UI_COLOR_DURATION)
        } else {
            lerp(blue, 1.0, time_delta.as_secs_f32() / GOLD_UI_COLOR_DURATION)
        };
        text.sections[0].style.color = Color::srgb(lerped, 1.0, lerped);
    }

    if gold.is_changed() {
        text.sections[0].value = format!("{}$", gold.amount);
        gold_ui_label.timer.reset();
    }
}

pub struct CharacterInfoPlugin;

impl Plugin for CharacterInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_health_information,
                update_movement_types_information,
                update_level_information,
                update_gold_information,
            )
                .run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Startup,
            setup_character_info.after(setup_ui).after(setup_game_info),
        );
    }
}
