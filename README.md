# Genetic-Birds-Simulator
A simulation of birds using neural networks and a genetic algorithm.

## About
### Motivation
This project is a kind of sequel to [MLP-Digits-Recognition](), an implementation of a neural network from scratch. This time, I used [this great tutorial](https://pwy.io/posts/learning-to-fly-pt1/) as a starting point, to train the neural network with a genetic algorithm, instead of backpropagation.

### Technical description
- The [`neural-network`](libs/neural-network/src/lib.rs) library contains an implementation of a simple, non-optimized FFNN (Feed-Forward Neural Network).
- The [`genetic-algorithm`](libs/genetic-algorithm/src/lib.rs) library implements a genetic algorithm, which selects, crossovers, and mutates individuals.


## Running and debugging
This project's workspace is structured in `libs`. There is no main app available yet. Nevertheless, tests can be executed by running:
```console
$ cargo test
```

## License
This work is licensed under the [CC-BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/) license.