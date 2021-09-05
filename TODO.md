# To do

Re-factor to be only stochastic processes and not sub-stochastic ones.

## Derive Macros

- State
  - Needs to read where_clauses to write them again in the implementation of State trait
  - Needs to be given (input) the type of Item (...which is always `T`). 
  - Nice error message!
- StateIterator
  - ??

Once implemented, clean the code for all the structs. 

## FiniteMarkovChain

- Construction
  - Checks of state_space!
    - What is the condition you want
    - state_space must be different elements
  - set_state_space
    - state_space must be different elements
  - From Vec<T>
    - From a sample, 
      - Consider the initial state
      - Discover the state space
      - Count each transition
      - Construct the chain 
      - Add to lib module documentation
- Move from panicking to errors
  - Create errors

## ContFiniteMarkovChain

- The same
- has_absorbing_state(&self) -> bool

## Macros

- Learn to do useful macros!

## Organization

- **Modules organization:** different algorithms for simulation
  - Exact
  - Fast
    - Sample speed
  - Accurate
  - epsilon-strong
    - Brownian motion

## Jump processes

In a few cases, the marginal distributions of the increments have a simple form such as a **gamma distribution**, a **stable distribution**, or an **inverse Gaussian distribution** so that special methods for such distributions allow to generate discrete skeletons.  

- [x] Possion process
- [ ] Levy process



## Abstract probabilities

No longer use only f64 to represent probabilities

Requirements:

- Need to represent cummulative probabilities, to simulate raw_dist!

### Options

### Best

`Unit` wrapper from `nalgebra`!

https://docs.rs/nalgebra/0.26.2/nalgebra/base/struct.Unit.html

Idea: `Distribution<[T; N]>` wrapper for vectors, with constant generics!

- Implementation: prob-num crate!

### Good

Struct P<T>: and from methods for different T: Copy + Clone + Debug + PartialOrd + Num

- Pros

  - More general than closed01
  - Checked initialization in debug mode (use assert_debug!)
- Implement any trait we want, e.g. [num_traits](https://docs.rs/num-traits/0.2.12/num_traits/index.html) and some [core::ops](https://doc.rust-lang.org/nightly/core/ops/index.html).
  - Accept unums or posits, e.g. [softposit](https://crates.io/crates/softposit) (best implementation of the best rivel to floating point!)
    - [Visualization](https://cse512-19s.github.io/FP-Well-Rounded/)
    - [Paper](http://www.johngustafson.net/pdfs/BeatingFloatingPoint.pdf)

#### Rejected

- [closed01](https://crates.io/crates/closed01): a new type of float with extra methods. 
  - Pros:
    - Add
    - WrappingAdd
    - SaturingAdd
  - Cons:
    - Still only floats
  
- New trait P: to fix the types you can use.