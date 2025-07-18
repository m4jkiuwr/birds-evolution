use lib_simulation as sim;
use rand::{self, rngs::ThreadRng};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(animal_num: usize, food_num: usize) -> Self {
        let mut rng = rand::rng();
        let sim = sim::Simulation::random(&mut rng, animal_num, food_num);
        Self { rng, sim }
    }

    pub fn world(&self) -> World {
        World::from(self.sim.world())
    }

    pub fn step(&mut self) {
        self.sim.step(&mut self.rng);
    }

    pub fn train(&mut self) {
        self.sim.train(&mut self.rng);
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Copy)]
pub struct Animal {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
}
#[wasm_bindgen]
#[derive(Clone, Debug, Copy)]
pub struct Food {
    pub x: f32,
    pub y: f32,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct World {
    #[wasm_bindgen(getter_with_clone)]
    pub animals: Vec<Animal>,
    #[wasm_bindgen(getter_with_clone)]
    pub foods: Vec<Food>,
}

impl From<&sim::Animal> for Animal {
    fn from(value: &sim::Animal) -> Self {
        Self {
            x: value.position().x,
            y: value.position().y,
            rotation: value.rotation().angle(),
        }
    }
}
impl From<&sim::Food> for Food {
    fn from(value: &sim::Food) -> Self {
        Self {
            x: value.position().x,
            y: value.position().y,
        }
    }
}

impl From<&sim::World> for World {
    fn from(value: &sim::World) -> Self {
        let animals = value.animals().iter().map(Animal::from).collect();
        let foods = value.foods().iter().map(Food::from).collect();
        Self { animals, foods }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn random() {
        let mut sim = super::Simulation::new(1, 1000);
        for _ in 0..=10 {
            for (ind, animal) in &mut sim.world().animals.iter().enumerate() {
                println!(
                    "num: {} coords : ({} | {}) | rotation: {}",
                    ind, animal.x, animal.y, animal.rotation
                );
            }
            sim.step();
        }
    }
}
