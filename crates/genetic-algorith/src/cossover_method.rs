use rand::{Rng, RngExt};

use crate::chromosome::Chromosome;

pub trait CrossoverMethod {
    fn crossover(
        &self,
        rng: &mut dyn Rng,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome;
}

#[derive(Clone, Debug)]
pub struct UniformCrossover;

impl CrossoverMethod for UniformCrossover {
    fn crossover(
        &self,
        rng: &mut dyn Rng,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome {
        assert_eq!(parent_a.len(), parent_b.len());
        parent_a
            .iter()
            .zip(parent_b.iter())
            .map(|(&a, &b)| if rng.random_bool(0.5) { a } else { b })
            .collect()
    }
}
