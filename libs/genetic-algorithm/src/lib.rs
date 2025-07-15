mod chromosome;
mod crossing;
mod roulette;
mod traits;
use chromosome::Chromosome;
use crossing::UniformCrossover;
use rand::{self, RngCore, seq::IndexedRandom};
use roulette::RouletteSelection;
use traits::*;

pub struct GeneticAlgorithm<S> {
    selection_method: S,
}
impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(selection_method: S) -> Self {
        Self { selection_method }
    }

    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                let par_a = self.selection_method.select(rng, &population);
                let par_b = self.selection_method.select(rng, &population);
                // offspring
                todo!();
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use std::collections::BTreeMap;
    use std::iter::FromIterator;

    struct TestIndividual {
        fitness: f32,
    }
    impl TestIndividual {
        fn new(fitness: f32) -> Self {
            Self { fitness }
        }
    }
    impl Individual for TestIndividual {
        fn fitness(&self) -> f32 {
            self.fitness
        }
        fn chromosome(&self) -> &Chromosome {
            panic!("fun chromosome is not supported in these tests")
        }
    }

    #[test]
    fn roullete_wheel_selection() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let population = vec![
            TestIndividual::new(1.0),
            TestIndividual::new(2.0),
            TestIndividual::new(3.0),
            TestIndividual::new(4.0),
        ];

        let mut act_histogram = BTreeMap::new();
        for _ in 0..1000 {
            let fitness = RouletteSelection::new()
                .select(&mut rng, &population)
                .fitness() as i32;
            *act_histogram.entry(fitness).or_insert(0) += 1;
        }
        let exp_histogram = BTreeMap::from_iter([(1, 102), (2, 198), (3, 301), (4, 399)]);
        assert_eq!(act_histogram, exp_histogram)
    }

    #[test]
    fn uniform_crossover() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let parent_a = (1..=100).map(|n| n as f32).collect();
        let parent_b = (1..=100).map(|n| -n as f32).collect();
        let child = UniformCrossover::new().crossover(&mut rng, &parent_a, &parent_b);

        let diff_a = child.iter().zip(parent_a).filter(|(a, b)| *a != b).count();
        let diff_b = child.iter().zip(parent_b).filter(|(a, b)| *a != b).count();

        assert_eq!(diff_a, 49);
        assert_eq!(diff_b, 51);
    }
}
