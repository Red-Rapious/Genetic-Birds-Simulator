pub use self::{bird::*, food::*, world::*, eye::*};
use lib_neural_network as nn;
use nalgebra as na;
use rand::{Rng, RngCore};
use std::f32::consts::FRAC_PI_2;

mod bird;
mod food;
mod world;
mod eye;

const SPEED_MIN: f32 = 0.001; // minimum speed of a bird, avoids getting stuck
const SPEED_MAX: f32 = 0.005; // maximum speed of a bird, avoids unrealistic behaviors
const SPEED_ACCEL: f32 = 0.2; // how much the brain can affect the speed in one step
const ROTATION_ACCEL: f32 = FRAC_PI_2; // how much the brain can change the rotation in one step

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
        self.process_brains();
        self.process_movement();
        self.process_collisions(rng);
    }

    fn process_brains(&mut self) {
        for bird in &mut self.world.birds {
            let vision = bird.eye.process_vision(
                bird.position,
                bird.rotation,
                &self.world.foods
            );

            let response = bird.brain.propagate(vision);
            let (speed, rotation) = (response[0], response[1]);

            let speed = speed.clamp(-SPEED_ACCEL, SPEED_ACCEL);
            let rotation = rotation.clamp(-ROTATION_ACCEL, ROTATION_ACCEL);

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
                    food.position = rng.gen();
                }
            }
        }
    }
}