use crate::*;

/// A method that can "fuse" two given parents into one child.
pub trait CrossoverMethod {
    /// Given two chromosomes of parents, crosses them over to create the chromosomes of a child.
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome
    ) -> Chromosome;
}

/// A simple method of crossover, which selects randomly and uniformely each gene from one of the parents.
#[derive(Clone, Debug)]
pub struct UniformCrossover;

impl UniformCrossover {
    pub fn new() -> Self {
        Self
    }
}

impl CrossoverMethod for UniformCrossover {
    fn crossover(
            &self,
            rng: &mut dyn RngCore,
            parent_a: &Chromosome,
            parent_b: &Chromosome
        ) -> Chromosome {
        
        assert_eq!(parent_a.len(), parent_b.len());

        parent_a
            .iter()
            .zip(parent_b.iter())
            .map(|(&a, &b)| if rng.gen_bool(0.5) { a } else { b })
            .collect()
    }
}