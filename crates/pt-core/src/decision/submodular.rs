//! Submodular probe selection utilities (overlap-aware).
//!
//! This module provides a simple, composable interface for selecting probe bundles
//! when probes overlap in information. The default utility is a weighted coverage
//! function over probe features, which is monotone submodular and admits greedy
//! approximation guarantees (1 - 1/e).

use crate::decision::voi::ProbeType;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Feature identifier used for overlap-aware utilities.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct FeatureKey(String);

impl FeatureKey {
    /// Create a new feature key.
    pub fn new<S: Into<String>>(value: S) -> Self {
        Self(value.into())
    }

    /// Access the feature key as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for FeatureKey {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

/// Probe profile with cost and covered features.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeProfile {
    pub probe: ProbeType,
    pub cost: f64,
    pub features: Vec<FeatureKey>,
}

impl ProbeProfile {
    pub fn new(probe: ProbeType, cost: f64, features: Vec<FeatureKey>) -> Self {
        Self {
            probe,
            cost: cost.max(0.0),
            features,
        }
    }
}

/// Result of a greedy selection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionResult {
    /// Selected probe types in order of selection.
    pub selected: Vec<ProbeType>,
    /// Total cost of selected probes.
    pub total_cost: f64,
    /// Total utility achieved.
    pub total_utility: f64,
}

/// Compute weighted coverage utility for a set of probes.
pub fn coverage_utility(
    probes: &[ProbeProfile],
    feature_weights: &HashMap<FeatureKey, f64>,
) -> f64 {
    let mut seen = HashSet::new();
    let mut total = 0.0;

    for probe in probes {
        for feature in &probe.features {
            if seen.insert(feature.clone()) {
                total += feature_weight(feature_weights, feature);
            }
        }
    }

    total
}

/// Compute marginal gain of adding a probe given currently covered features.
pub fn coverage_marginal_gain(
    covered: &HashSet<FeatureKey>,
    probe: &ProbeProfile,
    feature_weights: &HashMap<FeatureKey, f64>,
) -> f64 {
    let mut gain = 0.0;
    for feature in &probe.features {
        if !covered.contains(feature) {
            gain += feature_weight(feature_weights, feature);
        }
    }
    gain
}

/// Greedy selection under a total cost budget.
///
/// This is the standard greedy algorithm for monotone submodular maximization
/// with a knapsack constraint, using marginal gain per unit cost.
pub fn greedy_select_with_budget(
    probes: &[ProbeProfile],
    feature_weights: &HashMap<FeatureKey, f64>,
    budget: f64,
) -> SelectionResult {
    let mut covered = HashSet::new();
    let mut remaining: Vec<usize> = (0..probes.len()).collect();
    let mut selected = Vec::new();
    let mut total_cost = 0.0;
    let mut total_utility = 0.0;

    loop {
        let mut best_idx = None;
        let mut best_score = 0.0;
        let mut best_gain = 0.0;
        let mut best_cost = 0.0;

        for &idx in &remaining {
            let probe = &probes[idx];
            if total_cost + probe.cost > budget {
                continue;
            }

            let gain = coverage_marginal_gain(&covered, probe, feature_weights);
            if gain <= 0.0 {
                continue;
            }

            let score = if probe.cost > 0.0 {
                gain / probe.cost
            } else {
                f64::INFINITY
            };

            if score > best_score {
                best_score = score;
                best_idx = Some(idx);
                best_gain = gain;
                best_cost = probe.cost;
            }
        }

        let Some(idx) = best_idx else { break };
        let probe = &probes[idx];
        for feature in &probe.features {
            covered.insert(feature.clone());
        }

        total_cost += best_cost;
        total_utility += best_gain;
        selected.push(probe.probe);

        remaining.retain(|&i| i != idx);
        if remaining.is_empty() {
            break;
        }
    }

    SelectionResult {
        selected,
        total_cost,
        total_utility,
    }
}

/// Greedy selection of up to `k` probes by marginal gain.
pub fn greedy_select_k(
    probes: &[ProbeProfile],
    feature_weights: &HashMap<FeatureKey, f64>,
    k: usize,
) -> SelectionResult {
    let mut covered = HashSet::new();
    let mut remaining: Vec<usize> = (0..probes.len()).collect();
    let mut selected = Vec::new();
    let mut total_cost = 0.0;
    let mut total_utility = 0.0;

    for _ in 0..k {
        let mut best_idx = None;
        let mut best_gain = 0.0;

        for &idx in &remaining {
            let probe = &probes[idx];
            let gain = coverage_marginal_gain(&covered, probe, feature_weights);
            if gain > best_gain {
                best_gain = gain;
                best_idx = Some(idx);
            }
        }

        let Some(idx) = best_idx else { break };
        let probe = &probes[idx];
        for feature in &probe.features {
            covered.insert(feature.clone());
        }

        total_cost += probe.cost;
        total_utility += best_gain;
        selected.push(probe.probe);
        remaining.retain(|&i| i != idx);
        if remaining.is_empty() {
            break;
        }
    }

    SelectionResult {
        selected,
        total_cost,
        total_utility,
    }
}

fn feature_weight(feature_weights: &HashMap<FeatureKey, f64>, feature: &FeatureKey) -> f64 {
    feature_weights.get(feature).copied().unwrap_or(1.0).max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn weights() -> HashMap<FeatureKey, f64> {
        HashMap::new()
    }

    #[test]
    fn test_diminishing_returns_with_overlap() {
        let probes = vec![
            ProbeProfile::new(ProbeType::QuickScan, 1.0, vec!["cpu".into()]),
            ProbeProfile::new(ProbeType::DeepScan, 1.0, vec!["cpu".into()]),
            ProbeProfile::new(ProbeType::NetSnapshot, 1.0, vec!["net".into()]),
        ];

        let mut covered = HashSet::new();
        let gain_first = coverage_marginal_gain(&covered, &probes[0], &weights());
        for feature in &probes[0].features {
            covered.insert(feature.clone());
        }
        let gain_overlap = coverage_marginal_gain(&covered, &probes[1], &weights());
        let gain_distinct = coverage_marginal_gain(&covered, &probes[2], &weights());

        assert!(gain_first > 0.0);
        assert_eq!(gain_overlap, 0.0);
        assert!(gain_distinct > 0.0);
    }

    #[test]
    fn test_greedy_respects_budget() {
        let probes = vec![
            ProbeProfile::new(ProbeType::QuickScan, 2.0, vec!["cpu".into()]),
            ProbeProfile::new(ProbeType::DeepScan, 3.0, vec!["io".into()]),
            ProbeProfile::new(ProbeType::NetSnapshot, 4.0, vec!["net".into()]),
        ];

        let result = greedy_select_with_budget(&probes, &weights(), 5.0);
        assert!(result.total_cost <= 5.0 + 1e-8);
    }

    #[test]
    fn test_greedy_near_optimal_on_small_case() {
        let probes = vec![
            ProbeProfile::new(ProbeType::QuickScan, 2.0, vec!["cpu".into(), "io".into()]),
            ProbeProfile::new(ProbeType::DeepScan, 2.5, vec!["cpu".into(), "net".into()]),
            ProbeProfile::new(ProbeType::NetSnapshot, 1.0, vec!["net".into()]),
            ProbeProfile::new(ProbeType::IoSnapshot, 1.0, vec!["io".into()]),
        ];

        let budget = 3.0;
        let greedy = greedy_select_with_budget(&probes, &weights(), budget);
        let optimal = brute_force_best(&probes, &weights(), budget);

        assert!(optimal > 0.0);
        assert!(
            greedy.total_utility >= 0.9 * optimal,
            "greedy {:.2} should be close to optimal {:.2}",
            greedy.total_utility,
            optimal
        );
    }

    fn brute_force_best(
        probes: &[ProbeProfile],
        feature_weights: &HashMap<FeatureKey, f64>,
        budget: f64,
    ) -> f64 {
        let n = probes.len();
        let mut best = 0.0;

        for mask in 0..(1usize << n) {
            let mut subset = Vec::new();
            let mut cost = 0.0;
            for i in 0..n {
                if (mask & (1 << i)) != 0 {
                    subset.push(probes[i].clone());
                    cost += probes[i].cost;
                }
            }
            if cost > budget {
                continue;
            }
            let utility = coverage_utility(&subset, feature_weights);
            if utility > best {
                best = utility;
            }
        }

        best
    }
}
