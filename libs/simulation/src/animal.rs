use crate::*;

#[derive(Debug)]
pub struct Animal {
    pub(crate) position: na::Point2<f32>,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) speed: f32,
    pub(crate) eye: Eye,
    pub(crate) brain: nn::Network,
}
impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let brain = todo!();
        Self {
            position: na::Point2::new(rng.random(), rng.random()),
            rotation: na::Rotation2::new(rng.random::<f32>() * 4.0 * PI),
            speed: 0.002,
            eye: Eye::default(),
            brain,
        }
    }
    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }
    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }
}
