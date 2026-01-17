//! Fuzz target for /proc/[pid]/stat parsing.
//!
//! Tests that `parse_proc_stat_content` handles arbitrary input without panicking.

#![no_main]

use libfuzzer_sys::fuzz_target;
use pt_core::collect::proc_parsers::parse_proc_stat_content;

fuzz_target!(|data: &str| {
    // The parser should never panic, only return None for malformed input
    let _ = parse_proc_stat_content(data);
});
