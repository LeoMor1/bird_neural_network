#[cfg(test)]
mod genetic_algorith {
    use genetic_algorith::chromosome::Chromosome;
    use genetic_algorith::cossover_method::{CrossoverMethod, UniformCrossover};
    use genetic_algorith::individual::Individual;
    use genetic_algorith::selection_method::{RouletteWheelSelection, SelectionMethod};
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use std::collections::BTreeMap;
    use std::iter::FromIterator;

    #[test]
    fn roulette_wheel_selection() {
        #[derive(Clone, Debug, PartialEq)]
        enum TestIndividual {
            WithChromosome { chromosome: Chromosome },
            WithFitness { fitness: f32 },
        }

        impl TestIndividual {
            fn new(fitness: f32) -> Self {
                Self::WithFitness { fitness }
            }
        }

        impl Individual for TestIndividual {
            fn fitness(&self) -> f32 {
                match self {
                    Self::WithChromosome { chromosome } => chromosome.iter().sum(),

                    Self::WithFitness { fitness } => *fitness,
                }
            }

            fn chromosome(&self) -> &Chromosome {
                match self {
                    Self::WithChromosome { chromosome } => chromosome,

                    Self::WithFitness { .. } => {
                        panic!("Not implemented for TestIndividual::WithFitness")
                    }
                }
            }

            fn create(chromosome: Chromosome) -> Self {
                Self::WithChromosome { chromosome }
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

    #[test]
    fn unifrom_crossover() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let parent_a: Chromosome = (0..=100).map(|n| n as f32).collect();
        let parent_b: Chromosome = (0..=100).map(|n| -n as f32).collect();
        let child = UniformCrossover.crossover(&mut rng, &parent_a, &parent_b);

        let diff_a = child.iter().zip(parent_a).filter(|(c, p)| *c != p).count();
        let diff_b = child.iter().zip(parent_b).filter(|(c, p)| *c != p).count();

        assert_eq!(diff_a, 49);
        assert_eq!(diff_b, 51);
    }

    mod gaussian_mutation {
        use genetic_algorith::mutation_method::{GaussianMutation, MutationMethod};

        use super::*;

        fn actual(chance: f32, coeff: f32) -> Vec<f32> {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let mut child = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter().collect();

            GaussianMutation::new(chance, coeff).mutate(&mut rng, &mut child);

            child.into_iter().collect()
        }

        mod given_zero_chance {
            use approx::assert_relative_eq;

            fn actual(coeff: f32) -> Vec<f32> {
                super::actual(0.0, coeff)
            }
            mod and_zero_coeffician {
                use super::*;

                #[test]
                fn does_not_change_the_original_chromosome() {
                    let actual = actual(0.0);
                    let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }

            mod and_nonzero_coeffician {
                use super::*;

                #[test]
                fn does_not_change_the_original_chromosome() {
                    let actual = actual(0.5);
                    let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }
        }

        mod given_fifty_fifty_chance {
            use approx::assert_relative_eq;

            fn actual(coeff: f32) -> Vec<f32> {
                super::actual(0.5, coeff)
            }
            mod and_zero_coeffician {
                use super::*;

                #[test]
                fn does_not_change_the_original_chromosome() {
                    let actual = actual(0.0);
                    let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }

            mod and_nonzero_coeffician {
                use super::*;

                #[test]
                fn slightly_change_the_original_chromosome() {
                    let actual = actual(0.5);
                    let expected = vec![1.0, 1.7756249, 3.0, 4.1596804, 5.0];

                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }
        }

        mod given_max_chance {
            use approx::assert_relative_eq;

            fn actual(coeff: f32) -> Vec<f32> {
                super::actual(1.0, coeff)
            }
            mod and_zero_coeffician {
                use super::*;

                #[test]
                fn does_not_change_the_original_chromosome() {
                    let actual = actual(0.0);
                    let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }

            mod and_nonzero_coeffician {
                use super::*;
                #[test]
                fn entierly_change_the_original_chromosome() {
                    let actual = actual(0.5);
                    let expected = vec![1.4545316, 2.1162078, 2.7756248, 3.9505124, 4.638691];

                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }
        }
    }
}
