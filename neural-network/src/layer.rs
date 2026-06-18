use rand::Rng;

use super::neuron::Neuron;

#[derive(Debug)]
/// A layer of neurons in a neural network.
pub struct Layer {
    pub(crate) neurons: Vec<Neuron>, // neurons in the layer
}

impl Layer {
    /// Creates a new layer with random neurons.
    ///
    /// # Arguments
    ///
    /// * `rng` - The random number generator to use.
    /// * `input_size` - The number of inputs to each neuron.
    /// * `output_size` - The number of neurons in the layer.
    ///
    /// # Returns
    ///
    /// A new `Layer` instance with random neurons.
    pub fn random(rng: &mut dyn Rng, input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(rng, input_size))
            .collect();
        Self { neurons }
    }

    /// Propagates the inputs through the layer and returns the outputs.
    ///
    /// # Arguments
    ///
    /// * `inputs` - The inputs to propagate through the layer.
    ///
    /// # Returns
    ///
    /// The different outputs of the layer.
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }

    pub fn from_weights(
        input_size: usize,
        output_size: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::from_weights(input_size, weights))
            .collect();

        Self { neurons }
    }
}
