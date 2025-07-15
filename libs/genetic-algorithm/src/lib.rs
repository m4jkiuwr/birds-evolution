mod chromosome;
mod crossing;
mod mutation;
mod roulette;
mod traits;
use chromosome::Chromosome;
use crossing::UniformCrossover;
use mutation::GaussianMutation;
use rand::{self, RngCore, seq::IndexedRandom};
use roulette::RouletteSelection;
use traits::*;

pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}
impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(
        selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        mutation_method: impl MutationMethod + 'static,
    ) -> Self {
        Self {
            selection_method,
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method),
        }
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
                let mut child =
                    self.crossover_method
                        .crossover(rng, par_a.chromosome(), par_b.chromosome());
                self.mutation_method.mutate(rng, &mut child);
                I::create(child)
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
        fn create(chromosome: Chromosome) -> Self {
            todo!();
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

    mod gaussian_mutation {
        use super::*;
        use approx::assert_relative_eq;
        fn actual(primal_child: &Chromosome, chance: f32, coeff: f32) -> Vec<f32> {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let mut child: Chromosome = primal_child.clone();
            GaussianMutation::new(chance, coeff).mutate(&mut rng, &mut child);
            child.into_iter().collect()
        }

        mod with_zero_chance {
            use super::*;

            #[test]
            fn no_change() {
                let primal_child = Chromosome::test(10);
                let child = actual(&primal_child, 0.0, 0.0);
                assert_relative_eq!(
                    primal_child.clone().into_iter().as_slice(),
                    child.as_slice()
                );
                let child = actual(&primal_child, 0.0, 1000.0);
                assert_relative_eq!(
                    primal_child.clone().into_iter().as_slice(),
                    child.as_slice()
                );
            }
        }
        mod with_zero_coeff {
            use super::*;
            #[test]
            fn no_change() {
                let primal_child = Chromosome::test(10);
                let child = actual(&primal_child, 1.0, 0.0);
                assert_relative_eq!(primal_child.into_iter().as_slice(), child.as_slice());
            }
        }
        #[test]
        fn rand_test() {
            let primal_child = Chromosome::test(1000);
            let child = actual(&primal_child, 0.5, 3.0);
            let diff_cells = primal_child
                .clone()
                .into_iter()
                .zip(&child)
                .filter(|(a, b)| a != *b)
                .count();
            let sum_diff: f32 = primal_child
                .into_iter()
                .zip(child)
                .map(|(a, b)| a - b)
                .sum();

            assert_eq!(diff_cells, 492);
            assert_eq!(sum_diff, -6.7560215);
        }
    }
}
