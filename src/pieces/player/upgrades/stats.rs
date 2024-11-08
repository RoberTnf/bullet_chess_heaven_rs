#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Stat {
    Health,
    Attack,
}

#[derive(Clone, Debug)]
pub struct StatEffect {
    pub stat: Stat,
    pub additive: f32,
    pub multiplicative: f32,
}

impl Default for StatEffect {
    fn default() -> Self {
        Self {
            stat: Stat::Health,
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
