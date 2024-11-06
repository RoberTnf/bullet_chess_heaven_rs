use rand::{thread_rng, Rng};

pub trait Weighted {
    fn weight(&self) -> f32;
}

pub fn sample_weighted<T: Weighted + Clone>(n: usize, items: &[T]) -> Vec<T> {
    let mut rng = thread_rng();
    let total_weight = items.iter().map(|i| i.weight()).sum::<f32>();
    let mut upgrades: Vec<T> = Vec::with_capacity(n);
    for _ in 0..n {
        let draw = rng.gen_range(0.0..total_weight);
        let mut cumulative_weight = 0.0;
        for upgrade in items.iter() {
            cumulative_weight += upgrade.weight();
            if draw < cumulative_weight {
                upgrades.push(upgrade.clone());
                break;
            }
        }
    }
    upgrades
}
