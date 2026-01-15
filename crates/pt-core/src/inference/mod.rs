//! Inference engine modules.

pub mod ledger;
pub mod posterior;

pub use ledger::{
    default_glyph_map, get_glyph, BayesFactorEntry, Classification, Confidence, Direction,
    EvidenceLedger, FeatureGlyph,
};
pub use posterior::{
    compute_posterior, ClassScores, CpuEvidence, Evidence, EvidenceTerm, PosteriorError,
    PosteriorResult,
};
