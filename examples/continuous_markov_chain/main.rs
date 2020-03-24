use markovian::traits::CMarkovChainTrait;
use markovian::CMarkovChain;
use preexplorer::prelude::*;

fn main() {
    // Finite state-space Markov Chain

    let init_state: i32 = 0;
    // fn transition(state: i32) -> Vec<(i32, f32)> {
    // 	vec![
    // 		(state + 1, 1.),
    // 		(state - 1, 2.),
    // 	]
    // }
    let transition = |state: i32| vec![(state + 1, 1.), (state - 1, 1.)];

    let mc = CMarkovChain::new(init_state, Box::new(transition));

    assert_eq!(&init_state, mc.state());

    let iterations = 50;
    let mut trajectory = Vec::new();

    for (time_step, value) in mc.take(iterations) {
        println!("{:?} \n{}", time_step, value);
        trajectory.push(value)
    }

    trajectory.iter().preexplore().plot("1").unwrap();
}
