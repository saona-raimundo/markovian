# markovian
Simulation of Markov Processes as stochastic processes.

# Main features

- Easy construction of Markov processes, including:
  - Discrete time
  - Continuous time (exponential times)
- Type agnostic

# Changelog

Last version:

- 0.1.3
  - Documentation added
  - Re-export traits module to crate level

For more, see [Changelog](https://github.com/rasa200/markovian/blob/master/Changelog.md).

# To do list

- [ ] FiniteMarkovChain
  - [ ] From<>
    - [ ] (T, FnMut(usize) -> T)
  - [ ] TryFrom<>
    - [ ] (T, Array1<T>, Array2<T>)
    - [ ] FnMut(usize) -> 
    - [ ] ... 
- [ ] Branching
  - [ ] From<>
    - [ ] ... 
  - [ ] TryFrom<>
    - [ ] ... 
  - [ ] Change the trait Branching to simply implement the functions?
- [ ] Add `#[inline]` for pub methods as done in the `rand` crate. 
- [ ] Document 
  - [ ] Assumptions to create a process (probability distribution input)
  - [ ] main_traits
- [ ] Add `extern crate crate_name;` in examples for explicit dependences. 
- [ ] Tests
- [ ] Add links in documentation to easy access of (from lib level documentation)
  - [ ] MarkovChain
  - [ ] CMarkovChain
  - [ ] Branching

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

## Implement Distribution

**Goal:** Random processes are also source of random transitions, therefore, we should be able to sample transitions. 

**Current implementation:** None

**Options:**

- rand_distr::Distribution 
- [rand](https://docs.rs/rand/0.7.3/rand/index.html)::[distributions](https://docs.rs/rand/0.7.3/rand/distributions/index.html)::[Distribution](https://docs.rs/rand/0.7.3/rand/distributions/trait.Distribution.html) 

## Differentiate Markov Chains in continuous space

**Goal:** Easier and checkable implementation of continuous space markov processes by using randomness from the chain to simulate the next step.

**Current implementation:** Random transition function that leads a vector of one element.

**Options:**

- Needs
  - random generator choice. 

## Sample trajectory

**Goal:** Random processes are also source of random trajectories. Therefore, we should be able to sample them.

**Current implementation:** None

**Options:**

- method sample_trajectory
  - sample_trajectory_iter
    as in rand_distr::Distribution

## Random generator choice

**Goal:** Include random generator to the construction step.

**Current implementation:** New standard sampler for each step simulation. 

**Options:**

- rand

## Exact transitions

**Goal:** Integration with some crate for creation of a correct (sub-)distribution for each step. 

**Current implementation:** f64 for probabilities and there is no correctness check. 

**Options:**

- Rational numbers
- statrs
- rand_distr
- probability

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