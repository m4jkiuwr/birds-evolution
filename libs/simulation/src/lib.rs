mod world_props;
use world_props::*;

#[derive(Debug)]
pub struct World {
    animals: Vec<Animal>,
    foods: Vec<Food>,
}

#[derive(Debug)]
pub struct Simulation {
    world: World,
}
