//! Report section data structures.

pub mod overview;
pub mod candidates;
pub mod evidence;
pub mod actions;
pub mod galaxy_brain;

pub use overview::OverviewSection;
pub use candidates::{CandidateRow, CandidatesSection};
pub use evidence::{EvidenceFactor, EvidenceLedger, EvidenceSection};
pub use actions::{ActionRow, ActionsSection};
pub use galaxy_brain::GalaxyBrainSection;
