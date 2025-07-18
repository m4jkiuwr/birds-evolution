mod chromosome;
mod crossing;
mod mutation;
mod roulette;
mod traits;
use rand::{self, RngCore};

pub use self::{chromosome::*, crossing::*, mutation::*, roulette::*, traits::*};

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
    use chromosome::Chromosome;
    use crossing::UniformCrossover;
    use mutation::GaussianMutation;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use roulette::RouletteSelection;

    use std::collections::BTreeMap;
    use std::iter::FromIterator;

    #[derive(Debug, Clone, PartialEq)]
    enum TestIndividual {
        WithChromosome { chromosome: Chromosome },
        WithFitness { fitness: f32 },
    }
    impl TestIndividual {
        fn new(fitness: f32) -> Self {
            Self::WithFitness { fitness }
        }
    }
    impl Individual for TestIndividual {
        fn fitness(&self) -> f32 {
            match self {
                Self::WithChromosome { chromosome } => chromosome.iter().sum(),
                Self::WithFitness { fitness } => *fitness,
            }
        }
        fn chromosome(&self) -> &Chromosome {
            match self {
                Self::WithFitness { .. } => panic!("Wrong use of WithFitness in tests"),
                Self::WithChromosome { chromosome } => chromosome,
            }
        }
        fn create(chromosome: Chromosome) -> Self {
            Self::WithChromosome { chromosome }
        }
    }
    impl PartialEq for Chromosome {
        fn eq(&self, other: &Self) -> bool {
            approx::relative_eq!(
                self.clone().into_iter().as_slice(),
                other.clone().into_iter().as_slice()
            )
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
        let crossover = UniformCrossover {};
        let child = crossover.crossover(&mut rng, &parent_a, &parent_b);

        let diff_a = child.iter().zip(parent_a).filter(|(a, b)| *a != b).count();
        let diff_b = child.iter().zip(parent_b).filter(|(a, b)| *a != b).count();

        assert_eq!(diff_a, 49);
        assert_eq!(diff_b, 51);
    }

    #[test]
    fn genetic_algorithm() {
        fn individual(genes: &[f32]) -> TestIndividual {
            TestIndividual::WithChromosome {
                chromosome: Chromosome::new(genes.iter().cloned().collect()),
            }
        }

        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let ga = GeneticAlgorithm::new(
            RouletteSelection {},
            UniformCrossover {},
            GaussianMutation::new(0.5, 0.5),
        );
        let mut population = vec![
            individual(&[0.0, 0.0, 0.0]),
            individual(&[1.0, 1.0, 2.0]),
            individual(&[1.0, 2.0, 1.0]),
            individual(&[1.0, 2.0, 4.0]),
        ];

        for _ in 0..10 {
            population = ga.evolve(&mut rng, &population);
        }
        let expected_population = vec![
            individual(&[0.6002736, 1.3938547, 4.3595104]),
            individual(&[1.0955307, 2.4240465, 3.6959934]),
            individual(&[1.275308, 2.4675508, 3.8890047]),
            individual(&[1.0225875, 2.4240465, 4.3595104]),
        ];

        assert_eq!(population, expected_population);
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
