# To do

## FiniteMarkovChain

- Documentation
  - Examples
  - Add to the readme, so that crates.io is better presented
  - Compare with [mkv_chain](https://crates.io/crates/mkv_chain)

## Macros

- Learn to do useful macros!

## Organization

- **Modules organization:** different algorithms for simulation
  - Exact
  - Fast
  - Accurate
- Continuous time processes by epsilon-strong simulation
  - Brownian motion

## Abstract probabilities

No longer use only f64 to represent probabilities

Requirements:

- Need to represent cummulative probabilities, to simulate raw_dist!

### Options

### Best

Struct P<T>: and from methods for different T: Copy + Clone + Debug + PartialOrd + Num

- Pros

  - More general than closed01
  - Checked initialization in debug mode (use assert_debug!)

  - Implement any trait we want, e.g. [num_traits](https://docs.rs/num-traits/0.2.12/num_traits/index.html) and some [core::ops](https://doc.rust-lang.org/nightly/core/ops/index.html).

#### Rejected

- [closed01](https://crates.io/crates/closed01): a new type of float with extra methods. 
  - Pros:
    - Add
    - WrappingAdd
    - SaturingAdd
  - Cons:
    - Still only floats
  
- New trait P: to fix the types you can use.
