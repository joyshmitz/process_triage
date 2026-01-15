//! HTML report generator for process triage sessions.
//!
//! Generates self-contained HTML reports from session data or `.ptb` bundles.
//!
//! # Features
//!
//! - **Single-file output**: Reports are standalone HTML with CDN-loaded assets
//! - **Offline mode**: `--embed-assets` inlines all dependencies for `file://` usage
//! - **CDN pinning**: All libraries use pinned versions with SRI hashes
//! - **Galaxy-brain tab**: Optional math transparency with KaTeX rendering
//! - **Redaction-aware**: Respects export profile for sensitive data
//!
//! # Sections
//!
//! - Overview: Session summary, timing, system info
//! - Candidates: Interactive table of candidate processes
//! - Evidence: Expandable evidence ledgers with factor weights
//! - Actions: Timeline of actions taken and outcomes
//! - Telemetry: Interactive charts of resource usage
//! - Galaxy-brain: Mathematical derivation of Bayesian inference
//!
//! # Example
//!
//! ```no_run
//! use pt_report::{ReportGenerator, ReportConfig};
//! use pt_bundle::BundleReader;
//! use std::path::Path;
//!
//! // Generate from a bundle
//! let mut reader = BundleReader::open(Path::new("session.ptb")).unwrap();
//! let config = ReportConfig::default();
//! let generator = ReportGenerator::new(config);
//! let html = generator.generate_from_bundle(&mut reader).unwrap();
//! ```

pub mod config;
pub mod error;
pub mod generator;
pub mod sections;

pub use config::{CdnLibrary, ReportConfig, ReportSections, ReportTheme};
pub use error::{ReportError, Result};
pub use generator::ReportGenerator;
