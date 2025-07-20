use crate::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Copy)]
pub struct Food {
    pub x: f32,
    pub y: f32,
}

impl From<&sim::Food> for Food {
    fn from(value: &sim::Food) -> Self {
        Self {
            x: value.position().x,
            y: value.position().y,
        }
    }
}
