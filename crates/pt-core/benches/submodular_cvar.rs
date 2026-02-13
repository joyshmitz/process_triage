//! Criterion benchmarks for submodular probe selection and CVaR decision paths.
//!
//! Benchmarks `greedy_select_k`, `greedy_select_with_budget`, `coverage_utility`,
//! `compute_cvar`, and `decide_with_cvar` — risk-sensitive decision hotpaths.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use pt_core::config::Policy;
use pt_core::decision::cvar::{compute_cvar, decide_with_cvar};
use pt_core::decision::expected_loss::Action;
use pt_core::decision::submodular::{
    coverage_utility, greedy_select_k, greedy_select_with_budget, FeatureKey, ProbeProfile,
};
use pt_core::decision::voi::ProbeType;
use pt_core::inference::ClassScores;
use std::collections::HashMap;

fn make_feature_weights(n: usize) -> HashMap<FeatureKey, f64> {
    (0..n)
        .map(|i| {
            (
                FeatureKey::new(format!("f_{}", i)),
                1.0 + (i % 5) as f64 * 0.3,
            )
        })
        .collect()
}

fn make_probe_profiles(n_probes: usize, n_features: usize) -> Vec<ProbeProfile> {
    let probes = ProbeType::ALL;
    (0..n_probes)
        .map(|i| {
            let probe = probes[i % probes.len()];
            let cost = 0.1 + (i % 10) as f64 * 0.05;
            let features: Vec<FeatureKey> = (0..3)
                .map(|j| FeatureKey::new(format!("f_{}", (i + j) % n_features)))
                .collect();
            ProbeProfile::new(probe, cost, features)
        })
        .collect()
}

fn bench_greedy_select_k(c: &mut Criterion) {
    let mut group = c.benchmark_group("submodular/greedy_select_k");

    for (n_probes, n_features) in [(10, 15), (30, 30), (50, 40)] {
        let probes = make_probe_profiles(n_probes, n_features);
        let weights = make_feature_weights(n_features);

        for k in [3, 5] {
            group.bench_with_input(
                BenchmarkId::new(format!("n{}_k{}", n_probes, k), n_probes),
                &(&probes, &weights),
                |b, &(probes, weights)| {
                    b.iter(|| {
                        let result = greedy_select_k(black_box(probes), black_box(weights), k);
                        black_box(result.total_utility);
                    })
                },
            );
        }
    }

    group.finish();
}

fn bench_greedy_select_with_budget(c: &mut Criterion) {
    let mut group = c.benchmark_group("submodular/greedy_select_budget");

    for (n_probes, n_features, budget) in [(20, 20, 0.5), (50, 40, 1.0), (50, 40, 0.3)] {
        let probes = make_probe_profiles(n_probes, n_features);
        let weights = make_feature_weights(n_features);

        group.bench_with_input(
            BenchmarkId::new(format!("n{}_b{:.1}", n_probes, budget), n_probes),
            &(&probes, &weights),
            |b, &(probes, weights)| {
                b.iter(|| {
                    let result =
                        greedy_select_with_budget(black_box(probes), black_box(weights), budget);
                    black_box(result.total_utility);
                })
            },
        );
    }

    group.finish();
}

fn bench_coverage_utility(c: &mut Criterion) {
    let mut group = c.benchmark_group("submodular/coverage_utility");

    for (n_probes, n_features) in [(5, 10), (20, 30), (50, 40)] {
        let probes = make_probe_profiles(n_probes, n_features);
        let weights = make_feature_weights(n_features);

        group.bench_with_input(
            BenchmarkId::new("batch", n_probes),
            &(&probes, &weights),
            |b, &(probes, weights)| {
                b.iter(|| {
                    black_box(coverage_utility(black_box(probes), black_box(weights)));
                })
            },
        );
    }

    group.finish();
}

fn abandoned_posterior() -> ClassScores {
    ClassScores {
        useful: 0.05,
        useful_bad: 0.03,
        abandoned: 0.85,
        zombie: 0.07,
    }
}

fn ambiguous_posterior() -> ClassScores {
    ClassScores {
        useful: 0.30,
        useful_bad: 0.15,
        abandoned: 0.40,
        zombie: 0.15,
    }
}

fn bench_compute_cvar(c: &mut Criterion) {
    let policy = Policy::default();

    let mut group = c.benchmark_group("cvar/compute_cvar");

    for (name, posterior) in [
        ("abandoned", abandoned_posterior()),
        ("ambiguous", ambiguous_posterior()),
    ] {
        for alpha in [0.90, 0.95] {
            group.bench_with_input(
                BenchmarkId::new(format!("{}_a{:.2}", name, alpha), name),
                &posterior,
                |b, post| {
                    b.iter(|| {
                        let result = compute_cvar(
                            Action::Kill,
                            black_box(post),
                            black_box(&policy.loss_matrix),
                            alpha,
                        );
                        black_box(result.unwrap().cvar);
                    })
                },
            );
        }
    }

    group.finish();
}

fn bench_decide_with_cvar(c: &mut Criterion) {
    let policy = Policy::default();
    let feasible = vec![
        Action::Keep,
        Action::Renice,
        Action::Pause,
        Action::Freeze,
        Action::Throttle,
        Action::Quarantine,
        Action::Restart,
        Action::Kill,
    ];

    let mut group = c.benchmark_group("cvar/decide_with_cvar");

    for (name, posterior) in [
        ("abandoned", abandoned_posterior()),
        ("ambiguous", ambiguous_posterior()),
    ] {
        group.bench_with_input(BenchmarkId::new("full", name), &posterior, |b, post| {
            b.iter(|| {
                let result = decide_with_cvar(
                    black_box(post),
                    black_box(&policy),
                    black_box(&feasible),
                    0.95,
                    Action::Kill,
                    "benchmark",
                );
                black_box(result.unwrap().risk_adjusted_action);
            })
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_greedy_select_k,
    bench_greedy_select_with_budget,
    bench_coverage_utility,
    bench_compute_cvar,
    bench_decide_with_cvar
);
criterion_main!(benches);
