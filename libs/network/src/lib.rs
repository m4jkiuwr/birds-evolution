mod layer;
mod layer_topology;
mod neuron;

use crate::{layer::*, layer_topology::*, neuron::*};

use rand::Rng;

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

    pub fn random(topology: &[LayerTopology]) -> Self {
        assert!(topology.len() > 1);
        Self {
            layers: topology
                .windows(2)
                .map(|layers| Layer::random(layers[0].neuron_count, layers[1].neuron_count))
                .collect(),
        }
    }
}
