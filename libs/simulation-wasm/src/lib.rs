use wasm_bindgen::prelude::*;
use lib_simulation as sim;
use rand::prelude::*;
use serde::Serialize;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(&mut rng);

        Self { rng, sim }
    }

    pub fn world(&self) -> JsValue {
        let world = World::from(self.sim.world());

        #[allow(deprecated)]
        JsValue::from_serde(&world).unwrap()
    }

    pub fn step(&mut self) {
        self.sim.step();
    }
}

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

impl From<&sim::Bird> for Bird {
    fn from(bird: &sim::Bird) -> Self {
        Self {
            x: bird.position().x,
            y: bird.position().y,
            rotation: bird.rotation().angle()
        }
    }
}

impl From<&sim::Food> for Food {
    fn from(food: &sim::Food) -> Self {
        Self {
            x: food.position().x,
            y: food.position().y
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct World {
    pub birds: Vec<Bird>,
    pub foods: Vec<Food>
}

#[derive(Clone, Debug, Serialize)]
pub struct Bird {
    pub x: f32,
    pub y: f32,
    pub rotation: f32
}

#[derive(Clone, Debug, Serialize)]
pub struct Food {
    pub x: f32,
    pub y: f32
}