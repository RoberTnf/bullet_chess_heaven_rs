use bevy::prelude::*;

use crate::{
    board::position::BoardPosition,
    globals::{UI_FONT, UI_FONT_SIZE, UI_HEADER_FONT_SIZE},
    input::click_tile::HoveredTile,
    pieces::{damage::Attack, health::Health},
    states::game_state::GameState,
};

use super::right_side::HoverInfoNode;

fn display_enemy_information(
    hovered_tile: Res<HoveredTile>,
    pieces: Query<(&BoardPosition, &Attack, &Health, &Name)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut hover_info_node: Query<(Entity, &mut Visibility), With<HoverInfoNode>>,
) {
    if hovered_tile.is_changed() {
        let mut displayed = false;
        let (hover_info_node, mut visibility) = hover_info_node.single_mut();
        if let Some(tile_position) = hovered_tile.0 {
            for (board_position, attack, health, name) in pieces.iter() {
                if board_position == &tile_position {
                    displayed = true;
                    *visibility = Visibility::Visible;
                    commands.entity(hover_info_node).despawn_descendants();
                    commands.entity(hover_info_node).with_children(|parent| {
                        parent
                            .spawn((Node {
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },))
                            .with_children(|parent| {
                                parent.spawn((
                                    Text(name.to_string()),
                                    TextFont {
                                        font_size: UI_HEADER_FONT_SIZE,
                                        font: asset_server.load(UI_FONT),
                                        ..default()
                                    },
                                ));
                                parent.spawn((
                                    Text(format!(
                                        "Health: {} / {}",
                                        health.value, health.max_value.upgraded_value
                                    )),
                                    TextFont {
                                        font_size: UI_FONT_SIZE,
                                        font: asset_server.load(UI_FONT),
                                        ..default()
                                    },
                                ));
                                parent.spawn((
                                    Text(format!("Attack: {}", attack.0.upgraded_value)),
                                    TextFont {
                                        font_size: UI_FONT_SIZE,
                                        font: asset_server.load(UI_FONT),
                                        ..default()
                                    },
                                ));
                            });
                    });
                }
            }
        }
        if !displayed {
            commands.entity(hover_info_node).despawn_descendants();
            *visibility = Visibility::Hidden;
        }
    }
}

pub struct HoverInfoPlugin;

impl Plugin for HoverInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            display_enemy_information.run_if(in_state(GameState::Game)),
        );
    }
}
