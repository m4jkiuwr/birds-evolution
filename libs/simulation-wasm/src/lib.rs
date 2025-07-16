use lib_simulation as sim;
use rand::{self, rngs::ThreadRng};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn bruh() -> String {
    String::from("O bracie")
}

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let sim = sim::Simulation::random(&mut rng);
        Self { rng, sim }
    }

    pub fn world(&self) -> World {
        World::from(self.sim.world())
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Copy)]
pub struct Animal {
    pub x: f32,
    pub y: f32,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct World {
    #[wasm_bindgen(getter_with_clone)]
    pub animals: Vec<Animal>,
}

impl From<&sim::world_props::Animal> for Animal {
    fn from(value: &sim::world_props::Animal) -> Self {
        Self {
            x: value.position().x,
            y: value.position().y,
        }
    }
}

impl From<&sim::World> for World {
    fn from(value: &sim::World) -> Self {
        let animals = value.animals().iter().map(Animal::from).collect();
        Self { animals }
    }
}
