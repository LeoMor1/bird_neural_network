mod uniform;
pub use self::uniform::*;

use crate::*;
use rand::Rng;

pub trait CrossoverMethod {
    fn crossover(
        &self,
        rng: &mut dyn Rng,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome;
}
