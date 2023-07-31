use crate::*;

/// A structure for one Bird, holding its spatial parameters.
#[derive(Debug)]
pub struct Bird {
    pub(crate) position: na::Point2<f32>,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) speed: f32,
    pub(crate) eye: Eye,
    pub(crate) brain: Brain,
    /// Number of foods that the bird ate
    pub(crate) satiation: usize
}

impl Bird {
    /// Initializes a new bird at a random position, with given brain and eyes.
    pub fn new(eye: Eye, brain: Brain, rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            eye,
            brain,
            satiation: 0
        }
    }

    /// Initializes a new bird at a random position, with a default eye and a random brain.
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::random(&eye);

        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            eye,
            brain,
            satiation: 0
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

    /// Convert the bird to its chromosome
    pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
        self.brain.as_chromosome()
    }

    /// Initializes a bird from given chromosome
    pub(crate) fn from_chromosome(
        chromosome: ga::Chromosome,
        rng: &mut dyn RngCore
    ) -> Self {
        let eye = Eye::default();
        let brain = Brain::from_chromosome(chromosome, &eye);

        Self::new(eye, brain, rng)
    }
}