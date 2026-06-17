use rand::Rng;

use super::layer::Layer;

#[derive(Debug)]
/// A neural network composed of layers of neurons.
pub struct Network {
    /// The layers of the network.
    layers: Vec<Layer>,
}

#[derive(Debug)]
/// The topology of a layer, i.e. the number of neurons in it.
pub struct LayerToplology {
    /// The number of neurons in the layer.
    pub neurons: usize,
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
}
