//! Beta-Stacy discrete-time survival model (Bayesian nonparametric hazard).
//!
//! Implements a discrete-time hazard model with Beta priors per time bin.
//! Closed-form posterior updates:
//!   h_t ~ Beta(a_t, b_t)
//!   h_t | data ~ Beta(a_t + d_t, b_t + n_t - d_t)
//! Survival:
//!   S(t) = ∏_{j=1..t} (1 - E[h_j | data])

use serde::Serialize;
use thiserror::Error;

/// Versioned binning scheme for discrete-time hazards.
#[derive(Debug, Clone, Serialize)]
pub struct BinningScheme {
    pub version: String,
    pub kind: BinningKind,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BinningKind {
    Fixed {
        bin_width_s: f64,
        max_bins: usize,
    },
    Log {
        start_width_s: f64,
        growth_factor: f64,
        max_bins: usize,
    },
}

/// A single time bin specification.
#[derive(Debug, Clone, Serialize)]
pub struct BinSpec {
    pub index: usize,
    pub start_s: f64,
    pub end_s: f64,
}

impl BinningScheme {
    pub fn fixed(bin_width_s: f64, max_bins: usize) -> Self {
        Self {
            version: "1.0.0".to_string(),
            kind: BinningKind::Fixed {
                bin_width_s,
                max_bins,
            },
        }
    }

    pub fn log(start_width_s: f64, growth_factor: f64, max_bins: usize) -> Self {
        Self {
            version: "1.0.0".to_string(),
            kind: BinningKind::Log {
                start_width_s,
                growth_factor,
                max_bins,
            },
        }
    }

    pub fn bins(&self) -> Vec<BinSpec> {
        match self.kind {
            BinningKind::Fixed {
                bin_width_s,
                max_bins,
            } => (0..max_bins)
                .map(|idx| BinSpec {
                    index: idx,
                    start_s: bin_width_s * idx as f64,
                    end_s: bin_width_s * (idx as f64 + 1.0),
                })
                .collect(),
            BinningKind::Log {
                start_width_s,
                growth_factor,
                max_bins,
            } => {
                let mut bins = Vec::with_capacity(max_bins);
                let mut start = 0.0;
                let mut width = start_width_s;
                for idx in 0..max_bins {
                    let end = start + width;
                    bins.push(BinSpec {
                        index: idx,
                        start_s: start,
                        end_s: end,
                    });
                    start = end;
                    width *= growth_factor;
                }
                bins
            }
        }
    }

    pub fn index_for_duration(&self, duration_s: f64) -> Option<usize> {
        if duration_s < 0.0 || duration_s.is_nan() {
            return None;
        }
        match self.kind {
            BinningKind::Fixed {
                bin_width_s,
                max_bins,
            } => {
                if bin_width_s <= 0.0 {
                    return None;
                }
                let idx = (duration_s / bin_width_s).floor() as usize;
                if idx < max_bins {
                    Some(idx)
                } else {
                    None
                }
            }
            BinningKind::Log {
                start_width_s,
                growth_factor,
                max_bins,
            } => {
                if start_width_s <= 0.0 || growth_factor <= 1.0 {
                    return None;
                }
                let mut start = 0.0;
                let mut width = start_width_s;
                for idx in 0..max_bins {
                    let end = start + width;
                    if duration_s < end {
                        return Some(idx);
                    }
                    start = end;
                    width *= growth_factor;
                }
                None
            }
        }
    }
}

/// Beta prior parameters for a hazard bin.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct BetaParams {
    pub alpha: f64,
    pub beta: f64,
}

impl BetaParams {
    pub fn new(alpha: f64, beta: f64) -> Self {
        Self { alpha, beta }
    }

    pub fn mean(&self) -> f64 {
        let denom = self.alpha + self.beta;
        if denom <= 0.0 {
            0.0
        } else {
            self.alpha / denom
        }
    }
}

impl Default for BetaParams {
    fn default() -> Self {
        Self { alpha: 1.0, beta: 1.0 }
    }
}

/// A single Beta-Stacy hazard bin.
#[derive(Debug, Clone, Serialize)]
pub struct BetaStacyBin {
    pub index: usize,
    pub prior: BetaParams,
    pub at_risk: u32,
    pub events: u32,
}

impl BetaStacyBin {
    pub fn posterior(&self) -> BetaParams {
        BetaParams::new(
            self.prior.alpha + self.events as f64,
            self.prior.beta + (self.at_risk.saturating_sub(self.events)) as f64,
        )
    }

    pub fn hazard_mean(&self) -> f64 {
        let posterior = self.posterior();
        posterior.mean().clamp(0.0, 1.0)
    }
}

/// A discrete-time survival model with Beta priors per bin.
#[derive(Debug, Clone, Serialize)]
pub struct BetaStacyModel {
    pub scheme: BinningScheme,
    pub bins: Vec<BetaStacyBin>,
}

/// A lifetime sample for updating the model.
#[derive(Debug, Clone, Copy)]
pub struct LifetimeSample {
    pub duration_s: f64,
    pub event: bool,
}

#[derive(Debug, Error)]
pub enum BetaStacyError {
    #[error("binning scheme produced no bins")]
    NoBins,
    #[error("invalid duration {value}")]
    InvalidDuration { value: f64 },
}

impl BetaStacyModel {
    pub fn new(scheme: BinningScheme, prior: BetaParams) -> Result<Self, BetaStacyError> {
        let bins = scheme
            .bins()
            .into_iter()
            .map(|spec| BetaStacyBin {
                index: spec.index,
                prior,
                at_risk: 0,
                events: 0,
            })
            .collect::<Vec<_>>();
        if bins.is_empty() {
            return Err(BetaStacyError::NoBins);
        }
        Ok(Self { scheme, bins })
    }

    pub fn update_from_counts(&mut self, at_risk: &[u32], events: &[u32]) {
        for (idx, bin) in self.bins.iter_mut().enumerate() {
            if idx < at_risk.len() {
                bin.at_risk = bin.at_risk.saturating_add(at_risk[idx]);
            }
            if idx < events.len() {
                bin.events = bin.events.saturating_add(events[idx]);
            }
        }
    }

    pub fn update_from_samples(&mut self, samples: &[LifetimeSample]) -> Result<(), BetaStacyError> {
        let mut at_risk = vec![0u32; self.bins.len()];
        let mut events = vec![0u32; self.bins.len()];

        for sample in samples {
            if !sample.duration_s.is_finite() || sample.duration_s < 0.0 {
                return Err(BetaStacyError::InvalidDuration {
                    value: sample.duration_s,
                });
            }
            let idx = match self.scheme.index_for_duration(sample.duration_s) {
                Some(idx) => idx,
                None => continue,
            };
            for i in 0..=idx {
                at_risk[i] = at_risk[i].saturating_add(1);
            }
            if sample.event {
                events[idx] = events[idx].saturating_add(1);
            }
        }

        self.update_from_counts(&at_risk, &events);
        Ok(())
    }

    pub fn survival_curve(&self) -> Vec<f64> {
        let mut survival = Vec::with_capacity(self.bins.len());
        let mut s = 1.0;
        for bin in &self.bins {
            let h = bin.hazard_mean().clamp(0.0, 1.0);
            s *= 1.0 - h;
            survival.push(s.max(0.0));
        }
        survival
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn posterior_update_matches_counts() {
        let scheme = BinningScheme::fixed(10.0, 3);
        let prior = BetaParams::new(1.0, 1.0);
        let mut model = BetaStacyModel::new(scheme, prior).unwrap();

        let samples = vec![
            LifetimeSample { duration_s: 5.0, event: true },
            LifetimeSample { duration_s: 12.0, event: true },
            LifetimeSample { duration_s: 25.0, event: false },
        ];
        model.update_from_samples(&samples).unwrap();

        let bin0 = &model.bins[0];
        let bin1 = &model.bins[1];
        let bin2 = &model.bins[2];

        assert_eq!(bin0.at_risk, 3);
        assert_eq!(bin0.events, 1);
        assert_eq!(bin1.at_risk, 2);
        assert_eq!(bin1.events, 1);
        assert_eq!(bin2.at_risk, 1);
        assert_eq!(bin2.events, 0);

        let post0 = bin0.posterior();
        assert!((post0.alpha - 2.0).abs() < 1e-9);
        assert!((post0.beta - 3.0).abs() < 1e-9);
    }

    #[test]
    fn survival_curve_is_monotone() {
        let scheme = BinningScheme::fixed(10.0, 3);
        let prior = BetaParams::new(1.0, 1.0);
        let mut model = BetaStacyModel::new(scheme, prior).unwrap();
        let samples = vec![
            LifetimeSample { duration_s: 5.0, event: true },
            LifetimeSample { duration_s: 12.0, event: true },
            LifetimeSample { duration_s: 25.0, event: false },
        ];
        model.update_from_samples(&samples).unwrap();

        let curve = model.survival_curve();
        assert_eq!(curve.len(), 3);
        assert!(curve[0] >= curve[1]);
        assert!(curve[1] >= curve[2]);
    }
}
