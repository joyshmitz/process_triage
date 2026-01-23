//! Report section data structures.

pub mod actions;
pub mod candidates;
pub mod evidence;
pub mod galaxy_brain;
pub mod overview;

pub use actions::{ActionRow, ActionsSection};
pub use candidates::{CandidateRow, CandidatesSection};
pub use evidence::{EvidenceFactor, EvidenceLedger, EvidenceSection};
pub use galaxy_brain::GalaxyBrainSection;
pub use overview::OverviewSection;
