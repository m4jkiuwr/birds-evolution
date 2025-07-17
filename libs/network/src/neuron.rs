use crate::*;

#[derive(Debug)]
pub(crate) struct Neuron {
    bias: f32,
    input_weights: Vec<f32>,
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
    pub(crate) fn random(inputs: usize) -> Self {
        let mut rng = rand::rng();
        let bias = rng.random_range(-1.0..=1.0);
        let input_weights = (0..inputs).map(|_| rng.random_range(-1.0..=1.0)).collect();
        Self {
            bias,
            input_weights,
        }
    }
}
