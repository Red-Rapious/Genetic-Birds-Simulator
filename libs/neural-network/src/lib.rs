use rand::Rng;

pub mod test;

/// The structure of a neural network, capable of propagating an input through layers.
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    /// Given `inputs`, computes the output of the neural network.
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    /// Initializes a new neural network with random layers.
    pub fn random(layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layers| {
                Layer::random(layers[0].neurons, layers[1].neurons)
            })
            .collect();

        Self { layers }
    }
} 

struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    /// Propagates the output of the previous layer through the layer.
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }

    /// Initializes layer of random neurons.
    pub fn random(input_neurons: usize, output_neurons: usize) -> Self {
        let neurons = (0..output_neurons)
            .map(|_| Neuron::random(&mut rand::thread_rng(), input_neurons))
            .collect();

        Self { neurons }
    }
}

struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Neuron {
    /// Computes the activation of the neuron, given the inputs.
    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());

        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (output + self.bias).max(0.0)
    }

    /// Initializes neuron with randoms weights and a random bias.
    pub fn random(rng: &mut dyn rand::RngCore, output_size: usize) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..output_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }
} 

/// A structure containing the number of neurons of a layer.
pub struct LayerTopology {
    pub neurons: usize
}