use crate::*;

#[derive(Debug)]
pub struct Food {
    pub(crate) position: na::Point2<f32>,
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
