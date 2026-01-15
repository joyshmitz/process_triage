//! Property-based tests for inference engine invariants.

use proptest::prelude::*;
use pt_core::config::priors::Priors;
use pt_core::inference::{compute_posterior, CpuEvidence, Evidence};

fn cpu_evidence_strategy() -> impl Strategy<Value = CpuEvidence> {
    prop_oneof![
        (0.0f64..=1.0).prop_map(|occupancy| CpuEvidence::Fraction { occupancy }),
        (1u32..=10_000u32, 0u32..=10_000u32).prop_map(|(n, k)| {
            let k = k.min(n);
            CpuEvidence::Binomial {
                k: k as f64,
                n: n as f64,
                eta: None,
            }
        }),
    ]
}

fn evidence_strategy() -> impl Strategy<Value = Evidence> {
    let cpu = prop::option::of(cpu_evidence_strategy());
    let runtime = prop::option::of(0.1f64..=10_000_000.0f64);
    let orphan = prop::option::of(any::<bool>());
    let tty = prop::option::of(any::<bool>());
    let net = prop::option::of(any::<bool>());
    let io_active = prop::option::of(any::<bool>());

    (cpu, runtime, orphan, tty, net, io_active).prop_map(
        |(cpu, runtime_seconds, orphan, tty, net, io_active)| Evidence {
            cpu,
            runtime_seconds,
            orphan,
            tty,
            net,
            io_active,
            state_flag: None,
            command_category: None,
        },
    )
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10_000))]

    #[test]
    fn posterior_probabilities_are_valid(evidence in evidence_strategy()) {
        let priors = Priors::default();
        let result = compute_posterior(&priors, &evidence).expect("posterior computation failed");
        let posterior = result.posterior;

        let sum = posterior.useful
            + posterior.useful_bad
            + posterior.abandoned
            + posterior.zombie;

        prop_assert!(sum.is_finite());
        prop_assert!((sum - 1.0).abs() < 1e-9, "sum={sum}");

        for value in [
            posterior.useful,
            posterior.useful_bad,
            posterior.abandoned,
            posterior.zombie,
        ] {
            prop_assert!(value >= -1e-12, "probability below zero: {value}");
            prop_assert!(value <= 1.0 + 1e-12, "probability above one: {value}");
        }

        let log_posterior = result.log_posterior;
        for value in [
            log_posterior.useful,
            log_posterior.useful_bad,
            log_posterior.abandoned,
            log_posterior.zombie,
        ] {
            prop_assert!(value.is_finite());
            prop_assert!(value <= 1e-9, "log posterior should be <= 0, got {value}");
        }
    }
}

#[test]
fn empty_evidence_equals_prior() {
    let priors = Priors::default();
    let evidence = Evidence::default();
    let result = compute_posterior(&priors, &evidence).expect("posterior computation failed");

    let posterior = result.posterior;
    let expected = [
        priors.classes.useful.prior_prob,
        priors.classes.useful_bad.prior_prob,
        priors.classes.abandoned.prior_prob,
        priors.classes.zombie.prior_prob,
    ];

    let actual = [
        posterior.useful,
        posterior.useful_bad,
        posterior.abandoned,
        posterior.zombie,
    ];

    for (idx, (got, exp)) in actual.iter().zip(expected.iter()).enumerate() {
        let delta = (got - exp).abs();
        assert!(
            delta < 1e-9,
            "posterior mismatch at index {idx}: got {got}, expected {exp}"
        );
    }
}
