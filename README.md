# markovian
Simulation of Markov Processes as stochastic processes.

# Main features

- 

# Changelog

Last version:

- 0.1.2
  - Documentation added.
  - Fixed bug in method markovian::traits::BranchingTrait::approx_generating_fun.

For more, see [Changelog](https://github.com/rasa200/markovian/blob/master/Changelog.md).

# To do list

- [ ] Tests

# Roadmap

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

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a pull request. Note that any contribution submitted for inclusion in the project will be licensed according to the terms of the dual license (MIT and Apache-2.0).