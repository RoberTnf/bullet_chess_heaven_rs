use super::stats::StatEffect;
use crate::{
    globals::{SHOP_PIECE_VALUE_GOLD_MULTIPLIER, SPRITESHEET_WIDTH, WIP_SPRITE_INDEX},
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
use bevy::{prelude::*, utils::HashSet};
use once_cell::sync::Lazy;

#[derive(Clone, Debug, PartialEq, Component)]
pub struct Upgrade {
    weight: f32,
    pub display_name: String,
    pub description: String,
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
            description: "Allows the player to move and attack like a pawn.".to_string(),
            cost: (WHITE_PAWN_INFO.value as f32 * SHOP_PIECE_VALUE_GOLD_MULTIPLIER) as usize,
            rarity: Rarity::Common,
            effect: Effect::MovementType(vec![MovementType::WhitePawn]),
            icon_index: WHITE_PAWN_INFO.sprite_index + SPRITESHEET_WIDTH,
        },
        Upgrade {
            weight: 0.0,
            display_name: "White Pawn Movement".to_string(),
            description: "Allows the player to move and attack like a pawn.".to_string(),
            cost: (BLACK_PAWN_INFO.value as f32 * SHOP_PIECE_VALUE_GOLD_MULTIPLIER) as usize,
            rarity: Rarity::Common,
            effect: Effect::MovementType(vec![MovementType::BlackPawn]),
            icon_index: BLACK_PAWN_INFO.sprite_index + SPRITESHEET_WIDTH,
        },
        Upgrade {
            weight: 1.0,
            display_name: "Pawn Movement".to_string(),
            description: "Allows the player to move and attack like a pawn.".to_string(),
            cost: (WHITE_PAWN_INFO.value as f32 * SHOP_PIECE_VALUE_GOLD_MULTIPLIER) as usize,
            rarity: Rarity::Common,
            effect: Effect::MovementType(vec![MovementType::WhitePawn, MovementType::BlackPawn]),
            icon_index: WHITE_PAWN_INFO.sprite_index + SPRITESHEET_WIDTH,
        },
        Upgrade {
            weight: 1.0,
            display_name: "King Movement".to_string(),
            description: "Allows the player to move and attack like a king.".to_string(),
            cost: (WHITE_KING_INFO.value as f32 * SHOP_PIECE_VALUE_GOLD_MULTIPLIER) as usize,
            rarity: Rarity::Common,
            effect: Effect::MovementType(vec![MovementType::King]),
            icon_index: WHITE_KING_INFO.sprite_index + SPRITESHEET_WIDTH,
        },
        Upgrade {
            weight: 1.0,
            display_name: "Queen Movement".to_string(),
            description: "Allows the player to move and attack like a queen.".to_string(),
            cost: (WHITE_QUEEN_INFO.value as f32 * SHOP_PIECE_VALUE_GOLD_MULTIPLIER) as usize,
            rarity: Rarity::Common,
            effect: Effect::MovementType(vec![MovementType::Queen]),
            icon_index: WHITE_QUEEN_INFO.sprite_index + SPRITESHEET_WIDTH,
        },
        Upgrade {
            weight: 1.0,
            display_name: "Knight Movement".to_string(),
            description: "Allows the player to move and attack like a knight.".to_string(),
            cost: (WHITE_KNIGHT_INFO.value as f32 * SHOP_PIECE_VALUE_GOLD_MULTIPLIER) as usize,
            rarity: Rarity::Common,
            effect: Effect::MovementType(vec![MovementType::Knight]),
            icon_index: WHITE_KNIGHT_INFO.sprite_index + SPRITESHEET_WIDTH,
        },
        Upgrade {
            weight: 1.0,
            display_name: "Bishop Movement".to_string(),
            description: "Allows the player to move and attack like a bishop.".to_string(),
            cost: (WHITE_BISHOP_INFO.value as f32 * SHOP_PIECE_VALUE_GOLD_MULTIPLIER) as usize,
            rarity: Rarity::Common,
            effect: Effect::MovementType(vec![MovementType::Bishop]),
            icon_index: WHITE_BISHOP_INFO.sprite_index + SPRITESHEET_WIDTH,
        },
        Upgrade {
            weight: 1.0,
            display_name: "Rook Movement".to_string(),
            description: "Allows the player to move and attack like a rook.".to_string(),
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
            description: "Increases the player's health by 10.".to_string(),
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
            description: "Increases the player's health by 20.".to_string(),
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
            description: "Increases the player's attack by 1.".to_string(),
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
            description: "Increases the player's attack by 2.".to_string(),
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
