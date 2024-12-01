use bevy::prelude::*;
use button::ButtonPlugin;
use character_info::CharacterInfoPlugin;
#[cfg(debug_assertions)]
use debug::DebugPlugin;
use defeat::DefeatPlugin;
use game_info::GameInfoPlugin;
use messages::MessagesPlugin;
use right_side::RightSidePlugin;
use shop::ShopPlugin;
mod button;
mod character_info;
mod debug;
mod defeat;
mod game_info;
pub mod messages;
mod right_side;
pub mod shop;
use crate::states::turn_state::TurnInfo;

fn display_turn_information(turn_info: Res<TurnInfo>) {
    println!("Turn: {}", turn_info.number);
}

#[derive(Component)]
struct RootUINode;

#[derive(Component)]
struct LeftUINode;

#[derive(Component)]
struct RightUINode;

fn setup_ui(mut commands: Commands) {
    debug!("Setting up UI");
    // root node
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            RootUINode,
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Percent(25.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::all(Val::Px(16.0)),
                    ..default()
                },
                LeftUINode,
            ));
            parent.spawn((
                Node {
                    width: Val::Percent(25.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::all(Val::Px(16.0)),
                    ..default()
                },
                RightUINode,
            ));
        });
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_plugins((GameInfoPlugin, CharacterInfoPlugin))
            .add_plugins(DefeatPlugin)
            .add_plugins(ButtonPlugin)
            .add_plugins(ShopPlugin)
            .add_plugins(RightSidePlugin)
            .add_plugins(MessagesPlugin);

        #[cfg(debug_assertions)]
        app.add_plugins(DebugPlugin);
    }
}
