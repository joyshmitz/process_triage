//! Structured prediction output for agent consumption.
//!
//! Defines the `Predictions` schema that can be optionally included in
//! agent-oriented JSON output (gated behind `--include-predictions`).
//! Covers memory slope, CPU trend, ETAs, and trajectory confidence.
//!
//! Token-efficient: fields are `skip_serializing_if` by default so only
//! populated fields appear in output.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Top-level predictions container
// ---------------------------------------------------------------------------

/// Per-process prediction output (opt-in via `--include-predictions`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Predictions {
    /// Memory trend prediction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<MemoryPrediction>,

    /// CPU trend prediction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<CpuPrediction>,

    /// Estimated time to abandonment (if trending that way).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eta_abandoned: Option<EtaPrediction>,

    /// Estimated time to resource exhaustion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eta_resource_limit: Option<EtaPrediction>,

    /// Overall trajectory assessment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trajectory: Option<TrajectoryAssessment>,

    /// Diagnostics about prediction quality.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<PredictionDiagnostics>,
}

impl Predictions {
    /// Returns true if all fields are None (nothing to show).
    pub fn is_empty(&self) -> bool {
        self.memory.is_none()
            && self.cpu.is_none()
            && self.eta_abandoned.is_none()
            && self.eta_resource_limit.is_none()
            && self.trajectory.is_none()
            && self.diagnostics.is_none()
    }
}

// ---------------------------------------------------------------------------
// Prediction components
// ---------------------------------------------------------------------------

/// Memory trend prediction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPrediction {
    /// RSS slope in bytes/second (positive = growing).
    pub rss_slope_bytes_per_sec: f64,
    /// Trend direction.
    pub trend: Trend,
    /// Confidence in the slope estimate (0..1).
    pub confidence: f64,
    /// Observation window in seconds.
    pub window_secs: f64,
}

/// CPU trend prediction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuPrediction {
    /// CPU usage slope in percent/second.
    pub usage_slope_pct_per_sec: f64,
    /// Trend direction.
    pub trend: Trend,
    /// Confidence in the slope estimate (0..1).
    pub confidence: f64,
    /// Observation window in seconds.
    pub window_secs: f64,
}

/// ETA prediction (time until some event).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtaPrediction {
    /// Estimated seconds until the event.
    pub eta_secs: f64,
    /// Confidence in the estimate (0..1).
    pub confidence: f64,
    /// Lower bound of credible interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_bound_secs: Option<f64>,
    /// Upper bound of credible interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_bound_secs: Option<f64>,
}

/// Trend direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Trend {
    Rising,
    Stable,
    Falling,
}

/// Overall trajectory assessment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryAssessment {
    /// Current trajectory label.
    pub label: TrajectoryLabel,
    /// Confidence in assessment (0..1).
    pub confidence: f64,
    /// Human-readable one-liner.
    pub summary: String,
}

/// Trajectory labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrajectoryLabel {
    /// Process is becoming more idle over time.
    WindingDown,
    /// Process is stable with consistent resource usage.
    Steady,
    /// Process resource usage is growing.
    Growing,
    /// Process shows erratic behaviour.
    Erratic,
    /// Insufficient data to assess.
    Unknown,
}

/// Diagnostics about prediction quality.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionDiagnostics {
    /// Number of data points used for prediction.
    pub n_observations: usize,
    /// Whether predictions are within a calibrated regime.
    pub calibrated: bool,
    /// Model used for predictions.
    pub model: String,
    /// Any warnings about prediction quality.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
}

// ---------------------------------------------------------------------------
// Field selection
// ---------------------------------------------------------------------------

/// Select which prediction subfields to include.
#[derive(Debug, Clone, Default)]
pub struct PredictionFieldSelector {
    /// If non-empty, only include these fields.
    pub include: Vec<PredictionField>,
}

/// Selectable prediction fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PredictionField {
    Memory,
    Cpu,
    EtaAbandoned,
    EtaResourceLimit,
    Trajectory,
    Diagnostics,
}

/// Apply field selection to predictions, clearing non-selected fields.
pub fn apply_field_selection(
    predictions: &Predictions,
    selector: &PredictionFieldSelector,
) -> Predictions {
    if selector.include.is_empty() {
        return predictions.clone();
    }

    let has = |f: PredictionField| selector.include.contains(&f);

    Predictions {
        memory: if has(PredictionField::Memory) {
            predictions.memory.clone()
        } else {
            None
        },
        cpu: if has(PredictionField::Cpu) {
            predictions.cpu.clone()
        } else {
            None
        },
        eta_abandoned: if has(PredictionField::EtaAbandoned) {
            predictions.eta_abandoned.clone()
        } else {
            None
        },
        eta_resource_limit: if has(PredictionField::EtaResourceLimit) {
            predictions.eta_resource_limit.clone()
        } else {
            None
        },
        trajectory: if has(PredictionField::Trajectory) {
            predictions.trajectory.clone()
        } else {
            None
        },
        diagnostics: if has(PredictionField::Diagnostics) {
            predictions.diagnostics.clone()
        } else {
            None
        },
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_predictions() -> Predictions {
        Predictions {
            memory: Some(MemoryPrediction {
                rss_slope_bytes_per_sec: -1024.0,
                trend: Trend::Falling,
                confidence: 0.85,
                window_secs: 3600.0,
            }),
            cpu: Some(CpuPrediction {
                usage_slope_pct_per_sec: -0.001,
                trend: Trend::Falling,
                confidence: 0.72,
                window_secs: 3600.0,
            }),
            eta_abandoned: Some(EtaPrediction {
                eta_secs: 86400.0,
                confidence: 0.6,
                lower_bound_secs: Some(43200.0),
                upper_bound_secs: Some(172800.0),
            }),
            eta_resource_limit: None,
            trajectory: Some(TrajectoryAssessment {
                label: TrajectoryLabel::WindingDown,
                confidence: 0.78,
                summary: "Process is gradually becoming idle.".to_string(),
            }),
            diagnostics: Some(PredictionDiagnostics {
                n_observations: 48,
                calibrated: true,
                model: "kalman".to_string(),
                warnings: vec![],
            }),
        }
    }

    #[test]
    fn test_empty_predictions() {
        let p = Predictions::default();
        assert!(p.is_empty());
    }

    #[test]
    fn test_non_empty_predictions() {
        let p = sample_predictions();
        assert!(!p.is_empty());
    }

    #[test]
    fn test_serialization_skips_none() {
        let p = Predictions {
            memory: Some(MemoryPrediction {
                rss_slope_bytes_per_sec: 0.0,
                trend: Trend::Stable,
                confidence: 0.5,
                window_secs: 60.0,
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&p).unwrap();
        assert!(json.contains("memory"));
        assert!(!json.contains("cpu"));
        assert!(!json.contains("trajectory"));
    }

    #[test]
    fn test_full_roundtrip() {
        let p = sample_predictions();
        let json = serde_json::to_string_pretty(&p).unwrap();
        let restored: Predictions = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.memory.as_ref().unwrap().trend, Trend::Falling);
        assert_eq!(
            restored.trajectory.as_ref().unwrap().label,
            TrajectoryLabel::WindingDown,
        );
    }

    #[test]
    fn test_field_selection_all() {
        let p = sample_predictions();
        let selector = PredictionFieldSelector::default(); // empty = include all
        let filtered = apply_field_selection(&p, &selector);
        assert!(filtered.memory.is_some());
        assert!(filtered.cpu.is_some());
        assert!(filtered.trajectory.is_some());
    }

    #[test]
    fn test_field_selection_memory_only() {
        let p = sample_predictions();
        let selector = PredictionFieldSelector {
            include: vec![PredictionField::Memory],
        };
        let filtered = apply_field_selection(&p, &selector);
        assert!(filtered.memory.is_some());
        assert!(filtered.cpu.is_none());
        assert!(filtered.trajectory.is_none());
        assert!(filtered.diagnostics.is_none());
    }

    #[test]
    fn test_field_selection_multiple() {
        let p = sample_predictions();
        let selector = PredictionFieldSelector {
            include: vec![PredictionField::Memory, PredictionField::Trajectory],
        };
        let filtered = apply_field_selection(&p, &selector);
        assert!(filtered.memory.is_some());
        assert!(filtered.cpu.is_none());
        assert!(filtered.trajectory.is_some());
    }

    #[test]
    fn test_eta_bounds_serialization() {
        let eta = EtaPrediction {
            eta_secs: 3600.0,
            confidence: 0.8,
            lower_bound_secs: None,
            upper_bound_secs: None,
        };
        let json = serde_json::to_string(&eta).unwrap();
        assert!(!json.contains("lower_bound"));
        assert!(!json.contains("upper_bound"));
    }

    #[test]
    fn test_diagnostics_warnings_skipped_when_empty() {
        let diag = PredictionDiagnostics {
            n_observations: 10,
            calibrated: false,
            model: "linear".to_string(),
            warnings: vec![],
        };
        let json = serde_json::to_string(&diag).unwrap();
        assert!(!json.contains("warnings"));
    }

    #[test]
    fn test_trend_values() {
        let json_rising = serde_json::to_string(&Trend::Rising).unwrap();
        assert_eq!(json_rising, "\"rising\"");

        let json_stable = serde_json::to_string(&Trend::Stable).unwrap();
        assert_eq!(json_stable, "\"stable\"");
    }
}
