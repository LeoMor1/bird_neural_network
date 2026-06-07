use rand::{Rng, RngExt};

#[derive(Debug)]
/// A neuron in a neural network.
pub struct Neuron {
    pub bias: f32,         // bias of the neuron
    pub weights: Vec<f32>, // weight of each input (simulate synaptic weights)
}

impl Neuron {
    /// Creates a new neuron with random weights and bias.
    ///
    /// # Arguments
    ///
    /// * `rng` - The random number generator to use.
    /// * `input_size` - The number of input features.
    ///
    /// # Returns
    ///
    /// A new `Neuron` instance with random weights and bias.
    pub fn random(rng: &mut dyn Rng, input_size: usize) -> Self {
        let bias = rng.random_range(-1.0..=1.0); // bias of the neuron

        let weights = (0..input_size) // weight of each input
            .map(|_| rng.random_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }

    /// Propagates the inputs through the neuron and returns the output.
    ///
    /// # Arguments
    ///
    /// * `inputs` - The input features.
    ///
    /// # Returns
    ///
    /// The output of the neuron.
    pub fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());
        // For each input, multiply by the corresponding weight and sum them up
        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (self.bias + output).max(0.0) // Add bias and apply ReLU activation
    }
}
