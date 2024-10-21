use bevy::prelude::*;
use debug::{setup_debug_ui, DebugPlugin};
use game_info::{setup_game_info, update_turn_information};

mod debug;
mod game_info;
use crate::states::turn_state::TurnInfo;

fn display_turn_information(turn_info: Res<TurnInfo>) {
    println!("Turn: {}", turn_info.number);
}

#[derive(Component)]
struct RootUINode;

#[derive(Component)]
struct LeftUINode;
fn setup_ui(mut commands: Commands) {
    // root node
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            RootUINode,
        ))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(20.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    ..default()
                },
                LeftUINode,
            ));
        });
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_ui, setup_game_info, setup_debug_ui).chain())
            .add_systems(
                Update,
                (update_turn_information.run_if(resource_changed::<TurnInfo>),),
            )
            .add_plugins(DebugPlugin);
    }
}
