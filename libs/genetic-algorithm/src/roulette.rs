use crate::traits::*;
use rand::{RngCore, seq::IndexedRandom};

pub struct RouletteSelection {}
impl RouletteSelection {
    pub fn new() -> Self {
        Self {}
    }
}
impl SelectionMethod for RouletteSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("Empty population")
    }
}
