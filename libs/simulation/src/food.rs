use crate::*;

/// A struct for Food. Holds the position of the food in space.
#[derive(Debug)]
pub struct Food {
    pub(crate) position: na::Point2<f32>
}

impl Food {
    /// Creates food at a random position
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen()
        }
    }

    /// Getter for position
    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }
}