use crate::*;

/// A method to add random mutations to an existing genome.
pub trait MutationMethod {
    /// Given a child's chromosome, modify one or more genes from it.
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome);
}

/// A simple mutation method, that changes each genes with a given chance,
/// and with a given magnitude.
/// For `chance = 0.5` and `coeff = 2.0`, each gene will have a fifty-fifty
/// chance to be added or substracted 2.
#[derive(Clone, Debug)]
pub struct GaussianMutation {
    /// Probability of changing a gene (between `0.0` and `1.0`)
    chance: f32,

    /// Magnitude of that change
    coeff: f32
}

impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!(chance >= 0.0 && chance <= 1.0);

        Self { chance, coeff }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        for gene in child.iter_mut() {
            let sign = if rng.gen_bool(0.5)  { -1.0 } else { 1.0 };

            if rng.gen_bool(self.chance as _) {
                *gene += sign * self.coeff * rng.gen::<f32>();
            }
        }
    }
}