//! Fuzz target for policy.json configuration parsing.
//!
//! Tests that JSON policy configuration parsing handles arbitrary input
//! without panicking.

#![no_main]

use libfuzzer_sys::fuzz_target;
use pt_config::policy::Policy;

fuzz_target!(|data: &[u8]| {
    // Try to parse as JSON - should never panic, only return an error
    let _ = serde_json::from_slice::<Policy>(data);
});
