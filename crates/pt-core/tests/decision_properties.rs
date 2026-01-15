//! Property-based tests for decision theory invariants.

use proptest::prelude::*;
use pt_core::config::policy::Policy;
use pt_core::decision::{decide_action, ActionFeasibility};
use pt_core::inference::ClassScores;

fn posterior_strategy() -> impl Strategy<Value = ClassScores> {
    (0.0f64..=1.0, 0.0f64..=1.0, 0.0f64..=1.0, 0.0f64..=1.0).prop_map(
        |(useful, useful_bad, abandoned, zombie)| {
            let sum = useful + useful_bad + abandoned + zombie;
            if sum <= 0.0 {
                return ClassScores {
                    useful: 0.25,
                    useful_bad: 0.25,
                    abandoned: 0.25,
                    zombie: 0.25,
                };
            }
            ClassScores {
                useful: useful / sum,
                useful_bad: useful_bad / sum,
                abandoned: abandoned / sum,
                zombie: zombie / sum,
            }
        },
    )
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10_000))]

    #[test]
    fn expected_loss_is_non_negative_and_optimal_minimizes(posterior in posterior_strategy()) {
        let policy = Policy::default();
        let feasibility = ActionFeasibility::allow_all();
        let outcome = decide_action(&posterior, &policy, &feasibility)
            .expect("decision computation failed");

        for loss in &outcome.expected_loss {
            prop_assert!(loss.loss >= -1e-12, "expected loss below zero: {}", loss.loss);
        }

        let optimal_loss = outcome
            .expected_loss
            .iter()
            .find(|entry| entry.action == outcome.optimal_action)
            .map(|entry| entry.loss)
            .expect("optimal action missing from expected loss list");

        for loss in &outcome.expected_loss {
            prop_assert!(
                optimal_loss <= loss.loss + 1e-9,
                "optimal loss {optimal_loss} exceeds {}", loss.loss
            );
        }
    }
}
