use rand::Rng;

use crate::chromosome::Chromosome;
use crate::traits::MutationMethod;

pub struct GaussianMutation {
    // 0.0 - no genes modified, 1.0 - all genes modified
    chance: f32,
    // x - modified genes will be changed at atmost x, (+ or -)
    coeff: f32,
}

impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!(chance >= 0.0 && chance <= 1.0);

        Self { chance, coeff }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn rand::RngCore, child: &mut Chromosome) {
        for gene in child.iter_mut() {
            if rng.random_bool(self.chance as f64) {
                *gene += rng.random_range(-self.coeff..=self.coeff);
            }
        }
    }
}
