//! Process collection and scanning.
//!
//! This module provides the evidence collection layer for process triage:
//! - Quick scan via ps parsing (fast, universal)
//! - Deep scan via /proc inspection (detailed, Linux-only)
//!
//! The collection layer produces structured records that feed into the
//! inference engine for classification.

mod quick_scan;
mod types;

pub use quick_scan::{quick_scan, QuickScanError, QuickScanOptions};
pub use types::{ProcessRecord, ProcessState, ScanResult, ScanMetadata};
