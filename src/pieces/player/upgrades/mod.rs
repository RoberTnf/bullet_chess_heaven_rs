use stats::StatEffect;

use crate::pieces::movement_type::MovementType;

pub mod stats;

pub struct Upgrade {
    weight: f32,
    display_name: String,
    description: String,
    cost: u32,
    rarity: Rarity,
    effect: Effect,
    icon_index: u32,
}

pub enum Rarity {
    Common,
    Rare,
    Epic,
}

pub enum Effect {
    MovementType(MovementType),
    StatEffect(StatEffect),
}
