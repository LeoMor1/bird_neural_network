#[cfg(test)]
mod tests {
    use neural_network::neuron::Neuron;
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
        assert_relative_eq!(neuron.weights.as_slice(), [0.67383933, 0.81812596, 0.26284885, 0.5238805].as_ref());
    }

    #[test]
    fn propagate() {
        let neuron = Neuron {
            bias: 0.5,
            weights: vec![-0.3, 0.8],
        };

        assert_relative_eq!(
            neuron.propagate(&[-10.0, -10.0]),
            0.0
        );
        assert_relative_eq!(
            neuron.propagate(&[0.5, 1.0]),
            (-0.3 * 0.5) + (0.8 * 1.0) + 0.5
        )
    }
}
