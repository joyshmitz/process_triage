use pt_core::inference::mpp::{MarkedPointProcess, MppConfig};
use pt_core::inference::hawkes::{HawkesDetector, HawkesConfig};

#[test]
fn test_mpp_panic_on_out_of_order_insertion() {
    let config = MppConfig::default();
    let mut mpp = MarkedPointProcess::new(config);

    // Add initial event sets window_start = 10.0
    mpp.add_event(10.0, 1.0);
    
    // Add earlier event. If window_start isn't updated, 
    // compute_fano_factor will try (5.0 - 10.0) / bin_size -> negative index
    mpp.add_event(5.0, 1.0);
    mpp.add_event(6.0, 1.0);

    // This calls compute_fano_factor
    let _ = mpp.summarize(12.0);
}

#[test]
fn test_hawkes_unsorted_input_behavior() {
    let config = HawkesConfig::default();
    let detector = HawkesDetector::new(config);

    // Unsorted events: 1.0 comes before 0.5
    let events = vec![0.0, 1.0, 0.5];
    
    // This shouldn't crash, but might produce weird results if not handled.
    // If fit() expects sorted but gets unsorted, dt becomes negative (-0.5).
    // exp(-beta * -0.5) = exp(positive) -> recursion grows instead of decays.
    let result = detector.fit(&events, 2.0);
    
    // If handled correctly (e.g. sorted internally), likelihood should be reasonable
    assert!(result.log_likelihood.is_finite());
}
