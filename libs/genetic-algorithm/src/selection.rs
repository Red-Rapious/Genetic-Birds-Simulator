use crate::*;

/// A method that can select a parent in a given population.
pub trait SelectionMethod {
    /// Given a population, select an individual.
    fn select<'a, I>(
        &self, 
        rng: &mut dyn RngCore,
        population: &'a [I]) -> &'a I
    where
        I: Individual;
}

/// A simple selection method, that selects parents randomly.
/// The probability to be selected depends on the fitness.
pub struct RouletteWheelSelection;

impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(
        &self,
        rng: &mut dyn RngCore,
        population: &'a [I]
    ) -> &'a I
    where
        I: Individual
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("[ERROR] Empty population")
    }
}