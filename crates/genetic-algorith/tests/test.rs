#[cfg(test)]
mod genetic_algorith {
    use genetic_algorith::chromosome::Chromosome;
    use genetic_algorith::genetic_algorithm::Individual;
    use genetic_algorith::selection_method::{RouletteWheelSelection, SelectionMethod};
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use std::collections::BTreeMap;
    use std::iter::FromIterator;

    #[test]
    fn roulette_wheel_selection() {
        struct TestIndividual {
            fitness: f32,
        }

        impl TestIndividual {
            fn new(fitness: f32) -> Self {
                Self { fitness }
            }
        }

        impl Individual for TestIndividual {
            fn fitness(&self) -> f32 {
                self.fitness
            }

            fn chromosome(&self) -> &Chromosome {
                panic!("not supported for TestIndividual")
            }
        }

        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let population = vec![
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(4.0),
            TestIndividual::new(3.0),
        ];

        let mut actual_hist = BTreeMap::new();

        for _ in 0..1000 {
            let fitness = RouletteWheelSelection
                .select(&mut rng, &population)
                .fitness() as i32;

            *actual_hist.entry(fitness).or_insert(0) += 1;
        }
        let expected_hist = BTreeMap::from_iter([(1, 98), (2, 202), (3, 278), (4, 422)]);

        assert_eq!(actual_hist, expected_hist)
    }
}
