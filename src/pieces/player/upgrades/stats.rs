pub enum Stat {
    Health,
    Attack,
}

pub struct StatEffect {
    stat: Stat,
    additive: f64,
    multiplicative: f64,
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
