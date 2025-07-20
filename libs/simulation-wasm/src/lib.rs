mod animal;
mod food;
mod statistics;
mod world;

use lib_genetic_algorithm as ga;
use lib_simulation as sim;
use rand::{self, rngs::ThreadRng};
use wasm_bindgen::prelude::*;

pub use self::{animal::*, food::*, statistics::*, world::*};

#[wasm_bindgen]
pub fn test_fun() -> String {
    String::from("DZIAAALAAAA")
}

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

    pub fn train(&mut self) -> Statistics {
        Statistics::from(self.sim.train(&mut self.rng))
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
