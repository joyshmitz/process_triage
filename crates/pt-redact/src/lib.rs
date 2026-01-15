//! Redaction and hashing engine for process triage.
//!
//! This crate provides a single, reusable redaction engine that enforces
//! the redaction policy across all output surfaces: telemetry, bundles,
//! reports, agent outputs, and logs.
//!
//! # Key Features
//!
//! - **Field-aware sanitization**: Different field types (cmdline, env vars, paths)
//!   get appropriate handling based on their sensitivity level.
//! - **Keyed hashing**: HMAC-SHA256 with configurable key rotation prevents
//!   rainbow table attacks while enabling pattern matching across sessions.
//! - **Secret detection**: Automatic detection of API keys, tokens, and passwords
//!   using regex patterns and entropy analysis.
//! - **Canonicalization**: Normalizes values before hashing for stable pattern matching.
//! - **Fail-closed**: Errors never result in raw sensitive data being emitted.
//!
//! # Example
//!
//! ```no_run
//! use pt_redact::{RedactionEngine, FieldClass, RedactionPolicy};
//!
//! // Create engine with default policy
//! let policy = RedactionPolicy::default();
//! let engine = RedactionEngine::new(policy).unwrap();
//!
//! // Redact a command line
//! let result = engine.redact("--token=sk-1234567890", FieldClass::CmdlineArg);
//! assert!(!result.output.contains("sk-1234567890"));
//! ```

pub mod action;
pub mod canonicalize;
pub mod detect;
pub mod engine;
pub mod error;
pub mod field_class;
pub mod hash;
pub mod policy;

pub use action::Action;
pub use canonicalize::{Canonicalizer, CANONICALIZATION_VERSION};
pub use detect::{SecretDetector, SecretType};
pub use engine::{RedactedValue, RedactionEngine};
pub use error::{RedactionError, Result};
pub use field_class::FieldClass;
pub use hash::{KeyManager, KeyMaterial};
pub use policy::{ExportProfile, FieldRule, RedactionPolicy};
