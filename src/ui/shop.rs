use bevy::{a11y::accesskit::TextAlign, prelude::*};

use crate::{
    board::highlight::HighlightCache,
    globals::{
        SHOP_UPGRADES_COUNT_MOVEMENT, SHOP_UPGRADES_COUNT_STATS, UI_FONT, UI_FONT_SIZE,
        UI_HEADER_FONT_SIZE, UI_PIECE_SPRITE_SIZE_SHOP,
    },
    graphics::spritesheet::SpriteSheetAtlas,
    input::keyboard::ToggleShop,
    pieces::{
        damage::Attack,
        health::Health,
        player::{
            gold::Gold,
            spawn::Player,
            upgrades::{
                data::{Effect, Upgrade, Upgrades, UPGRADES_MOVEMENT, UPGRADES_STATS},
                stats::StatVariant,
            },
        },
    },
    states::{game_state::GameState, pause_state::GamePauseState, turn_state::TurnState},
    utils::rng::sample_weighted,
};

use super::{
    button::{ButtonFunction, ButtonHoverEvent, ButtonPressedEvent},
    right_side::get_shop_button,
    RootUINode,
};

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
                    width: Val::Vw(50.0),
                    height: Val::Vh(100.0),
                    top: Val::Px(0.0),
                    left: Val::Percent(25.0),
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

fn update_shop(shop_upgrades: &mut ResMut<ShopUpgrades>) {
    let upgrades_mov = sample_weighted(SHOP_UPGRADES_COUNT_MOVEMENT, &UPGRADES_MOVEMENT);
    let upgrades_stats = sample_weighted(SHOP_UPGRADES_COUNT_STATS, &UPGRADES_STATS);
    let chosen_upgrades = upgrades_mov.into_iter().chain(upgrades_stats);
    **shop_upgrades = ShopUpgrades(chosen_upgrades.collect());
}

#[derive(Event)]
pub struct ApplyUpgrades(pub Upgrade);

fn buy_upgrade(
    mut event_reader: EventReader<ButtonPressedEvent>,
    upgrade: Query<&Upgrade>,
    mut gold: ResMut<Gold>,
    mut refresh_event_writer: EventWriter<RefreshShop>,
    mut player_upgrades: Query<&mut Upgrades, With<Player>>,
    mut apply_upgrades_event_writer: EventWriter<ApplyUpgrades>,
) {
    for event in event_reader.read() {
        if event.function == ButtonFunction::BuyUpgrade {
            let upgrade = upgrade.get(event.entity).expect("Upgrade not found");
            if gold.amount >= upgrade.cost {
                gold.amount -= upgrade.cost;
                player_upgrades.single_mut().0.push(upgrade.clone());
                debug!("Bought upgrade: {}", upgrade.display_name);
                refresh_event_writer.send(RefreshShop { cost: 0 });
                apply_upgrades_event_writer.send(ApplyUpgrades(upgrade.clone()));
            } else {
                debug!("Not enough gold to buy upgrade: {}", upgrade.display_name);
            }
        }
    }
}

fn update_shop_description(
    mut hover_event_reader: EventReader<ButtonHoverEvent>,
    mut description_ui: Query<&mut Text, With<ShopDescriptionUI>>,
    upgrade: Query<&Upgrade>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load(UI_FONT);
    for event in hover_event_reader.read() {
        if event.function == ButtonFunction::BuyUpgrade {
            let mut description_ui = description_ui
                .get_single_mut()
                .expect("Description UI not found");
            let upgrade = upgrade.get(event.entity).expect("Upgrade not found");
            description_ui.sections = upgrade
                .description
                .sections
                .iter()
                .map(|s| {
                    let mut section = s.clone();
                    section.style = TextStyle {
                        font: font.clone(),
                        ..section.style
                    };
                    section
                })
                .collect();
        }
    }
}

fn apply_upgrades(
    mut player: Query<(&Upgrades, &mut Health, &mut Attack), With<Player>>,
    mut event_reader: EventReader<ApplyUpgrades>,
    mut highlight_cache: ResMut<HighlightCache>,
) {
    for upgrade in event_reader.read() {
        let (upgrades, mut health, mut attack) = player.single_mut();
        match &upgrade.0.effect {
            Effect::StatEffect(stat_effect) => match stat_effect.stat {
                StatVariant::MaxHealth => {
                    let prev_health = health.max_value.upgraded_value;
                    health.max_value.apply_upgrades(upgrades);
                    let new_health = health.max_value.upgraded_value;
                    let health_diff = (new_health / prev_health * health.value) - health.value;
                    health.heal(health_diff);
                }
                StatVariant::Attack => {
                    attack.0.apply_upgrades(upgrades);
                }
            },
            Effect::MovementType(_) => {
                highlight_cache.invalidate();
            }
        }
    }
}

#[derive(Event)]
pub struct RefreshShop {
    pub cost: usize,
}

fn update_shop_system(
    mut shop_upgrades: ResMut<ShopUpgrades>,
    mut refresh_event: EventReader<RefreshShop>,
    mut gold: ResMut<Gold>,
) {
    // ensure the shop is filled
    if shop_upgrades.0.len() != (SHOP_UPGRADES_COUNT_MOVEMENT + SHOP_UPGRADES_COUNT_STATS) {
        update_shop(&mut shop_upgrades);
    }

    for event in refresh_event.read() {
        if gold.amount >= event.cost {
            gold.amount -= event.cost;
            update_shop(&mut shop_upgrades);
        }
    }
}

#[derive(Event)]
struct RefreshShopUI;

#[derive(Component)]
pub struct ShopUpgradeUI;

#[derive(Component)]
struct ShopDescriptionUI;

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
                            width: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(4.0),
                            justify_content: JustifyContent::Center,
                            align_content: AlignContent::Center,
                            padding: UiRect::all(Val::Px(4.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        border_radius: BorderRadius::all(Val::Px(2.0)),
                        background_color: BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                        ..default()
                    },
                    ButtonFunction::BuyUpgrade,
                    upgrade.clone(),
                ))
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            upgrade.display_name.clone(),
                            TextStyle {
                                font_size: UI_FONT_SIZE,
                                font: asset_server.load(UI_FONT),
                                ..default()
                            },
                        )
                        .with_text_justify(JustifyText::Center),
                    );
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
                                width: Val::Px(UI_PIECE_SPRITE_SIZE_SHOP),
                                height: Val::Px(UI_PIECE_SPRITE_SIZE_SHOP),
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
        let description_box = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(4.0)),
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                    ..default()
                },
                ShopUpgradeUI,
            ))
            .with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        "",
                        TextStyle {
                            font_size: UI_FONT_SIZE,
                            font: asset_server.load(UI_FONT),
                            ..default()
                        },
                    ),
                    ShopDescriptionUI,
                ));
            })
            .id();
        commands.entity(shop_node).add_child(description_box);
        let refresh_button = commands
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
                ButtonFunction::RefreshShop,
                ShopUpgradeUI,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Refresh (R)",
                    TextStyle {
                        font_size: UI_FONT_SIZE,
                        font: asset_server.load(UI_FONT),
                        ..default()
                    },
                ));
            })
            .id();
        let bottom_container = commands
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
        commands.entity(shop_node).add_child(bottom_container);
        commands.entity(bottom_container).add_child(refresh_button);
        let toggle_shop_button = get_shop_button(&mut commands, &asset_server, "Exit Shop (S)");
        commands
            .entity(bottom_container)
            .add_child(toggle_shop_button);
    }
}

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(ShopState::Closed)
            .enable_state_scoped_entities::<ShopState>()
            .insert_resource(ShopUpgrades(Vec::new()));
        app.add_systems(
            Update,
            toggle_shop
                .run_if(in_state(GameState::Game))
                .run_if(in_state(TurnState::PlayerInput)),
        );
        app.add_systems(
            Update,
            (
                update_shop_system,
                display_shop,
                buy_upgrade.run_if(on_event::<ButtonPressedEvent>()),
                update_shop_description.run_if(on_event::<ButtonHoverEvent>()),
            )
                .run_if(in_state(GameState::Game))
                .run_if(in_state(ShopState::Open)),
        );
        app.add_systems(
            Update,
            apply_upgrades
                .run_if(in_state(GameState::Game))
                .run_if(on_event::<ApplyUpgrades>()),
        );
        app.add_event::<RefreshShop>().add_event::<RefreshShopUI>();
        app.add_event::<ApplyUpgrades>();
    }
}
