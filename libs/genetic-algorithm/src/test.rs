#[cfg(test)]
use crate::*;

#[cfg(test)]
impl PartialEq for Chromosome {
    fn eq(&self, other: &Self) -> bool {
        approx::relative_eq!(
            self.genes.as_slice(),
            other.genes.as_slice(),
        )
    }
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq)]
pub enum TestIndividual {
    WithChromosome { chromosome: Chromosome },
    WithFitness { fitness: f32 }
}

#[cfg(test)]
impl TestIndividual {
    pub fn new(fitness: f32) -> Self {
        Self::WithFitness { fitness }
    }
}

#[cfg(test)]
impl Individual for TestIndividual {
    fn fitness(&self) -> f32 {
        match self {
            Self::WithChromosome { chromosome } => chromosome.iter().sum(), // simple fitness function
            Self::WithFitness { fitness } => *fitness
        }
    }

    fn chromosome(&self) -> &Chromosome {
        match self {
            Self::WithChromosome { chromosome } => chromosome,
            Self::WithFitness { .. } => panic!("not supported for TestIndividual::WithFitness")
        }
    }

    fn create(chromosome: Chromosome) -> Self {
        Self::WithChromosome { chromosome }
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

    mod index {
        use super::*;

        #[test]
        fn test() {
            let chromosome = chromosome();

            assert_eq!(chromosome[0], 3.0);
            assert_eq!(chromosome[1], 1.0);
            assert_eq!(chromosome[2], 2.0);
        }
    }

    mod from_iterator {
        use super::*;

        #[test]
        fn test() {
            let chromosome: Chromosome = vec![3.0, 1.0, 2.0]
                .into_iter().collect();

            assert_eq!(chromosome[0], 3.0);
            assert_eq!(chromosome[1], 1.0);
            assert_eq!(chromosome[2], 2.0);
        }
    }

    mod into_iterator {
        use super::*;

        #[test]
        fn test() {
            let chromosome = chromosome();

            let genes: Vec<_> = chromosome.into_iter().collect();

            assert_eq!(genes.len(), 3);
            assert_eq!(genes[0], 3.0);
            assert_eq!(genes[1], 1.0);
            assert_eq!(genes[2], 2.0);
        }
    }

    mod crossover {
        use super::*;
        
        #[test]
        fn test() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let parent_a = 
                (1..=100)
                    .map(|n| n as f32)
                    .collect();
            let parent_b = 
                (1..=100)
                    .map(|n| -n as f32)
                    .collect();
            
            let child = UniformCrossover::new()
                .crossover(&mut rng, &parent_a, &parent_b);

            let diff_a = child
                .iter()
                .zip(parent_a)
                .filter(|(c, p)| *c != p)
                .count();

            let diff_b = child
                .iter()
                .zip(parent_b)
                .filter(|(c, p)| *c != p)
                .count();

            assert_eq!(diff_a, 49);
            assert_eq!(diff_b, 51);
        }
    }

    mod mutation {
        use super::*;

        fn mutate_a_child(chance: f32, coeff: f32) -> Vec<f32> {
            let mut child = vec![1.0, 2.0, 3.0, 4.0, 5.0]
                .into_iter()
                .collect();

            let mut rng = ChaCha8Rng::from_seed(Default::default());

            GaussianMutation::new(chance, coeff).mutate(&mut rng, &mut child);
            child.into_iter().collect()
        }

        mod given_zero_chance {
            use super::*;
            mod and_zero_coefficient {
                use super::*;

                #[test]
                fn does_not_change_original_chromosome() {
                    let result = mutate_a_child(0.0, 0.0);
                    let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                    approx::assert_relative_eq!(
                        result.as_slice(),
                        expected.as_slice()
                    );
                }
            }

            mod and_nonzero_coefficient {
                use super::*;

                #[test]
                fn does_not_change_original_chromosome() {
                    let result = mutate_a_child(0.0, 0.0);
                    let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                    approx::assert_relative_eq!(
                        result.as_slice(),
                        expected.as_slice()
                    );
                }
            }
        }

        mod given_fifty_fifty_chance {
            use super::*;

            mod and_zero_coefficient {
                use super::*;

                #[test]
                fn does_not_change_original_chromosome() {
                    let result = mutate_a_child(0.0, 0.0);
                    let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                    approx::assert_relative_eq!(
                        result.as_slice(),
                        expected.as_slice()
                    );
                }
            }

            mod and_nonzero_coefficient {
                use super::*;

                #[test]
                fn slightly_changes_original_chromosome() {
                    let result = mutate_a_child(0.5, 0.5);
                    let expected = vec![1.0, 1.7756249, 3.0, 4.1596804, 5.0];

                    approx::assert_relative_eq!(
                        result.as_slice(),
                        expected.as_slice(),
                    );
                }
            }
        }

        mod given_max_chance {
            use super::*;

            mod and_zero_coefficient {
                use super::*;

                #[test]
                fn does_not_change_original_chromosome() {
                    let result = mutate_a_child(0.0, 0.0);
                    let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                    approx::assert_relative_eq!(
                        result.as_slice(),
                        expected.as_slice()
                    );
                }
            }

            mod and_nonzero_coefficient {
                use super::*;

                #[test]
                fn entirely_changes_original_chromosome() {
                    let result = mutate_a_child(1.0, 0.5);

                    let expected = vec![
                        1.4545316,
                        2.1162078,
                        2.7756248,
                        3.9505124,
                        4.638691,
                    ];

                    approx::assert_relative_eq!(
                        result.as_slice(),
                        expected.as_slice(),
                    );
                }
            }
        }
    }

    mod evolution {
        use super::*;

        fn individual(genes: &[f32]) -> TestIndividual {
            let chromosome = genes.iter().cloned().collect();
            TestIndividual::create(chromosome)
        }

        #[test]
        fn test() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());

            let genetic_algorithm = GeneticAlgorithm::new(
                RouletteWheelSelection::new(),
                UniformCrossover::new(),
                GaussianMutation::new(0.5, 0.5)
            );

            let mut population = vec![
                individual(&[0.0, 0.0, 0.0]), // fitness = 0.0
                individual(&[1.0, 1.0, 1.0]), // fitness = 3.0
                individual(&[1.0, 2.0, 1.0]), // fitness = 4.0
                individual(&[1.0, 2.0, 4.0]), // fitness = 7.0
            ];

            let sum_fitness_initial: f32 = population
                .iter()
                .map(|individual| individual.fitness())
                .sum();

            for _ in 0..10 {
                (population, _) = genetic_algorithm.evolve(&mut rng, &population);
            }

            let expected_population = vec![
                individual(&[0.44769490, 2.0648358, 4.3058133]),
                individual(&[1.21268670, 1.5538777, 2.8869110]),
                individual(&[1.06176780, 2.2657390, 4.4287640]),
                individual(&[0.95909685, 2.4618788, 4.0247330]),
            ];

            assert_eq!(population, expected_population);

            let sum_fitness_population: f32 = population
                .iter()
                .map(|individual| individual.fitness())
                .sum();

            assert!(sum_fitness_initial < sum_fitness_population)
        }
    }
}