use nalgebra as na;
use rand::{Rng, RngCore};
use std::f32::consts::FRAC_PI_2;

pub use self::{bird::*, food::*, world::*, eye::*, bird_individual::*, brain:: *};
use lib_neural_network as nn;
use lib_genetic_algorithm as ga;

mod bird;
mod bird_individual;
mod brain;
mod eye;
mod food;
mod world;

/// Minimum speed of a bird, avoids getting stuck
const SPEED_MIN: f32 = 0.001; 
/// Maximum speed of a bird, avoids unrealistic behaviors
const SPEED_MAX: f32 = 0.005; 
/// How much the brain can affect the speed in one step
const SPEED_ACCEL: f32 = 0.2; 
// How much the brain can change the rotation in one step
const ROTATION_ACCEL: f32 = FRAC_PI_2; 

/// How many steps each bird gets to live
const GENERATION_LENGTH: usize = 2500; 

/// Gaussian Mutation chance of mutation
const MUTATION_CHANCE: f32 = 0.01;
/// Gaussian Mutation magnitude of mutation
const MUTATION_COEFF: f32 = 0.03;

const NB_BIRDS: usize = 40;
const NB_FOODS: usize = 60;

/// A back-end structure holding the world and handling movement, collisions...
pub struct Simulation {
    world: World,
    genetic_algorithm: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    age: usize
}

impl Simulation {
    /// Initializes a random simulation with a random world
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(NB_BIRDS, NB_FOODS, rng),
            genetic_algorithm: ga::GeneticAlgorithm::new(
                ga::RouletteWheelSelection::new(),
                ga::UniformCrossover::new(),
                ga::GaussianMutation::new(MUTATION_CHANCE, MUTATION_COEFF)
            ),
            age: 0
        }
    }

    /// Getter for world
    pub fn world(&self) -> &World {
        &self.world
    }

    /// Computes the next step of the simulation.
    /// Handles movement of the birds, collisions with food, and brains changes.
    /// Returns `Some(stats)` if `evolve` was called, `None` either.
    pub fn step(&mut self, rng: &mut dyn RngCore) -> Option<ga::Statistics> {
        self.age += 1;

        self.process_brains();
        self.process_movement();
        self.process_collisions(rng);

        if self.age > GENERATION_LENGTH {
            Some(self.evolve(rng))
        } else {
            None
        }
    }

    /// Loops until the end of the current generation
    pub fn train(&mut self, rng: &mut dyn RngCore) -> ga::Statistics {
        loop {
            if let Some(summary) = self.step(rng) {
                return summary;
            }
        }
    }

    /// Adjusts the speed and rotation of each bird according to the brain
    fn process_brains(&mut self) {
        for bird in &mut self.world.birds {
            // What the bird sees
            let vision = bird.eye.process_vision(
                bird.position,
                bird.rotation,
                &self.world.foods
            );

            // Response of the brain
            let response = bird.brain.neural_network.propagate(vision);
            let (speed, rotation) = (response[0], response[1]);

            // Clamp the response to make sure that the brain doesn't change speed and rotation too much
            let speed = speed.clamp(-SPEED_ACCEL, SPEED_ACCEL);
            let rotation = rotation.clamp(-ROTATION_ACCEL, ROTATION_ACCEL);

            // Adjusts speed and rotation
            bird.speed = (bird.speed + speed).clamp(SPEED_MIN, SPEED_MAX);
            bird.rotation = na::Rotation2::new(
                bird.rotation.angle() + rotation
            ); // no need to clamp since rotation wraps inside [0, 2*PI]
        }
    }

    /// Moves the birds depending on their position, speed and rotation.
    fn process_movement(&mut self) {
        for bird in &mut self.world.birds {
            // Rotation relative to the y axis
            bird.position += bird.rotation * na::Vector2::new(0.0, bird.speed);
        
            bird.position.x = na::wrap(bird.position.x, 0.0, 1.0);
            bird.position.y = na::wrap(bird.position.y, 0.0, 1.0);
        }
    }

    /// If a bird is close enough to the food, handles the collision.
    /// Food will then reappear somewhere else randomly.
    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for bird in &mut self.world.birds {
            for food in &mut self.world.foods {
                let distance = na::distance(
                    &bird.position, 
                    &food.position
                );

                if distance <= 0.01 {
                    bird.satiation += 1;
                    food.position = rng.gen();
                }
            }
        }
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) -> ga::Statistics {
        self.age = 0;

        // Prepare birds
        let current_population: Vec<_> = self
            .world
            .birds
            .iter()
            .map(|bird| BirdIndividual::from_bird(bird))
            .collect();

        // Evolve birds
        let (evolved_population, stats) = self.genetic_algorithm.evolve(
            rng,
            &current_population
        );

        // Add birds to the world
        self.world.birds = evolved_population
            .into_iter()
            .map(|individual| individual.into_bird(rng))
            .collect();

        // Changes the place of the food (for UI purposes to spot a new generation)
        for food in &mut self.world.foods {
            food.position = rng.gen();
        }

        stats
    }
}