# markovian
Simulation of [Stochastic processes](https://en.wikipedia.org/wiki/Stochastic_process).

# Main features

- Easy construction of Markov processes, including:
  - Discrete time
  - Continuous time (exponential clocks)
- Type agnostic

# Changelog

See [Changelog](https://github.com/rasa200/markovian/blob/master/Changelog.md).

# Roadmap

## Change design

**Goal:** Use a more robust design for the creation of random iterators. One that uses extensively the `rand` and `rand_distr` crate.

**Current implementation:** Initial state and transitions are states and functions that give another iterator.

**Options:**

- Ask for `Distribution` trait in both initial state and transition function. 
  - Need to save a `Next_state` variable type to sample the next step without the need of a new allocation.
  - Variables that yield `None`would solve the problem of sub-stochastic random iterators. 
  - Continuous time Markov Chains are more complicated to construct
  - Will need helper macros for ease of construction. 

## Separate sub and proper stochastic processes

**Goal:** Construct correctly stochastic and sub-stochastic process in different structs.

**Current implementation:** Sub-stochastic process for all structs.

**Options:**

- **Needs:** 
  - Exact transitions

## Differentiate Markov Chains in continuous space

**Goal:** Easier and checkable implementation of continuous space markov processes by using randomness from the chain to simulate the next step.

**Current implementation:** Random transition function that leads a vector of one element.

**Options:**

- Needs
  - random generator choice. 

## Enlarge traits

**Goal:** Give more blank implementations and facilitate the implementation of Iterator trait. In particular, the following methods:

- transition(&self) -> &I
- can_move_to(&self, state: T) -> bool
- rate_to(&self, state: T) -> Option<f64>
  CMarkovChainTrait only
- probability_to(&self, state: T) -> Option<f64>
  MarkovChainTrait only

**Current implementation:** None

**Options:** 

# Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a pull request. Note that any contribution submitted for inclusion in the project will be licensed according to the terms of the dual license (MIT and Apache-2.0).