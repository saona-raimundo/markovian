# To do

- [ ] Abstract probabilities
  - No longer use only f64 to represent probabilities	
  - Options
    - [closed01](https://crates.io/crates/closed01): a new type of float with extra methods. 
      
      - Pros:
        - Add
        - WrappingAdd
        - SaturingAdd
      - Cons:
        - Still only floats
      
    - New trait P: to fix the types you can use.
    
    - Struct P<T>: and from methods for different T: Copy + Clone + Debug + PartialOrd + Num
      
      - Pros
        
        - More general than closed01
        - Checked initialization in debug mode (use assert_debug!)
        
        - Implement any trait we want, e.g. [num_traits](https://docs.rs/num-traits/0.2.12/num_traits/index.html) and some [core::ops](https://doc.rust-lang.org/nightly/core/ops/index.html).
  
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
