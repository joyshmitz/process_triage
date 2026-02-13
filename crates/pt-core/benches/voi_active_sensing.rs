//! Criterion benchmarks for the VOI and active-sensing decision paths in `pt-core`.
//!
//! Benchmarks `compute_voi`, `select_probe_by_information_gain`, and
//! `allocate_probes` — the hotpaths used to decide whether to gather more
//! evidence or act immediately on a process candidate.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use pt_core::config::Policy;
use pt_core::decision::active_sensing::{ActiveSensingPolicy, ProbeBudget, ProbeCandidate};
use pt_core::decision::expected_loss::ActionFeasibility;
use pt_core::decision::voi::{ProbeCostModel, ProbeType, VoiAnalysis};
use pt_core::decision::{allocate_probes, compute_voi, select_probe_by_information_gain};
use pt_core::inference::ClassScores;

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

fn confident_posterior() -> ClassScores {
    ClassScores {
        useful: 0.95,
        useful_bad: 0.02,
        abandoned: 0.02,
        zombie: 0.01,
    }
}

fn bench_compute_voi(c: &mut Criterion) {
    let policy = Policy::default();
    let feasibility = ActionFeasibility::allow_all();
    let cost_model = ProbeCostModel::default();

    let mut group = c.benchmark_group("decision/compute_voi");

    for (name, posterior) in [
        ("abandoned", abandoned_posterior()),
        ("ambiguous", ambiguous_posterior()),
        ("confident", confident_posterior()),
    ] {
        group.bench_with_input(
            BenchmarkId::new("all_probes", name),
            &posterior,
            |b, post| {
                b.iter(|| {
                    let result: Result<VoiAnalysis, _> = compute_voi(
                        black_box(post),
                        black_box(&policy),
                        &feasibility,
                        &cost_model,
                        None,
                    );
                    black_box(result.unwrap().act_now);
                })
            },
        );
    }

    // Subset: only cheap probes
    let cheap_probes = [ProbeType::QuickScan, ProbeType::CgroupInspect];
    group.bench_with_input(
        BenchmarkId::new("cheap_probes", "ambiguous"),
        &ambiguous_posterior(),
        |b, post| {
            b.iter(|| {
                let result: Result<VoiAnalysis, _> = compute_voi(
                    black_box(post),
                    black_box(&policy),
                    &feasibility,
                    &cost_model,
                    Some(&cheap_probes),
                );
                black_box(result.unwrap().act_now);
            })
        },
    );

    group.finish();
}

fn bench_select_probe_by_information_gain(c: &mut Criterion) {
    let cost_model = ProbeCostModel::default();

    let mut group = c.benchmark_group("decision/select_probe_info_gain");

    for (name, posterior) in [
        ("abandoned", abandoned_posterior()),
        ("ambiguous", ambiguous_posterior()),
        ("confident", confident_posterior()),
    ] {
        group.bench_with_input(
            BenchmarkId::new("all_probes", name),
            &posterior,
            |b, post| {
                b.iter(|| {
                    let result: Option<ProbeType> = select_probe_by_information_gain(
                        black_box(post),
                        black_box(&cost_model),
                        None,
                    );
                    black_box(result);
                })
            },
        );
    }

    group.finish();
}

fn bench_allocate_probes(c: &mut Criterion) {
    let policy = Policy::default();
    let cost_model = ProbeCostModel::default();
    let selection_policy = ActiveSensingPolicy::default();

    let mut group = c.benchmark_group("decision/allocate_probes");

    // Varied posteriors for N candidates.
    let make_candidates = |n: usize| -> Vec<ProbeCandidate> {
        (0..n)
            .map(|i| {
                let useful = (10 + (i % 60)) as f64 / 100.0;
                let useful_bad = ((i % 15) + 1) as f64 / 100.0;
                let abandoned = (85.0 - useful * 100.0 - useful_bad * 100.0).max(5.0) / 100.0;
                let zombie = (1.0 - useful - useful_bad - abandoned).max(0.01);

                ProbeCandidate::new(
                    format!("pid_{}", i),
                    ClassScores {
                        useful,
                        useful_bad,
                        abandoned,
                        zombie,
                    },
                    ActionFeasibility::allow_all(),
                    ProbeType::ALL.to_vec(),
                )
            })
            .collect()
    };

    for n in [5, 20, 50] {
        let candidates = make_candidates(n);
        let budget = ProbeBudget::new(60.0, 2.0);

        group.bench_with_input(
            BenchmarkId::new("whittle_index", n),
            &candidates,
            |b, cands| {
                b.iter(|| {
                    let plan = allocate_probes(
                        black_box(cands),
                        black_box(&policy),
                        &cost_model,
                        &selection_policy,
                        budget,
                    )
                    .expect("allocation should succeed");
                    black_box(plan.selections.len());
                })
            },
        );
    }

    // Tight budget: only enough for a few probes across 20 candidates.
    let candidates = make_candidates(20);
    let tight_budget = ProbeBudget::new(5.0, 0.5);
    group.bench_with_input(
        BenchmarkId::new("tight_budget", 20),
        &candidates,
        |b, cands| {
            b.iter(|| {
                let plan = allocate_probes(
                    black_box(cands),
                    black_box(&policy),
                    &cost_model,
                    &selection_policy,
                    tight_budget,
                )
                .expect("allocation should succeed");
                black_box(plan.selections.len());
            })
        },
    );

    group.finish();
}

criterion_group!(
    benches,
    bench_compute_voi,
    bench_select_probe_by_information_gain,
    bench_allocate_probes
);
criterion_main!(benches);
