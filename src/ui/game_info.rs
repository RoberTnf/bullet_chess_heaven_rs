use bevy::prelude::*;

use crate::{
    globals::{UI_FONT, UI_FONT_SIZE},
    states::turn_state::TurnInfo,
};

use super::LeftUINode;

#[derive(Component)]
struct GameInfoNode;

#[derive(Component)]
pub struct TurnUILabel;

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
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        "Turn: 1",
                        TextStyle {
                            font_size: UI_FONT_SIZE,
                            font: asset_server.load(UI_FONT),
                            ..default()
                        },
                    ),
                    TurnUILabel,
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
