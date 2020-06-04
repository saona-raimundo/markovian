// use criterion::*;
use rand::Rng;
use rand::prelude::thread_rng;
use rand_distr::StandardNormal;
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};


// To bench
fn brownian_motion(steps: usize) -> Vec<f64> {
	let state = 0.;
	let transition = |_: &f64| StandardNormal;
	let rng = thread_rng();

	let mc = markovian::MarkovChain::new(state, transition, rng);

	mc.take(steps).collect()
}

// To compare with
fn direct_brownian_motion(steps: usize) -> Vec<f64> {
	let mut rng = thread_rng();
	(0..steps).map(|_| rng.sample(StandardNormal)).collect()
}


fn bench_brownian(c: &mut Criterion) {
    let mut group = c.benchmark_group("Brownian Motion");
    for i in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("Markovian", i), i, 
            |b, i| b.iter(|| brownian_motion(*i)));
        group.bench_with_input(BenchmarkId::new("Direct", i), i, 
            |b, i| b.iter(|| direct_brownian_motion(*i)));
    }
    group.finish();
}

criterion_group!(benches, bench_brownian);
criterion_main!(benches);