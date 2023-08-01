use rand::{RngCore, seq::SliceRandom, Rng};
pub use crate::{crossover::*, mutation::*, selection::*, chromosome::*, statistics::*};
use std::ops::Index;

pub mod chromosome;
pub mod crossover;
pub mod mutation;
pub mod selection;
pub mod statistics;
pub mod test;

/// A wrapping structure of the Genetic Algorithm, holding the evolution methods.
pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod
{
    pub fn new(
        selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        mutation_method: impl MutationMethod + 'static
        ) -> Self {
        Self { 
            selection_method, 
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method)
        }
    }

    /// Given a population, selects, crosses over, and mutates each individual.
    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> (Vec<I>, Statistics)
    where 
        I: Individual,
    {
        assert!(!population.is_empty());

        let new_population: Vec<I> = (0..population.len())
            .map(|_| {
                // Selection of two random parents
                let parent_a = self
                    .selection_method
                    .select(rng, population)
                    .chromosome();

                let parent_b = self
                    .selection_method
                    .select(rng, population)
                    .chromosome();

                // Crossover
                let mut child = self
                    .crossover_method
                    .crossover(rng, parent_a, parent_b);

                // Mutation
                self.mutation_method.mutate(rng, &mut child);

                // Convert the Chromosome back to an Individual
                I::create(child)
            })
            .collect();

        let stats = Statistics::new(population);

        (new_population, stats)
    }
}

/// An abstract individual, which holds chromosomes.
/// These chromosomes can be computed into a fitness function.
pub trait Individual {
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
    fn create(chromosome: Chromosome) -> Self;
}