use nalgebra as na;

#[derive(Debug)]
pub struct Animal {
    position: na::Point2<f32>,
    rotation: na::Rotation2<f32>,
    speed: f32,
}

#[derive(Debug)]
pub struct Food {
    position: na::Point2<f32>,
}
