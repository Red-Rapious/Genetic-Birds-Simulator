use crate::*;

/// A structure holding the birds and the food.
#[derive(Debug)]
pub struct World {
    pub(crate) birds: Vec<Bird>,
    pub(crate) foods: Vec<Food>
}

impl World {
    /// Initializes a random world with 40 random birds and 60 random food.
    pub fn random(nb_birds: usize, nb_foods: usize, rng: &mut dyn RngCore) -> Self {
        let birds = (0..nb_birds)
            .map(|_| Bird::random(rng))
            .collect();

        let foods = (0..nb_foods)
            .map(|_| Food::random(rng))
            .collect();

        Self { birds, foods }
    }

    /// Getter for birds
    pub fn birds(&self) -> &[Bird] {
        &self.birds
    }

    /// Getter for foods.
    pub fn foods(&self) -> &[Food] {
        &self.foods
    }
}