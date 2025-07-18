use crate::*;

pub struct AnimalIndividual {
    fitness: f32,
    chromosome: ga::Chromosome,
}

impl ga::Individual for AnimalIndividual {
    fn create(chromosome: ga::Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
        }
    }
    fn fitness(&self) -> f32 {
        self.fitness
    }
    fn chromosome(&self) -> &ga::Chromosome {
        &self.chromosome
    }
}

impl AnimalIndividual {
    pub(crate) fn from_animal(animal: &Animal) -> Self {
        let fitness = animal.food_counter as f32;
        let chromosome: ga::Chromosome = animal.as_chromosome();

        Self {
            fitness,
            chromosome,
        }
    }
    pub(crate) fn into_animal(&self, rng: &mut dyn RngCore) -> Animal {
        Animal::from_chromosome(rng, self.chromosome.clone())
    }
}
