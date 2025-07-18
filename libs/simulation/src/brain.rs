use crate::*;
use ga::Chromosome;
use nn::LayerTopology;

#[derive(Debug)]
pub struct Brain {
    pub(crate) network: nn::Network,
}

impl Brain {
    pub fn random(rng: &mut dyn RngCore, eye: &Eye) -> Self {
        let topology = Brain::topology(eye);
        let network: nn::Network = nn::Network::random(rng, &topology);
        Self { network }
    }

    pub(crate) fn as_chromosome(&self) -> Chromosome {
        Chromosome::new(self.network.weights().collect())
    }
    pub(crate) fn from_chromosome(eye: &Eye, chromosome: Chromosome) -> Self {
        let network = nn::Network::from_weights(&Self::topology(eye), chromosome);
        Self { network }
    }

    fn topology(eye: &Eye) -> [LayerTopology; 3] {
        [
            // our input - one from each eye-cell
            nn::LayerTopology {
                neuron_count: eye.cells(),
            },
            // Just more than first layer *instinct*
            nn::LayerTopology {
                neuron_count: 2 * eye.cells(),
            },
            // 2 Cause we output rotation and speed
            nn::LayerTopology { neuron_count: 2 },
        ]
    }
}
