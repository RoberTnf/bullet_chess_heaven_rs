use bevy::prelude::*;

use crate::{
    game_logic::score::GameScore,
    globals::{DEFEAT_HEADER_FONT_SIZE, DEFEAT_SCORE_FONT_SIZE, UI_FONT},
    states::game_state::GameState,
};

use super::{button::ButtonFunction, RootUINode};

pub struct DefeatPlugin;

impl Plugin for DefeatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Defeat), show_defeat_screen);
    }
}

fn show_defeat_screen(
    mut commands: Commands,
    root_node: Query<Entity, With<RootUINode>>,
    asset_server: Res<AssetServer>,
    score: Res<GameScore>,
) {
    debug!("Showing defeat screen");
    let root_node = root_node.single();

    let defeat_node = commands
        .spawn((
            Node {
                width: Val::Vw(100.0),
                height: Val::Vh(100.0),
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.0, 0.0, 0.0)),
            Name::new("DefeatUI"),
            StateScoped(GameState::Defeat),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text("Defeat".to_string()),
                TextFont {
                    font_size: DEFEAT_HEADER_FONT_SIZE,
                    font: asset_server.load(UI_FONT),
                    ..default()
                },
            ));
            parent.spawn((
                Text(format!("Score: {}", score.0)),
                TextFont {
                    font_size: DEFEAT_SCORE_FONT_SIZE,
                    font: asset_server.load(UI_FONT),
                    ..default()
                },
            ));
            parent
                .spawn((
                    Node {
                        padding: UiRect::all(Val::Px(10.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    BorderRadius::all(Val::Px(2.0)),
                    Button,
                    ButtonFunction::RestartGame,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text("Restart".to_string()),
                        TextFont {
                            font_size: DEFEAT_SCORE_FONT_SIZE,
                            font: asset_server.load(UI_FONT),
                            ..default()
                        },
                    ));
                });
        })
        .id();

    commands.entity(root_node).add_child(defeat_node);
}
