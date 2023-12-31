use crate::*;

/// An implementation of Individual in the case of a bird.
pub struct BirdIndividual {
    fitness: f32,
    chromosome: ga::Chromosome
}

impl ga::Individual for BirdIndividual {
    fn create(chromosome: ga::Chromosome) -> Self {
        Self { fitness: 0.0, chromosome }
    }

    fn chromosome(&self) -> &ga::Chromosome {
        &self.chromosome
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }
}

impl BirdIndividual {
    /// Convert a Bird to a BirdIndividual
    pub fn from_bird(bird: &Bird) -> Self {
        Self {
            fitness: bird.satiation as f32,
            chromosome: bird.as_chromosome()
        }
    }

    /// Convert a BirdIndividual to a Bird
    pub fn into_bird(self, rng: &mut dyn RngCore) -> Bird {
        Bird::from_chromosome(self.chromosome, rng)
    }
}
