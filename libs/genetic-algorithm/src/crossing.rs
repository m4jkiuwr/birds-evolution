use crate::chromosome::Chromosome;
use crate::traits::CrossoverMethod;
use rand::Rng;

pub struct UniformCrossover {}

impl CrossoverMethod for UniformCrossover {
    fn crossover<'a>(
        &self,
        rng: &mut dyn rand::RngCore,
        par_1: &Chromosome,
        par_2: &Chromosome,
    ) -> Chromosome {
        assert!(!(par_1.len() == 0 || par_2.len() == 0));
        Chromosome::new(
            par_1
                .iter()
                .zip(par_2.iter())
                .map(|(&gene_1, &gene_2)| if rng.random_bool(0.5) { gene_1 } else { gene_2 })
                .collect(),
        )
    }
}
