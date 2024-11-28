use super::stats::StatEffect;
use crate::{
    globals::{
        CONVERT_ENEMY_TURNS_TO_CONVERT, PRIMARY_COLOR, SHOP_PIECE_VALUE_GOLD_MULTIPLIER,
        SPRITESHEET_WIDTH, UI_FONT_SIZE, UNIQUE_ABILITY_UNLOCK_UPGRADE_NUMBER,
        UNIQUE_UPGRADE_DAMAGE_MULTIPLIER, WIP_SPRITE_INDEX,
    },
    pieces::{
        enemies::{
            bishop::WHITE_BISHOP_INFO,
            king::WHITE_KING_INFO,
            knight::WHITE_KNIGHT_INFO,
            pawn::{BLACK_PAWN_INFO, WHITE_PAWN_INFO},
            queen::WHITE_QUEEN_INFO,
            rook::WHITE_ROOK_INFO,
        },
        movement_type::MovementType,
        player::upgrades::stats::StatVariant,
    },
    utils::rng::Weighted,
};
use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
use once_cell::sync::Lazy;

#[derive(Clone, Debug, Component)]
pub struct Upgrade {
    weight: f32,
    pub display_name: String,
    pub description: Text,
    pub cost: usize,
    pub rarity: Rarity,
    pub effect: Effect,
    pub icon_index: usize,
}

#[derive(Clone, Debug, Component)]
pub struct Upgrades(pub Vec<Upgrade>);

impl Upgrades {
    pub fn get_movement_types_set(&self) -> HashSet<MovementType> {
        let mut set = HashSet::new();
        for upgrade in &self.0 {
            if let Effect::MovementType(movement_types) = upgrade.effect.clone() {
                for movement_type in movement_types {
                    set.insert(movement_type);
                }
            }
        }
        set
    }

    pub fn get_movement_types_count(&self) -> HashMap<MovementType, usize> {
        let mut map = HashMap::new();
        for upgrade in &self.0 {
            if let Effect::MovementType(movement_types) = upgrade.effect.clone() {
                for movement_type in movement_types {
                    *map.entry(movement_type).or_insert(0) += 1;
                }
            }
        }
        map
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Effect {
    MovementType(Vec<MovementType>),
    StatEffect(StatEffect),
}

pub static UPGRADES_MOVEMENT: Lazy<Vec<Upgrade>> = Lazy::new(|| {
    vec![
        Upgrade {
            weight: 0.0,
            display_name: "White Pawn Movement".to_string(),
            description: Text::from_section(
                "Black Pawn movement".to_string(),
                TextStyle {
                    font_size: UI_FONT_SIZE,
                    ..default()
                },
            ),
            cost: (WHITE_PAWN_INFO.value as f32 * SHOP_PIECE_VALUE_GOLD_MULTIPLIER) as usize,
            rarity: Rarity::Common,
            effect: Effect::MovementType(vec![MovementType::WhitePawn]),
            icon_index: WHITE_PAWN_INFO.sprite_index + SPRITESHEET_WIDTH,
        },
        Upgrade {
            weight: 0.0,
            display_name: "White Pawn Movement".to_string(),
            description: Text::from_section(
                "Allows the player to move and attack like a pawn.".to_string(),
                TextStyle {
                    font_size: UI_FONT_SIZE,
                    ..default()
                },
            ),
            cost: (BLACK_PAWN_INFO.value as f32 * SHOP_PIECE_VALUE_GOLD_MULTIPLIER) as usize,
            rarity: Rarity::Common,
            effect: Effect::MovementType(vec![MovementType::BlackPawn]),
            icon_index: BLACK_PAWN_INFO.sprite_index + SPRITESHEET_WIDTH,
        },
        Upgrade {
            weight: 1.0,
            display_name: "Pawn Movement".to_string(),
            description: Text::from_sections([
                TextSection {
                    value: "Allows the player to move and attack like a pawn.\n\n".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
                TextSection {
                    value: "Level 2+:".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        color: PRIMARY_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: format!(
                        " Increases pawn damage by {}% per level.\n\n",
                        UNIQUE_UPGRADE_DAMAGE_MULTIPLIER * 100.0
                    ),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
                TextSection {
                    value: format!("Level {}+:", UNIQUE_ABILITY_UNLOCK_UPGRADE_NUMBER),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        color: PRIMARY_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: " Converts enemies to allies on hit for ".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
                TextSection {
                    value: format!("{} turns.", CONVERT_ENEMY_TURNS_TO_CONVERT),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        color: PRIMARY_COLOR,
                        ..default()
                    },
                },
            ]),
            cost: (WHITE_PAWN_INFO.value as f32 * SHOP_PIECE_VALUE_GOLD_MULTIPLIER) as usize,
            rarity: Rarity::Common,
            effect: Effect::MovementType(vec![MovementType::WhitePawn, MovementType::BlackPawn]),
            icon_index: WHITE_PAWN_INFO.sprite_index + SPRITESHEET_WIDTH,
        },
        Upgrade {
            weight: 1.0,
            display_name: "King Movement".to_string(),
            description: Text::from_sections([
                TextSection {
                    value: "Allows the player to move and attack like a king.\n\n".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
                TextSection {
                    value: "Level 2+:".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        color: PRIMARY_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: format!(
                        " Increases king damage by {}% per level.\n\n",
                        UNIQUE_UPGRADE_DAMAGE_MULTIPLIER * 100.0
                    ),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
                TextSection {
                    value: format!("Level {}+:", UNIQUE_ABILITY_UNLOCK_UPGRADE_NUMBER),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        color: PRIMARY_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: " WIP".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
            ]),
            cost: (WHITE_KING_INFO.value as f32 * SHOP_PIECE_VALUE_GOLD_MULTIPLIER) as usize,
            rarity: Rarity::Common,
            effect: Effect::MovementType(vec![MovementType::King]),
            icon_index: WHITE_KING_INFO.sprite_index + SPRITESHEET_WIDTH,
        },
        Upgrade {
            weight: 1.0,
            display_name: "Queen Movement".to_string(),
            description: Text::from_sections([
                TextSection {
                    value: "Allows the player to move and attack like a queen.\n\n".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
                TextSection {
                    value: "Level 2+:".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        color: PRIMARY_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: format!(
                        " Increases queen damage by {}% per level.\n\n",
                        UNIQUE_UPGRADE_DAMAGE_MULTIPLIER * 100.0
                    ),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
                TextSection {
                    value: format!("Level {}+:", UNIQUE_ABILITY_UNLOCK_UPGRADE_NUMBER),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        color: PRIMARY_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: " WIP".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
            ]),
            cost: (WHITE_QUEEN_INFO.value as f32 * SHOP_PIECE_VALUE_GOLD_MULTIPLIER) as usize,
            rarity: Rarity::Common,
            effect: Effect::MovementType(vec![MovementType::Queen]),
            icon_index: WHITE_QUEEN_INFO.sprite_index + SPRITESHEET_WIDTH,
        },
        Upgrade {
            weight: 1.0,
            display_name: "Knight Movement".to_string(),
            description: Text::from_sections([
                TextSection {
                    value: "Allows the player to move and attack like a knight.\n\n".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
                TextSection {
                    value: "Level 2+:".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        color: PRIMARY_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: format!(
                        " Increases knight damage by {}% per level.\n\n",
                        UNIQUE_UPGRADE_DAMAGE_MULTIPLIER * 100.0
                    ),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
                TextSection {
                    value: format!("Level {}+:", UNIQUE_ABILITY_UNLOCK_UPGRADE_NUMBER),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        color: PRIMARY_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: " Knight attacks ".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
                TextSection {
                    value: "Chain".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        color: PRIMARY_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: " once.".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
            ]),
            cost: (WHITE_KNIGHT_INFO.value as f32 * SHOP_PIECE_VALUE_GOLD_MULTIPLIER) as usize,
            rarity: Rarity::Common,
            effect: Effect::MovementType(vec![MovementType::Knight]),
            icon_index: WHITE_KNIGHT_INFO.sprite_index + SPRITESHEET_WIDTH,
        },
        Upgrade {
            weight: 1.0,
            display_name: "Bishop Movement".to_string(),
            description: Text::from_sections([
                TextSection {
                    value: "Allows the player to move and attack like a bishop.\n\n".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
                TextSection {
                    value: "Level 2+:".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        color: PRIMARY_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: format!(
                        " Increases bishop damage by {}% per level.\n\n",
                        UNIQUE_UPGRADE_DAMAGE_MULTIPLIER * 100.0
                    ),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
                TextSection {
                    value: format!("Level {}+:", UNIQUE_ABILITY_UNLOCK_UPGRADE_NUMBER),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        color: PRIMARY_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: " Bishop attacks ".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
                TextSection {
                    value: "Pierce".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        color: PRIMARY_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: " enemies.".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
            ]),
            cost: (WHITE_BISHOP_INFO.value as f32 * SHOP_PIECE_VALUE_GOLD_MULTIPLIER) as usize,
            rarity: Rarity::Common,
            effect: Effect::MovementType(vec![MovementType::Bishop]),
            icon_index: WHITE_BISHOP_INFO.sprite_index + SPRITESHEET_WIDTH,
        },
        Upgrade {
            weight: 1.0,
            display_name: "Rook Movement".to_string(),
            description: Text::from_sections([
                TextSection {
                    value: "Allows the player to move and attack like a rook.\n\n".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
                TextSection {
                    value: "Level 2+:".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        color: PRIMARY_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: format!(
                        " Increases rook damage by {}% per level.\n\n",
                        UNIQUE_UPGRADE_DAMAGE_MULTIPLIER * 100.0
                    ),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
                TextSection {
                    value: format!("Level {}+:", UNIQUE_ABILITY_UNLOCK_UPGRADE_NUMBER),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        color: PRIMARY_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: " Rook attacks grant ".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
                TextSection {
                    value: "Block(1)".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        color: PRIMARY_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: ". It does not stack.".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
            ]),
            cost: (WHITE_ROOK_INFO.value as f32 * SHOP_PIECE_VALUE_GOLD_MULTIPLIER) as usize,
            rarity: Rarity::Common,
            effect: Effect::MovementType(vec![MovementType::Rook]),
            icon_index: WHITE_ROOK_INFO.sprite_index + SPRITESHEET_WIDTH,
        },
    ]
});

pub static UPGRADES_STATS: Lazy<Vec<Upgrade>> = Lazy::new(|| {
    vec![
        Upgrade {
            weight: 1.0,
            display_name: "Health +10".to_string(),
            description: Text::from_sections([
                TextSection {
                    value: "Increases maximum ".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
                TextSection {
                    value: "Health".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        color: PRIMARY_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: format!(" by {}.", 10),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
            ]),
            cost: 100,
            rarity: Rarity::Common,
            effect: Effect::StatEffect(StatEffect {
                stat: StatVariant::MaxHealth,
                additive: 10.0,
                multiplicative: 1.0,
            }),
            icon_index: WIP_SPRITE_INDEX,
        },
        Upgrade {
            weight: 0.3,
            display_name: "Health +20".to_string(),
            description: Text::from_sections([
                TextSection {
                    value: "Increases maximum ".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
                TextSection {
                    value: "Health".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        color: PRIMARY_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: " by ".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
                TextSection {
                    value: "20".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        color: PRIMARY_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: ".".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
            ]),
            cost: 150,
            icon_index: WIP_SPRITE_INDEX,
            rarity: Rarity::Rare,
            effect: Effect::StatEffect(StatEffect {
                stat: StatVariant::MaxHealth,
                additive: 20.0,
                multiplicative: 1.0,
            }),
        },
        Upgrade {
            weight: 1.0,
            display_name: "Attack +1".to_string(),
            description: Text::from_sections([
                TextSection {
                    value: "Increases ".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
                TextSection {
                    value: "Attack".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        color: PRIMARY_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: " by ".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
                TextSection {
                    value: "1".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        color: PRIMARY_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: ".".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
            ]),
            cost: 100,
            icon_index: WIP_SPRITE_INDEX,
            rarity: Rarity::Common,
            effect: Effect::StatEffect(StatEffect {
                stat: StatVariant::Attack,
                additive: 1.0,
                multiplicative: 1.0,
            }),
        },
        Upgrade {
            weight: 0.3,
            display_name: "Attack +2".to_string(),
            description: Text::from_sections([
                TextSection {
                    value: "Increases ".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
                TextSection {
                    value: "Attack".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        color: PRIMARY_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: " by ".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
                TextSection {
                    value: "2".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        color: PRIMARY_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: ".".into(),
                    style: TextStyle {
                        font_size: UI_FONT_SIZE,
                        ..default()
                    },
                },
            ]),
            cost: 150,
            icon_index: WIP_SPRITE_INDEX,
            rarity: Rarity::Rare,
            effect: Effect::StatEffect(StatEffect {
                stat: StatVariant::Attack,
                additive: 2.0,
                multiplicative: 1.0,
            }),
        },
    ]
});

impl Weighted for Upgrade {
    fn weight(&self) -> f32 {
        self.weight
    }
}

pub fn get_movement_upgrade(movement_type: &MovementType) -> Upgrade {
    debug!("Searching for movement upgrade: {:?}", movement_type);

    UPGRADES_MOVEMENT
        .iter()
        .find(|u| matches!(u.effect, Effect::MovementType(ref mts) if mts.contains(movement_type) && mts.len() == 1))
        .cloned()
        .unwrap_or_else(|| panic!("No upgrade found for movement type: {:?}", movement_type))
}
