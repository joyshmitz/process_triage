//! Bayesian model averaging utilities for inference submodels.
//!
//! Combines multiple model posteriors into a weighted mixture, providing
//! explicit, auditable weights for explainability.

use serde::Serialize;
use thiserror::Error;

use super::ClassScores;

/// A single model posterior with an associated weight.
#[derive(Debug, Clone, Serialize)]
pub struct ModelPosterior {
    pub name: String,
    pub weight: f64,
    pub posterior: ClassScores,
}

/// Result of model averaging.
#[derive(Debug, Clone, Serialize)]
pub struct ModelAveragedPosterior {
    pub combined: ClassScores,
    pub normalized_weights: Vec<ModelWeight>,
    pub models: Vec<ModelPosterior>,
}

/// Normalized weight for an individual model.
#[derive(Debug, Clone, Serialize)]
pub struct ModelWeight {
    pub name: String,
    pub weight: f64,
}

/// Errors that can occur during model averaging.
#[derive(Debug, Error)]
pub enum BmaError {
    #[error("no models provided")]
    EmptyModels,
    #[error("invalid weight for model {name}: {weight}")]
    InvalidWeight { name: String, weight: f64 },
    #[error("sum of weights is zero")]
    ZeroWeightSum,
    #[error("posterior for model {name} is not normalized: sum={sum}")]
    PosteriorNotNormalized { name: String, sum: f64 },
}

/// Combine model posteriors via weighted average.
pub fn combine_posteriors(models: &[ModelPosterior]) -> Result<ModelAveragedPosterior, BmaError> {
    if models.is_empty() {
        return Err(BmaError::EmptyModels);
    }

    let mut weight_sum = 0.0;
    for model in models {
        if !model.weight.is_finite() || model.weight < 0.0 {
            return Err(BmaError::InvalidWeight {
                name: model.name.clone(),
                weight: model.weight,
            });
        }
        let sum = model.posterior.useful
            + model.posterior.useful_bad
            + model.posterior.abandoned
            + model.posterior.zombie;
        if (sum - 1.0).abs() > 1e-6 || !sum.is_finite() {
            return Err(BmaError::PosteriorNotNormalized {
                name: model.name.clone(),
                sum,
            });
        }
        weight_sum += model.weight;
    }

    if weight_sum <= 0.0 || !weight_sum.is_finite() {
        return Err(BmaError::ZeroWeightSum);
    }

    let mut combined = ClassScores {
        useful: 0.0,
        useful_bad: 0.0,
        abandoned: 0.0,
        zombie: 0.0,
    };

    let mut normalized_weights = Vec::with_capacity(models.len());
    for model in models {
        let weight = model.weight / weight_sum;
        combined.useful += weight * model.posterior.useful;
        combined.useful_bad += weight * model.posterior.useful_bad;
        combined.abandoned += weight * model.posterior.abandoned;
        combined.zombie += weight * model.posterior.zombie;
        normalized_weights.push(ModelWeight {
            name: model.name.clone(),
            weight,
        });
    }

    Ok(ModelAveragedPosterior {
        combined,
        normalized_weights,
        models: models.to_vec(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scores(useful: f64, useful_bad: f64, abandoned: f64, zombie: f64) -> ClassScores {
        ClassScores {
            useful,
            useful_bad,
            abandoned,
            zombie,
        }
    }

    #[test]
    fn combines_weighted_posteriors() {
        let models = vec![
            ModelPosterior {
                name: "m1".to_string(),
                weight: 0.25,
                posterior: scores(0.7, 0.1, 0.15, 0.05),
            },
            ModelPosterior {
                name: "m2".to_string(),
                weight: 0.75,
                posterior: scores(0.2, 0.2, 0.5, 0.1),
            },
        ];

        let result = combine_posteriors(&models).expect("combine");
        let combined = result.combined;

        assert!((combined.useful - 0.325).abs() < 1e-9);
        assert!((combined.useful_bad - 0.175).abs() < 1e-9);
        assert!((combined.abandoned - 0.4125).abs() < 1e-9);
        assert!((combined.zombie - 0.0875).abs() < 1e-9);
    }

    #[test]
    fn normalizes_weights() {
        let models = vec![
            ModelPosterior {
                name: "m1".to_string(),
                weight: 2.0,
                posterior: scores(0.25, 0.25, 0.25, 0.25),
            },
            ModelPosterior {
                name: "m2".to_string(),
                weight: 1.0,
                posterior: scores(0.25, 0.25, 0.25, 0.25),
            },
        ];

        let result = combine_posteriors(&models).expect("combine");
        let weights = result.normalized_weights;
        assert!((weights[0].weight - 2.0 / 3.0).abs() < 1e-9);
        assert!((weights[1].weight - 1.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn rejects_invalid_weights() {
        let models = vec![ModelPosterior {
            name: "m1".to_string(),
            weight: -0.1,
            posterior: scores(0.25, 0.25, 0.25, 0.25),
        }];

        let err = combine_posteriors(&models).unwrap_err();
        matches!(err, BmaError::InvalidWeight { .. });
    }

    #[test]
    fn rejects_non_normalized_posterior() {
        let models = vec![ModelPosterior {
            name: "m1".to_string(),
            weight: 1.0,
            posterior: scores(0.1, 0.1, 0.1, 0.1),
        }];

        let err = combine_posteriors(&models).unwrap_err();
        matches!(err, BmaError::PosteriorNotNormalized { .. });
    }
}
