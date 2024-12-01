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
    debug!("Setting up game info");
    let root_node = query.single();
    commands.entity(root_node).with_children(|parent| {
        parent
            .spawn((
                Node {
                    padding: UiRect::all(Val::Px(8.0)),
                    row_gap: Val::Px(2.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text("Game".to_string()),
                    TextFont {
                        font_size: UI_HEADER_FONT_SIZE,
                        font: asset_server.load(UI_FONT),
                        ..default()
                    },
                ));
                parent.spawn((
                    Text("TurnPlaceholder".to_string()),
                    TextFont {
                        font_size: UI_FONT_SIZE,
                        font: asset_server.load(UI_FONT),
                        ..default()
                    },
                    TurnUILabel,
                ));
                parent.spawn((
                    Text("ScorePlaceholder".to_string()),
                    TextFont {
                        font_size: UI_FONT_SIZE,
                        font: asset_server.load(UI_FONT),
                        ..default()
                    },
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
    text.0 = format!("Turn: {}", turn_info.number);
}

fn update_score_information(
    score: Res<GameScore>,
    mut query: Query<&mut Text, With<ScoreUILabel>>,
) {
    let mut text = query.get_single_mut().unwrap();
    text.0 = format!("Score: {}", score.0);
}

pub struct GameInfoPlugin;

impl Plugin for GameInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_turn_information, update_score_information))
            .add_systems(Startup, setup_game_info.after(setup_ui));
    }
}
