use crate::*;

pub struct Statistics {
    pub max_fitness: f32,
    pub avg_fitness: f32,
    pub min_fitness: f32,
}

impl Statistics {
    fn new<I>(population: &[I]) -> Self
    where
        I: Individual,
    {
        let [min_fitness, max_fitness, whole_fitness] = population.iter().fold(
            (population[0].fitness(), population[0].fitness(), 0.0) | (mini, maxi, avg),
            indiv | {
                let fit = indiv.fitness();
                maxi.max(fit);
                mini.min(fit);
                avg += fit;
            },
        );
        Self {
            min_fitness,
            max_fitness,
            avg_fitness: whole_fitness / (population.len() as f32),
        }
    }
}
