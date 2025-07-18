use crate::*;

#[derive(Debug)]
pub(crate) struct Neuron {
    pub(crate) bias: f32,
    pub(crate) input_weights: Vec<f32>,
}
impl Neuron {
    pub(crate) fn propagate(&self, inputs: &Vec<f32>) -> f32 {
        assert_eq!(inputs.len(), self.input_weights.len());

        self.input_weights
            .iter()
            .zip(inputs)
            .fold(self.bias, |init, (&w, &i)| init + w * i)
            .max(0.0)
    }
    pub(crate) fn random(rng: &mut dyn RngCore, inputs: usize) -> Self {
        let bias = rng.random_range(-1.0..=1.0);
        let input_weights = (0..inputs).map(|_| rng.random_range(-1.0..=1.0)).collect();
        Self {
            bias,
            input_weights,
        }
    }
    pub(crate) fn from_weights(inputs: usize, weights: &mut dyn Iterator<Item = f32>) -> Self {
        let bias = weights.next().expect("Not enough weights provided!");
        let input_weights = (0..inputs)
            .map(|_| weights.next().expect("Not enough weights provided!"))
            .collect();
        Self {
            bias,
            input_weights,
        }
    }
}
