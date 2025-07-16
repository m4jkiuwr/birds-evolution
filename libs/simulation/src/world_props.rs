use nalgebra as na;
use rand::{Rng, RngCore};

#[derive(Debug)]
pub struct Animal {
    position: na::Point2<f32>,
    rotation: na::Rotation2<f32>,
    speed: f32,
}
impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: na::Point2::new(rng.random(), rng.random()),
            rotation: na::Rotation2::new(rng.random()),
            speed: 0.002,
        }
    }
    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }
}

#[derive(Debug)]
pub struct Food {
    position: na::Point2<f32>,
}
impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: na::Point2::new(rng.random(), rng.random()),
        }
    }
    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }
}
