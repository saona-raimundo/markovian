//! Poisson process example.
//! 
//! We compute and plot a sample path of the Poisson process.

use rand::prelude::*;
use markovian::prelude::*;
use preexplorer::prelude::*;
use itertools_num::ItertoolsNum;

const STEPS: usize = 10;
const LAMBDA: f64 = 2.;

fn main() {
    // Monte Carlo
    let (times, values): (Vec<_>, Vec<_>) = Poisson::<f64, usize, _>::new(LAMBDA, thread_rng())
        .unwrap()
        .trajectory()
        .take(STEPS)
        .unzip();
    // Plotting
    (times.iter().cumsum::<f64>(), values).preexplore()
        .set_title(format!("Poisson process, lambda = {}", LAMBDA))
        .set_xlabel("time")
        .set_ylabel("state")
        .set_style("_|")
        .plot("poisson")
        .unwrap();
}