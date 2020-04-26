//! Branching process example

use ndarray::Array1;
use markovian::prelude::*;
use markovian::Branching;
use preexplorer::prelude::*;
use rayon::prelude::*;

fn main() {
    // Compute extinction probability for a fixed time
    if false {
        let birth_prob = 0.2;
        let death_prob = 0.1;
        let max_iterations = 150;
        let samples = 100;

        let population_samples = sample_population(birth_prob, death_prob, max_iterations, samples);
        let result = extinction_prob(population_samples, samples);

        println!("Approsimated exitction probability: {}", result);
    }

    // Plot extinction probability over time
    if false {
        let birth_prob = 0.2;
        let death_prob = 0.1;
        let max_iterations = vec![10, 20, 50, 80, 100, 150]; // , 1000];// , 10_000, 100_000];
        let samples = 1_000;

        plot_extinction_prob(birth_prob, death_prob, max_iterations, samples);
    }

    // Plot expected populate over time
    if false {
        let birth_prob = 0.2;
        let death_prob = 0.1;
        let times = (1..20).map(|i| i * 5).collect::<Vec<usize>>();
        let samples = 1_000;

        plot_expected_population(birth_prob, death_prob, times, samples);
    }

    // Ploting exact finite time extinction probabilities
    if false {
        plot_extinction_prob_small_times();
    }

    // Expected population assuming subsistence
    if false {
        let birth_prob = 0.2;
        let death_prob = 0.1;
        let times = (1..15).collect::<Vec<usize>>();

        plot_expected_population_under_subsistence(birth_prob, death_prob, times);
    }

    // Another branching process
    if true {
        const C: f64 = 4.34992;
        const A: f64 = 1.89754;
        const X_MIN: f64 = 0.97;
        const X_MAX: f64 = 1.0;

        let init_state: u32 = 1;
        let density = (0..).map(|i| (i, C / (A + i as f64).powi(3_i32)));

        let branching_process = Branching::new(init_state, density);

        let mut all_generating_functions_approx = Vec::new();
        let approximation_levels = [50, 100, 500, 1_000, 10_000]; //[50, 100, 200, 300, 400];

        for i in 0..approximation_levels.len() {
            let approximation_level: usize = approximation_levels[i];
            let grid: Array1<f64> = ndarray::ArrayBase::linspace(X_MIN, X_MAX, approximation_level);

            let image = grid
                .to_vec()
                .par_iter()
                .map(|&x| branching_process.approx_generating_fun(x, approximation_level) - x)
                .collect::<Vec<f64>>();

            all_generating_functions_approx.push(
                (grid.to_vec(), image)
                    .preexplore()
                    .title(format!("n = {:?}", approximation_level))
                    .to_owned(),
            )
        }
        let approximation_level = approximation_levels[approximation_levels.len() - 1];
        let grid: Array1<f64> = ndarray::ArrayBase::linspace(X_MIN, X_MAX, approximation_level);

        all_generating_functions_approx.push(
            (
                grid.clone().to_vec(),
                grid.iter().map(|_| 0.0).collect::<Vec<f64>>(),
            )
                .preexplore()
                .title("zero")
                .to_owned(),
        );

        pre::process::Comparison::new(all_generating_functions_approx)
            .title("Various approximations for extinction probability")
            .labelx("z")
            .plot("all_extinction_probability_approx")
            .unwrap();
    }
}

fn plot_expected_population_under_subsistence(birth_prob: f64, death_prob: f64, times: Vec<usize>) {
    let subsistence_prob_exact = vec![
        1.0,
        1.0 - 0.100000000000000,
        1.0 - 0.172000000000000,
        1.0 - 0.226316800000000,
        1.0 - 0.268665618792448,
        1.0 - 0.302502176098939,
        1.0 - 0.330053036578176,
        1.0 - 0.352824126995618,
        1.0 - 0.371873861814977,
        1.0 - 0.387969737090721,
        1.0 - 0.401682919343153,
        1.0 - 0.413447877078615,
        1.0 - 0.423601343367193,
        1.0 - 0.432408559977533,
    ];

    let exact: Vec<f64> = times
        .iter()
        .map(|&i| {
            if i < subsistence_prob_exact.len() {
                (1.0 + birth_prob - death_prob).powi(i as i32) / subsistence_prob_exact[i]
            } else {
                (1.0 + birth_prob - death_prob).powi(i as i32) * 2.0
            }
        })
        .collect();

    (&times, &exact)
        .preexplore()
        .title("Expected population under subsistence over time")
        .labelx("time")
        .labely("population")
        .plot("expected_population_under_subsistence")
        .unwrap();

    // Compare with non subsistence expectation
    if false {
        let unconditional_exact: Vec<f64> = times
            .iter()
            .map(|&i| (1.0 + birth_prob - death_prob).powi(i as i32))
            .collect();

        pre::process::Comparison::new(vec![
            (&times, exact)
                .preexplore()
                .title("under subsistence")
                .to_owned(),
            (&times, unconditional_exact)
                .preexplore()
                .title("unconditional")
                .to_owned(),
        ])
        .title("expected population over time")
        .labelx("time")
        .labely("population")
        .plot("extinction_population")
        .unwrap();
    }
}

fn plot_extinction_prob_small_times() {
    vec![
        0.0,
        0.100000000000000,
        0.172000000000000,
        0.226316800000000,
        0.268665618792448,
        0.302502176098939,
        0.330053036578176,
        0.352824126995618,
        0.371873861814977,
        0.387969737090721,
        0.401682919343153,
        0.413447877078615,
        0.423601343367193,
        0.432408559977533,
    ]
    .preexplore()
    .title("Exact extinction probability over time")
    .labelx("time")
    .labely("probability")
    .plot("extinction_prob_exact")
    .unwrap();
}

fn plot_expected_population(birth_prob: f64, death_prob: f64, times: Vec<usize>, samples: usize) {
    let exact: Vec<f64> = times
        .iter()
        .map(|i| (1.0 + birth_prob - death_prob).powi(*i as i32))
        .collect();

    let simulated: Vec<f64> = times
        .iter()
        .map(|i| {
            let pop_sample = sample_population(birth_prob, death_prob, *i, samples);
            pop_sample.iter().sum::<u32>() as f64 / samples as f64
        })
        .collect();

    pre::process::Comparison::new(vec![
        (&times, exact).preexplore().title("exact").to_owned(),
        (&times, simulated)
            .preexplore()
            .title("simulated")
            .to_owned(),
    ])
    .title("expected population over time")
    .labelx("time")
    .labely("population")
    .plot("extinction_population")
    .unwrap();
}

fn plot_extinction_prob(
    birth_prob: f64,
    death_prob: f64,
    max_iterations: Vec<usize>,
    samples: usize,
) {
    let simulations = (
        max_iterations.clone(),
        max_iterations
            .clone()
            .into_iter()
            .map(|max_iter| {
                let population_samples =
                    sample_population(birth_prob, death_prob, max_iter, samples);
                extinction_prob(population_samples, samples)
            })
            .collect::<Vec<f64>>(),
    )
        .preexplore()
        .title("simulations")
        .to_owned();

    let limit = (
        max_iterations.clone(),
        max_iterations
            .clone()
            .into_iter()
            .map(|_| f64::min(1.0, death_prob / birth_prob))
            .collect::<Vec<f64>>(),
    )
        .preexplore()
        .title("limit")
        .to_owned();

    pre::process::Comparison::new(vec![simulations, limit])
        .title("Finite time extinction probability")
        .labelx("time horizon")
        .labely("probability")
        .plot("extinction_prob_approx")
        .unwrap();
}

/// Counts the fraction of the trayectories that are not extinct.
/// Formally, it gives
/// \PP_1( X_{max_iterations} = 0),
/// approximated by samples number of simuations.
fn extinction_prob(population_samples: Vec<u32>, samples: usize) -> f64 {
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
            let density = vec![
                (0, death_prob),
                (2, birth_prob),
                (1, 1.0 - birth_prob - death_prob),
            ];
            let mut branching_process = Branching::new(init_state, density);

            branching_process.nth(iterations).unwrap()
        })
        .collect::<Vec<u32>>()
}
