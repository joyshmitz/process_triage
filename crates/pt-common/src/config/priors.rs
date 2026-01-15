//! Bayesian priors configuration types for Process Triage inference engine.
//!
//! These types correspond to priors.schema.json and encode domain knowledge
//! about process behavior patterns.

use serde::{Deserialize, Serialize};
use crate::error::{Error, Result};

/// Schema version for priors configuration.
pub const PRIORS_SCHEMA_VERSION: &str = "1.0.0";

/// Root configuration for Bayesian priors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Priors {
    /// Schema version for compatibility checking
    pub schema_version: String,

    /// Human-readable description
    #[serde(default)]
    pub description: Option<String>,

    /// Optional host profile tag for fleet sharing
    #[serde(default)]
    pub host_profile: Option<String>,

    /// Per-class Bayesian hyperparameters
    pub classes: ClassPriors,

    /// Hazard regimes for survival modeling
    #[serde(default)]
    pub hazard_regimes: Vec<HazardRegime>,

    /// Semi-Markov state duration parameters
    #[serde(default)]
    pub semi_markov: Option<SemiMarkov>,

    /// Change-point detection priors
    #[serde(default)]
    pub change_point: Option<ChangePoint>,

    /// Priors for action outcome models
    #[serde(default)]
    pub causal_interventions: Option<CausalInterventions>,

    /// Dirichlet priors for command category classification
    #[serde(default)]
    pub command_categories: Option<CommandCategories>,

    /// Dirichlet priors for process state flags
    #[serde(default)]
    pub state_flags: Option<StateFlags>,

    /// Hierarchical/empirical Bayes settings
    #[serde(default)]
    pub hierarchical: Option<Hierarchical>,

    /// Robust Bayes / imprecise prior settings
    #[serde(default)]
    pub robust_bayes: Option<RobustBayes>,

    /// Error rate tracking priors
    #[serde(default)]
    pub error_rate: Option<ErrorRate>,

    /// BOCPD settings
    #[serde(default)]
    pub bocpd: Option<Bocpd>,
}

impl Priors {
    /// Validate priors semantically.
    pub fn validate(&self) -> Result<()> {
        // Check schema version
        if self.schema_version != PRIORS_SCHEMA_VERSION {
            return Err(Error::InvalidPriors(format!(
                "schema version mismatch: expected {}, got {}",
                PRIORS_SCHEMA_VERSION, self.schema_version
            )));
        }

        // Validate class priors sum to ~1.0
        let prob_sum = self.classes.useful.prior_prob
            + self.classes.useful_bad.prior_prob
            + self.classes.abandoned.prior_prob
            + self.classes.zombie.prior_prob;

        if (prob_sum - 1.0).abs() > 0.001 {
            return Err(Error::InvalidPriors(format!(
                "class prior probabilities must sum to 1.0 (got {:.6})",
                prob_sum
            )));
        }

        // Validate each class
        self.classes.useful.validate("useful")?;
        self.classes.useful_bad.validate("useful_bad")?;
        self.classes.abandoned.validate("abandoned")?;
        self.classes.zombie.validate("zombie")?;

        // Validate hazard regimes
        for regime in &self.hazard_regimes {
            regime.gamma.validate(&format!("hazard_regime.{}", regime.name))?;
        }

        Ok(())
    }
}

impl Default for Priors {
    fn default() -> Self {
        Priors {
            schema_version: PRIORS_SCHEMA_VERSION.to_string(),
            description: Some("Built-in default priors".to_string()),
            host_profile: Some("default".to_string()),
            classes: ClassPriors::default(),
            hazard_regimes: vec![],
            semi_markov: None,
            change_point: None,
            causal_interventions: None,
            command_categories: None,
            state_flags: None,
            hierarchical: None,
            robust_bayes: None,
            error_rate: None,
            bocpd: None,
        }
    }
}

/// Per-class Bayesian hyperparameters for the four process states.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassPriors {
    pub useful: ClassPrior,
    pub useful_bad: ClassPrior,
    pub abandoned: ClassPrior,
    pub zombie: ClassPrior,
}

impl Default for ClassPriors {
    fn default() -> Self {
        ClassPriors {
            useful: ClassPrior {
                prior_prob: 0.70,
                cpu_beta: BetaParams { alpha: 2.0, beta: 5.0 },
                runtime_gamma: Some(GammaParams { shape: 2.0, rate: 0.0001 }),
                orphan_beta: BetaParams { alpha: 1.0, beta: 20.0 },
                tty_beta: BetaParams { alpha: 5.0, beta: 3.0 },
                net_beta: BetaParams { alpha: 3.0, beta: 5.0 },
                io_active_beta: Some(BetaParams { alpha: 5.0, beta: 3.0 }),
                hazard_gamma: Some(GammaParams { shape: 1.0, rate: 100000.0 }),
                competing_hazards: None,
            },
            useful_bad: ClassPrior {
                prior_prob: 0.10,
                cpu_beta: BetaParams { alpha: 8.0, beta: 2.0 },
                runtime_gamma: Some(GammaParams { shape: 3.0, rate: 0.0002 }),
                orphan_beta: BetaParams { alpha: 2.0, beta: 8.0 },
                tty_beta: BetaParams { alpha: 3.0, beta: 5.0 },
                net_beta: BetaParams { alpha: 4.0, beta: 4.0 },
                io_active_beta: Some(BetaParams { alpha: 6.0, beta: 2.0 }),
                hazard_gamma: Some(GammaParams { shape: 2.0, rate: 50000.0 }),
                competing_hazards: None,
            },
            abandoned: ClassPrior {
                prior_prob: 0.15,
                cpu_beta: BetaParams { alpha: 1.0, beta: 10.0 },
                runtime_gamma: Some(GammaParams { shape: 4.0, rate: 0.00005 }),
                orphan_beta: BetaParams { alpha: 8.0, beta: 2.0 },
                tty_beta: BetaParams { alpha: 1.0, beta: 10.0 },
                net_beta: BetaParams { alpha: 1.0, beta: 8.0 },
                io_active_beta: Some(BetaParams { alpha: 1.0, beta: 12.0 }),
                hazard_gamma: Some(GammaParams { shape: 1.5, rate: 10000.0 }),
                competing_hazards: None,
            },
            zombie: ClassPrior {
                prior_prob: 0.05,
                cpu_beta: BetaParams { alpha: 1.0, beta: 100.0 },
                runtime_gamma: Some(GammaParams { shape: 2.0, rate: 0.0001 }),
                orphan_beta: BetaParams { alpha: 15.0, beta: 1.0 },
                tty_beta: BetaParams { alpha: 1.0, beta: 50.0 },
                net_beta: BetaParams { alpha: 1.0, beta: 100.0 },
                io_active_beta: Some(BetaParams { alpha: 1.0, beta: 100.0 }),
                hazard_gamma: Some(GammaParams { shape: 0.5, rate: 1000.0 }),
                competing_hazards: None,
            },
        }
    }
}

/// Bayesian hyperparameters for a single process class.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassPrior {
    /// Prior probability P(C) for this class
    pub prior_prob: f64,

    /// Beta prior for CPU occupancy
    pub cpu_beta: BetaParams,

    /// Gamma prior for runtime (optional if using hazard model)
    #[serde(default)]
    pub runtime_gamma: Option<GammaParams>,

    /// Beta prior for orphan probability
    pub orphan_beta: BetaParams,

    /// Beta prior for TTY attachment probability
    pub tty_beta: BetaParams,

    /// Beta prior for network activity probability
    pub net_beta: BetaParams,

    /// Beta prior for I/O activity probability
    #[serde(default)]
    pub io_active_beta: Option<BetaParams>,

    /// Gamma prior for base hazard rate
    #[serde(default)]
    pub hazard_gamma: Option<GammaParams>,

    /// Competing hazard rates
    #[serde(default)]
    pub competing_hazards: Option<CompetingHazards>,
}

impl ClassPrior {
    /// Validate this class prior semantically.
    pub fn validate(&self, class_name: &str) -> Result<()> {
        // Prior probability must be in [0, 1]
        if !(0.0..=1.0).contains(&self.prior_prob) {
            return Err(Error::InvalidPriors(format!(
                "{}.prior_prob must be in [0, 1] (got {})",
                class_name, self.prior_prob
            )));
        }

        // Validate Beta params
        self.cpu_beta.validate(&format!("{}.cpu_beta", class_name))?;
        self.orphan_beta.validate(&format!("{}.orphan_beta", class_name))?;
        self.tty_beta.validate(&format!("{}.tty_beta", class_name))?;
        self.net_beta.validate(&format!("{}.net_beta", class_name))?;

        if let Some(ref io_beta) = self.io_active_beta {
            io_beta.validate(&format!("{}.io_active_beta", class_name))?;
        }

        // Validate Gamma params
        if let Some(ref runtime) = self.runtime_gamma {
            runtime.validate(&format!("{}.runtime_gamma", class_name))?;
        }

        if let Some(ref hazard) = self.hazard_gamma {
            hazard.validate(&format!("{}.hazard_gamma", class_name))?;
        }

        Ok(())
    }
}

/// Beta distribution parameters: Beta(alpha, beta).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaParams {
    pub alpha: f64,
    pub beta: f64,
}

impl BetaParams {
    /// Validate Beta parameters are positive.
    pub fn validate(&self, path: &str) -> Result<()> {
        if self.alpha <= 0.0 {
            return Err(Error::InvalidPriors(format!(
                "{}.alpha must be positive (got {})", path, self.alpha
            )));
        }
        if self.beta <= 0.0 {
            return Err(Error::InvalidPriors(format!(
                "{}.beta must be positive (got {})", path, self.beta
            )));
        }
        Ok(())
    }

    /// Compute the mean of this Beta distribution.
    pub fn mean(&self) -> f64 {
        self.alpha / (self.alpha + self.beta)
    }
}

/// Gamma distribution parameters: Gamma(shape, rate).
/// Note: uses RATE parameterization, not scale.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GammaParams {
    pub shape: f64,
    pub rate: f64,
}

impl GammaParams {
    /// Validate Gamma parameters are positive.
    pub fn validate(&self, path: &str) -> Result<()> {
        if self.shape <= 0.0 {
            return Err(Error::InvalidPriors(format!(
                "{}.shape must be positive (got {})", path, self.shape
            )));
        }
        if self.rate <= 0.0 {
            return Err(Error::InvalidPriors(format!(
                "{}.rate must be positive (got {})", path, self.rate
            )));
        }
        Ok(())
    }

    /// Compute the mean of this Gamma distribution.
    pub fn mean(&self) -> f64 {
        self.shape / self.rate
    }
}

/// Dirichlet distribution parameters: Dir(alpha_1, ..., alpha_k).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirichletParams {
    pub alpha: Vec<f64>,
}

impl DirichletParams {
    /// Validate Dirichlet parameters are positive.
    pub fn validate(&self, path: &str) -> Result<()> {
        if self.alpha.len() < 2 {
            return Err(Error::InvalidPriors(format!(
                "{}.alpha must have at least 2 elements (got {})", path, self.alpha.len()
            )));
        }
        for (i, &a) in self.alpha.iter().enumerate() {
            if a <= 0.0 {
                return Err(Error::InvalidPriors(format!(
                    "{}.alpha[{}] must be positive (got {})", path, i, a
                )));
            }
        }
        Ok(())
    }
}

/// Competing hazard rates for a class.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetingHazards {
    pub finish: Option<GammaParams>,
    pub abandon: Option<GammaParams>,
    pub degrade: Option<GammaParams>,
}

/// A piecewise-constant hazard regime.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HazardRegime {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub gamma: GammaParams,
    #[serde(default)]
    pub trigger_conditions: Vec<String>,
}

/// Semi-Markov state duration parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemiMarkov {
    pub useful_duration: GammaParams,
    pub useful_bad_duration: GammaParams,
    pub abandoned_duration: GammaParams,
    pub zombie_duration: GammaParams,
}

/// Change-point detection priors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePoint {
    pub p_before: BetaParams,
    pub p_after: BetaParams,
    pub tau_geometric_p: f64,
}

/// Priors for causal intervention outcomes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalInterventions {
    pub pause: Option<InterventionPriors>,
    pub throttle: Option<InterventionPriors>,
    pub kill: Option<InterventionPriors>,
    pub restart: Option<InterventionPriors>,
}

/// Beta priors for action outcomes per class.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionPriors {
    pub useful: BetaParams,
    pub useful_bad: BetaParams,
    pub abandoned: BetaParams,
    pub zombie: BetaParams,
}

/// Dirichlet priors for command category classification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandCategories {
    pub category_names: Vec<String>,
    pub useful: DirichletParams,
    pub useful_bad: DirichletParams,
    pub abandoned: DirichletParams,
    pub zombie: DirichletParams,
}

/// Dirichlet priors for process state flags.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateFlags {
    pub flag_names: Vec<String>,
    pub useful: DirichletParams,
    pub useful_bad: DirichletParams,
    pub abandoned: DirichletParams,
    pub zombie: DirichletParams,
}

/// Hierarchical/empirical Bayes settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hierarchical {
    pub shrinkage_enabled: bool,
    pub shrinkage_strength: f64,
}

/// Robust Bayes / imprecise prior settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobustBayes {
    pub class_prior_bounds: Option<ClassPriorBounds>,
    pub safe_bayes_eta: Option<f64>,
    pub auto_eta_enabled: Option<bool>,
}

/// Lower and upper bounds for class priors (credal set).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassPriorBounds {
    pub useful: PriorBound,
    pub useful_bad: PriorBound,
    pub abandoned: PriorBound,
    pub zombie: PriorBound,
}

/// Lower and upper bounds for a single prior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorBound {
    pub lower: f64,
    pub upper: f64,
}

/// Beta priors for error rate tracking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorRate {
    pub false_kill: BetaParams,
    pub false_spare: BetaParams,
}

/// BOCPD (Bayesian Online Change-Point Detection) settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bocpd {
    pub hazard_lambda: f64,
    pub min_run_length: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_priors_valid() {
        let priors = Priors::default();
        assert!(priors.validate().is_ok());
    }

    #[test]
    fn test_class_probs_sum_to_one() {
        let priors = Priors::default();
        let sum = priors.classes.useful.prior_prob
            + priors.classes.useful_bad.prior_prob
            + priors.classes.abandoned.prior_prob
            + priors.classes.zombie.prior_prob;
        assert!((sum - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_beta_validation() {
        let valid = BetaParams { alpha: 2.0, beta: 3.0 };
        assert!(valid.validate("test").is_ok());

        let invalid_alpha = BetaParams { alpha: 0.0, beta: 3.0 };
        assert!(invalid_alpha.validate("test").is_err());

        let invalid_beta = BetaParams { alpha: 2.0, beta: -1.0 };
        assert!(invalid_beta.validate("test").is_err());
    }

    #[test]
    fn test_gamma_validation() {
        let valid = GammaParams { shape: 2.0, rate: 0.5 };
        assert!(valid.validate("test").is_ok());

        let invalid_shape = GammaParams { shape: 0.0, rate: 0.5 };
        assert!(invalid_shape.validate("test").is_err());
    }
}
