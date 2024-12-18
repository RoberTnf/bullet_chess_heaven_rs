use bevy::prelude::*;

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
                unique_upgrades::limit::MovementTypeLimit,
            },
        },
    },
    states::{game_state::GameState, pause_state::GamePauseState, turn_state::TurnState},
    utils::rng::sample_weighted,
};

use super::{
    button::{ButtonFunction, ButtonHoverEvent, ButtonPressedEvent},
    messages::MessageEvent,
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
            Node {
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
            BackgroundColor(Color::srgb(0.0, 0.0, 0.0)),
            Name::new("ShopUI"),
            StateScoped(ShopState::Open),
            ShopNode,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text("Shop".to_string()),
                TextFont {
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
    mut player_upgrades_query: Query<(&mut Upgrades, &MovementTypeLimit), With<Player>>,
    mut apply_upgrades_event_writer: EventWriter<ApplyUpgrades>,
    mut message_event_writer: EventWriter<MessageEvent>,
) {
    for event in event_reader.read() {
        if event.function == ButtonFunction::BuyUpgrade {
            let upgrade = upgrade.get(event.entity).expect("Upgrade not found");
            if gold.amount >= upgrade.cost {
                if let Effect::MovementType(movement_types) = &upgrade.effect {
                    let (player_upgrades, player_limit) = player_upgrades_query
                        .get_single()
                        .expect("Player upgrades not found");
                    let movement_types_set = player_upgrades.get_movement_types_set();
                    if !movement_types_set
                        .contains(movement_types.first().expect("Movement type not found"))
                    {
                        let limit = player_limit.limit;
                        let current_count = movement_types_set.len();
                        println!("{:?} {:?} {:?}", limit, current_count, movement_types_set);
                        if current_count >= limit {
                            message_event_writer.send(MessageEvent {
                                message: "You already have the maximum number of movement types. Upgrade your existing movement types to unlock new ones.".to_string(),
                                ..default()
                            });
                            return;
                        }
                    }
                }
                gold.amount -= upgrade.cost;
                let (mut player_upgrades, _) = player_upgrades_query.single_mut();
                player_upgrades.0.push(upgrade.clone());
                refresh_event_writer.send(RefreshShop { cost: 0 });
                apply_upgrades_event_writer.send(ApplyUpgrades(upgrade.clone()));
            } else {
                message_event_writer.send(MessageEvent {
                    message: "Not enough gold to buy upgrade.".to_string(),
                    ..default()
                });
            }
        }
    }
}

fn update_shop_description(
    mut hover_event_reader: EventReader<ButtonHoverEvent>,
    description_ui: Query<Entity, With<ShopDescriptionUI>>,
    upgrade_query: Query<&Upgrade>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for event in hover_event_reader.read() {
        if event.function == ButtonFunction::BuyUpgrade {
            let description_ui = description_ui
                .get_single()
                .expect("Description UI not found");
            commands.entity(description_ui).despawn_descendants();
            // I never figured out why sometimes the entity is not found,
            // but given that it is not critical to the game, I will just
            // ignore the error
            if let Ok(upgrade) = upgrade_query.get(event.entity) {
                commands.entity(description_ui).with_children(|parent| {
                    for (text, text_color) in upgrade.description.iter() {
                        parent.spawn((
                            text.clone(),
                            *text_color,
                            TextFont {
                                font: asset_server.load(UI_FONT),
                                font_size: UI_FONT_SIZE,
                                ..default()
                            },
                        ));
                    }
                });
            }
        }
    }
}

fn apply_upgrades(
    mut player: Query<(&Upgrades, &mut Health, &mut Attack), With<Player>>,
    mut event_reader: EventReader<ApplyUpgrades>,
    mut highlight_cache: ResMut<HighlightCache>,
) {
    debug!("Applying upgrades");
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
        debug!("Refreshing shop");
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
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(10.0),
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
                    Node {
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
                    BorderRadius::all(Val::Px(2.0)),
                    BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                    Button,
                    ButtonFunction::BuyUpgrade,
                    upgrade.clone(),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text(upgrade.display_name.clone()),
                        TextFont {
                            font_size: UI_FONT_SIZE,
                            font: asset_server.load(UI_FONT),
                            ..default()
                        },
                    ));
                    parent.spawn((
                        Text(format!("${}", upgrade.cost)),
                        TextFont {
                            font_size: UI_FONT_SIZE,
                            font: asset_server.load(UI_FONT),
                            ..default()
                        },
                    ));
                    parent.spawn((
                        Node {
                            width: Val::Px(UI_PIECE_SPRITE_SIZE_SHOP),
                            height: Val::Px(UI_PIECE_SPRITE_SIZE_SHOP),
                            ..default()
                        },
                        ImageNode::from_atlas_image(
                            asset_server.load("custom/spritesheet.png"),
                            TextureAtlas {
                                layout: atlas_layout.handle.clone(),
                                index: upgrade.icon_index,
                            },
                        ),
                    ));
                })
                .id();
            commands
                .entity(upgrades_container)
                .add_child(shop_upgrade_ui);
        }
        let description_box = commands
            .spawn((
                Node {
                    padding: UiRect::all(Val::Px(4.0)),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                ShopUpgradeUI,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text("".to_string()),
                    TextFont {
                        font_size: UI_FONT_SIZE,
                        font: asset_server.load(UI_FONT),
                        ..default()
                    },
                    ShopDescriptionUI,
                ));
            })
            .id();
        commands.entity(shop_node).add_child(description_box);
        let refresh_button = commands
            .spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(4.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    width: Val::Percent(100.0),
                    ..default()
                },
                BorderRadius::all(Val::Px(2.0)),
                BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                ButtonFunction::RefreshShop,
                ShopUpgradeUI,
                Button,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text("Refresh (R)".to_string()),
                    TextFont {
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
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(10.0),
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
                buy_upgrade.run_if(on_event::<ButtonPressedEvent>),
                update_shop_system,
                display_shop,
                update_shop_description.run_if(on_event::<ButtonHoverEvent>),
            )
                .run_if(in_state(GameState::Game))
                .run_if(in_state(ShopState::Open)),
        );
        app.add_systems(
            Update,
            apply_upgrades
                .run_if(in_state(GameState::Game))
                .run_if(on_event::<ApplyUpgrades>),
        );
        app.add_event::<RefreshShop>().add_event::<RefreshShopUI>();
        app.add_event::<ApplyUpgrades>();
    }
}
