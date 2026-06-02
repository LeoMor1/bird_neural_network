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

pub trait Individual {
    fn fitness(&self) -> f32;
}

pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn Rng ,population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(rng, |indidual| indidual.fitness())
            .expect("got empty populqtion")
    }
}
