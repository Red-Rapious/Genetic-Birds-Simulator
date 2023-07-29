use rand::{RngCore, seq::SliceRandom};

pub struct GeneticAlgorithm<S> {
    selection_method: S
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod
{
    pub fn new(selection_method: S) -> Self {
        Self { selection_method }
    }

    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I> 
    where 
        I: Individual,
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                // Selection of two random parents
                let parent_a = self
                    .selection_method
                    .select(rng, population);

                let parent_b = self
                    .selection_method
                    .select(rng, population);

                // Crossover
                // Mutation
                todo!()
            })
            .collect()
    }
}

pub trait Individual {
    fn fitness(&self) -> f32;
}

pub trait SelectionMethod {
    fn select<'a, I>(
        &self, 
        rng: &mut dyn RngCore,
        population: &'a [I]) -> &'a I
    where
        I: Individual;
}

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

#[cfg(test)]
#[derive(Clone, Debug)]
pub struct TestIndividual {
    fitness: f32
}

#[cfg(test)]
impl TestIndividual {
    pub fn new(fitness: f32) -> Self {
        Self { fitness }
    }
}

#[cfg(test)]
impl Individual for TestIndividual {
    fn fitness(&self) -> f32 {
        self.fitness
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    mod selection {
        use super::*;
        
        #[test]
        fn test() {
            let method = RouletteWheelSelection::new();
            let mut rng = ChaCha8Rng::from_seed(Default::default());

            let population = vec![
                TestIndividual::new(2.0),
                TestIndividual::new(1.0),
                TestIndividual::new(4.0),
                TestIndividual::new(3.0),
            ];
            
            let mut actual_histogram = BTreeMap::new();

            for _ in 0..1_000 { // 1_000 is just a big number
                let fitness = method
                    .select(&mut rng, &population)
                    .fitness() as i32; // f32 doesn't `Ord` trait because of NaN

                *actual_histogram
                    .entry(fitness)
                    .or_insert(0) += 1;
            }

            let expected_histogram = BTreeMap::from_iter(vec![
                (1, 98),
                (2, 202),
                (3, 278),
                (4, 422),
            ]);

            assert_eq!(actual_histogram, expected_histogram);
        }
    }

    fn chromosome() -> Chromosome {
        Chromosome { genes: vec![3.0, 1.0, 2.0] }
    }

    mod len {
        use super::*;

        #[test]
        fn test() {
            assert_eq!(chromosome().len(), 3);
        }
    }

    mod iter {
        use super::*;

        #[test]
        fn test() {
            let chromosome = chromosome();
            let genes: Vec<_> = chromosome.iter().collect();

            assert_eq!(genes.len(), 3);
            assert_eq!(genes[0], &3.0);
            assert_eq!(genes[1], &1.0);
            assert_eq!(genes[2], &2.0);
        }
    }

    mod iter_mut {
        use super::*;

        #[test]
        fn test() {
            let mut chromosome = chromosome();

            chromosome.iter_mut().for_each(|gene| {
                *gene *= 10.0;
            });

            let genes: Vec<_> = chromosome.iter().collect();

            assert_eq!(genes.len(), 3);
            assert_eq!(genes[0], &30.0);
            assert_eq!(genes[1], &10.0);
            assert_eq!(genes[2], &20.0);
        }
    }
}

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