use lib_genetic_algorithm::Chromosome;
use lib_neural_network::{LayerToplology, Network};
use rand::Rng;

use crate::Eye;

#[derive(Debug)]
pub struct Brain {
    pub(crate) neural_network: Network,
}

impl Brain {
    fn topology(eye: &Eye) -> [LayerToplology; 3] {
        [
            LayerToplology {
                neurons: eye.cells(),
            },
            LayerToplology {
                neurons: 2 * eye.cells(),
            },
            LayerToplology { neurons: 2 },
        ]
    }

    pub fn random(rng: &mut dyn Rng, eye: &Eye) -> Self {
        Self {
            neural_network: Network::random(rng, &Self::topology(eye)),
        }
    }

    pub(crate) fn as_chromosome(&self) -> Chromosome {
        self.neural_network.weights().collect()
    }

    pub(crate) fn from_chromosome(chromosome: Chromosome, eye: &Eye) -> Self {
        Self {
            neural_network: Network::from_weights(&Self::topology(eye), chromosome),
        }
    }
}
