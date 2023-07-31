use nalgebra as na;
use rand::{Rng, RngCore};
pub use self::{bird::*, food::*, world::*};

mod bird;
mod food;
mod world;

pub struct Simulation {
    world: World
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng)
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_movement();
        self.process_collisions(rng);
    }

    pub fn process_movement(&mut self) {
        for bird in &mut self.world.birds {
            // Rotation relative to the y axis
            bird.position += bird.rotation * na::Vector2::new(0.0, bird.speed);
        
            bird.position.x = na::wrap(bird.position.x, 0.0, 1.0);
            bird.position.y = na::wrap(bird.position.y, 0.0, 1.0);
        }
    }

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