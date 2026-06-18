mod animal;
mod animal_individual;
mod brain;
mod eye;
mod food;
mod world;

pub use self::animal::*;
pub use self::animal_individual::*;
pub use self::brain::*;
pub use self::eye::*;
pub use self::food::*;
pub use self::world::*;

use lib_genetic_algorithm::GaussianMutation;
use lib_genetic_algorithm::GeneticAlgorithm;
use lib_genetic_algorithm::RouletteWheelSelection;
use lib_genetic_algorithm::UniformCrossover;
use nalgebra::{self as na};
use rand::{Rng, RngExt};
use std::f32::consts::FRAC_PI_2;

const SPEED_MIN: f32 = 0.001;
const SPEED_MAX: f32 = 0.005;
const SPEED_ACCEL: f32 = 0.2;
const ROTATION_ACCEL: f32 = FRAC_PI_2;
const GENERATION_LENGHT: usize = 2500;

pub struct Simulation {
    world: World,
    ga: GeneticAlgorithm<RouletteWheelSelection>,
    age: usize,
}

impl Simulation {
    pub fn random(rng: &mut dyn Rng) -> Self {
        let world = World::random(rng);
        let ga = GeneticAlgorithm::new(
            RouletteWheelSelection,
            UniformCrossover,
            GaussianMutation::new(0.01, 0.3),
        );
        Self { world, ga, age: 0 }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn Rng) {
        self.process_collisions(rng);
        self.process_brain();
        self.process_movements();

        self.age += 1;
        if self.age > GENERATION_LENGHT {
            self.evolve(rng);
        }
    }

    fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * na::Vector2::new(0.0, animal.speed);

            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn Rng) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = na::distance(&animal.position, &food.position);

                if distance <= 0.01 {
                    animal.satiation += 1;
                    food.position = rng.random();
                }
            }
        }
    }

    fn process_brain(&mut self) {
        for animal in &mut self.world.animals {
            let vision =
                animal
                    .eye
                    .process_vision(animal.position, animal.rotation, &self.world.foods);
            let response = animal.brain.neural_network.propagate(vision);

            let speed = response[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);
            let rotation = response[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);

            animal.speed = (animal.speed + speed).clamp(SPEED_MIN, SPEED_MAX);
            animal.rotation = nalgebra::Rotation2::new(animal.rotation.angle() + rotation);
        }
    }

    fn evolve(&mut self, rng: &mut dyn Rng) {
        self.age = 0;

        let current_population: Vec<AnimalIndividual> = self
            .world
            .animals
            .iter()
            .map(AnimalIndividual::from_animal)
            .collect();

        let evolved_population = self.ga.evolve(rng, &current_population);

        self.world.animals = evolved_population
            .into_iter()
            .map(|individual| individual.into_animal(rng))
            .collect();

        for food in &mut self.world.foods {
            food.position = rng.random();
        }
    }
}
