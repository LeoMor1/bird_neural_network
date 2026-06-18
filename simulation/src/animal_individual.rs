use lib_genetic_algorithm::{Chromosome, Individual};
use rand::Rng;

use crate::Animal;

pub struct AnimalIndividual {
    fitness: f32,
    chromosome: Chromosome,
}

impl Individual for AnimalIndividual {
    fn create(chromosome: lib_genetic_algorithm::Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
        }
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }

    fn chromosome(&self) -> &lib_genetic_algorithm::Chromosome {
        &self.chromosome
    }
}

impl AnimalIndividual {
    pub fn from_animal(animal: &Animal) -> Self {
        Self {
            fitness: animal.satiation as f32,
            chromosome: animal.as_chromosome(),
        }
    }

    pub fn into_animal(self, rng: &mut dyn Rng) -> Animal {
        Animal::from_chromosome(self.chromosome, rng)
    }
}
