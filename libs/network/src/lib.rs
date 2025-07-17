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
