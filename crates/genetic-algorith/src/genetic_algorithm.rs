use crate::chromosome::Chromosome;
use rand::Rng;

use super::selection_method::SelectionMethod;

/// A genetic algorithm for evolving neural networks.
pub struct GeneticAlgorithm<S> {
    selection_method: S,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    /// Evolves a population of neural networks using genetic algorithms.
    pub fn new(selection_method: S) -> Self {
        Self { selection_method }
    }

    pub fn evolve<I>(&self, rng: &mut dyn Rng, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());
        (0..population.len())
            .map(|_| {
                let parent_a = self.selection_method.select(rng, population).chromosome();
                let parent_b = self.selection_method.select(rng, population).chromosome();

                todo!()
            })
            .collect()
    }
}

pub trait Individual {
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
}
