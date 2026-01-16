//! Load-aware decision tuning for adaptive thresholds.

use crate::config::policy::{LoadAwareDecision, LossMatrix, LossRow};

/// Observed system signals used to compute load score.
#[derive(Debug, Clone)]
pub struct LoadSignals {
    pub queue_len: usize,
    pub load1: Option<f64>,
    pub cores: Option<u32>,
    pub memory_used_fraction: Option<f64>,
    pub psi_avg10: Option<f64>,
}

/// Computed adjustment derived from load signals.
#[derive(Debug, Clone)]
pub struct LoadAdjustment {
    pub load_score: f64,
    pub keep_multiplier: f64,
    pub reversible_multiplier: f64,
    pub risky_multiplier: f64,
}

impl LoadSignals {
    /// Build load signals from system_state JSON and queue length.
    pub fn from_system_state(system_state: &serde_json::Value, queue_len: usize) -> Self {
        let load1 = system_state
            .get("load")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_f64());

        let cores = system_state
            .get("cores")
            .and_then(|v| v.as_u64())
            .map(|v| v as u32);

        let (used_gb, total_gb) = system_state
            .get("memory")
            .and_then(|mem| {
                let used = mem.get("used_gb").and_then(|v| v.as_f64())?;
                let total = mem.get("total_gb").and_then(|v| v.as_f64())?;
                Some((used, total))
            })
            .unwrap_or((0.0, 0.0));

        let memory_used_fraction = if total_gb > 0.0 {
            Some((used_gb / total_gb).clamp(0.0, 1.0))
        } else {
            None
        };

        let psi_avg10 = system_state.get("psi").and_then(|psi| {
            let cpu = psi.get("cpu").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let mem = psi.get("memory").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let io = psi.get("io").and_then(|v| v.as_f64()).unwrap_or(0.0);
            Some(cpu.max(mem).max(io))
        });

        Self {
            queue_len,
            load1,
            cores,
            memory_used_fraction,
            psi_avg10,
        }
    }
}

/// Compute load adjustment from signals and policy configuration.
pub fn compute_load_adjustment(
    config: &LoadAwareDecision,
    signals: &LoadSignals,
) -> Option<LoadAdjustment> {
    if !config.enabled {
        return None;
    }

    let queue_score = if config.queue_high > 0 {
        (signals.queue_len as f64 / config.queue_high as f64).min(1.0)
    } else {
        0.0
    };

    let load_score = match (signals.load1, signals.cores) {
        (Some(load1), Some(cores)) if cores > 0 && config.load_per_core_high > 0.0 => {
            (load1 / (cores as f64 * config.load_per_core_high)).min(1.0)
        }
        _ => 0.0,
    };

    let memory_score = match signals.memory_used_fraction {
        Some(frac) if config.memory_used_fraction_high > 0.0 => {
            (frac / config.memory_used_fraction_high).min(1.0)
        }
        _ => 0.0,
    };

    let psi_score = match signals.psi_avg10 {
        Some(psi) if config.psi_avg10_high > 0.0 => (psi / config.psi_avg10_high).min(1.0),
        _ => 0.0,
    };

    let weight_sum =
        config.weights.queue + config.weights.load + config.weights.memory + config.weights.psi;
    if weight_sum <= 0.0 {
        return None;
    }

    let load_score = ((config.weights.queue * queue_score)
        + (config.weights.load * load_score)
        + (config.weights.memory * memory_score)
        + (config.weights.psi * psi_score))
        / weight_sum;

    let keep_multiplier =
        1.0 + load_score * (config.multipliers.keep_max - 1.0).max(0.0);
    let reversible_multiplier = 1.0
        - load_score * (1.0 - config.multipliers.reversible_min).max(0.0);
    let risky_multiplier =
        1.0 + load_score * (config.multipliers.risky_max - 1.0).max(0.0);

    Some(LoadAdjustment {
        load_score,
        keep_multiplier,
        reversible_multiplier,
        risky_multiplier,
    })
}

/// Apply a load adjustment to the loss matrix.
pub fn apply_load_to_loss_matrix(loss: &LossMatrix, adjustment: &LoadAdjustment) -> LossMatrix {
    LossMatrix {
        useful: apply_load_to_loss_row(loss.useful.clone(), adjustment),
        useful_bad: apply_load_to_loss_row(loss.useful_bad.clone(), adjustment),
        abandoned: apply_load_to_loss_row(loss.abandoned.clone(), adjustment),
        zombie: apply_load_to_loss_row(loss.zombie.clone(), adjustment),
    }
}

fn apply_load_to_loss_row(row: LossRow, adjustment: &LoadAdjustment) -> LossRow {
    LossRow {
        keep: row.keep * adjustment.keep_multiplier,
        pause: row.pause.map(|v| v * adjustment.reversible_multiplier),
        throttle: row.throttle.map(|v| v * adjustment.reversible_multiplier),
        renice: row.renice.map(|v| v * adjustment.reversible_multiplier),
        kill: row.kill * adjustment.risky_multiplier,
        restart: row.restart.map(|v| v * adjustment.risky_multiplier),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::policy::{LoadAwareDecision, LossMatrix, LossRow};

    #[test]
    fn test_load_adjustment_zero_load() {
        let cfg = LoadAwareDecision {
            enabled: true,
            ..LoadAwareDecision::default()
        };
        let signals = LoadSignals {
            queue_len: 0,
            load1: Some(0.0),
            cores: Some(8),
            memory_used_fraction: Some(0.0),
            psi_avg10: Some(0.0),
        };
        let adj = compute_load_adjustment(&cfg, &signals).expect("adjustment");
        assert!((adj.load_score - 0.0).abs() < 1e-6);
        assert!((adj.keep_multiplier - 1.0).abs() < 1e-6);
        assert!((adj.reversible_multiplier - 1.0).abs() < 1e-6);
        assert!((adj.risky_multiplier - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_load_adjustment_high_load_saturates() {
        let cfg = LoadAwareDecision {
            enabled: true,
            ..LoadAwareDecision::default()
        };
        let signals = LoadSignals {
            queue_len: 10_000,
            load1: Some(10_000.0),
            cores: Some(1),
            memory_used_fraction: Some(1.0),
            psi_avg10: Some(100.0),
        };
        let adj = compute_load_adjustment(&cfg, &signals).expect("adjustment");
        assert!((adj.load_score - 1.0).abs() < 1e-6);
        assert!((adj.keep_multiplier - cfg.multipliers.keep_max).abs() < 1e-6);
        assert!((adj.reversible_multiplier - cfg.multipliers.reversible_min).abs() < 1e-6);
        assert!((adj.risky_multiplier - cfg.multipliers.risky_max).abs() < 1e-6);
    }

    #[test]
    fn test_apply_load_to_loss_matrix() {
        let loss = LossMatrix {
            useful: LossRow {
                keep: 10.0,
                pause: Some(4.0),
                throttle: Some(6.0),
                renice: Some(3.0),
                kill: 100.0,
                restart: Some(50.0),
            },
            useful_bad: LossRow {
                keep: 10.0,
                pause: Some(4.0),
                throttle: Some(6.0),
                renice: Some(3.0),
                kill: 100.0,
                restart: Some(50.0),
            },
            abandoned: LossRow {
                keep: 10.0,
                pause: Some(4.0),
                throttle: Some(6.0),
                renice: Some(3.0),
                kill: 100.0,
                restart: Some(50.0),
            },
            zombie: LossRow {
                keep: 10.0,
                pause: Some(4.0),
                throttle: Some(6.0),
                renice: Some(3.0),
                kill: 100.0,
                restart: Some(50.0),
            },
        };

        let adjustment = LoadAdjustment {
            load_score: 0.5,
            keep_multiplier: 1.2,
            reversible_multiplier: 0.8,
            risky_multiplier: 1.5,
        };
        let adjusted = apply_load_to_loss_matrix(&loss, &adjustment);

        let epsilon = 1e-10;
        assert!((adjusted.useful.keep - 12.0).abs() < epsilon);
        assert!((adjusted.useful.pause.unwrap() - 3.2).abs() < epsilon);
        assert!((adjusted.useful.renice.unwrap() - 2.4).abs() < epsilon);
        assert!((adjusted.useful.kill - 150.0).abs() < epsilon);
        assert!((adjusted.useful.restart.unwrap() - 75.0).abs() < epsilon);
    }
}
