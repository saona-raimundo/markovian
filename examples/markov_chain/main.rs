use markovian::traits::MarkovChainTrait;
use markovian::{Branching, MarkovChain};

fn main() {
    if false {
        markov();
    }

    if true {
        branching();
    }
}

fn markov() {
    // Finite state-space Markov Chain

    let init_state: i32 = 0;
    // fn transition(state: i32) -> Vec<(i32, f32)> {
    // 	vec![
    // 		(state + 1, 0.5),
    // 		(state - 1, 0.5),
    // 	]
    // }
    let transition = |state: i32| vec![(state + 1, 0.5), (state - 1, 0.5)];

    let mut mc = MarkovChain::new(init_state, &transition);

    println!("{:?}", mc.state());
    println!("{:?}", mc.next());
    println!("{:?}", mc.next());
    println!("{:?}", mc.next());

    // Infinite state-space Markov Chain

    let init_state: i32 = 0;
    let transition2 = |state: i32| {
        (1..)
            .map(move |s| (state + s as i32, (2.0_f32).powf(-s as f32)))
            .map(|(s, p)| (s as i32, p as f64))
    };

    let mut mc = MarkovChain::new(init_state, transition2);

    println!("{:?}", mc.state());
    println!("{:?}", mc.next());
    println!("{:?}", mc.next());
    println!("{:?}", mc.next());
    println!("{:?}", mc.next());
    println!("{:?}", mc.next());
    println!("{:?}", mc.next());
}

fn branching() {
    // Finite state-space Markov Chain

    let init_state: u32 = 1;
    let density_map = |state: u32| {
        if state == 0 {
            0.5
        } else if state == 2 {
            0.5
        } else {
            0.0
        }
    };
    let density = (0..).map(|state| (state, density_map(state)));

    let mut br = Branching::new(init_state, density);

    println!("{:?}", br.state());
    println!("{:?}", br.next());
    println!("{:?}", br.next());
    println!("{:?}", br.next());

    // Plotting
    use preexplorer::prelude::*;

    (0..100)
        .map(|_| br.next().unwrap())
        .collect::<Vec<u32>>()
        .preexplore()
        .plot("test")
        .unwrap();
}
