pub use self::{bird::*, food::*, world::*, eye::*};
use nalgebra as na;
use rand::{Rng, RngCore};

mod bird;
mod food;
mod world;
mod eye;

/// A back-end structure holding the world and handling movement, collisions...
pub struct Simulation {
    world: World
}

impl Simulation {
    /// Initializes a random simulation with a random world
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng)
        }
    }

    /// Getter for world
    pub fn world(&self) -> &World {
        &self.world
    }

    /// Computes the next step of the simulation.
    /// Handles movement of the birds and collisions with food.
    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_movement();
        self.process_collisions(rng);
    }

    /// Moves the birds depending on their position, speed and rotation.
    pub fn process_movement(&mut self) {
        for bird in &mut self.world.birds {
            // Rotation relative to the y axis
            bird.position += bird.rotation * na::Vector2::new(0.0, bird.speed);
        
            bird.position.x = na::wrap(bird.position.x, 0.0, 1.0);
            bird.position.y = na::wrap(bird.position.y, 0.0, 1.0);
        }
    }

    /// If a bird is close enough to the food, handles the collision.
    /// Food will then reappear somewhere else randomly.
    pub fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for bird in &mut self.world.birds {
            for food in &mut self.world.foods {
                let distance = na::distance(
                    &bird.position, 
                    &food.position
                );

                if distance <= 0.01 {
                    food.position = rng.gen();
                }
            }
        }
    }
}