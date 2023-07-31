use crate::*;

#[derive(Debug)]
pub struct Brain {
    pub(crate) neural_network: nn::Network
}

impl Brain {
    pub fn random(eye: &Eye) -> Self {
        Self {
            neural_network: nn::Network::random(&Self::topology(eye))
        }
    }

    pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
        self.neural_network.weights().into_iter().collect()
    }

    pub(crate) fn from_chromosome(
        chromosome: ga::Chromosome,
        eye: &Eye
    ) -> Self {
        Self {
            neural_network: nn::Network::from_weights(
                &Self::topology(eye), 
                chromosome
            )
        }
    }

    fn topology(eye: &Eye) -> [nn::LayerTopology; 3] {
        [
            // Input layer
            nn::LayerTopology {
                neurons: eye.cells()
            },
            // Intermediate layer
            nn::LayerTopology {
                neurons: 2 * eye.cells()
            },
            // Output layer
            nn::LayerTopology {
                neurons: 2
            }
        ]
    }
}