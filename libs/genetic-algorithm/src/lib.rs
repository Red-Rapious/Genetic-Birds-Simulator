use rand::{RngCore, seq::SliceRandom, Rng};
use std::ops::Index;

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

        let stats = Statistics::new(&new_population);

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


/// A wrapper for genes, a vector of `f32`.
#[derive(Clone, Debug)]
pub struct  Chromosome {
    genes: Vec<f32>
}

impl Chromosome {
    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }
}

impl Index<usize> for Chromosome {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}

impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect()
        }
    }
}

impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = std::vec::IntoIter<f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}

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

/// A method to add random mutations to an existing genome.
pub trait MutationMethod {
    /// Given a child's chromosome, modify one or more genes from it.
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome);
}

/// A simple mutation method, that changes each genes with a given chance,
/// and with a given magnitude.
/// For `chance = 0.5` and `coeff = 2.0`, each gene will have a fifty-fifty
/// chance to be added or substracted 2.
#[derive(Clone, Debug)]
pub struct GaussianMutation {
    /// Probability of changing a gene (between `0.0` and `1.0`)
    chance: f32,

    /// Magnitude of that change
    coeff: f32
}

impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!(chance >= 0.0 && chance <= 1.0);

        Self { chance, coeff }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        for gene in child.iter_mut() {
            let sign = if rng.gen_bool(0.5)  { -1.0 } else { 1.0 };

            if rng.gen_bool(self.chance as _) {
                *gene += sign * self.coeff * rng.gen::<f32>();
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Statistics {
    min_fitness: f32,
    max_fitness: f32,
    average_fitness: f32
}

impl Statistics {
    fn new<I>(population: &[I]) -> Self 
    where
        I: Individual
    {
        assert!(!population.is_empty());

        let mut min_fitness = population[0].fitness();
        let mut max_fitness = population[0].fitness();
        let mut sum_fitness = 0.0;

        for individual in population {
            let fitness = individual.fitness();

            min_fitness = min_fitness.min(fitness);
            max_fitness = max_fitness.max(fitness);
            sum_fitness += fitness;
        }

        Self {
            min_fitness,
            max_fitness,
            average_fitness: sum_fitness / (population.len() as f32)
        }
    }

    pub fn min_fitness(&self) -> f32 {
        self.min_fitness
    }

    pub fn max_fitness(&self) -> f32 {
        self.max_fitness
    }

    pub fn avg_fitness(&self) -> f32 {
        self.average_fitness
    }
}