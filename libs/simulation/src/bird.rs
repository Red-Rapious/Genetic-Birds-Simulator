use crate::*;

/// A structure for one Bird, holding its spatial parameters.
#[derive(Debug)]
pub struct Bird {
    pub(crate) position: na::Point2<f32>,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) speed: f32
}

impl Bird {
    /// Initializes a bird at a random position with a base speed
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002
        }
    }

    /// Getter for position of the bird
    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    /// Getter for rotation of the bird
    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }
}