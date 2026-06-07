use rand::{Rng, RngExt};

use crate::chromosome::Chromosome;

pub trait MutationMethod {
    fn mutate(&self, rng: &mut dyn Rng, child: &mut Chromosome);
}

pub struct GaussianMutation {
    // 0.0 : no genes modified
    // 1.0 : all genes modified
    chance: f32,

    // 0.0 : touched genes will not by modified
    // 3.0 : touched genes will be += 3.0 or -= 3.0 at most
    coeff: f32,
}

impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!((0.0..=1.0).contains(&chance));

        Self { chance, coeff }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn Rng, child: &mut Chromosome) {
        for gene in child.iter_mut() {
            let sign = if rng.random_bool(0.5) { -1.0 } else { 1.0 };

            if rng.random_bool(self.chance as f64) {
                *gene += sign * self.coeff * rng.random::<f32>();
            }
        }
    }
}
