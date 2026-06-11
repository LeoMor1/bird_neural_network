mod gaussian;
pub use self::gaussian::*;

use crate::*;
use rand::Rng;

pub trait MutationMethod {
    fn mutate(&self, rng: &mut dyn Rng, child: &mut Chromosome);
}
