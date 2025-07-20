use crate::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Statistics {
    pub min_score: usize,
    pub max_score: usize,
    pub avg_score: f32,
}

impl From<ga::Statistics> for Statistics {
    fn from(value: ga::Statistics) -> Self {
        Self {
            min_score: value.min_fitness as usize,
            max_score: value.max_fitness as usize,
            avg_score: value.avg_fitness,
        }
    }
}
