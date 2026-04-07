use rand::{Rng, RngExt};

#[derive(Debug)]
pub struct Neuron {
    pub bias: f32,
    pub weights: Vec<f32>,
}

impl Neuron {
    pub fn random(rng: &mut dyn Rng, input_size: usize) -> Self {
        let bias = rng.random_range(-1.0..=1.0);

        let weights = (0..input_size)
            .map(|_| rng.random_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }

    pub fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());
        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (self.bias + output).max(0.0)
    }
}
