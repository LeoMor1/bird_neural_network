use crate::*;
use rand::{Rng, RngExt};

pub struct GaussianMutation {
    // 0.0 : no genes modified
    // 1.0 : all genes modified
    chance: f32,

    // 0.0 : touched genes will not by modified
    // 3.0 : touched genes will be += 3.0 or -= 3.0 at most
    coeff: f32,
}

impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!((0.0..=1.0).contains(&chance));

        Self { chance, coeff }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn Rng, child: &mut Chromosome) {
        for gene in child.iter_mut() {
            let sign = if rng.random_bool(0.5) { -1.0 } else { 1.0 };

            if rng.random_bool(self.chance as f64) {
                *gene += sign * self.coeff * rng.random::<f32>();
            }
        }
    }
}

#[cfg(test)]
mod gaussian_mutation {

    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

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
