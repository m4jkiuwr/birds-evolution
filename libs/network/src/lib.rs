mod layer;
mod neuron;

use crate::{layer::*, neuron::*};

use rand::{Rng, RngCore};

pub struct LayerTopology {
    pub neuron_count: usize,
}

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |input, layer| layer.propagate(input))
    }

    pub fn random(rng: &mut dyn RngCore, topology: &[LayerTopology]) -> Self {
        assert!(topology.len() > 1);
        Self {
            layers: topology
                .windows(2)
                .map(|layers| Layer::random(rng, layers[0].neuron_count, layers[1].neuron_count))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn neuron_random() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let neuron = Neuron::random(&mut rng, 4);

        assert_relative_eq!(neuron.bias, -0.6255188);
        assert_relative_eq!(
            neuron.input_weights.as_slice(),
            &[0.67383933, 0.81812596, 0.26284885, 0.5238805].as_ref()
        );
    }
    #[test]
    fn neuron_propagate() {
        let neuron = Neuron {
            bias: 0.5,
            input_weights: vec![-0.3, 0.8],
        };
        assert_relative_eq!(neuron.propagate(&vec![-10.0, -10.0]), 0.0);

        assert_relative_eq!(
            neuron.propagate(&vec![0.5, 1.0]),
            (0.5 * -0.3) + (1.0 * 0.8) + 0.5
        )
    }
}
