//! Fuzz target for /proc/[pid]/schedstat parsing.
//!
//! Tests that `parse_schedstat_content` handles arbitrary input without panicking.

#![no_main]

use libfuzzer_sys::fuzz_target;
use pt_core::collect::proc_parsers::parse_schedstat_content;

fuzz_target!(|data: &str| {
    // The parser should never panic, only return None for malformed input
    let _ = parse_schedstat_content(data);
});
