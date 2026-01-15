//! Evidence section data.

use serde::{Deserialize, Serialize};

/// Single evidence factor with weight.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceFactor {
    /// Factor name (e.g., "age", "cpu", "memory").
    pub name: String,
    /// Human-readable label.
    pub label: String,
    /// Log-odds contribution (positive = toward abandoned).
    pub log_odds: f64,
    /// Whether this factor favors abandoned classification.
    pub favors_abandoned: bool,
    /// Raw value for context.
    pub raw_value: Option<String>,
    /// Interpretation text.
    pub interpretation: Option<String>,
}

impl EvidenceFactor {
    /// Get display class based on factor direction.
    pub fn direction_class(&self) -> &'static str {
        if self.favors_abandoned {
            "text-red-600 dark:text-red-400"
        } else {
            "text-green-600 dark:text-green-400"
        }
    }

    /// Get arrow indicator.
    pub fn direction_arrow(&self) -> &'static str {
        if self.favors_abandoned {
            "\u{2191}" // up arrow
        } else {
            "\u{2193}" // down arrow
        }
    }

    /// Get absolute magnitude for bar width (0-100%).
    pub fn bar_width(&self) -> u8 {
        // Cap at 2.0 log-odds for visualization
        let abs = self.log_odds.abs().min(2.0);
        (abs * 50.0) as u8
    }
}

/// Evidence ledger for a single candidate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceLedger {
    /// Process ID.
    pub pid: u32,
    /// Process start ID.
    pub start_id: String,
    /// Command name.
    pub cmd: String,
    /// Prior probability.
    pub prior_p: f64,
    /// Posterior probability (p_abandoned).
    pub posterior_p: f64,
    /// Log Bayes factor.
    pub log_bf: f64,
    /// Bayes factor interpretation.
    pub bf_interpretation: String,
    /// Individual evidence factors.
    pub factors: Vec<EvidenceFactor>,
    /// Evidence tags.
    pub tags: Vec<String>,
}

impl EvidenceLedger {
    /// Get total log-odds change.
    pub fn total_log_odds(&self) -> f64 {
        self.factors.iter().map(|f| f.log_odds).sum()
    }

    /// Get factors sorted by absolute contribution.
    pub fn factors_by_importance(&self) -> Vec<&EvidenceFactor> {
        let mut sorted: Vec<_> = self.factors.iter().collect();
        sorted.sort_by(|a, b| {
            b.log_odds
                .abs()
                .partial_cmp(&a.log_odds.abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        sorted
    }

    /// Get top N contributing factors.
    pub fn top_factors(&self, n: usize) -> Vec<&EvidenceFactor> {
        self.factors_by_importance().into_iter().take(n).collect()
    }
}

/// Evidence section containing all ledgers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceSection {
    /// Evidence ledgers by candidate.
    pub ledgers: Vec<EvidenceLedger>,
    /// Factor definitions for legend.
    pub factor_definitions: Vec<FactorDefinition>,
}

/// Definition of an evidence factor type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactorDefinition {
    /// Factor key.
    pub key: String,
    /// Human-readable name.
    pub name: String,
    /// Description.
    pub description: String,
    /// Category (timing, resource, state, etc.).
    pub category: String,
}

impl EvidenceSection {
    /// Create a new evidence section.
    pub fn new(ledgers: Vec<EvidenceLedger>) -> Self {
        Self {
            ledgers,
            factor_definitions: default_factor_definitions(),
        }
    }

    /// Get ledger for a specific PID.
    pub fn ledger_for(&self, pid: u32) -> Option<&EvidenceLedger> {
        self.ledgers.iter().find(|l| l.pid == pid)
    }
}

/// Default factor definitions.
fn default_factor_definitions() -> Vec<FactorDefinition> {
    vec![
        FactorDefinition {
            key: "prior".to_string(),
            name: "Prior".to_string(),
            description: "Base probability from process classification".to_string(),
            category: "classification".to_string(),
        },
        FactorDefinition {
            key: "age".to_string(),
            name: "Age".to_string(),
            description: "Process age relative to session duration".to_string(),
            category: "timing".to_string(),
        },
        FactorDefinition {
            key: "cpu".to_string(),
            name: "CPU".to_string(),
            description: "CPU utilization and activity patterns".to_string(),
            category: "resource".to_string(),
        },
        FactorDefinition {
            key: "memory".to_string(),
            name: "Memory".to_string(),
            description: "Memory usage and growth patterns".to_string(),
            category: "resource".to_string(),
        },
        FactorDefinition {
            key: "io".to_string(),
            name: "I/O".to_string(),
            description: "Disk and network I/O activity".to_string(),
            category: "resource".to_string(),
        },
        FactorDefinition {
            key: "state".to_string(),
            name: "State".to_string(),
            description: "Process state (sleeping, stopped, zombie)".to_string(),
            category: "state".to_string(),
        },
        FactorDefinition {
            key: "network".to_string(),
            name: "Network".to_string(),
            description: "Network connections and listening ports".to_string(),
            category: "connectivity".to_string(),
        },
        FactorDefinition {
            key: "children".to_string(),
            name: "Children".to_string(),
            description: "Child process activity".to_string(),
            category: "hierarchy".to_string(),
        },
        FactorDefinition {
            key: "history".to_string(),
            name: "History".to_string(),
            description: "Prior decisions on similar processes".to_string(),
            category: "learning".to_string(),
        },
        FactorDefinition {
            key: "deep".to_string(),
            name: "Deep Scan".to_string(),
            description: "Evidence from deep scan (file handles, syscalls)".to_string(),
            category: "deep".to_string(),
        },
    ]
}
