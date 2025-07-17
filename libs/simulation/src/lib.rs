use nalgebra as na;
use rand::{Rng, RngCore};
use std::f32::consts::PI;

const EAT_RADIUS: f32 = 0.01;

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
            rotation: na::Rotation2::new(rng.random::<f32>() * 4.0 * PI),
            speed: 0.002,
        }
    }
    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }
    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
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

#[derive(Debug)]
pub struct World {
    animals: Vec<Animal>,
    foods: Vec<Food>,
}

impl World {
    fn random(rng: &mut dyn RngCore) -> Self {
        let animals = (0..40).map(|_| Animal::random(rng)).collect();
        let foods = (0..60).map(|_| Food::random(rng)).collect();
        Self { animals, foods }
    }
    pub fn animals(&self) -> &Vec<Animal> {
        &self.animals
    }
    pub fn foods(&self) -> &Vec<Food> {
        &self.foods
    }
}

#[derive(Debug)]
pub struct Simulation {
    world: World,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng),
        }
    }
}

impl Simulation {
    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.movement();
        self.collisions(rng);
    }

    fn movement(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * na::Vector2::new(0.0, animal.speed);
            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        }
    }
    fn collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = na::distance(&animal.position, &food.position);
                if distance <= EAT_RADIUS {
                    food.position = na::Point2::new(rng.random(), rng.random());
                }
            }
        }
    }
}
