mod animal;
mod eye;
mod food;
mod world;

use lib_genetic_algorithm as ga;
use lib_neural_network as nn;
use nalgebra as na;
use rand::{Rng, RngCore};
use std::f32::consts::{FRAC_PI_2, PI};

pub use self::{animal::*, eye::*, food::*, world::*};

const EAT_RADIUS: f32 = 0.01;

const SPEED_MIN: f32 = 0.001;
const SPEED_MAX: f32 = 0.007;

const SPEED_ACCEL: f32 = 0.01;
const ROTATION_ACCEL: f32 = FRAC_PI_2;

const GENERATION_AGE: usize = 2500;

pub struct Simulation {
    world: World,
    ga: ga::GeneticAlgorithm<ga::roulette::RouletteSelection>,
    age: usize,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore, animal_num: usize, food_num: usize) -> Self {
        Self {
            world: World::random(rng, animal_num, food_num),
        }
    }
}

impl Simulation {
    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.collisions(rng);
        self.movement();
        self.think();
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
    fn think(&mut self) {
        for animal in &mut self.world.animals {
            let vision =
                animal
                    .eye
                    .process_vision(animal.position, animal.rotation, &self.world.foods);
            let response = animal.brain.propagate(vision);

            let speed = response[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);
            let rotation = response[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);

            animal.speed = (animal.speed + speed).clamp(SPEED_MIN, SPEED_MAX);
            animal.rotation = na::Rotation2::new(animal.rotation.angle() + rotation);
        }
    }
}
