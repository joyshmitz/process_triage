//! Fuzz target for /proc/[pid]/cgroup parsing.
//!
//! Tests that `parse_cgroup_content` handles arbitrary input without panicking.

#![no_main]

use libfuzzer_sys::fuzz_target;
use pt_core::collect::proc_parsers::parse_cgroup_content;

fuzz_target!(|data: &str| {
    // The parser should never panic, only return None for malformed input
    let _ = parse_cgroup_content(data);
});
