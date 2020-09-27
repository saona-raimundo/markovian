use markovian::{FiniteMarkovChain, prelude::*};
use rand::prelude::*;

fn main() {

    let state_index = 0;
    let transition_matrix = ndarray::array![
        [1, 2, 3],
        [1, 0, 0],
        [0, 1, 1],
    ];
    let state_space = vec!['a', 'b', 'c'];
    let rng = thread_rng();

    let mut mc = FiniteMarkovChain::from((state_index, transition_matrix, state_space, rng));

    println!("{:?}", mc.state());
    println!("{:?}", mc.next());
    println!("{:?}", mc.next());
    println!("{:?}", mc.next());
}