//! Fuzz target for /proc/[pid]/io parsing.
//!
//! Tests that `parse_io_content` handles arbitrary input without panicking.

#![no_main]

use libfuzzer_sys::fuzz_target;
use pt_core::collect::proc_parsers::parse_io_content;

fuzz_target!(|data: &str| {
    // The parser should never panic, only return None for malformed input
    let _ = parse_io_content(data);
});
