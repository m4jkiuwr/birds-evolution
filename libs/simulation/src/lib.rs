mod world_props;
use rand::{Rng, RngCore};
use world_props::*;

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
}
