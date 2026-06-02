use rand::{Rng, RngExt};
use rand::seq::{IndexedRandom, SliceRandom};

/// A genetic algorithm for evolving neural networks.
pub struct GeneticAlgorithm;

impl GeneticAlgorithm {
    /// Evolves a population of neural networks using genetic algorithms.
    pub fn evolve<I>(&self, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(population.is_empty());
        (0..population.len())
            .map(|_| {

                todo!()
            })
            .collect()
    }
}
