mod animal;
mod eye;
mod food;
mod world;

pub use self::animal::*;
pub use self::eye::*;
pub use self::food::*;
pub use self::world::*;

use nalgebra::{self as na};
use rand::{Rng, RngExt};

pub struct Simulation {
    world: World,
}

impl Simulation {
    pub fn random(rng: &mut dyn Rng) -> Self {
        Self {
            world: World::random(rng),
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn Rng) {
        self.process_movements();
        self.process_collisions(rng);
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
                    food.position = rng.random()
                }
            }
        }
    }
}
