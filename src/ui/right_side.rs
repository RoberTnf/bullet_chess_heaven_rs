use bevy::prelude::*;

use crate::{
    globals::{UI_FONT, UI_FONT_SIZE},
    input::keyboard::ToggleShop,
    states::game_state::GameState,
};

use super::{
    button::{ButtonFunction, ButtonPressedEvent},
    setup_ui, RightUINode,
};

pub struct RightSidePlugin;
impl Plugin for RightSidePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_right_side.after(setup_ui));
        app.add_systems(
            Update,
            on_click_toggle_shop
                .run_if(in_state(GameState::Game))
                .run_if(on_event::<ButtonPressedEvent>()),
        );
        app.add_event::<ButtonPressedEvent>();
    }
}

pub fn setup_right_side(
    mut commands: Commands,
    right_side_node: Query<Entity, With<RightUINode>>,
    asset_server: Res<AssetServer>,
) {
    let right_side_node = right_side_node.single();
    let buttons_node = commands
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
        .id();
    commands.entity(right_side_node).add_child(buttons_node);
    let shop_button = get_shop_button(&mut commands, &asset_server, "Shop (S)");
    commands.entity(buttons_node).add_child(shop_button);

    let restart_button = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(10.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                border_radius: BorderRadius::all(Val::Px(2.0)),
                ..default()
            },
            ButtonFunction::RestartGame,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Restart",
                TextStyle {
                    font_size: UI_FONT_SIZE,
                    font: asset_server.load(UI_FONT),
                    ..default()
                },
            ));
        })
        .id();
    commands.entity(right_side_node).add_child(restart_button);
}

pub fn get_shop_button(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    text: &str,
) -> Entity {
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(4.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    width: Val::Percent(100.0),
                    ..default()
                },
                border_radius: BorderRadius::all(Val::Px(2.0)),
                background_color: BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                ..default()
            },
            ButtonFunction::ShowShop,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font_size: UI_FONT_SIZE,
                    font: asset_server.load(UI_FONT),
                    ..default()
                },
            ));
        })
        .id()
}

pub fn on_click_toggle_shop(
    mut event_reader: EventReader<ButtonPressedEvent>,
    mut toggle_shop_event: EventWriter<ToggleShop>,
) {
    for event in event_reader.read() {
        if event.function == ButtonFunction::ShowShop {
            toggle_shop_event.send(ToggleShop);
        }
    }
}
