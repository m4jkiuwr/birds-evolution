use crate::*;

#[derive(Debug)]
pub(crate) struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    pub(crate) fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }

    pub(crate) fn random(inputs: usize, outputs: usize) -> Self {
        let neurons = (0..outputs).map(|_| Neuron::random(inputs)).collect();
        Self { neurons }
    }
}
