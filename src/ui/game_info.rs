use bevy::prelude::*;

use crate::{
    globals::{UI_FONT, UI_FONT_SIZE, UI_HEADER_FONT_SIZE},
    pieces::experience::PlayerLevel,
    states::turn_state::TurnInfo,
};

use super::{setup_ui, LeftUINode};

#[derive(Component)]
struct GameInfoNode;

#[derive(Component)]
pub struct TurnUILabel;

#[derive(Component)]
pub struct LevelUILabel;

#[derive(Component)]
pub struct ExpUILabel;

pub fn setup_game_info(
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
                parent.spawn(TextBundle::from_section(
                    "Game",
                    TextStyle {
                        font_size: UI_HEADER_FONT_SIZE,
                        font: asset_server.load(UI_FONT),
                        ..default()
                    },
                ));
                parent.spawn((
                    TextBundle::from_section(
                        "TurnPlaceholder",
                        TextStyle {
                            font_size: UI_FONT_SIZE,
                            font: asset_server.load(UI_FONT),
                            ..default()
                        },
                    ),
                    TurnUILabel,
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
            });
    });
}

pub fn update_turn_information(
    turn_info: Res<TurnInfo>,
    mut query: Query<&mut Text, With<TurnUILabel>>,
) {
    let mut text = query.get_single_mut().unwrap();
    text.sections[0].value = format!("Turn: {}", turn_info.number);
}

pub fn update_level_information(
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

pub struct GameInfoPlugin;

impl Plugin for GameInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_turn_information, update_level_information))
            .add_systems(Startup, setup_game_info.after(setup_ui));
    }
}
