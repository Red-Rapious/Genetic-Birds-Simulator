use crate::*;

pub struct BirdIndividual {
    fitness: f32,
    chromosome: ga::Chromosome
}

impl ga::Individual for BirdIndividual {
    fn create(chromosome: ga::Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome
        }
    }

    fn chromosome(&self) -> &ga::Chromosome {
        &self.chromosome
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }
}

impl BirdIndividual {
    pub fn from_bird(bird: &Bird) -> Self {
        Self {
            fitness: bird.satiation as f32,
            chromosome: bird.as_chromosome()
        }
    }

    pub fn into_bird(self, rng: &mut dyn RngCore) -> Bird {
        Bird::from_chromosome(self.chromosome, rng)
    }
}
