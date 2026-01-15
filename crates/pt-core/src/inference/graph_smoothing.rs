//! Graph-based Laplacian smoothing utilities.
//!
//! Applies conservative smoothing to per-node values (e.g., log-odds or
//! feature summaries) over an undirected graph. This is a deterministic
//! adjustment that preserves closed-form inference in the core.

use serde::Serialize;
use thiserror::Error;

/// Configuration for graph smoothing.
#[derive(Debug, Clone, Serialize)]
pub struct GraphSmoothingConfig {
    /// Enable smoothing. When disabled, values are returned unchanged.
    pub enabled: bool,
    /// Mixing coefficient in [0, 1]. Higher values smooth more aggressively.
    pub alpha: f64,
    /// Number of smoothing iterations.
    pub iterations: usize,
}

impl Default for GraphSmoothingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            alpha: 0.3,
            iterations: 2,
        }
    }
}

/// Result of smoothing.
#[derive(Debug, Clone, Serialize)]
pub struct GraphSmoothingResult {
    pub values: Vec<f64>,
    pub iterations: usize,
    pub alpha: f64,
    pub average_delta: f64,
    pub enabled: bool,
}

/// Errors returned by graph smoothing.
#[derive(Debug, Error)]
pub enum GraphSmoothingError {
    #[error("invalid alpha: {value}")]
    InvalidAlpha { value: f64 },
    #[error("node count mismatch: values={values}, nodes={nodes}")]
    NodeCountMismatch { values: usize, nodes: usize },
}

/// Apply Laplacian-style smoothing over an undirected graph.
pub fn smooth_values(
    values: &[f64],
    edges: &[(usize, usize)],
    config: &GraphSmoothingConfig,
) -> Result<GraphSmoothingResult, GraphSmoothingError> {
    if !config.enabled {
        return Ok(GraphSmoothingResult {
            values: values.to_vec(),
            iterations: 0,
            alpha: config.alpha,
            average_delta: 0.0,
            enabled: false,
        });
    }
    if !(0.0..=1.0).contains(&config.alpha) || !config.alpha.is_finite() {
        return Err(GraphSmoothingError::InvalidAlpha {
            value: config.alpha,
        });
    }

    let n = values.len();
    let neighbors = build_neighbors(n, edges)?;
    let mut current = values.to_vec();
    let mut avg_delta = 0.0;

    for _ in 0..config.iterations {
        let mut next = current.clone();
        let mut total_delta = 0.0;
        let mut count = 0usize;

        for idx in 0..n {
            let neighbor_avg = if neighbors[idx].is_empty() {
                current[idx]
            } else {
                neighbors[idx]
                    .iter()
                    .map(|&j| current[j])
                    .sum::<f64>()
                    / neighbors[idx].len() as f64
            };
            let updated = (1.0 - config.alpha) * current[idx] + config.alpha * neighbor_avg;
            total_delta += (updated - current[idx]).abs();
            count += 1;
            next[idx] = updated;
        }

        avg_delta = if count == 0 {
            0.0
        } else {
            total_delta / count as f64
        };
        current = next;
    }

    Ok(GraphSmoothingResult {
        values: current,
        iterations: config.iterations,
        alpha: config.alpha,
        average_delta: avg_delta,
        enabled: true,
    })
}

/// Build neighbor lists from undirected edges.
pub fn build_neighbors(
    node_count: usize,
    edges: &[(usize, usize)],
) -> Result<Vec<Vec<usize>>, GraphSmoothingError> {
    let mut neighbors = vec![Vec::new(); node_count];
    for &(a, b) in edges {
        if a >= node_count || b >= node_count {
            return Err(GraphSmoothingError::NodeCountMismatch {
                values: node_count,
                nodes: node_count,
            });
        }
        if a == b {
            continue;
        }
        neighbors[a].push(b);
        neighbors[b].push(a);
    }
    Ok(neighbors)
}

/// Create undirected edges for each cluster of nodes.
pub fn edges_from_clusters(clusters: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    for cluster in clusters {
        for i in 0..cluster.len() {
            for j in (i + 1)..cluster.len() {
                edges.push((cluster[i], cluster[j]));
            }
        }
    }
    edges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled_returns_original_values() {
        let values = vec![1.0, 2.0, 3.0];
        let result = smooth_values(&values, &[], &GraphSmoothingConfig::default()).unwrap();
        assert_eq!(result.values, values);
        assert!(!result.enabled);
    }

    #[test]
    fn smoothing_moves_values_toward_neighbors() {
        let values = vec![0.0, 10.0, 0.0];
        let edges = vec![(0, 1), (1, 2)];
        let config = GraphSmoothingConfig {
            enabled: true,
            alpha: 0.5,
            iterations: 1,
        };
        let result = smooth_values(&values, &edges, &config).unwrap();
        assert!(result.values[1] < values[1]);
        assert!(result.values[0] > values[0]);
        assert!(result.values[2] > values[2]);
    }

    #[test]
    fn edges_from_clusters_builds_cliques() {
        let clusters = vec![vec![0, 1, 2], vec![3, 4]];
        let edges = edges_from_clusters(&clusters);
        assert!(edges.contains(&(0, 1)));
        assert!(edges.contains(&(1, 2)));
        assert!(edges.contains(&(0, 2)));
        assert!(edges.contains(&(3, 4)));
    }
}
