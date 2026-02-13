//! Property-based tests for decision theory invariants.

use proptest::prelude::*;
use pt_core::config::policy::Policy;
use pt_core::decision::expected_loss::ActionFeasibility;
use pt_core::decision::myopic_policy::{compute_loss_table, decide_from_belief};
use pt_core::decision::{compute_voi, decide_action, Action, ProbeCostModel};
use pt_core::inference::belief_state::BeliefState;
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

    /// VOI property: high confidence posteriors should make probing less valuable.
    /// When we're already very confident, VOI should be close to cost (probing has little benefit).
    #[test]
    fn voi_high_confidence_makes_probing_less_valuable(posterior in posterior_strategy()) {
        let policy = Policy::default();
        let feasibility = ActionFeasibility::allow_all();
        let cost_model = ProbeCostModel::default();

        let result = compute_voi(
            &posterior,
            &policy,
            &feasibility,
            &cost_model,
            None,
        );

        if let Ok(analysis) = result {
            // Check if posterior is very confident (one class >> others)
            let max_prob = posterior.useful
                .max(posterior.useful_bad)
                .max(posterior.abandoned)
                .max(posterior.zombie);

            // Loss penalty is 100.0, probe costs are ~0.1-0.5.
            // Risk at 95% is 5.0, which > cost, so probing is still rational.
            // We need much higher confidence (risk < cost) to stop probing.
            if max_prob > 0.999 {
                // When very confident, most probes should have VOI close to cost
                // (little benefit from probing)
                let worthwhile_count = analysis.probes.iter()
                    .filter(|p| p.voi < -0.05)  // Significantly worthwhile
                    .count();

                // With high confidence, at most half of probes should be worthwhile
                prop_assert!(
                    worthwhile_count <= analysis.probes.len() / 2,
                    "High-confidence posterior (max_prob={}) has {} worthwhile probes out of {} (expected fewer)",
                    max_prob,
                    worthwhile_count,
                    analysis.probes.len()
                );
            }
        }
    }

    /// VOI structural invariant: all probes should have finite, non-NaN values.
    #[test]
    fn voi_outputs_are_finite(posterior in posterior_strategy()) {
        let policy = Policy::default();
        let cost_model = ProbeCostModel::default();
        let feasibility = ActionFeasibility::allow_all();

        let result = compute_voi(
            &posterior,
            &policy,
            &feasibility,
            &cost_model,
            None,
        );

        if let Ok(analysis) = result {
            prop_assert!(analysis.current_min_loss.is_finite());

            for probe_voi in &analysis.probes {
                prop_assert!(probe_voi.voi.is_finite(), "VOI for {} is not finite", probe_voi.probe.name());
                prop_assert!(probe_voi.cost.is_finite(), "Cost for {} is not finite", probe_voi.probe.name());
                prop_assert!(probe_voi.expected_loss_after.is_finite(), "Expected loss after {} is not finite", probe_voi.probe.name());
            }
        }
    }

    /// Property: probe cost should always be non-negative.
    #[test]
    fn probe_costs_are_non_negative(posterior in posterior_strategy()) {
        let policy = Policy::default();
        let cost_model = ProbeCostModel::default();
        let feasibility = ActionFeasibility::allow_all();

        let result = compute_voi(
            &posterior,
            &policy,
            &feasibility,
            &cost_model,
            None,
        );

        if let Ok(analysis) = result {
            for probe_voi in &analysis.probes {
                prop_assert!(
                    probe_voi.cost >= -1e-12,
                    "Probe {} has negative cost: {}",
                    probe_voi.probe.name(),
                    probe_voi.cost
                );
            }
        }
    }
}

// ── Myopic policy property tests ──────────────────────────────────

fn belief_strategy() -> impl Strategy<Value = BeliefState> {
    (0.01f64..=1.0, 0.01f64..=1.0, 0.01f64..=1.0, 0.01f64..=1.0).prop_map(|(u, ub, a, z)| {
        let sum = u + ub + a + z;
        BeliefState::from_probs([u / sum, ub / sum, a / sum, z / sum])
            .expect("normalised probs should form valid belief")
    })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(5_000))]

    /// decide_from_belief should always succeed for valid belief states.
    #[test]
    fn myopic_decide_from_belief_never_panics(belief in belief_strategy()) {
        let policy = Policy::default();
        let feasibility = ActionFeasibility::allow_all();
        let decision = decide_from_belief(&belief, &policy, &feasibility);
        prop_assert!(decision.is_ok(), "decide_from_belief failed: {:?}", decision.err());
    }

    /// The optimal action from decide_from_belief should be consistent with
    /// compute_loss_table: it should pick the action with minimal expected loss.
    #[test]
    fn myopic_optimal_action_matches_loss_table(belief in belief_strategy()) {
        let policy = Policy::default();
        let feasibility = ActionFeasibility::allow_all();

        let decision = decide_from_belief(&belief, &policy, &feasibility)
            .expect("decide_from_belief failed");
        let table = compute_loss_table(&belief, &policy.loss_matrix, &feasibility);

        // Find the minimum loss among feasible actions in the table.
        let table_min = table.iter()
            .filter(|b| b.feasible)
            .min_by(|a, b| a.expected_loss.partial_cmp(&b.expected_loss).unwrap())
            .expect("loss table should have feasible entries");

        prop_assert!(
            (decision.optimal_loss - table_min.expected_loss).abs() < 1e-9,
            "decision loss {} != table min loss {}",
            decision.optimal_loss,
            table_min.expected_loss
        );
    }

    /// The loss table should be sorted by expected loss (ascending).
    #[test]
    fn myopic_loss_table_is_sorted(belief in belief_strategy()) {
        let policy = Policy::default();
        let feasibility = ActionFeasibility::allow_all();
        let table = compute_loss_table(&belief, &policy.loss_matrix, &feasibility);

        for window in table.windows(2) {
            prop_assert!(
                window[0].expected_loss <= window[1].expected_loss + 1e-12,
                "loss table not sorted: {} > {}",
                window[0].expected_loss,
                window[1].expected_loss
            );
        }
    }

    /// decide_action and decide_from_belief should agree on the optimal action.
    #[test]
    fn decide_action_and_belief_agree(belief in belief_strategy()) {
        let policy = Policy::default();
        let feasibility = ActionFeasibility::allow_all();

        let posterior = ClassScores {
            useful: belief.prob(pt_core::inference::belief_state::ProcessState::Useful),
            useful_bad: belief.prob(pt_core::inference::belief_state::ProcessState::UsefulBad),
            abandoned: belief.prob(pt_core::inference::belief_state::ProcessState::Abandoned),
            zombie: belief.prob(pt_core::inference::belief_state::ProcessState::Zombie),
        };

        let action_outcome = decide_action(&posterior, &policy, &feasibility)
            .expect("decide_action failed");
        let belief_decision = decide_from_belief(&belief, &policy, &feasibility)
            .expect("decide_from_belief failed");

        prop_assert_eq!(
            action_outcome.optimal_action,
            belief_decision.optimal_action,
            "decide_action chose {:?} but decide_from_belief chose {:?}",
            action_outcome.optimal_action,
            belief_decision.optimal_action
        );
    }

    /// With zombie feasibility constraints, Kill should never be the optimal action.
    #[test]
    fn zombie_feasibility_blocks_kill(belief in belief_strategy()) {
        let policy = Policy::default();
        let feasibility = ActionFeasibility::from_process_state(true, false, None);

        let decision = decide_from_belief(&belief, &policy, &feasibility)
            .expect("decide_from_belief failed");

        prop_assert_ne!(
            decision.optimal_action,
            Action::Kill,
            "Kill should be blocked for zombie processes"
        );
    }
}
