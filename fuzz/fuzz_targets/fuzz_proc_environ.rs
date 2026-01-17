//! Fuzz target for /proc/[pid]/environ parsing.
//!
//! Tests that `parse_environ_content` handles arbitrary byte input without panicking.
//! This is particularly important as environ can contain non-UTF8 bytes.

#![no_main]

use libfuzzer_sys::fuzz_target;
use pt_core::collect::proc_parsers::parse_environ_content;

fuzz_target!(|data: &[u8]| {
    // The parser should never panic, only return None for malformed input
    // Environ parsing must handle arbitrary bytes since environment variables
    // can contain non-UTF8 content in some edge cases
    let _ = parse_environ_content(data);
});
