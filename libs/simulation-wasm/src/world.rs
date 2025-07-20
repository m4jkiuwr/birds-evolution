use crate::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct World {
    #[wasm_bindgen(getter_with_clone)]
    pub animals: Vec<Animal>,
    #[wasm_bindgen(getter_with_clone)]
    pub foods: Vec<Food>,
}

impl From<&sim::World> for World {
    fn from(value: &sim::World) -> Self {
        let animals = value.animals().iter().map(Animal::from).collect();
        let foods = value.foods().iter().map(Food::from).collect();
        Self { animals, foods }
    }
}
