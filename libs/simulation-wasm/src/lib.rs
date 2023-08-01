use wasm_bindgen::prelude::*;
use lib_simulation as sim;
use rand::prelude::*;
use serde::Serialize;
use lib_genetic_algorithm as ga;

/// WebAssembly-aware wrapper for the simulation
#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
    generation: usize,
    stats: ga::Statistics
}

#[wasm_bindgen]
impl Simulation {
    /// Initializes a new random simulation
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(&mut rng);

        let birds = sim.world().birds();
        let birds_individual: Vec<lib_simulation::BirdIndividual> = birds
            .iter()
            .map(|bird| lib_simulation::BirdIndividual::from_bird(bird))
            .collect();
        let stats = ga::Statistics::new(&birds_individual);

        Self { rng, sim, generation: 0, stats }
    }

    /// Getter for `world`
    pub fn world(&self) -> JsValue {
        let world = World::from(self.sim.world());

        #[allow(deprecated)]
        JsValue::from_serde(&world).unwrap()
    }

    /// Steps the back-end simulation.
    /// Returns true if the population evolved.
    pub fn step(&mut self) -> usize {
        // If it just evolved, returns true
        if let Some(stats) = self.sim.step(&mut self.rng) {
            self.generation += 1;
            self.stats = stats;
        } 
        self.generation
    }

    /// Fast-forwards to the next generation
    pub fn train(&mut self) {
        let stats = self.sim.train(&mut self.rng);
        self.generation += 1; // TODO: change the generation system
        self.stats = stats;
    }

    pub fn min_fitness(&self) -> usize {
        self.stats.min_fitness() as usize
    }

    pub fn max_fitness(&self) -> usize {
        self.stats.max_fitness() as usize
    }

    pub fn avg_fitness(&self) -> usize {
        self.stats.avg_fitness() as usize
    }
}

/// Front-end World
#[derive(Clone, Debug, Serialize)]
pub struct World {
    pub birds: Vec<Bird>,
    pub foods: Vec<Food>
}

/// Convert back-end World to front-end World
impl From<&sim::World> for World {
    fn from(world: &sim::World) -> Self {
        let birds = world
            .birds()
            .iter()
            .map(Bird::from)
            .collect();

        let foods = world
            .foods()
            .iter()
            .map(Food::from)
            .collect();

        Self { birds, foods }
    }
}

/// Front-end Bird
#[derive(Clone, Debug, Serialize)]
pub struct Bird {
    pub x: f32,
    pub y: f32,
    pub rotation: f32
}

/// Convert back-end Bird to front-end Bird
impl From<&sim::Bird> for Bird {
    fn from(bird: &sim::Bird) -> Self {
        Self {
            x: bird.position().x,
            y: bird.position().y,
            rotation: bird.rotation().angle()
        }
    }
}

/// Front-end Food
#[derive(Clone, Debug, Serialize)]
pub struct Food {
    pub x: f32,
    pub y: f32
}

/// Convert back-end Food to front-end Food
impl From<&sim::Food> for Food {
    fn from(food: &sim::Food) -> Self {
        Self {
            x: food.position().x,
            y: food.position().y
        }
    }
}