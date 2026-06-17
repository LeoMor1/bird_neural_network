use lib_neural_network::network::{LayerToplology, Network};
use nalgebra as na;
use rand::{Rng, RngExt};

use crate::Eye;

#[derive(Debug)]
pub struct Animal {
    pub(crate) position: na::Point2<f32>,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) speed: f32,
    pub(crate) eye: Eye,
    pub(crate) brain: Network,
}

impl Animal {
    pub fn random(rng: &mut dyn Rng) -> Self {
        let eye = Eye::default();
        let brain = Network::random(
            rng,
            &[
                LayerToplology {
                    neurons: eye.cells(),
                },
                LayerToplology {
                    neurons: 2 * eye.cells(),
                },
                LayerToplology { neurons: 2 },
            ],
        );
        Self {
            position: rng.random(),
            rotation: rng.random(),
            speed: 0.002,
            eye,
            brain,
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }
}
