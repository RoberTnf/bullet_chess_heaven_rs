use super::data::{Effect, Upgrades};

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub enum StatVariant {
    #[default]
    MaxHealth,
    Attack,
}

#[derive(Default, Debug)]
pub struct Stat {
    pub base_value: f32,
    pub stat_variant: StatVariant,
    pub upgraded_value: f32,
}

impl Stat {
    pub fn apply_upgrades(&mut self, upgrades: &Upgrades) {
        self.upgraded_value = self.base_value;
        // first apply additive upgrades
        for upgrade in &upgrades.0 {
            if let Effect::StatEffect(stat_effect) = &upgrade.effect {
                if stat_effect.stat == self.stat_variant {
                    self.upgraded_value += stat_effect.additive;
                }
            }
        }
        // then apply multiplicative upgrades
        for upgrade in &upgrades.0 {
            if let Effect::StatEffect(stat_effect) = &upgrade.effect {
                if stat_effect.stat == self.stat_variant {
                    self.upgraded_value *= stat_effect.multiplicative;
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct StatEffect {
    pub stat: StatVariant,
    pub additive: f32,
    pub multiplicative: f32,
}

impl Default for StatEffect {
    fn default() -> Self {
        Self {
            stat: StatVariant::MaxHealth,
            additive: 0.0,
            multiplicative: 1.0,
        }
    }
}

impl PartialEq for StatEffect {
    fn eq(&self, other: &Self) -> bool {
        self.stat == other.stat
            && self.additive == other.additive
            && self.multiplicative == other.multiplicative
    }
}
