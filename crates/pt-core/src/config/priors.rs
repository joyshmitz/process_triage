//! Priors configuration types.
//!
//! These types match the priors.schema.json specification.

use serde::{Deserialize, Serialize};

/// Beta distribution parameters: Beta(alpha, beta).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaParams {
    /// Alpha (shape1) parameter, must be positive.
    pub alpha: f64,
    /// Beta (shape2) parameter, must be positive.
    pub beta: f64,
}

impl Default for BetaParams {
    fn default() -> Self {
        Self {
            alpha: 1.0,
            beta: 1.0,
        }
    }
}

/// Gamma distribution parameters: Gamma(shape, rate).
/// Note: uses RATE parameterization, not scale.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GammaParams {
    /// Shape (alpha) parameter, must be positive.
    pub shape: f64,
    /// Rate (beta) parameter, must be positive. rate = 1/scale.
    pub rate: f64,
}

impl Default for GammaParams {
    fn default() -> Self {
        Self {
            shape: 1.0,
            rate: 1.0,
        }
    }
}

/// Dirichlet distribution parameters: Dir(alpha_1, ..., alpha_k).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirichletParams {
    /// Alpha concentration parameters, all must be positive.
    pub alpha: Vec<f64>,
}

impl Default for DirichletParams {
    fn default() -> Self {
        Self {
            alpha: vec![1.0, 1.0],
        }
    }
}

/// Competing hazard rates for a class.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompetingHazards {
    #[serde(default)]
    pub finish: Option<GammaParams>,
    #[serde(default)]
    pub abandon: Option<GammaParams>,
    #[serde(default)]
    pub degrade: Option<GammaParams>,
}

/// Bayesian hyperparameters for a single process class.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassPriors {
    /// Prior probability P(C) for this class.
    pub prior_prob: f64,
    /// Beta prior for CPU occupancy.
    pub cpu_beta: BetaParams,
    /// Gamma prior for runtime (optional, use either this OR hazard modeling).
    #[serde(default)]
    pub runtime_gamma: Option<GammaParams>,
    /// Beta prior for orphan probability.
    pub orphan_beta: BetaParams,
    /// Beta prior for TTY attachment probability.
    pub tty_beta: BetaParams,
    /// Beta prior for network activity probability.
    pub net_beta: BetaParams,
    /// Beta prior for I/O activity probability.
    #[serde(default)]
    pub io_active_beta: Option<BetaParams>,
    /// Gamma prior for base hazard rate.
    #[serde(default)]
    pub hazard_gamma: Option<GammaParams>,
    /// Per-class competing hazard rates.
    #[serde(default)]
    pub competing_hazards: Option<CompetingHazards>,
}

/// A piecewise-constant hazard regime.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HazardRegime {
    /// Identifier for this regime.
    pub name: String,
    /// Human-readable description.
    #[serde(default)]
    pub description: Option<String>,
    /// Gamma prior for hazard rate in this regime.
    pub gamma: GammaParams,
    /// Conditions that activate this regime.
    #[serde(default)]
    pub trigger_conditions: Vec<String>,
}

/// Semi-Markov state duration parameters.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SemiMarkov {
    #[serde(default)]
    pub useful_duration: Option<GammaParams>,
    #[serde(default)]
    pub useful_bad_duration: Option<GammaParams>,
    #[serde(default)]
    pub abandoned_duration: Option<GammaParams>,
    #[serde(default)]
    pub zombie_duration: Option<GammaParams>,
}

/// Change-point detection priors.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChangePoint {
    /// Beta prior for activity rate before change point.
    #[serde(default)]
    pub p_before: Option<BetaParams>,
    /// Beta prior for activity rate after change point.
    #[serde(default)]
    pub p_after: Option<BetaParams>,
    /// Parameter p for geometric prior on change-point location.
    #[serde(default)]
    pub tau_geometric_p: Option<f64>,
}

/// Beta priors for action outcomes per class.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InterventionPriors {
    #[serde(default)]
    pub useful: Option<BetaParams>,
    #[serde(default)]
    pub useful_bad: Option<BetaParams>,
    #[serde(default)]
    pub abandoned: Option<BetaParams>,
    #[serde(default)]
    pub zombie: Option<BetaParams>,
}

/// Priors for action outcome models.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CausalInterventions {
    #[serde(default)]
    pub pause: Option<InterventionPriors>,
    #[serde(default)]
    pub throttle: Option<InterventionPriors>,
    #[serde(default)]
    pub kill: Option<InterventionPriors>,
    #[serde(default)]
    pub restart: Option<InterventionPriors>,
}

/// Dirichlet priors for command category classification.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommandCategories {
    /// Ordered list of category names.
    #[serde(default)]
    pub category_names: Vec<String>,
    #[serde(default)]
    pub useful: Option<DirichletParams>,
    #[serde(default)]
    pub useful_bad: Option<DirichletParams>,
    #[serde(default)]
    pub abandoned: Option<DirichletParams>,
    #[serde(default)]
    pub zombie: Option<DirichletParams>,
}

/// Dirichlet priors for process state flag classification.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StateFlags {
    /// Ordered list of state flags.
    #[serde(default)]
    pub flag_names: Vec<String>,
    #[serde(default)]
    pub useful: Option<DirichletParams>,
    #[serde(default)]
    pub useful_bad: Option<DirichletParams>,
    #[serde(default)]
    pub abandoned: Option<DirichletParams>,
    #[serde(default)]
    pub zombie: Option<DirichletParams>,
}

/// Hierarchical/empirical Bayes settings.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Hierarchical {
    /// Whether to apply empirical Bayes shrinkage.
    #[serde(default)]
    pub shrinkage_enabled: Option<bool>,
    /// How strongly to pull category-specific priors toward global.
    #[serde(default)]
    pub shrinkage_strength: Option<f64>,
}

/// Lower and upper bounds for a class prior (credal set).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorBounds {
    pub lower: f64,
    pub upper: f64,
}

/// Class prior bounds for robust Bayes.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClassPriorBounds {
    #[serde(default)]
    pub useful: Option<PriorBounds>,
    #[serde(default)]
    pub useful_bad: Option<PriorBounds>,
    #[serde(default)]
    pub abandoned: Option<PriorBounds>,
    #[serde(default)]
    pub zombie: Option<PriorBounds>,
}

/// Imprecise prior settings and Safe Bayes tempering.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RobustBayes {
    /// Lower and upper bounds for each class prior.
    #[serde(default)]
    pub class_prior_bounds: Option<ClassPriorBounds>,
    /// Default learning rate eta for Safe Bayes tempering.
    #[serde(default)]
    pub safe_bayes_eta: Option<f64>,
    /// Whether to auto-adjust eta based on prequential log-loss.
    #[serde(default)]
    pub auto_eta_enabled: Option<bool>,
}

/// Beta prior for error rate tracking.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ErrorRate {
    /// Beta prior for false-kill rate.
    #[serde(default)]
    pub false_kill: Option<BetaParams>,
    /// Beta prior for false-spare rate.
    #[serde(default)]
    pub false_spare: Option<BetaParams>,
}

/// Bayesian Online Change-Point Detection settings.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Bocpd {
    /// Constant hazard rate for geometric run-length prior.
    #[serde(default)]
    pub hazard_lambda: Option<f64>,
    /// Minimum run length before considering a change point.
    #[serde(default)]
    pub min_run_length: Option<u32>,
}

/// Per-class Bayesian hyperparameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Classes {
    pub useful: ClassPriors,
    pub useful_bad: ClassPriors,
    pub abandoned: ClassPriors,
    pub zombie: ClassPriors,
}

/// Complete priors configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Priors {
    /// Schema version for compatibility checking.
    pub schema_version: String,
    /// Human-readable description.
    #[serde(default)]
    pub description: Option<String>,
    /// When this prior file was created.
    #[serde(default)]
    pub created_at: Option<String>,
    /// When this prior file was last updated.
    #[serde(default)]
    pub updated_at: Option<String>,
    /// Optional host profile tag.
    #[serde(default)]
    pub host_profile: Option<String>,
    /// Per-class Bayesian hyperparameters.
    pub classes: Classes,
    /// Piecewise-constant hazard regimes.
    #[serde(default)]
    pub hazard_regimes: Vec<HazardRegime>,
    /// Semi-Markov state duration parameters.
    #[serde(default)]
    pub semi_markov: Option<SemiMarkov>,
    /// Change-point detection priors.
    #[serde(default)]
    pub change_point: Option<ChangePoint>,
    /// Priors for action outcome models.
    #[serde(default)]
    pub causal_interventions: Option<CausalInterventions>,
    /// Dirichlet priors for command category classification.
    #[serde(default)]
    pub command_categories: Option<CommandCategories>,
    /// Dirichlet priors for process state flag classification.
    #[serde(default)]
    pub state_flags: Option<StateFlags>,
    /// Hierarchical/empirical Bayes settings.
    #[serde(default)]
    pub hierarchical: Option<Hierarchical>,
    /// Imprecise prior settings and Safe Bayes tempering.
    #[serde(default)]
    pub robust_bayes: Option<RobustBayes>,
    /// Beta prior for error rate tracking.
    #[serde(default)]
    pub error_rate: Option<ErrorRate>,
    /// Bayesian Online Change-Point Detection settings.
    #[serde(default)]
    pub bocpd: Option<Bocpd>,
}

impl Default for Priors {
    fn default() -> Self {
        Self {
            schema_version: super::CONFIG_SCHEMA_VERSION.to_string(),
            description: Some("Default priors (built-in)".to_string()),
            created_at: None,
            updated_at: None,
            host_profile: None,
            classes: Classes {
                useful: ClassPriors {
                    prior_prob: 0.70,
                    cpu_beta: BetaParams { alpha: 5.0, beta: 3.0 },
                    runtime_gamma: Some(GammaParams { shape: 2.0, rate: 0.0001 }),
                    orphan_beta: BetaParams { alpha: 1.0, beta: 20.0 },
                    tty_beta: BetaParams { alpha: 8.0, beta: 2.0 },
                    net_beta: BetaParams { alpha: 5.0, beta: 3.0 },
                    io_active_beta: Some(BetaParams { alpha: 6.0, beta: 2.0 }),
                    hazard_gamma: Some(GammaParams { shape: 2.0, rate: 10.0 }),
                    competing_hazards: None,
                },
                useful_bad: ClassPriors {
                    prior_prob: 0.05,
                    cpu_beta: BetaParams { alpha: 9.0, beta: 1.0 },
                    runtime_gamma: Some(GammaParams { shape: 1.5, rate: 0.00005 }),
                    orphan_beta: BetaParams { alpha: 2.0, beta: 8.0 },
                    tty_beta: BetaParams { alpha: 4.0, beta: 4.0 },
                    net_beta: BetaParams { alpha: 3.0, beta: 5.0 },
                    io_active_beta: Some(BetaParams { alpha: 2.0, beta: 6.0 }),
                    hazard_gamma: Some(GammaParams { shape: 1.5, rate: 20.0 }),
                    competing_hazards: None,
                },
                abandoned: ClassPriors {
                    prior_prob: 0.20,
                    cpu_beta: BetaParams { alpha: 1.0, beta: 8.0 },
                    runtime_gamma: Some(GammaParams { shape: 1.0, rate: 0.00001 }),
                    orphan_beta: BetaParams { alpha: 6.0, beta: 2.0 },
                    tty_beta: BetaParams { alpha: 1.0, beta: 8.0 },
                    net_beta: BetaParams { alpha: 1.0, beta: 6.0 },
                    io_active_beta: Some(BetaParams { alpha: 1.0, beta: 10.0 }),
                    hazard_gamma: Some(GammaParams { shape: 1.0, rate: 50.0 }),
                    competing_hazards: None,
                },
                zombie: ClassPriors {
                    prior_prob: 0.05,
                    cpu_beta: BetaParams { alpha: 1.0, beta: 100.0 },
                    runtime_gamma: Some(GammaParams { shape: 0.5, rate: 0.00001 }),
                    orphan_beta: BetaParams { alpha: 10.0, beta: 1.0 },
                    tty_beta: BetaParams { alpha: 1.0, beta: 20.0 },
                    net_beta: BetaParams { alpha: 1.0, beta: 50.0 },
                    io_active_beta: Some(BetaParams { alpha: 1.0, beta: 100.0 }),
                    hazard_gamma: Some(GammaParams { shape: 0.5, rate: 100.0 }),
                    competing_hazards: None,
                },
            },
            hazard_regimes: vec![],
            semi_markov: None,
            change_point: None,
            causal_interventions: None,
            command_categories: None,
            state_flags: None,
            hierarchical: Some(Hierarchical {
                shrinkage_enabled: Some(true),
                shrinkage_strength: Some(0.3),
            }),
            robust_bayes: None,
            error_rate: Some(ErrorRate {
                false_kill: Some(BetaParams { alpha: 1.0, beta: 99.0 }),
                false_spare: Some(BetaParams { alpha: 5.0, beta: 95.0 }),
            }),
            bocpd: Some(Bocpd {
                hazard_lambda: Some(0.01),
                min_run_length: Some(10),
            }),
        }
    }
}
