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

    pub(crate) fn random(rng: &mut dyn RngCore, inputs: usize, outputs: usize) -> Self {
        let neurons = (0..outputs).map(|_| Neuron::random(rng, inputs)).collect();
        Self { neurons }
    }
}
