// Crates
use preexplorer::prelude::*;

// Traits
use markovian::MarkovChainTrait;

fn main() {
	let init_state: f64 = 0.0;
	let transition = |_| {
		let next_state = rand::random::<f64>() * 2.0 - 1.0;
		vec![(next_state, 1.0)]
	};
	let mut mc = markovian::MarkovChain::new(init_state, &transition);
	mc.next();
	let current_state: f64 = mc.state().clone();
	 
	// current_state is between -1.0 and 1.0 
	assert!(current_state != 0.0);
	assert!(current_state < 1.0 && current_state >= -1.0);

	let samples = 20;
	mc.take(samples)
		.collect::<Vec<f64>>()
		.preexplore()
		.plot("continuous_space")
		.unwrap();

}