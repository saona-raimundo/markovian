# markovian
Simulation of Markov Processes as stochastic processes.

# Main features

- 

# Changelog

Last version:

- 

For more, see [Changelog](https://github.com/rasa200/markovian/blob/master/Changelog.md).

# To do list

- [ ] Documentation

# Roadmap

## Separate sub and proper stochastic processes

**Goal:** Construct correctly stochastic and sub-stochastic process in different structs.

**Current implementation:** Sub-stochastic process for all structs.

**Options:**

- **Needs:** 
  - Exact transitions

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