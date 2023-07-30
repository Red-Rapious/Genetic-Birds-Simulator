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
}

impl From<&sim::World> for World {
    fn from(world: &sim::World) -> Self {
        let birds = world
            .birds()
            .iter()
            .map(Bird::from)
            .collect();

        Self { birds }
    }
}

impl From<&sim::Bird> for Bird {
    fn from(bird: &sim::Bird) -> Self {
        Self {
            x: bird.position().x,
            y: bird.position().y
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct World {
    pub birds: Vec<Bird>
}

#[derive(Clone, Debug, Serialize)]
pub struct Bird {
    pub x: f32,
    pub y: f32
}