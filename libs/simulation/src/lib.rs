use nalgebra as na;
use rand::{Rng, RngCore};

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
}

#[derive(Debug)]
pub struct World {
    birds: Vec<Bird>,
    foods: Vec<Food>
}

impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let birds = (0..40)
            .map(|_| Bird::random(rng))
            .collect();

        let foods = (0..60)
            .map(|_| Food::random(rng))
            .collect();

        Self { birds, foods }
    }

    pub fn birds(&self) -> &[Bird] {
        &self.birds
    }

    pub fn foods(&self) -> &[Food] {
        &self.foods
    }
}

#[derive(Debug)]
pub struct Bird {
    position: na::Point2<f32>,
    rotation: na::Rotation2<f32>,
    speed: f32
}

impl Bird {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }
}

#[derive(Debug)]
pub struct Food {
    position: na::Point2<f32>
}

impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen()
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }
}