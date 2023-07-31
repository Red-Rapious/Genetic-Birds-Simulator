use crate::*;

#[derive(Debug)]
pub struct World {
    pub(crate) birds: Vec<Bird>,
    pub(crate) foods: Vec<Food>
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