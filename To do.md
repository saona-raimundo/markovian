# To do

- [ ] Abstract probabilities
  - No longer use only f64, but a type based on closed01:
    - https://crates.io/crates/closed01 
  
- Different algorithms for simulation
  - [ ] fractional brownian motion
    - [ ] Exact
    - [ ] Approximate
  - [ ] default + fast / accurate
    - [ ] There is an epsilon-strong simulation method for brownian motion!! (accurate)
  
- Interoperability

  - FiniteMarkovChain
    - [ ] From<>
      - [ ] (T, FnMut(usize) -> T, R)
    - [ ] TryFrom<>
      - [ ] (T, Array1<T>, Array2<T>, R)
      - [ ] FnMut(usize) -> 
      - [ ] ... 
  - Branching, MarkovChain
    - [ ] From<>
      - [ ] ... 
    - [ ] TryFrom<>
      - [ ] ...  
