use markovian::{MarkovChain, prelude::*};
use rand::prelude::*;

fn main() {
    // Finite state-space Markov Chain

    let init_state: i32 = 0;

    let transition = |state: &i32| Raw::new(vec![(0.5, state + 1), (0.5, state - 1)]);

    let mut mc = MarkovChain::new(init_state, &transition, thread_rng());

    println!("{:?}", mc.state());
    println!("{:?}", mc.next());
    println!("{:?}", mc.next());
    println!("{:?}", mc.next());

    // Infinite state-space Markov Chain

    let init_state: i32 = 0;
    let transition2 = |state: &i32| {
        let v: Vec<(f64, i32)> = (1..)
            .map(move |s| ((2.0_f32).powf(-s as f32), state + s as i32))
            .map(|(p, s)| (p as f64, s as i32))
            .collect();
        Raw::new(v)
    };

    let mut mc = MarkovChain::new(init_state, transition2, thread_rng());

    println!("{:?}", mc.state());
    println!("{:?}", mc.next());
    println!("{:?}", mc.next());
    println!("{:?}", mc.next());
    println!("{:?}", mc.next());
    println!("{:?}", mc.next());
    println!("{:?}", mc.next());
}