// Crates
use preexplorer::prelude::*;

fn main() {
	let init_state: Vec<i32> = vec![0];
	let transition = |state: Vec<i32>| {
		// New possible states
		let mut right = state.clone();
		right.push(state[state.len() - 1] + 1);
		let mut left = state.clone();
		left.push(state[state.len() - 1] - 1);

		// Some non markovian transtion
		let path_stadistic: i32 = state.iter().sum();
		if path_stadistic.is_positive() {
			vec![
				(right, 1.0 / (path_stadistic.abs() + 1) as f64), 
				(left, 1.0 - 1.0 / (path_stadistic.abs() + 1) as f64), 
	 		]
		} else {
			vec![
				(right, 1.0 - 1.0 / (path_stadistic.abs() + 1) as f64), 
				(left, 1.0 / (path_stadistic.abs() + 1) as f64), 
			]
		}
	};
	let mc = markovian::MarkovChain::new(init_state, &transition);
	
	let samples = 200;
	mc.take(samples)
		.map(|vec_state| vec_state[vec_state.len() - 1])
		.collect::<Vec<i32>>()
		.preexplore()
		.plot("non_markovian")
		.unwrap();
}