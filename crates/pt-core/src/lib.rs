//! Process Triage Core Library
//!
//! This library provides the core functionality for process triage:
//! - Exit codes for CLI operations
//! - Configuration loading and validation
//! - CLI utilities and helpers
//!
//! The binary entry point is in `main.rs`.

pub mod cli;
pub mod config;
pub mod exit_codes;

// Re-export test utilities for integration tests
#[cfg(test)]
pub mod test_utils;
