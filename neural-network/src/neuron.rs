use rand::{Rng, RngExt};

#[derive(Debug)]
/// A neuron in a neural network.
pub struct Neuron {
    pub(crate) bias: f32,         // bias of the neuron
    pub(crate) weights: Vec<f32>, // weight of each input (simulate synaptic weights)
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

    pub fn from_weights(input_size: usize, weights: &mut dyn Iterator<Item = f32>) -> Self {
        let bias = weights.next().expect("got not enough weights");

        let weights = (0..input_size)
            .map(|_| weights.next().expect("got not enough weights"))
            .collect();

        Self { bias, weights }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn random() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let neuron = Neuron::random(&mut rng, 4);

        assert_ne!(0.45f32, 0.15 + 0.15 + 0.15);
        assert_relative_eq!(0.45f32, 0.15 + 0.15 + 0.15);
        assert_relative_eq!(neuron.bias, -0.6255188);
        assert_relative_eq!(
            neuron.weights.as_slice(),
            [0.67383933, 0.81812596, 0.26284885, 0.5238805].as_ref()
        );
    }

    #[test]
    fn propagate() {
        let neuron = Neuron {
            bias: 0.5,
            weights: vec![-0.3, 0.8],
        };

        assert_relative_eq!(neuron.propagate(&[-10.0, -10.0]), 0.0);
        assert_relative_eq!(
            neuron.propagate(&[0.5, 1.0]),
            (-0.3 * 0.5) + (0.8 * 1.0) + 0.5
        )
    }
}
