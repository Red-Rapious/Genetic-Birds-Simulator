#[cfg(test)]
use crate::*;

#[cfg(test)]
mod tests {
    use super::*;

    mod random {
        use super::*;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        #[test]
        fn test() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let neuron = Neuron::random(&mut rng, 4);

            approx::assert_relative_eq!(
                neuron.bias, 
                -0.6255188
            );
            approx::assert_relative_eq!(
                neuron.weights
                    .as_slice(), 
                [0.67383957, 0.8181262, 0.26284897, 0.5238807]
                    .as_ref()
            );
        }
    }

    mod propagate {
        use super::*;

        #[test]
        fn test() {
            let neuron = Neuron {
                bias: 0.5,
                weights: vec![-0.3, 0.8]
            };

            // Makes sure that ReLU works
            approx::assert_relative_eq!(
                neuron.propagate(&[-10.0, -10.0]), 
                0.0
            );

            approx::assert_relative_eq!(
                neuron.propagate(&[0.5, 1.0]),
                (-0.3 * 0.5) + (0.8 * 1.0) + 0.5
            );
        }
    }

    mod weights {
        use super::*;

        #[test]
        fn test() {
            let network = Network::new(vec![
                Layer::new(vec![Neuron::new(0.1, vec![0.2, 0.3, 0.4])]),
                Layer::new(vec![Neuron::new(0.5, vec![0.6, 0.7, 0.8])])
            ]);

            let actual = network.weights();
            let expected = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

            approx::assert_relative_eq!(
                actual.as_slice(),
                expected.as_slice()
            )
        }
    }

    mod from_weights {
        use super::*;

        #[test]
        fn test() {
            let layers = &[
                LayerTopology { neurons: 3 },
                LayerTopology { neurons: 2 },
            ];

            let weights = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

            let network = Network::from_weights(layers, weights.clone());
            let actual: Vec<_> = network.weights().into_iter().collect();

            approx::assert_relative_eq!(
                actual.as_slice(),
                weights.as_slice()
            );
        }
    }
}