//! Branching process example.
//! 
//! We compute and plot against time the extinction probability of the Branching process.
//! We use Monte Carlo to approximate the real extinction porbability..

use rand::prelude::*;
use markovian::prelude::*;
use markovian::Branching;
use preexplorer::prelude::*;
use rayon::prelude::*;

const BIRTH: f64 = 0.2;
const DEATH: f64 = 0.1;
const TIME: usize = 100;
const SAMPLES: usize = 100;

fn main() {
    // Monte Carlo
    let monte_carlo_approx: Vec<f64> = (0..TIME).map(|time| {
            // Simulate a sample from Branchng ´processes at a fixed time
            let simulations = sample_population(BIRTH, DEATH, time, SAMPLES);
            // Compute extinction probability for this time
            extinction_prob(simulations)
        }).collect();
    // Plotting
    monte_carlo_approx.preexplore()
        .set_title("Approximate extinction probability")
        .set_xlabel("time")
        .set_ylabel("extinction_probability")
        .plot("extinction")
        .unwrap();
}

/// Counts the fraction of the trayectories that are not extinct.
/// Formally, it gives
/// \PP_1( X_{max_iterations} = 0),
/// approximated by samples number of simuations.
fn extinction_prob(population_samples: Vec<u32>) -> f64 {
    let samples = population_samples.len();
    population_samples
        .into_iter()
        .filter(|&x| x == 0_u32)
        .count() as f64
        / samples as f64
}

/// Samples population starting from 1 until time iterations.
fn sample_population(
    birth_prob: f64,
    death_prob: f64,
    iterations: usize,
    samples: usize,
) -> Vec<u32> {
    (0..samples)
        .collect::<Vec<usize>>()
        .par_iter()
        .map(|_| {
            let init_state: u32 = 1;
            let density = raw_dist![
                (death_prob, 0),
                (birth_prob, 2),
                (1.0 - birth_prob - death_prob, 1)
            ];
            let mut branching_process = Branching::new(init_state, density, thread_rng());

            branching_process.nth(iterations).unwrap()
        })
        .collect()
}
