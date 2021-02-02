# markovian
[![Download](https://img.shields.io/crates/d/markovian)](https://crates.io/crates/markovian)
[![License](https://img.shields.io/crates/l/markovian)](https://github.com/saona-raimundo/markovian)
[![Docs](https://docs.rs/markovian/badge.svg)](https://docs.rs/markovian/)
[![Crate](https://img.shields.io/crates/v/markovian.svg)](https://crates.io/crates/markovian)

Simulation of [Stochastic processes](https://en.wikipedia.org/wiki/Stochastic_process).

# Goal
Serve as an extension of the [rand crate](https://crates.io/crates/rand) for sub-stochastic markovian processes.

# Main features

- Easy construction of Markov processes, including:
  - Finite/Infinite states
  - Discrete/continuous time
- Type agnostic
- Zero-cost abstraction

# Changelog

See [Changelog](https://github.com/saona-raimundo/markovian/blob/master/Changelog.md).

# Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a pull request. Note that any contribution submitted for inclusion in the project will be licensed according to the terms of the dual license (MIT and Apache-2.0).

# Related crates

## Statistics

Crates that serve for statistics (estimation, model fitting or prediction):

- [bio::stats](https://crates.io/crates/bio) - Mathematical and statistical tools for bioinformatics, including Hidden Markov Chains.

- [statrs](https://crates.io/crates/statrs) - Statistical utilities for Rust scientific computing.

- [mkv_chain](https://crates.io/crates/mkv_chain) - Finite state Markov Chains

## Simulation

[rand](https://crates.io/crates/rand) - Random number generation.

[noise](https://crates.io/crates/noise) - A procedural noise generation library for Rust.

[rv](https://crates.io/crates/rv) - Random variables (RV) for rust.

[sim](https://crates.io/crates/sim) - "Sim" or "SimRS" is a discrete event simulation package that facilitates Rust- and npm-based simulation products and projects.