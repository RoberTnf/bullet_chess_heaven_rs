use bevy::prelude::*;

use crate::{
    game_logic::score::GameScore,
    globals::{DEFEAT_HEADER_FONT_SIZE, DEFEAT_SCORE_FONT_SIZE, UI_FONT},
    states::game_state::GameState,
};

use super::{
    button::{ButtonFunction, ButtonPressedEvent},
    RootUINode,
};

pub struct DefeatPlugin;

impl Plugin for DefeatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Defeat), show_defeat_screen)
            .add_systems(
                Update,
                handle_defeat_button_pressed.run_if(in_state(GameState::Defeat)),
            );
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
            NodeBundle {
                style: Style {
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
                background_color: BackgroundColor(Color::srgb(0.0, 0.0, 0.0)),
                ..default()
            },
            Name::new("DefeatUI"),
            StateScoped(GameState::Defeat),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Defeat",
                TextStyle {
                    font_size: DEFEAT_HEADER_FONT_SIZE,
                    font: asset_server.load(UI_FONT),
                    ..default()
                },
            ));
            parent.spawn(TextBundle::from_section(
                format!("Score: {}", score.0),
                TextStyle {
                    font_size: DEFEAT_SCORE_FONT_SIZE,
                    font: asset_server.load(UI_FONT),
                    ..default()
                },
            ));
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(10.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        border_radius: BorderRadius::all(Val::Px(2.0)),
                        ..default()
                    },
                    ButtonFunction::Restart,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Restart",
                        TextStyle {
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

pub fn handle_defeat_button_pressed(
    mut event_reader: EventReader<ButtonPressedEvent>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for event in event_reader.read() {
        match event.function {
            ButtonFunction::Restart => {
                game_state.set(GameState::Game);
            }
        }
    }
}
