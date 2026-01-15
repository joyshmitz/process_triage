//! Configuration loading and validation for Process Triage.
//!
//! This module provides:
//! - Typed configuration structures for priors.json and policy.json
//! - Deterministic config resolution (CLI > env > XDG > defaults)
//! - Schema and semantic validation
//! - Config snapshots for telemetry and audit

pub mod policy;
pub mod priors;
pub mod resolve;
pub mod snapshot;

pub use policy::Policy;
pub use priors::Priors;
pub use resolve::{ConfigPaths, ConfigResolver};
pub use snapshot::ConfigSnapshot;

use crate::error::Result;

/// The complete loaded configuration for pt-core.
#[derive(Debug, Clone)]
pub struct Config {
    /// Bayesian hyperparameters for inference
    pub priors: Priors,
    /// Decision policy, guardrails, and gates
    pub policy: Policy,
    /// Metadata about how this config was loaded
    pub snapshot: ConfigSnapshot,
}

impl Config {
    /// Load configuration with resolution from CLI, env, or defaults.
    pub fn load(resolver: &ConfigResolver) -> Result<Self> {
        let (priors, priors_source) = resolver.load_priors()?;
        let (policy, policy_source) = resolver.load_policy()?;

        let snapshot = ConfigSnapshot::new(&priors, &policy, priors_source, policy_source)?;

        Ok(Config {
            priors,
            policy,
            snapshot,
        })
    }

    /// Load configuration with built-in defaults only.
    /// Used when no config files are found and zero-config mode is acceptable.
    pub fn load_defaults() -> Result<Self> {
        let priors = Priors::default();
        let policy = Policy::default();
        let snapshot = ConfigSnapshot::from_defaults(&priors, &policy)?;

        Ok(Config {
            priors,
            policy,
            snapshot,
        })
    }

    /// Validate configuration semantically.
    /// Returns Ok(()) if valid, or Err with detailed validation errors.
    pub fn validate(&self) -> Result<()> {
        self.priors.validate()?;
        self.policy.validate()?;
        Ok(())
    }
}

/// Configuration source for a file.
#[derive(Debug, Clone)]
pub struct ConfigSource {
    /// Path to the config file, or None if using defaults
    pub path: Option<String>,
    /// SHA-256 hash of file contents, or None if defaults
    pub hash: Option<String>,
    /// How this source was resolved
    pub resolution: ConfigResolution,
}

/// How a config file was resolved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigResolution {
    /// From explicit CLI flag
    CliFlag,
    /// From environment variable
    EnvVar,
    /// From XDG config directory
    XdgConfig,
    /// Using built-in defaults
    Default,
}

impl std::fmt::Display for ConfigResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigResolution::CliFlag => write!(f, "cli"),
            ConfigResolution::EnvVar => write!(f, "env"),
            ConfigResolution::XdgConfig => write!(f, "xdg"),
            ConfigResolution::Default => write!(f, "default"),
        }
    }
}
