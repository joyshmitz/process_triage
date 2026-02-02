//! Bayesian credible bounds for shadow-mode error rates.
//!
//! Computes upper credible bounds on an error rate using a Beta posterior.
//! This is used to gate more aggressive robot thresholds.

use pt_math::beta_inv_cdf;
use serde::{Deserialize, Serialize};

/// Assumptions used for a credible-bounds computation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredibleBoundAssumptions {
    /// Definition of a "trial" for the error rate.
    pub trial_definition: String,
    /// Optional windowing metadata for time-bounded analysis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<WindowSpec>,
}

/// Optional window specification for time-bounded bounds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowSpec {
    /// Window length in seconds.
    pub window_seconds: u64,
    /// Optional human-readable label (e.g., "last_30_days").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// Computed credible bounds for an error rate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredibleBounds {
    pub prior_alpha: f64,
    pub prior_beta: f64,
    pub posterior_alpha: f64,
    pub posterior_beta: f64,
    pub errors: u64,
    pub trials: u64,
    pub deltas: Vec<f64>,
    pub upper_bounds: Vec<f64>,
    pub assumptions: CredibleBoundAssumptions,
}

/// Errors returned by credible bound computation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CredibleBoundError {
    InvalidPrior,
    InvalidCounts,
    InvalidDelta,
}

/// Compute upper credible bounds on error rate e ~ Beta(a+k, b+n-k).
pub fn compute_credible_bounds(
    prior_alpha: f64,
    prior_beta: f64,
    errors: u64,
    trials: u64,
    deltas: &[f64],
    assumptions: CredibleBoundAssumptions,
) -> Result<CredibleBounds, CredibleBoundError> {
    if prior_alpha <= 0.0 || prior_beta <= 0.0 {
        return Err(CredibleBoundError::InvalidPrior);
    }
    if errors > trials {
        return Err(CredibleBoundError::InvalidCounts);
    }
    for &delta in deltas {
        if !(0.0 < delta && delta < 1.0) {
            return Err(CredibleBoundError::InvalidDelta);
        }
    }

    let posterior_alpha = prior_alpha + errors as f64;
    let posterior_beta = prior_beta + (trials - errors) as f64;

    let mut upper_bounds = Vec::with_capacity(deltas.len());
    for &delta in deltas {
        let p = 1.0 - delta;
        let bound = beta_inv_cdf(p, posterior_alpha, posterior_beta);
        upper_bounds.push(bound);
    }

    Ok(CredibleBounds {
        prior_alpha,
        prior_beta,
        posterior_alpha,
        posterior_beta,
        errors,
        trials,
        deltas: deltas.to_vec(),
        upper_bounds,
        assumptions,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uniform_prior_zero_trials() {
        let assumptions = CredibleBoundAssumptions {
            trial_definition: "recommended kill vs spared".to_string(),
            window: None,
        };
        let bounds = compute_credible_bounds(1.0, 1.0, 0, 0, &[0.05], assumptions).unwrap();
        assert!((bounds.upper_bounds[0] - 0.95).abs() < 1e-6);
    }

    #[test]
    fn smaller_delta_gives_larger_bound() {
        let assumptions = CredibleBoundAssumptions {
            trial_definition: "recommended kill vs spared".to_string(),
            window: None,
        };
        let bounds = compute_credible_bounds(1.0, 1.0, 2, 10, &[0.1, 0.01], assumptions).unwrap();
        assert!(bounds.upper_bounds[1] >= bounds.upper_bounds[0]);
    }

    #[test]
    fn invalid_counts_rejected() {
        let assumptions = CredibleBoundAssumptions {
            trial_definition: "recommended kill vs spared".to_string(),
            window: None,
        };
        let err = compute_credible_bounds(1.0, 1.0, 3, 2, &[0.05], assumptions)
            .err()
            .unwrap();
        assert_eq!(err, CredibleBoundError::InvalidCounts);
    }

    #[test]
    fn invalid_delta_rejected() {
        let assumptions = CredibleBoundAssumptions {
            trial_definition: "recommended kill vs spared".to_string(),
            window: None,
        };
        let err = compute_credible_bounds(1.0, 1.0, 0, 0, &[1.0], assumptions)
            .err()
            .unwrap();
        assert_eq!(err, CredibleBoundError::InvalidDelta);
    }
}
