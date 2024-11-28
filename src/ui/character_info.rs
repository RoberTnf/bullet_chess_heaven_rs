use bevy::prelude::*;

use crate::{
    globals::{
        GOLD_UI_COLOR_DURATION, PRIMARY_COLOR, SPRITESHEET_WIDTH, UI_FONT, UI_FONT_SIZE,
        UI_HEADER_FONT_SIZE, UI_PIECE_SPRITE_SIZE_INFO, UNIQUE_ABILITY_UNLOCK_UPGRADE_NUMBER,
    },
    graphics::spritesheet::SpriteSheetAtlas,
    pieces::{
        damage::Attack,
        health::Health,
        movement_type::MovementType,
        player::{
            experience::PlayerLevel,
            gold::Gold,
            spawn::Player,
            upgrades::{data::Upgrades, unique_upgrades::block::Block},
        },
    },
    states::game_state::GameState,
    utils::math::lerp,
};

use super::{game_info::setup_game_info, setup_ui, shop::ApplyUpgrades, LeftUINode};

#[derive(Component)]
struct CharacterInfoNode;

#[derive(Component)]
struct HealthUILabel;

#[derive(Component)]
struct AttackUILabel;

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
                    padding: UiRect::all(Val::Px(8.0)),
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
                                "AttackPlaceholder",
                                TextStyle {
                                    font_size: UI_FONT_SIZE,
                                    font: asset_server.load(UI_FONT),
                                    ..default()
                                },
                            ),
                            AttackUILabel,
                        ));
                        // parent.spawn((
                        //     TextBundle::from_section(
                        //         "LevelPlaceholder",
                        //         TextStyle {
                        //             font_size: UI_FONT_SIZE,
                        //             font: asset_server.load(UI_FONT),
                        //             ..default()
                        //         },
                        //     ),
                        //     LevelUILabel,
                        // ));
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
                        parent.spawn((TextBundle::from_section(
                            "Moves:",
                            TextStyle {
                                font_size: UI_FONT_SIZE,
                                font: asset_server.load(UI_FONT),
                                ..default()
                            },
                        ),));
                        parent.spawn((
                            NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Row,
                                    ..default()
                                },
                                ..default()
                            },
                            MovementTypesUIContainer,
                        ));
                    });
            });
    });
}

fn update_movement_types_information(
    movement_types_query: Query<&Upgrades, With<Player>>,
    parent_query: Query<Entity, With<MovementTypesUIContainer>>,
    text_query: Query<Entity, With<MovementTypesUILabel>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    atlas_layout: Res<SpriteSheetAtlas>,
) {
    let container_entity = parent_query.single();
    let upgrades = movement_types_query.single();
    let mut movement_types = upgrades.get_movement_types_count();
    // Even tho black and white pawns are different, In the the UI there is just pawn
    movement_types.retain(|movement_type, _| movement_type != &MovementType::BlackPawn);

    // Despawn all labels
    for entity in text_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    // Spawn all labels
    for (movement_type, &count) in movement_types.iter() {
        let player_sprite_index;
        let text_color;
        if count >= UNIQUE_ABILITY_UNLOCK_UPGRADE_NUMBER {
            player_sprite_index = movement_type.sprite_index() + SPRITESHEET_WIDTH;
            text_color = PRIMARY_COLOR;
        } else {
            player_sprite_index = movement_type.sprite_index();
            text_color = Color::srgb(1.0, 1.0, 1.0);
        };
        debug!("Spawning movement type in UI: {}", player_sprite_index);
        commands.entity(container_entity).with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            flex_wrap: FlexWrap::Wrap,
                            ..default()
                        },
                        ..default()
                    },
                    MovementTypesUILabel {
                        sprite_index: player_sprite_index,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ImageBundle {
                            style: Style {
                                width: Val::Px(UI_PIECE_SPRITE_SIZE_INFO),
                                height: Val::Px(UI_PIECE_SPRITE_SIZE_INFO),
                                ..default()
                            },
                            image: UiImage::new(asset_server.load("custom/spritesheet.png")),
                            ..default()
                        },
                        TextureAtlas {
                            layout: atlas_layout.handle.clone(),
                            index: player_sprite_index,
                        },
                    ));
                    parent.spawn((TextBundle::from_section(
                        count.to_string(),
                        TextStyle {
                            font_size: UI_FONT_SIZE,
                            font: asset_server.load(UI_FONT),
                            color: text_color,
                        },
                    ),));
                });
        });
    }
}

fn update_health_information(
    health: Query<(&Health, &Block), With<Player>>,
    mut query: Query<&mut Text, With<HealthUILabel>>,
) {
    let mut text = query.get_single_mut().unwrap();
    let (health, block) = health.single();

    text.sections[0].value = format!(
        "Health: {} / {}",
        health.value, health.max_value.upgraded_value
    );
    if block.amount > 0 {
        text.sections[0].value = format!(
            "Health: {} / {}\nBlock({})",
            health.value, health.max_value.upgraded_value, block.amount
        );
    }
}

fn update_attack_information(
    mut query: Query<&mut Text, With<AttackUILabel>>,
    attack: Query<&Attack, With<Player>>,
) {
    let mut text = query.get_single_mut().unwrap();
    let attack = attack.single();
    text.sections[0].value = format!("Attack: {}", attack.0.upgraded_value);
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
                update_movement_types_information.run_if(on_event::<ApplyUpgrades>()),
                update_attack_information.run_if(on_event::<ApplyUpgrades>()),
                // update_level_information,
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
