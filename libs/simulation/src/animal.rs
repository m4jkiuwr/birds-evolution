use lib_genetic_algorithm::Chromosome;

use crate::*;

#[derive(Debug)]
pub struct Animal {
    pub(crate) position: na::Point2<f32>,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) speed: f32,
    pub(crate) food_counter: usize,
    pub(crate) eye: Eye,
    pub(crate) brain: Brain,
}
impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::random(rng, &eye);
        Self {
            position: na::Point2::new(rng.random(), rng.random()),
            rotation: na::Rotation2::new(rng.random::<f32>() * 4.0 * PI),
            speed: 0.002,
            food_counter: 0,
            eye,
            brain,
        }
    }
    fn new(rng: &mut dyn RngCore, eye: Eye, brain: Brain) -> Self {
        Self {
            position: na::Point2::new(rng.random(), rng.random()),
            rotation: na::Rotation2::new(rng.random::<f32>() * 4.0 * PI),
            speed: 0.002,
            food_counter: 0,
            eye,
            brain,
        }
    }
    pub fn from_chromosome(rng: &mut dyn RngCore, chromosome: Chromosome) -> Self {
        let eye = Eye::default();
        let brain = Brain::from_chromosome(&eye, chromosome);
        Self::new(rng, eye, brain)
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }
    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }
    pub fn as_chromosome(&self) -> ga::Chromosome {
        self.brain.as_chromosome()
    }
    pub fn food_counter(&self) -> usize {
        self.food_counter
    }
}
