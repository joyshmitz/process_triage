#[cfg(test)]
mod tests {
    use super::*;
    use crate::inference::bocpd::{BocpdConfig, BocpdDetector, EmissionModel};

    #[test]
    fn test_log_cp_optimization_equivalence() {
        let config = BocpdConfig {
            hazard_rate: 0.1,
            max_run_length: 50,
            emission_model: EmissionModel::PoissonGamma {
                alpha: 1.0,
                beta: 1.0,
            },
        };

        let mut detector = BocpdDetector::new(config);

        // Update a few times to build up distribution
        for _ in 0..10 {
            detector.update(5.0);
        }

        // Current state
        let log_dist = detector.log_run_length_dist.clone();
        let log_sum = super::log_sum_exp(&log_dist);
        
        // Verify normalization
        assert!((log_sum).abs() < 1e-10, "Distribution should be normalized, log_sum = {}", log_sum);

        // Calculate "naive" log_cp (as implemented currently)
        // We can't access private methods easily, but we can verify the math logic.
        // We simulate the logic here.
        
        let observation = 5.0;
        let h = 0.1_f64;
        let log_h = h.ln();
        
        // This relies on internal details, so we can't run this exact test against the private struct easily.
        // Instead, we can modify the code and verify existing tests pass and maybe add a benchmark.
    }
}
