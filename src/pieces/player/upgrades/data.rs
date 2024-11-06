use super::stats::StatEffect;
use crate::{
    globals::{SHOP_PIECE_VALUE_GOLD_MULTIPLIER, SPRITESHEET_WIDTH, WIP_SPRITE_INDEX},
    pieces::{
        enemies::{
            bishop::WHITE_BISHOP_INFO, king::WHITE_KING_INFO, knight::WHITE_KNIGHT_INFO,
            pawn::WHITE_PAWN_INFO, queen::WHITE_QUEEN_INFO, rook::WHITE_ROOK_INFO,
        },
        movement_type::MovementType,
        player::upgrades::stats::Stat,
    },
    utils::rng::Weighted,
};
use once_cell::sync::Lazy;

#[derive(Clone, Debug)]
pub struct Upgrade {
    weight: f32,
    pub display_name: String,
    pub description: String,
    pub cost: usize,
    pub rarity: Rarity,
    pub effect: Effect,
    pub icon_index: usize,
}

#[derive(Clone, Debug)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
}

#[derive(Clone, Debug)]
pub enum Effect {
    MovementType(MovementType),
    StatEffect(StatEffect),
}

pub static UPGRADES: Lazy<Vec<Upgrade>> = Lazy::new(|| {
    vec![
        Upgrade {
            weight: 1.0,
            display_name: "Pawn Movement".to_string(),
            description: "Allows the player to move and attack like a pawn.".to_string(),
            cost: (WHITE_PAWN_INFO.value as f32 * SHOP_PIECE_VALUE_GOLD_MULTIPLIER) as usize,
            rarity: Rarity::Common,
            effect: Effect::MovementType(MovementType::WhitePawn),
            icon_index: WHITE_PAWN_INFO.sprite_index + SPRITESHEET_WIDTH,
        },
        Upgrade {
            weight: 1.0,
            display_name: "King Movement".to_string(),
            description: "Allows the player to move and attack like a king.".to_string(),
            cost: (WHITE_KING_INFO.value as f32 * SHOP_PIECE_VALUE_GOLD_MULTIPLIER) as usize,
            rarity: Rarity::Common,
            effect: Effect::MovementType(MovementType::King),
            icon_index: WHITE_KING_INFO.sprite_index + SPRITESHEET_WIDTH,
        },
        Upgrade {
            weight: 1.0,
            display_name: "Queen Movement".to_string(),
            description: "Allows the player to move and attack like a queen.".to_string(),
            cost: (WHITE_QUEEN_INFO.value as f32 * SHOP_PIECE_VALUE_GOLD_MULTIPLIER) as usize,
            rarity: Rarity::Common,
            effect: Effect::MovementType(MovementType::Queen),
            icon_index: WHITE_QUEEN_INFO.sprite_index + SPRITESHEET_WIDTH,
        },
        Upgrade {
            weight: 1.0,
            display_name: "Knight Movement".to_string(),
            description: "Allows the player to move and attack like a knight.".to_string(),
            cost: (WHITE_KNIGHT_INFO.value as f32 * SHOP_PIECE_VALUE_GOLD_MULTIPLIER) as usize,
            rarity: Rarity::Common,
            effect: Effect::MovementType(MovementType::Knight),
            icon_index: WHITE_KNIGHT_INFO.sprite_index + SPRITESHEET_WIDTH,
        },
        Upgrade {
            weight: 1.0,
            display_name: "Bishop Movement".to_string(),
            description: "Allows the player to move and attack like a bishop.".to_string(),
            cost: (WHITE_BISHOP_INFO.value as f32 * SHOP_PIECE_VALUE_GOLD_MULTIPLIER) as usize,
            rarity: Rarity::Common,
            effect: Effect::MovementType(MovementType::Bishop),
            icon_index: WHITE_BISHOP_INFO.sprite_index + SPRITESHEET_WIDTH,
        },
        Upgrade {
            weight: 1.0,
            display_name: "Rook Movement".to_string(),
            description: "Allows the player to move and attack like a rook.".to_string(),
            cost: (WHITE_ROOK_INFO.value as f32 * SHOP_PIECE_VALUE_GOLD_MULTIPLIER) as usize,
            rarity: Rarity::Common,
            effect: Effect::MovementType(MovementType::Rook),
            icon_index: WHITE_ROOK_INFO.sprite_index + SPRITESHEET_WIDTH,
        },
        Upgrade {
            weight: 1.0,
            display_name: "Health +10".to_string(),
            description: "Increases the player's health by 10.".to_string(),
            cost: 100,
            rarity: Rarity::Common,
            effect: Effect::StatEffect(StatEffect {
                stat: Stat::Health,
                additive: 10.0,
                multiplicative: 1.0,
            }),
            icon_index: WIP_SPRITE_INDEX,
        },
        Upgrade {
            weight: 1.0,
            display_name: "Health +20".to_string(),
            description: "Increases the player's health by 20.".to_string(),
            cost: 200,
            icon_index: WIP_SPRITE_INDEX,
            rarity: Rarity::Rare,
            effect: Effect::StatEffect(StatEffect {
                stat: Stat::Health,
                additive: 20.0,
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
