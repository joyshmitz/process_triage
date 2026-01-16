//! Configuration validation errors and semantic validation.

use thiserror::Error;

/// Validation result type.
pub type ValidationResult<T> = Result<T, ValidationError>;

/// Configuration validation errors.
#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("I/O error: {0}")]
    IoError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Schema validation failed: {0}")]
    SchemaError(String),

    #[error("Semantic validation failed: {0}")]
    SemanticError(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid value for {field}: {message}")]
    InvalidValue { field: String, message: String },

    #[error("Version mismatch: expected {expected}, got {actual}")]
    VersionMismatch { expected: String, actual: String },
}

impl ValidationError {
    /// Error code for structured error reporting.
    pub fn code(&self) -> u32 {
        match self {
            ValidationError::IoError(_) => 60,
            ValidationError::ParseError(_) => 61,
            ValidationError::SchemaError(_) => 62,
            ValidationError::SemanticError(_) => 63,
            ValidationError::MissingField(_) => 64,
            ValidationError::InvalidValue { .. } => 65,
            ValidationError::VersionMismatch { .. } => 66,
        }
    }
}

/// Validate priors configuration semantically.
pub fn validate_priors(priors: &crate::priors::Priors) -> ValidationResult<()> {
    // Check schema version
    if priors.schema_version != crate::CONFIG_SCHEMA_VERSION {
        return Err(ValidationError::VersionMismatch {
            expected: crate::CONFIG_SCHEMA_VERSION.to_string(),
            actual: priors.schema_version.clone(),
        });
    }

    // Check that class priors sum to 1.0 (within tolerance)
    let prior_sum = priors.classes.useful.prior_prob
        + priors.classes.useful_bad.prior_prob
        + priors.classes.abandoned.prior_prob
        + priors.classes.zombie.prior_prob;

    if (prior_sum - 1.0).abs() > 0.01 {
        return Err(ValidationError::SemanticError(format!(
            "Class priors must sum to 1.0, got {} (useful={}, useful_bad={}, abandoned={}, zombie={})",
            prior_sum,
            priors.classes.useful.prior_prob,
            priors.classes.useful_bad.prior_prob,
            priors.classes.abandoned.prior_prob,
            priors.classes.zombie.prior_prob,
        )));
    }

    // Validate each class
    validate_class_params("useful", &priors.classes.useful)?;
    validate_class_params("useful_bad", &priors.classes.useful_bad)?;
    validate_class_params("abandoned", &priors.classes.abandoned)?;
    validate_class_params("zombie", &priors.classes.zombie)?;

    // Validate Beta params in hazard regimes
    for regime in &priors.hazard_regimes {
        validate_gamma_params(
            &format!("hazard_regimes.{}.gamma", regime.name),
            &regime.gamma,
        )?;
    }

    Ok(())
}

/// Validate a single class's parameters.
fn validate_class_params(name: &str, params: &crate::priors::ClassParams) -> ValidationResult<()> {
    // Prior probability must be in [0, 1]
    if params.prior_prob < 0.0 || params.prior_prob > 1.0 {
        return Err(ValidationError::InvalidValue {
            field: format!("classes.{}.prior_prob", name),
            message: format!("Must be in [0, 1], got {}", params.prior_prob),
        });
    }

    // Validate Beta parameters
    validate_beta_params(&format!("classes.{}.cpu_beta", name), &params.cpu_beta)?;
    validate_beta_params(
        &format!("classes.{}.orphan_beta", name),
        &params.orphan_beta,
    )?;
    validate_beta_params(&format!("classes.{}.tty_beta", name), &params.tty_beta)?;
    validate_beta_params(&format!("classes.{}.net_beta", name), &params.net_beta)?;

    if let Some(ref beta) = params.io_active_beta {
        validate_beta_params(&format!("classes.{}.io_active_beta", name), beta)?;
    }

    // Validate Gamma parameters
    if let Some(ref gamma) = params.runtime_gamma {
        validate_gamma_params(&format!("classes.{}.runtime_gamma", name), gamma)?;
    }

    if let Some(ref gamma) = params.hazard_gamma {
        validate_gamma_params(&format!("classes.{}.hazard_gamma", name), gamma)?;
    }

    Ok(())
}

/// Validate Beta distribution parameters.
fn validate_beta_params(field: &str, params: &crate::priors::BetaParams) -> ValidationResult<()> {
    if params.alpha <= 0.0 {
        return Err(ValidationError::InvalidValue {
            field: format!("{}.alpha", field),
            message: format!("Must be positive, got {}", params.alpha),
        });
    }

    if params.beta <= 0.0 {
        return Err(ValidationError::InvalidValue {
            field: format!("{}.beta", field),
            message: format!("Must be positive, got {}", params.beta),
        });
    }

    Ok(())
}

/// Validate Gamma distribution parameters.
fn validate_gamma_params(field: &str, params: &crate::priors::GammaParams) -> ValidationResult<()> {
    if params.shape <= 0.0 {
        return Err(ValidationError::InvalidValue {
            field: format!("{}.shape", field),
            message: format!("Must be positive, got {}", params.shape),
        });
    }

    if params.rate <= 0.0 {
        return Err(ValidationError::InvalidValue {
            field: format!("{}.rate", field),
            message: format!("Must be positive, got {}", params.rate),
        });
    }

    Ok(())
}

/// Validate policy configuration semantically.
pub fn validate_policy(policy: &crate::policy::Policy) -> ValidationResult<()> {
    // Check schema version
    if policy.schema_version != crate::CONFIG_SCHEMA_VERSION {
        return Err(ValidationError::VersionMismatch {
            expected: crate::CONFIG_SCHEMA_VERSION.to_string(),
            actual: policy.schema_version.clone(),
        });
    }

    // Validate loss matrix completeness
    validate_loss_matrix(&policy.loss_matrix)?;

    // Validate FDR alpha is in valid range
    if policy.fdr_control.alpha < 0.0 || policy.fdr_control.alpha > 1.0 {
        return Err(ValidationError::InvalidValue {
            field: "fdr_control.alpha".to_string(),
            message: format!("Must be in [0, 1], got {}", policy.fdr_control.alpha),
        });
    }

    // Validate robot mode settings
    if policy.robot_mode.min_posterior < 0.0 || policy.robot_mode.min_posterior > 1.0 {
        return Err(ValidationError::InvalidValue {
            field: "robot_mode.min_posterior".to_string(),
            message: format!("Must be in [0, 1], got {}", policy.robot_mode.min_posterior),
        });
    }

    // Validate guardrails
    if policy.guardrails.never_kill_ppid.is_empty() {
        return Err(ValidationError::SemanticError(
            "guardrails.never_kill_ppid must contain at least PID 1".to_string(),
        ));
    }

    if !policy.guardrails.never_kill_ppid.contains(&1) {
        return Err(ValidationError::SemanticError(
            "guardrails.never_kill_ppid must contain PID 1 (init)".to_string(),
        ));
    }

    validate_load_aware(&policy.load_aware)?;

    Ok(())
}

fn validate_load_aware(load_aware: &crate::policy::LoadAwareDecision) -> ValidationResult<()> {
    if !load_aware.enabled {
        return Ok(());
    }

    let weight_sum =
        load_aware.weights.queue + load_aware.weights.load + load_aware.weights.memory + load_aware.weights.psi;
    if weight_sum <= 0.0 {
        return Err(ValidationError::InvalidValue {
            field: "load_aware.weights".to_string(),
            message: "weights must have positive sum".to_string(),
        });
    }

    if load_aware.weights.queue > 0.0 && load_aware.queue_high == 0 {
        return Err(ValidationError::InvalidValue {
            field: "load_aware.queue_high".to_string(),
            message: "must be > 0 when queue weight is set".to_string(),
        });
    }

    if load_aware.weights.load > 0.0 && load_aware.load_per_core_high <= 0.0 {
        return Err(ValidationError::InvalidValue {
            field: "load_aware.load_per_core_high".to_string(),
            message: "must be > 0 when load weight is set".to_string(),
        });
    }

    if load_aware.weights.memory > 0.0
        && (load_aware.memory_used_fraction_high <= 0.0
            || load_aware.memory_used_fraction_high > 1.0)
    {
        return Err(ValidationError::InvalidValue {
            field: "load_aware.memory_used_fraction_high".to_string(),
            message: "must be in (0, 1] when memory weight is set".to_string(),
        });
    }

    if load_aware.weights.psi > 0.0 && load_aware.psi_avg10_high <= 0.0 {
        return Err(ValidationError::InvalidValue {
            field: "load_aware.psi_avg10_high".to_string(),
            message: "must be > 0 when psi weight is set".to_string(),
        });
    }

    if load_aware.multipliers.keep_max < 1.0 {
        return Err(ValidationError::InvalidValue {
            field: "load_aware.multipliers.keep_max".to_string(),
            message: "must be >= 1.0".to_string(),
        });
    }
    if load_aware.multipliers.risky_max < 1.0 {
        return Err(ValidationError::InvalidValue {
            field: "load_aware.multipliers.risky_max".to_string(),
            message: "must be >= 1.0".to_string(),
        });
    }
    if load_aware.multipliers.reversible_min <= 0.0 || load_aware.multipliers.reversible_min > 1.0
    {
        return Err(ValidationError::InvalidValue {
            field: "load_aware.multipliers.reversible_min".to_string(),
            message: "must be in (0, 1]".to_string(),
        });
    }

    Ok(())
}

/// Validate loss matrix has all required values.
fn validate_loss_matrix(matrix: &crate::policy::LossMatrix) -> ValidationResult<()> {
    // All losses must be non-negative
    let classes = [
        ("useful", &matrix.useful),
        ("useful_bad", &matrix.useful_bad),
        ("abandoned", &matrix.abandoned),
        ("zombie", &matrix.zombie),
    ];

    for (name, row) in classes {
        if row.keep < 0.0 {
            return Err(ValidationError::InvalidValue {
                field: format!("loss_matrix.{}.keep", name),
                message: "Must be non-negative".to_string(),
            });
        }
        if row.kill < 0.0 {
            return Err(ValidationError::InvalidValue {
                field: format!("loss_matrix.{}.kill", name),
                message: "Must be non-negative".to_string(),
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beta_validation() {
        let valid = crate::priors::BetaParams {
            alpha: 2.0,
            beta: 5.0,
            comment: None,
        };
        assert!(validate_beta_params("test", &valid).is_ok());

        let invalid = crate::priors::BetaParams {
            alpha: -1.0,
            beta: 5.0,
            comment: None,
        };
        assert!(validate_beta_params("test", &invalid).is_err());
    }

    #[test]
    fn test_gamma_validation() {
        let valid = crate::priors::GammaParams {
            shape: 2.0,
            rate: 0.001,
            comment: None,
        };
        assert!(validate_gamma_params("test", &valid).is_ok());

        let invalid = crate::priors::GammaParams {
            shape: 0.0,
            rate: 0.001,
            comment: None,
        };
        assert!(validate_gamma_params("test", &invalid).is_err());
    }
}
