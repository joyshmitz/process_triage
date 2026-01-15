//! Galaxy-brain math transparency section.
//!
//! Provides detailed mathematical derivation of the Bayesian inference
//! for process abandonment classification.

use serde::{Deserialize, Serialize};

/// Galaxy-brain section with math explanations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalaxyBrainSection {
    /// Prior probability configuration.
    pub priors: PriorConfig,
    /// Evidence factor explanations.
    pub factors: Vec<FactorMath>,
    /// Bayes factor interpretation guide.
    pub bf_guide: BayesFactorGuide,
    /// Example calculation walkthrough.
    pub example: Option<CalculationExample>,
}

/// Prior probability configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorConfig {
    /// Prior for useful processes.
    pub p_useful: f64,
    /// Prior for useful-but-bad processes.
    pub p_useful_bad: f64,
    /// Prior for abandoned processes.
    pub p_abandoned: f64,
    /// KaTeX formula for prior.
    pub formula: String,
    /// Explanation text.
    pub explanation: String,
}

impl Default for PriorConfig {
    fn default() -> Self {
        Self {
            p_useful: 0.7,
            p_useful_bad: 0.1,
            p_abandoned: 0.2,
            formula: r"P(\text{abandoned}) = 0.2, \quad P(\text{useful}) = 0.7, \quad P(\text{useful-bad}) = 0.1".to_string(),
            explanation: "Base rates estimated from historical process behavior data. Most processes are useful and should not be terminated.".to_string(),
        }
    }
}

/// Mathematical explanation for an evidence factor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactorMath {
    /// Factor name.
    pub name: String,
    /// Factor category.
    pub category: String,
    /// KaTeX formula for likelihood ratio.
    pub formula: String,
    /// Intuitive explanation.
    pub intuition: String,
    /// Example values.
    pub examples: Vec<FactorExample>,
}

/// Example value for a factor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactorExample {
    /// Input value description.
    pub input: String,
    /// Resulting log-odds.
    pub log_odds: f64,
    /// Interpretation.
    pub interpretation: String,
}

/// Guide to interpreting Bayes factors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BayesFactorGuide {
    /// KaTeX formula for Bayes factor.
    pub formula: String,
    /// Interpretation thresholds.
    pub thresholds: Vec<BFThreshold>,
    /// Explanation of log-odds scale.
    pub log_odds_explanation: String,
}

/// Bayes factor interpretation threshold.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BFThreshold {
    /// Minimum log BF value.
    pub min_log_bf: f64,
    /// Maximum log BF value (None = unbounded).
    pub max_log_bf: Option<f64>,
    /// Interpretation label.
    pub label: String,
    /// Description.
    pub description: String,
}

impl Default for BayesFactorGuide {
    fn default() -> Self {
        Self {
            formula: r"BF = \frac{P(E|\text{abandoned})}{P(E|\text{legitimate})} = \exp(\log BF)".to_string(),
            thresholds: vec![
                BFThreshold {
                    min_log_bf: f64::NEG_INFINITY,
                    max_log_bf: Some(0.0),
                    label: "Supports Legitimate".to_string(),
                    description: "Evidence favors the process being legitimate".to_string(),
                },
                BFThreshold {
                    min_log_bf: 0.0,
                    max_log_bf: Some(1.0),
                    label: "Weak".to_string(),
                    description: "Barely worth mentioning (BF < 3)".to_string(),
                },
                BFThreshold {
                    min_log_bf: 1.0,
                    max_log_bf: Some(2.0),
                    label: "Substantial".to_string(),
                    description: "Substantial evidence (BF 3-10)".to_string(),
                },
                BFThreshold {
                    min_log_bf: 2.0,
                    max_log_bf: Some(3.0),
                    label: "Strong".to_string(),
                    description: "Strong evidence (BF 10-30)".to_string(),
                },
                BFThreshold {
                    min_log_bf: 3.0,
                    max_log_bf: None,
                    label: "Very Strong".to_string(),
                    description: "Very strong evidence (BF > 30)".to_string(),
                },
            ],
            log_odds_explanation: "Log-odds provide an additive scale for combining evidence. A log-odds of 1.0 corresponds to a Bayes factor of e \u{2248} 2.7.".to_string(),
        }
    }
}

/// Calculation example walkthrough.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationExample {
    /// Process description.
    pub process_desc: String,
    /// Prior odds.
    pub prior_odds: f64,
    /// Steps in the calculation.
    pub steps: Vec<CalculationStep>,
    /// Final posterior probability.
    pub posterior_p: f64,
    /// Final recommendation.
    pub recommendation: String,
}

/// Single step in calculation example.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationStep {
    /// Step description.
    pub description: String,
    /// KaTeX formula.
    pub formula: String,
    /// Running log-odds after this step.
    pub cumulative_log_odds: f64,
}

impl Default for GalaxyBrainSection {
    fn default() -> Self {
        Self {
            priors: PriorConfig::default(),
            factors: default_factor_math(),
            bf_guide: BayesFactorGuide::default(),
            example: None,
        }
    }
}

/// Default factor math explanations.
fn default_factor_math() -> Vec<FactorMath> {
    vec![
        FactorMath {
            name: "Age".to_string(),
            category: "Timing".to_string(),
            formula: r"\log \text{LR}_{\text{age}} = \alpha \cdot \log\left(\frac{\text{age}_s}{\text{median}_{\text{age}}}\right)".to_string(),
            intuition: "Older processes are more likely to be abandoned. The effect is logarithmic - doubling age adds a constant to log-odds.".to_string(),
            examples: vec![
                FactorExample {
                    input: "5 minutes".to_string(),
                    log_odds: -0.5,
                    interpretation: "Young process, likely still in use".to_string(),
                },
                FactorExample {
                    input: "4 hours".to_string(),
                    log_odds: 0.3,
                    interpretation: "Moderate age, slightly suspicious".to_string(),
                },
                FactorExample {
                    input: "24 hours".to_string(),
                    log_odds: 0.8,
                    interpretation: "Old process, likely forgotten".to_string(),
                },
            ],
        },
        FactorMath {
            name: "CPU".to_string(),
            category: "Resource".to_string(),
            formula: r"\log \text{LR}_{\text{cpu}} = -\beta \cdot \mathbb{1}[\text{cpu}_{\%} > \epsilon] - \gamma \cdot \text{cpu}_{\text{avg}}".to_string(),
            intuition: "Active CPU usage strongly suggests the process is doing useful work. Idle processes are more suspicious.".to_string(),
            examples: vec![
                FactorExample {
                    input: "0% CPU".to_string(),
                    log_odds: 0.4,
                    interpretation: "Completely idle, suspicious".to_string(),
                },
                FactorExample {
                    input: "5% CPU".to_string(),
                    log_odds: -0.3,
                    interpretation: "Light activity, probably useful".to_string(),
                },
                FactorExample {
                    input: "50% CPU".to_string(),
                    log_odds: -1.0,
                    interpretation: "Heavy usage, definitely working".to_string(),
                },
            ],
        },
        FactorMath {
            name: "Memory".to_string(),
            category: "Resource".to_string(),
            formula: r"\log \text{LR}_{\text{mem}} = \delta \cdot \mathbb{1}[\text{mem}_{\text{MB}} > \tau] + \eta \cdot \mathbb{1}[\dot{\text{mem}} \approx 0]".to_string(),
            intuition: "Large memory footprint with no growth suggests a stale process. Growing memory suggests active allocation.".to_string(),
            examples: vec![
                FactorExample {
                    input: "10 MB, stable".to_string(),
                    log_odds: 0.0,
                    interpretation: "Small footprint, neutral".to_string(),
                },
                FactorExample {
                    input: "500 MB, stable".to_string(),
                    log_odds: 0.5,
                    interpretation: "Large but static, possibly stale".to_string(),
                },
                FactorExample {
                    input: "500 MB, growing".to_string(),
                    log_odds: -0.3,
                    interpretation: "Active allocation, working".to_string(),
                },
            ],
        },
        FactorMath {
            name: "State".to_string(),
            category: "Process State".to_string(),
            formula: r"\log \text{LR}_{\text{state}} = \begin{cases} 2.0 & \text{zombie} \\ 0.5 & \text{stopped} \\ 0.3 & \text{orphan} \\ 0 & \text{otherwise} \end{cases}".to_string(),
            intuition: "Zombie and stopped processes are strong indicators of abandonment. Orphan processes have lost their parent.".to_string(),
            examples: vec![
                FactorExample {
                    input: "Zombie (Z)".to_string(),
                    log_odds: 2.0,
                    interpretation: "Dead but not reaped - abandoned".to_string(),
                },
                FactorExample {
                    input: "Orphan".to_string(),
                    log_odds: 0.3,
                    interpretation: "Parent died, possibly forgotten".to_string(),
                },
            ],
        },
        FactorMath {
            name: "Network".to_string(),
            category: "Connectivity".to_string(),
            formula: r"\log \text{LR}_{\text{net}} = -\kappa \cdot \mathbb{1}[\text{listen}] - \lambda \cdot \log(1 + \text{connections})".to_string(),
            intuition: "Listening sockets indicate a server. Active connections suggest ongoing work.".to_string(),
            examples: vec![
                FactorExample {
                    input: "No network".to_string(),
                    log_odds: 0.1,
                    interpretation: "No connections, slightly suspicious".to_string(),
                },
                FactorExample {
                    input: "Listening on port".to_string(),
                    log_odds: -0.8,
                    interpretation: "Server process, probably needed".to_string(),
                },
                FactorExample {
                    input: "5 active connections".to_string(),
                    log_odds: -0.5,
                    interpretation: "Active client, doing work".to_string(),
                },
            ],
        },
    ]
}
