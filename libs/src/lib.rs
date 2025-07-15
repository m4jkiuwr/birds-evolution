use rand::Rng;
use std::io::repeat;

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}
#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}
pub struct LayerTopology {
    pub neuron_count: usize,
}
#[derive(Debug)]
struct Neuron {
    bias: f32,
    input_weights: Vec<f32>,
}
impl Neuron {
    fn propagate(&self, inputs: &Vec<f32>) -> f32 {
        assert_eq!(inputs.len(), self.input_weights.len());

        self.input_weights
            .iter()
            .zip(inputs)
            .fold(self.bias, |init, (&w, &i)| init + w * i)
            .max(0.0)
    }
    fn random(inputs: usize) -> Self {
        let mut rng = rand::rng();
        let bias = rng.random_range(-1.0..=1.0);
        let input_weights = (0..inputs).map(|_| rng.random_range(-1.0..=1.0)).collect();
        Self {
            bias,
            input_weights,
        }
    }
}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }

    fn random(inputs: usize, outputs: usize) -> Self {
        let neurons = (0..outputs).map(|_| Neuron::random(inputs)).collect();
        Self { neurons }
    }
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
