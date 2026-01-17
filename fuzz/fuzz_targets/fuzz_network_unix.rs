//! Fuzz target for /proc/net/unix parsing.
//!
//! Tests that Unix socket parsing handles arbitrary input without panicking.

#![no_main]

use libfuzzer_sys::fuzz_target;
use pt_core::collect::network::parse_proc_net_unix_reader;
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    let cursor = Cursor::new(data);
    let _ = parse_proc_net_unix_reader(cursor);
});
