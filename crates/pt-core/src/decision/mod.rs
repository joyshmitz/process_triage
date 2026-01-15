//! Decision theory utilities (expected loss, thresholds, FDR control, policy enforcement).

pub mod enforcer;
pub mod expected_loss;
pub mod alpha_investing;
pub mod fdr_selection;

pub use enforcer::{
    EnforcerError, PolicyCheckResult, PolicyEnforcer, PolicyViolation, ProcessCandidate,
    ViolationKind,
};
pub use expected_loss::{
    decide_action, Action, ActionFeasibility, DecisionError, DecisionOutcome, DecisionRationale,
    DisabledAction, ExpectedLoss, SprtBoundary,
};
pub use alpha_investing::{AlphaInvestingPolicy, AlphaInvestingStore, AlphaUpdate, AlphaWealthState};
pub use fdr_selection::{
    by_correction_factor, select_fdr, CandidateSelection, FdrCandidate, FdrError, FdrMethod,
    FdrSelectionResult, TargetIdentity,
};
