use pt_core::inference::robust::{worst_case_expected_loss, best_case_expected_loss, CredalSet};

#[test]
fn test_robust_bayes_constraints_bug() {
    // Scenario: High loss class A, Low loss class B.
    // Class B has a mandatory minimum probability (lower bound).
    // The naive greedy algorithm ignores this lower bound and assigns all mass to A.
    
    let losses = [10.0, 0.0];
    // Class A: [0.0, 1.0] (High Loss)
    // Class B: [0.5, 1.0] (Low Loss, min 0.5)
    let credals = [
        CredalSet::interval(0.0, 1.0),
        CredalSet::interval(0.5, 1.0),
    ];

    // True worst case:
    // We must assign 0.5 to B (loss 0).
    // Remaining 0.5 goes to A (loss 10).
    // Expected loss = 0.5 * 10 + 0.5 * 0 = 5.0.
    
    // Naive algo:
    // Assigns 1.0 to A (fits in [0,1]).
    // Assigns 0.0 to B.
    // Result = 10.0.
    // But B=0.0 violates B >= 0.5.
    
    let worst = worst_case_expected_loss(&losses, &credals);
    
    assert!((worst - 5.0).abs() < 1e-6, "Worst case should be 5.0, got {}", worst);
}

#[test]
fn test_best_case_constraints_bug() {
    // Scenario: High loss class A, Low loss class B.
    // Class A has a mandatory minimum probability.
    
    let losses = [10.0, 0.0];
    // Class A: [0.5, 1.0] (High Loss, min 0.5)
    // Class B: [0.0, 1.0] (Low Loss)
    let credals = [
        CredalSet::interval(0.5, 1.0),
        CredalSet::interval(0.0, 1.0),
    ];

    // True best case (minimize loss):
    // Must assign 0.5 to A (loss 10).
    // Assign remaining 0.5 to B (loss 0).
    // Expected loss = 0.5 * 10 + 0.5 * 0 = 5.0.
    
    // Naive algo (sort ascending loss):
    // Fills B (loss 0) first. Capacity 1.0.
    // Assigns 1.0 to B.
    // Assigns 0.0 to A.
    // Result = 0.0.
    // But A=0.0 violates A >= 0.5.
    
    let best = best_case_expected_loss(&losses, &credals);
    
    assert!((best - 5.0).abs() < 1e-6, "Best case should be 5.0, got {}", best);
}
