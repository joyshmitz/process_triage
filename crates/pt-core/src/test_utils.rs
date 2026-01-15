//! Test utilities for pt-core.
//!
//! This module provides test infrastructure including:
//! - Test logging with structured JSONL output
//! - Fixture loading helpers
//! - Common assertions
//! - Tempdir management

use std::path::{Path, PathBuf};
use std::time::Instant;

// ============================================================================
// Macros (must be defined first for use in this module)
// ============================================================================

/// Assert that a Result is Ok and return the value.
#[macro_export]
macro_rules! assert_ok {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => panic!("Expected Ok, got Err: {:?}", e),
        }
    };
    ($expr:expr, $msg:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => panic!("{}: {:?}", $msg, e),
        }
    };
}

/// Assert that a Result is Err.
#[macro_export]
macro_rules! assert_err {
    ($expr:expr) => {
        match $expr {
            Ok(val) => panic!("Expected Err, got Ok: {:?}", val),
            Err(_) => {}
        }
    };
    ($expr:expr, $msg:expr) => {
        match $expr {
            Ok(val) => panic!("{}: got Ok({:?})", $msg, val),
            Err(_) => {}
        }
    };
}

/// Assert that two floating point numbers are approximately equal.
#[macro_export]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {
        $crate::assert_approx_eq!($a, $b, 1e-6_f64)
    };
    ($a:expr, $b:expr, $epsilon:expr) => {{
        let a: f64 = $a;
        let b: f64 = $b;
        let eps: f64 = $epsilon;
        let diff = (a - b).abs();
        if diff > eps {
            panic!(
                "assertion failed: `(left ~= right)` (left: `{}`, right: `{}`, diff: `{}`, epsilon: `{}`)",
                a, b, diff, eps
            );
        }
    }};
}

// ============================================================================
// Fixtures
// ============================================================================

/// Fixture directory relative to crate root.
pub const FIXTURES_DIR: &str = "tests/fixtures";

/// Get the path to a test fixture file.
///
/// # Example
/// ```ignore
/// let priors_path = fixture_path("priors.json");
/// ```
pub fn fixture_path(name: &str) -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    Path::new(manifest_dir).join(FIXTURES_DIR).join(name)
}

/// Load a fixture file as a string.
pub fn load_fixture(name: &str) -> std::io::Result<String> {
    std::fs::read_to_string(fixture_path(name))
}

/// Load a fixture file and parse as JSON.
pub fn load_fixture_json<T: serde::de::DeserializeOwned>(name: &str) -> Result<T, String> {
    let content = load_fixture(name).map_err(|e| format!("Failed to read fixture {}: {}", name, e))?;
    serde_json::from_str(&content).map_err(|e| format!("Failed to parse fixture {}: {}", name, e))
}

// ============================================================================
// Test Timer
// ============================================================================

/// Test timer for measuring duration of operations.
pub struct TestTimer {
    name: String,
    start: Instant,
}

impl TestTimer {
    /// Start a new timer with the given name.
    pub fn new(name: &str) -> Self {
        let timer = Self {
            name: name.to_string(),
            start: Instant::now(),
        };
        eprintln!("[TIMER] {} started", name);
        timer
    }

    /// Get elapsed time in milliseconds.
    pub fn elapsed_ms(&self) -> u128 {
        self.start.elapsed().as_millis()
    }
}

impl Drop for TestTimer {
    fn drop(&mut self) {
        eprintln!("[TIMER] {} completed in {}ms", self.name, self.elapsed_ms());
    }
}

// ============================================================================
// Tempdir Helper
// ============================================================================

/// Create a temporary directory that is automatically cleaned up.
///
/// Uses the `tempfile` crate's TempDir.
#[cfg(feature = "test-tempdir")]
pub fn temp_dir() -> tempfile::TempDir {
    tempfile::tempdir().expect("Failed to create temp dir")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixture_path() {
        let path = fixture_path("priors.json");
        assert!(path.to_string_lossy().contains("fixtures"));
        assert!(path.to_string_lossy().ends_with("priors.json"));
    }

    #[test]
    fn test_load_fixture() {
        let content = load_fixture("priors.json").expect("Should load priors.json");
        assert!(content.contains("schema_version"));
        assert!(content.contains("useful"));
    }

    #[test]
    fn test_load_fixture_missing() {
        let result = load_fixture("nonexistent.json");
        assert!(result.is_err());
    }

    #[test]
    fn test_timer() {
        let _timer = TestTimer::new("test_operation");
        std::thread::sleep(std::time::Duration::from_millis(10));
        // Timer will log on drop
    }

    #[test]
    fn test_log_macro() {
        crate::test_log!("Test message: {}", 42);
        crate::test_log!("Another message");
    }

    #[test]
    fn test_assert_ok_macro() {
        let result: Result<i32, &str> = Ok(42);
        let val = assert_ok!(result);
        assert_eq!(val, 42);
    }

    #[test]
    #[should_panic(expected = "Expected Ok")]
    fn test_assert_ok_fails() {
        let result: Result<i32, &str> = Err("error");
        let _ = assert_ok!(result);
    }

    #[test]
    fn test_assert_err_macro() {
        let result: Result<i32, &str> = Err("error");
        assert_err!(result);
    }

    #[test]
    fn test_assert_approx_eq() {
        assert_approx_eq!(1.0_f64, 1.0_f64);
        assert_approx_eq!(1.0_f64, 1.0000001_f64);
        assert_approx_eq!(0.1_f64 + 0.2_f64, 0.3_f64, 1e-10_f64);
    }
}
