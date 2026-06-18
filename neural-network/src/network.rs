use std::iter::once;

use rand::Rng;

use crate::LayerToplology;

use super::layer::Layer;

#[derive(Debug)]
/// A neural network composed of layers of neurons.
pub struct Network {
    /// The layers of the network.
    pub(crate) layers: Vec<Layer>,
}

impl Network {
    /// Creates a random network with the given topology.
    ///
    /// Arguments:
    ///
    /// * `rng` - The random number generator to use.
    /// * `layers` - The topology of the layers.
    ///
    /// Returns:
    ///
    /// A new random network with the given topology.
    pub fn random(rng: &mut dyn Rng, layers: &[LayerToplology]) -> Self {
        assert!(layers.len() > 1);
        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();
        Self { layers }
    }

    /// Propagates the given inputs through the network and returns the outputs.
    ///
    /// Arguments:
    ///
    /// * `inputs` - The inputs to propagate.
    ///
    /// Returns:
    ///
    /// The outputs of the network.
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        // layers = niveaux du reseaux donc layer 1 calcule et donne sont resultat a layer 2
        // la boucle simule ce transfers de valeurs
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
        self.layers
            .iter()
            .flat_map(|layer| layer.neurons.iter())
            .flat_map(|neuron| once(&neuron.bias).chain(&neuron.weights))
            .copied()
    }

    pub fn from_weights(layer: &[LayerToplology], weights: impl IntoIterator<Item = f32>) -> Self {
        assert!(layer.len() > 1);

        let mut weights = weights.into_iter();

        let layers = layer
            .windows(2)
            .map(|layer| Layer::from_weights(layer[0].neurons, layer[1].neurons, &mut weights))
            .collect();

        if weights.next().is_none() {
            panic!("got to many weights");
        }

        Self { layers }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use crate::Neuron;

    use super::*;

    #[test]
    fn weights() {
        let network = Network {
            layers: vec![
                Layer {
                    neurons: vec![Neuron {
                        bias: 0.1,
                        weights: vec![0.2, 0.3, 0.4],
                    }],
                },
                Layer {
                    neurons: vec![Neuron {
                        bias: 0.5,
                        weights: vec![0.6, 0.7, 0.8],
                    }],
                },
            ],
        };

        let actual: Vec<f32> = network.weights().collect();
        let expected = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        assert_relative_eq!(actual.as_slice(), expected.as_slice());
    }

    #[test]
    fn from_weights() {
        let layer = &[LayerToplology { neurons: 3 }, LayerToplology { neurons: 2 }];

        let weights = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        let network = Network::from_weights(layer, weights.clone());
        let actual: Vec<f32> = network.weights().collect();

        assert_relative_eq!(actual.as_slice(), weights.as_slice());
    }
}
