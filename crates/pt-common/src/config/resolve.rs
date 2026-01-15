//! Configuration resolution for Process Triage.
//!
//! Implements deterministic config resolution order:
//! 1. Explicit CLI flags (--config-dir, --priors, --policy)
//! 2. Environment variables (PROCESS_TRIAGE_CONFIG, XDG_CONFIG_HOME)
//! 3. XDG default (~/.config/process_triage/)
//! 4. Built-in defaults

use std::env;
use std::fs;
use std::path::PathBuf;

use sha2::{Digest, Sha256};

use super::{ConfigResolution, ConfigSource, Policy, Priors};
use crate::error::{Error, Result};

/// Configuration file paths.
#[derive(Debug, Clone)]
pub struct ConfigPaths {
    /// Directory containing config files
    pub config_dir: Option<PathBuf>,
    /// Explicit path to priors.json
    pub priors_path: Option<PathBuf>,
    /// Explicit path to policy.json
    pub policy_path: Option<PathBuf>,
}

impl Default for ConfigPaths {
    fn default() -> Self {
        ConfigPaths {
            config_dir: None,
            priors_path: None,
            policy_path: None,
        }
    }
}

/// Configuration resolver with deterministic resolution order.
#[derive(Debug)]
pub struct ConfigResolver {
    /// Paths from CLI flags
    cli_paths: ConfigPaths,
}

impl ConfigResolver {
    /// Create a new resolver with CLI paths.
    pub fn new(paths: ConfigPaths) -> Self {
        ConfigResolver { cli_paths: paths }
    }

    /// Create a resolver with no CLI overrides.
    pub fn with_defaults() -> Self {
        ConfigResolver {
            cli_paths: ConfigPaths::default(),
        }
    }

    /// Resolve the config directory path.
    pub fn resolve_config_dir(&self) -> Option<PathBuf> {
        // 1. CLI flag
        if let Some(ref dir) = self.cli_paths.config_dir {
            return Some(dir.clone());
        }

        // 2. PROCESS_TRIAGE_CONFIG env var
        if let Ok(dir) = env::var("PROCESS_TRIAGE_CONFIG") {
            return Some(PathBuf::from(dir));
        }

        // 3. XDG_CONFIG_HOME/process_triage
        if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
            return Some(PathBuf::from(xdg).join("process_triage"));
        }

        // 4. ~/.config/process_triage (XDG default)
        dirs::config_dir().map(|d| d.join("process_triage"))
    }

    /// Resolve the priors.json path.
    pub fn resolve_priors_path(&self) -> (Option<PathBuf>, ConfigResolution) {
        // 1. CLI flag
        if let Some(ref path) = self.cli_paths.priors_path {
            return (Some(path.clone()), ConfigResolution::CliFlag);
        }

        // 2. PROCESS_TRIAGE_PRIORS env var
        if let Ok(path) = env::var("PROCESS_TRIAGE_PRIORS") {
            return (Some(PathBuf::from(path)), ConfigResolution::EnvVar);
        }

        // 3. XDG config dir
        if let Some(config_dir) = self.resolve_config_dir() {
            let path = config_dir.join("priors.json");
            if path.exists() {
                return (Some(path), ConfigResolution::XdgConfig);
            }
        }

        // 4. Default
        (None, ConfigResolution::Default)
    }

    /// Resolve the policy.json path.
    pub fn resolve_policy_path(&self) -> (Option<PathBuf>, ConfigResolution) {
        // 1. CLI flag
        if let Some(ref path) = self.cli_paths.policy_path {
            return (Some(path.clone()), ConfigResolution::CliFlag);
        }

        // 2. PROCESS_TRIAGE_POLICY env var
        if let Ok(path) = env::var("PROCESS_TRIAGE_POLICY") {
            return (Some(PathBuf::from(path)), ConfigResolution::EnvVar);
        }

        // 3. XDG config dir
        if let Some(config_dir) = self.resolve_config_dir() {
            let path = config_dir.join("policy.json");
            if path.exists() {
                return (Some(path), ConfigResolution::XdgConfig);
            }
        }

        // 4. Default
        (None, ConfigResolution::Default)
    }

    /// Load priors from resolved path or defaults.
    pub fn load_priors(&self) -> Result<(Priors, ConfigSource)> {
        let (path, resolution) = self.resolve_priors_path();

        match path {
            Some(p) => {
                let content = fs::read_to_string(&p).map_err(|e| {
                    Error::Config(format!("failed to read priors from {}: {}", p.display(), e))
                })?;

                let hash = compute_sha256(&content);

                let priors: Priors = serde_json::from_str(&content).map_err(|e| {
                    Error::InvalidPriors(format!("failed to parse {}: {}", p.display(), e))
                })?;

                priors.validate()?;

                Ok((
                    priors,
                    ConfigSource {
                        path: Some(p.to_string_lossy().to_string()),
                        hash: Some(hash),
                        resolution,
                    },
                ))
            }
            None => {
                let priors = Priors::default();
                Ok((
                    priors,
                    ConfigSource {
                        path: None,
                        hash: None,
                        resolution: ConfigResolution::Default,
                    },
                ))
            }
        }
    }

    /// Load policy from resolved path or defaults.
    pub fn load_policy(&self) -> Result<(Policy, ConfigSource)> {
        let (path, resolution) = self.resolve_policy_path();

        match path {
            Some(p) => {
                let content = fs::read_to_string(&p).map_err(|e| {
                    Error::Config(format!("failed to read policy from {}: {}", p.display(), e))
                })?;

                let hash = compute_sha256(&content);

                let policy: Policy = serde_json::from_str(&content).map_err(|e| {
                    Error::InvalidPolicy(format!("failed to parse {}: {}", p.display(), e))
                })?;

                policy.validate()?;

                Ok((
                    policy,
                    ConfigSource {
                        path: Some(p.to_string_lossy().to_string()),
                        hash: Some(hash),
                        resolution,
                    },
                ))
            }
            None => {
                let policy = Policy::default();
                Ok((
                    policy,
                    ConfigSource {
                        path: None,
                        hash: None,
                        resolution: ConfigResolution::Default,
                    },
                ))
            }
        }
    }
}

/// Compute SHA-256 hash of a string.
fn compute_sha256(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolver_defaults() {
        // Override env vars to avoid picking up user's actual config
        std::env::set_var("PROCESS_TRIAGE_CONFIG", "/nonexistent/path");

        let resolver = ConfigResolver::with_defaults();
        let (priors, source) = resolver.load_priors().unwrap();
        assert_eq!(source.resolution, ConfigResolution::Default);
        assert!(source.path.is_none());
        assert!(priors.validate().is_ok());

        std::env::remove_var("PROCESS_TRIAGE_CONFIG");
    }

    #[test]
    fn test_sha256_hash() {
        let content = "test content";
        let hash = compute_sha256(content);
        assert_eq!(hash.len(), 64); // SHA-256 produces 64 hex chars
    }

    #[test]
    fn test_load_priors_from_file() {
        use std::io::Write;

        let priors_json = r#"{
            "schema_version": "1.0.0",
            "classes": {
                "useful": {
                    "prior_prob": 0.70,
                    "cpu_beta": {"alpha": 2.0, "beta": 5.0},
                    "orphan_beta": {"alpha": 1.0, "beta": 20.0},
                    "tty_beta": {"alpha": 5.0, "beta": 3.0},
                    "net_beta": {"alpha": 3.0, "beta": 5.0}
                },
                "useful_bad": {
                    "prior_prob": 0.10,
                    "cpu_beta": {"alpha": 8.0, "beta": 2.0},
                    "orphan_beta": {"alpha": 2.0, "beta": 8.0},
                    "tty_beta": {"alpha": 3.0, "beta": 5.0},
                    "net_beta": {"alpha": 4.0, "beta": 4.0}
                },
                "abandoned": {
                    "prior_prob": 0.15,
                    "cpu_beta": {"alpha": 1.0, "beta": 10.0},
                    "orphan_beta": {"alpha": 8.0, "beta": 2.0},
                    "tty_beta": {"alpha": 1.0, "beta": 10.0},
                    "net_beta": {"alpha": 1.0, "beta": 8.0}
                },
                "zombie": {
                    "prior_prob": 0.05,
                    "cpu_beta": {"alpha": 1.0, "beta": 100.0},
                    "orphan_beta": {"alpha": 15.0, "beta": 1.0},
                    "tty_beta": {"alpha": 1.0, "beta": 50.0},
                    "net_beta": {"alpha": 1.0, "beta": 100.0}
                }
            }
        }"#;

        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        tmp.write_all(priors_json.as_bytes()).unwrap();
        let path = tmp.path().to_path_buf();

        let resolver = ConfigResolver::new(ConfigPaths {
            config_dir: None,
            priors_path: Some(path),
            policy_path: None,
        });

        let (priors, source) = resolver.load_priors().unwrap();
        assert_eq!(source.resolution, ConfigResolution::CliFlag);
        assert!(source.path.is_some());
        assert!(source.hash.is_some());
        assert!(priors.validate().is_ok());
    }
}
