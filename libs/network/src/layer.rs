use crate::*;

#[derive(Debug)]
pub(crate) struct Layer {
    pub(crate) neurons: Vec<Neuron>,
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

    pub(crate) fn from_weights(
        inputs: usize,
        outputs: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        let neurons: Vec<Neuron> = (0..outputs)
            .map(|_| Neuron::from_weights(inputs, weights))
            .collect();
        Self { neurons }
    }
}
