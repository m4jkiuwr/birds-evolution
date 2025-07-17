use crate::*;

#[derive(Debug)]
pub struct World {
    pub(crate) animals: Vec<Animal>,
    pub(crate) foods: Vec<Food>,
}

impl World {
    pub(crate) fn random(rng: &mut dyn RngCore, animal_num: usize, food_num: usize) -> Self {
        let animals = (0..animal_num).map(|_| Animal::random(rng)).collect();
        let foods = (0..food_num).map(|_| Food::random(rng)).collect();
        Self { animals, foods }
    }
    pub fn animals(&self) -> &Vec<Animal> {
        &self.animals
    }
    pub fn foods(&self) -> &Vec<Food> {
        &self.foods
    }
}
