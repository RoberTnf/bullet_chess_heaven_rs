use bevy::prelude::*;

use crate::{
    globals::{SHOP_UPGRADES_COUNT, UI_FONT, UI_FONT_SIZE, UI_HEADER_FONT_SIZE},
    graphics::spritesheet::SpriteSheetAtlas,
    input::keyboard::ToggleShop,
    pieces::player::upgrades::data::{Upgrade, UPGRADES},
    states::{game_state::GameState, pause_state::GamePauseState},
    utils::rng::sample_weighted,
};

use super::{button::ButtonFunction, RootUINode};

#[derive(Component)]
struct ShopNode;

#[derive(Resource)]
struct ShopUpgrades(Vec<Upgrade>);

fn toggle_shop(
    mut event_reader: EventReader<ToggleShop>,
    current_state: Res<State<ShopState>>,
    mut next_state: ResMut<NextState<ShopState>>,
    mut pause_state: ResMut<NextState<GamePauseState>>,
    mut commands: Commands,
    root_node: Query<Entity, With<RootUINode>>,
    asset_server: Res<AssetServer>,
    mut refresh_event: EventWriter<RefreshShopUI>,
) {
    for _ in event_reader.read() {
        let root_node = root_node.get_single().expect("Root node not found");
        match current_state.get() {
            ShopState::Closed => {
                debug!("Shop opened");
                next_state.set(ShopState::Open);
                pause_state.set(GamePauseState::Paused);
                spawn_shop(&mut commands, &root_node, &asset_server);
                refresh_event.send(RefreshShopUI);
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
                    padding: UiRect::all(Val::Px(8.0)),
                    row_gap: Val::Px(8.0),
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

fn update_shop(mut shop_upgrades: ResMut<ShopUpgrades>) {
    *shop_upgrades = ShopUpgrades(
        sample_weighted(SHOP_UPGRADES_COUNT, &UPGRADES)
            .into_iter()
            .collect(),
    );
}

#[derive(Event)]
pub struct RefreshShop;

fn update_shop_system(
    shop_upgrades: ResMut<ShopUpgrades>,
    mut refresh_event: EventReader<RefreshShop>,
) {
    if shop_upgrades.0.len() != SHOP_UPGRADES_COUNT || refresh_event.read().count() > 0 {
        update_shop(shop_upgrades);
    }
}

#[derive(Event)]
struct RefreshShopUI;

#[derive(Component)]
struct ShopUpgradeUI;

fn display_shop(
    shop_upgrades: Res<ShopUpgrades>,
    mut commands: Commands,
    shop_upgrade_ui: Query<Entity, With<ShopUpgradeUI>>,
    shop_node: Query<Entity, With<ShopNode>>,
    asset_server: Res<AssetServer>,
    mut refresh_event: EventReader<RefreshShopUI>,
    atlas_layout: Res<SpriteSheetAtlas>,
) {
    if shop_upgrades.is_changed() || refresh_event.read().count() > 0 {
        // reset shop
        for entity in shop_upgrade_ui.iter() {
            commands.entity(entity).despawn_recursive();
        }
        let shop_node = shop_node.get_single().expect("Shop node not found");
        let upgrades_container = commands
            .spawn((
                ShopUpgradeUI,
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                },
                Name::new("UpgradesContainer"),
            ))
            .id();
        commands.entity(shop_node).add_child(upgrades_container);
        // display shop
        for upgrade in shop_upgrades.0.iter() {
            let shop_upgrade_ui = commands
                .spawn((
                    ButtonBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(4.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        border_radius: BorderRadius::all(Val::Px(2.0)),
                        background_color: BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                        ..default()
                    },
                    ButtonFunction::BuyUpgrade,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        upgrade.display_name.clone(),
                        TextStyle {
                            font_size: UI_FONT_SIZE,
                            font: asset_server.load(UI_FONT),
                            ..default()
                        },
                    ));
                    parent.spawn(TextBundle::from_section(
                        format!("${}", upgrade.cost),
                        TextStyle {
                            font_size: UI_FONT_SIZE,
                            font: asset_server.load(UI_FONT),
                            ..default()
                        },
                    ));
                    parent.spawn((
                        ImageBundle {
                            style: Style {
                                width: Val::Px(12.0),
                                height: Val::Px(12.0),
                                ..default()
                            },
                            image: UiImage::new(asset_server.load("custom/spritesheet.png")),
                            ..default()
                        },
                        TextureAtlas {
                            layout: atlas_layout.handle.clone(),
                            index: upgrade.icon_index,
                        },
                    ));
                })
                .id();
            commands
                .entity(upgrades_container)
                .add_child(shop_upgrade_ui);
        }
        let refresh_button = commands
            .spawn((
                ButtonBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(4.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    border_radius: BorderRadius::all(Val::Px(2.0)),
                    background_color: BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                    ..default()
                },
                ButtonFunction::RefreshShop,
                ShopUpgradeUI,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Refresh",
                    TextStyle {
                        font_size: UI_FONT_SIZE,
                        font: asset_server.load(UI_FONT),
                        ..default()
                    },
                ));
            })
            .id();
        commands.entity(shop_node).add_child(refresh_button);
    }
}

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(ShopState::Closed)
            .enable_state_scoped_entities::<ShopState>()
            .insert_resource(ShopUpgrades(Vec::new()));
        app.add_systems(Update, toggle_shop.run_if(in_state(GameState::Game)));
        app.add_systems(
            Update,
            (update_shop_system, display_shop)
                .run_if(in_state(GameState::Game))
                .run_if(in_state(ShopState::Open)),
        );
        app.add_event::<RefreshShop>().add_event::<RefreshShopUI>();
    }
}
