use bevy::prelude::*;

use crate::{
    game_logic::score::GameScore,
    globals::{UI_FONT, UI_FONT_SIZE, UI_HEADER_FONT_SIZE},
    states::turn_state::TurnInfo,
};

use super::{setup_ui, LeftUINode};

#[derive(Component)]
struct GameInfoNode;

#[derive(Component)]
struct TurnUILabel;

#[derive(Component)]
struct ScoreUILabel;

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
                        "ScorePlaceholder",
                        TextStyle {
                            font_size: UI_FONT_SIZE,
                            font: asset_server.load(UI_FONT),
                            ..default()
                        },
                    ),
                    ScoreUILabel,
                ));
            });
    });
}

fn update_turn_information(
    turn_info: Res<TurnInfo>,
    mut query: Query<&mut Text, With<TurnUILabel>>,
) {
    let mut text = query.get_single_mut().unwrap();
    text.sections[0].value = format!("Turn: {}", turn_info.number);
}

fn update_score_information(
    score: Res<GameScore>,
    mut query: Query<&mut Text, With<ScoreUILabel>>,
) {
    let mut text = query.get_single_mut().unwrap();
    text.sections[0].value = format!("Score: {}", score.0);
}

pub struct GameInfoPlugin;

impl Plugin for GameInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_turn_information, update_score_information))
            .add_systems(Startup, setup_game_info.after(setup_ui));
    }
}
