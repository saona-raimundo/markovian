// Crates
use preexplorer::prelude::*;

fn main() {
	let init_state: (usize, i32) = (0, 0);
	let transition = |(time, state): (usize, i32)| vec![
		((time + 1, state + 1), 0.6 - 1.0 / (time + 1) as f64), 
		((time + 1, state - 1), 0.4 + 1.0 / (time + 1) as f64)
	];
	let mc = markovian::MarkovChain::new(init_state, &transition);

	let samples = 10;
	mc.take(samples).map(|(_, state)| state)
		.collect::<Vec<i32>>()
		.preexplore()
		.plot("time_dependent")
		.unwrap();
}