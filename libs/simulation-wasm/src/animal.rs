use crate::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Copy)]
pub struct Animal {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub score: usize,
}

impl From<&sim::Animal> for Animal {
    fn from(value: &sim::Animal) -> Self {
        Self {
            x: value.position().x,
            y: value.position().y,
            rotation: value.rotation().angle(),
            score: value.food_counter(),
        }
    }
}
