use bevy::prelude::*;

use crate::{
    globals::{UI_FONT, UI_HEADER_FONT_SIZE},
    input::keyboard::ToggleShop,
    states::{game_state::GameState, pause_state::GamePauseState},
};

use super::RootUINode;

#[derive(Component)]
struct ShopNode;

pub fn toggle_shop(
    mut event_reader: EventReader<ToggleShop>,
    current_state: Res<State<ShopState>>,
    mut next_state: ResMut<NextState<ShopState>>,
    mut pause_state: ResMut<NextState<GamePauseState>>,
    mut commands: Commands,
    root_node: Query<Entity, With<RootUINode>>,
    asset_server: Res<AssetServer>,
) {
    for _ in event_reader.read() {
        let root_node = root_node.get_single().expect("Root node not found");
        match current_state.get() {
            ShopState::Closed => {
                debug!("Shop opened");
                next_state.set(ShopState::Open);
                pause_state.set(GamePauseState::Paused);
                spawn_shop(&mut commands, &root_node, &asset_server);
            }
            ShopState::Open => {
                debug!("Shop closed");
                next_state.set(ShopState::Closed);
                pause_state.set(GamePauseState::Playing);
            }
        }
    }
}

fn spawn_shop(commands: &mut Commands, root_node: &Entity, asset_server: &Res<AssetServer>) {
    let shop_node = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.0, 0.0, 0.0)),
                ..default()
            },
            Name::new("ShopUI"),
            StateScoped(ShopState::Open),
            ShopNode,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Shop",
                TextStyle {
                    font_size: UI_HEADER_FONT_SIZE,
                    font: asset_server.load(UI_FONT),
                    ..default()
                },
            ));
        })
        .id();

    commands.entity(*root_node).add_child(shop_node);
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum ShopState {
    Open,
    #[default]
    Closed,
}

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(ShopState::Closed)
            .enable_state_scoped_entities::<ShopState>();
        app.add_systems(Update, toggle_shop.run_if(in_state(GameState::Game)));
    }
}
