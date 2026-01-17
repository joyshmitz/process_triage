//! Fuzz target for /proc/net/tcp parsing.
//!
//! Tests that TCP connection parsing handles arbitrary input without panicking.

#![no_main]

use libfuzzer_sys::fuzz_target;
use pt_core::collect::network::parse_proc_net_tcp_reader;
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    // Test IPv4 parsing
    let cursor = Cursor::new(data);
    let _ = parse_proc_net_tcp_reader(cursor, false);

    // Test IPv6 parsing
    let cursor = Cursor::new(data);
    let _ = parse_proc_net_tcp_reader(cursor, true);
});
