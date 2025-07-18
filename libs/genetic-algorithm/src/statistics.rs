use crate::*;

pub struct Statistics {
    pub max_fitness: f32,
    pub avg_fitness: f32,
    pub min_fitness: f32,
}

impl Statistics {
    pub(crate) fn new<I>(population: &[I]) -> Self
    where
        I: Individual,
    {
        assert!(!population.is_empty());
        let mut min_fitness = population[0].fitness();
        let mut max_fitness = min_fitness;
        let mut avg_fitness = 0.0;
        for i in population {
            let fit = i.fitness();
            min_fitness = min_fitness.min(fit);
            max_fitness = max_fitness.max(fit);
            avg_fitness += fit;
        }
        avg_fitness /= population.len() as f32;
        Self {
            min_fitness,
            max_fitness,
            avg_fitness,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "min={:.2} | max={:.2} | avg={:.2}",
            self.min_fitness, self.max_fitness, self.avg_fitness
        )
    }
}
