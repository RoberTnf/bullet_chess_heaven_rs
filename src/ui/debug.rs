use bevy::prelude::*;

use crate::{
    globals::{UI_FONT, UI_FONT_SIZE, UI_HEADER_FONT_SIZE},
    states::{game_state::GameState, pause_state::GamePauseState, turn_state::TurnState},
};

use super::{
    character_info::setup_character_info, game_info::setup_game_info, setup_ui, LeftUINode,
};

#[derive(Component)]
struct DebugUINode;

#[derive(Component)]
pub struct GameStateLabel;

#[derive(Component)]
pub struct TurnStateLabel;

#[derive(Component)]
pub struct PauseStateLabel;

pub fn setup_debug_ui(
    mut commands: Commands,
    query: Query<Entity, With<LeftUINode>>,
    asset_server: Res<AssetServer>,
) {
    let root_node = query.single();
    commands.entity(root_node).with_children(|parent| {
        parent
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    row_gap: Val::Px(2.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            ))
            .with_children(|p2| {
                p2.spawn((
                    Text("Debug".to_string()),
                    TextFont {
                        font_size: UI_HEADER_FONT_SIZE,
                        font: asset_server.load(UI_FONT),
                        ..default()
                    },
                ));
                p2.spawn((
                    Text("Game State:".to_string()),
                    TextFont {
                        font_size: UI_FONT_SIZE,
                        font: asset_server.load(UI_FONT),
                        ..default()
                    },
                    GameStateLabel,
                ));
                p2.spawn((
                    Text("Turn State:".to_string()),
                    TextFont {
                        font_size: UI_FONT_SIZE,
                        font: asset_server.load(UI_FONT),
                        ..default()
                    },
                    TurnStateLabel,
                ));
                p2.spawn((
                    Text("Pause State:".to_string()),
                    TextFont {
                        font_size: UI_FONT_SIZE,
                        font: asset_server.load(UI_FONT),
                        ..default()
                    },
                    PauseStateLabel,
                ));
            });
    });
}

pub fn update_debug_game_state_information(
    mut game_state_label: Query<(&mut Text, &mut TextColor), With<GameStateLabel>>,
    game_state: Res<State<GameState>>,
) {
    let game_state = game_state.get();
    let game_state_text = game_state.to_string();
    let game_state_color = game_state.get_color();

    let (mut game_state_label, mut game_state_color_) = game_state_label
        .get_single_mut()
        .expect("Game state label not found");
    game_state_label.0 = game_state_text;
    game_state_color_.0 = game_state_color;
}

pub fn update_debug_turn_state_information(
    mut turn_state_label: Query<(&mut Text, &mut TextColor), With<TurnStateLabel>>,
    turn_state: Res<State<TurnState>>,
) {
    let turn_state = turn_state.get();
    let turn_state_text = turn_state.to_string();
    let turn_state_color = turn_state.get_color();

    let (mut turn_state_label, mut turn_state_color_) = turn_state_label
        .get_single_mut()
        .expect("Turn state label not found");
    turn_state_label.0 = turn_state_text;
    turn_state_color_.0 = turn_state_color;
}

pub fn update_debug_pause_state_information(
    mut pause_state_label: Query<(&mut Text, &mut TextColor), With<PauseStateLabel>>,
    pause_state: Res<State<GamePauseState>>,
) {
    let pause_state = pause_state.get();
    let pause_state_text = pause_state.to_string();
    let pause_state_color = pause_state.get_color();

    let (mut pause_state_label, mut pause_state_color_) = pause_state_label
        .get_single_mut()
        .expect("Pause state label not found");
    pause_state_label.0 = pause_state_text;
    pause_state_color_.0 = pause_state_color;
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_debug_game_state_information.run_if(state_changed::<GameState>),
                update_debug_turn_state_information.run_if(state_changed::<TurnState>),
                update_debug_pause_state_information.run_if(state_changed::<GamePauseState>),
            ),
        );

        app.add_systems(
            Startup,
            setup_debug_ui
                .after(setup_ui)
                .after(setup_game_info)
                .after(setup_character_info),
        );
    }
}
