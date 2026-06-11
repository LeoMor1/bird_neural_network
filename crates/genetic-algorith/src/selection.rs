mod roulette_wheel;
pub use self::roulette_wheel::*;

use crate::*;
use rand::Rng;

pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn Rng, population: &'a [I]) -> &'a I
    where
        I: Individual;
}
