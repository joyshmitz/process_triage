//! Criterion benchmarks for composite-hypothesis testing (SPRT/GLR) in `pt-core`.
//!
//! Benchmarks `mixture_sprt_bernoulli`, `mixture_sprt_multiclass`,
//! `glr_bernoulli`, `MixtureSprtState::update`, and `needs_composite_test`
//! — sequential testing hotpaths used during evidence accumulation.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use pt_core::decision::composite_test::{
    glr_bernoulli, mixture_sprt_bernoulli, mixture_sprt_multiclass, needs_composite_test,
    GlrConfig, MixtureSprtConfig, MixtureSprtState,
};

fn bench_mixture_sprt_bernoulli(c: &mut Criterion) {
    let config = MixtureSprtConfig::default();

    let mut group = c.benchmark_group("composite/sprt_bernoulli");

    // Short sequence: 20 observations (early decision)
    let short_obs: Vec<bool> = (0..20).map(|i| i % 3 != 0).collect();
    group.bench_with_input(BenchmarkId::new("sequence", 20), &short_obs, |b, obs| {
        b.iter(|| {
            let result = mixture_sprt_bernoulli(black_box(obs), 0.5, 2.0, 2.0, black_box(&config));
            black_box(result.unwrap().crossed_upper);
        })
    });

    // Medium sequence: 100 observations
    let med_obs: Vec<bool> = (0..100).map(|i| i % 4 != 0).collect();
    group.bench_with_input(BenchmarkId::new("sequence", 100), &med_obs, |b, obs| {
        b.iter(|| {
            let result = mixture_sprt_bernoulli(black_box(obs), 0.5, 2.0, 2.0, black_box(&config));
            black_box(result.unwrap().crossed_upper);
        })
    });

    // Long sequence: 500 observations
    let long_obs: Vec<bool> = (0..500).map(|i| i % 5 != 0).collect();
    group.bench_with_input(BenchmarkId::new("sequence", 500), &long_obs, |b, obs| {
        b.iter(|| {
            let result = mixture_sprt_bernoulli(black_box(obs), 0.5, 2.0, 2.0, black_box(&config));
            black_box(result.unwrap().crossed_upper);
        })
    });

    group.finish();
}

fn bench_mixture_sprt_multiclass(c: &mut Criterion) {
    let mut group = c.benchmark_group("composite/sprt_multiclass");

    // Typical 4-class log-likelihoods
    let clear_signal: [f64; 4] = [-0.5, -2.0, -3.0, -4.0]; // strong useful signal
    let ambiguous: [f64; 4] = [-1.0, -1.2, -1.1, -1.3]; // unclear

    for (name, log_liks) in [("clear_signal", clear_signal), ("ambiguous", ambiguous)] {
        group.bench_with_input(BenchmarkId::new("single", name), &log_liks, |b, liks| {
            b.iter(|| {
                let log_bf = mixture_sprt_multiclass(black_box(liks), 0.5, 0.3, 0.5, 0.5);
                black_box(log_bf.unwrap());
            })
        });
    }

    group.finish();
}

fn bench_glr_bernoulli(c: &mut Criterion) {
    let config = GlrConfig::default();

    let mut group = c.benchmark_group("composite/glr_bernoulli");

    for (name, successes, n) in [
        ("small_n", 15, 20),
        ("medium_n", 70, 100),
        ("large_n", 350, 500),
    ] {
        group.bench_with_input(
            BenchmarkId::new("batch", name),
            &(successes, n),
            |b, &(s, n)| {
                b.iter(|| {
                    let result = glr_bernoulli(black_box(s), black_box(n), 0.5, black_box(&config));
                    black_box(result.unwrap().exceeds_threshold);
                })
            },
        );
    }

    group.finish();
}

fn bench_sprt_state_update(c: &mut Criterion) {
    let mut group = c.benchmark_group("composite/sprt_state_update");

    // Sequential update: 100 observations streamed one at a time
    group.bench_function("sequential_100", |b| {
        b.iter(|| {
            let mut state = MixtureSprtState::default_config();
            for i in 0..100u32 {
                let ll1 = -0.5 - (i as f64 * 0.01);
                let ll0 = -1.0;
                if state.update(black_box(ll1), black_box(ll0)) {
                    break;
                }
            }
            black_box(state.e_value());
        })
    });

    // Batch update: 100 observations at once
    let lls_h1: Vec<f64> = (0..100).map(|i| -0.5 - (i as f64 * 0.01)).collect();
    let lls_h0: Vec<f64> = vec![-1.0; 100];
    group.bench_with_input(
        BenchmarkId::new("batch", 100),
        &(&lls_h1, &lls_h0),
        |b, &(h1, h0)| {
            b.iter(|| {
                let mut state = MixtureSprtState::default_config();
                state.update_batch(black_box(h1), black_box(h0));
                black_box(state.e_value());
            })
        },
    );

    group.finish();
}

fn bench_needs_composite_test(c: &mut Criterion) {
    let mut group = c.benchmark_group("composite/needs_composite_test");

    // Typical inputs
    for (name, log_bf, entropy, uncertainty) in [
        ("confident", 5.0, 0.3, 0.1),
        ("uncertain", 1.5, 1.8, 0.6),
        ("borderline", 2.5, 1.0, 0.3),
    ] {
        group.bench_with_input(
            BenchmarkId::new("check", name),
            &(log_bf, entropy, uncertainty),
            |b, &(bf, ent, unc)| {
                b.iter(|| {
                    black_box(needs_composite_test(
                        black_box(bf),
                        black_box(ent),
                        black_box(unc),
                    ));
                })
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_mixture_sprt_bernoulli,
    bench_mixture_sprt_multiclass,
    bench_glr_bernoulli,
    bench_sprt_state_update,
    bench_needs_composite_test
);
criterion_main!(benches);
