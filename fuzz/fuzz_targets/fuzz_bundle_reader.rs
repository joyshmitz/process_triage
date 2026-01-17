//! Fuzz target for .ptb bundle reading.
//!
//! Tests that bundle parsing handles arbitrary input without panicking.
//! This is important for security as bundles may come from untrusted sources.

#![no_main]

use libfuzzer_sys::fuzz_target;
use pt_bundle::BundleReader;

fuzz_target!(|data: &[u8]| {
    // Try to parse as a bundle - should never panic, only return an error
    // The bundle reader expects ZIP format, so most random data will fail quickly
    let _ = BundleReader::from_bytes(data.to_vec());
});
