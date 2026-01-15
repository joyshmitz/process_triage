//! Live-system integration tests for network collectors.
//!
//! These tests are Linux-only and do not use mocks. They validate that
//! network collectors can observe real sockets created by the harness.

#![cfg(target_os = "linux")]

use pt_core::collect::collect_network_info;

mod support;
use support::live_harness::LiveHarness;

fn skip_if_proc_unavailable() -> bool {
    if !std::path::Path::new("/proc").exists() {
        eprintln!("skipping: /proc not available on this system");
        return true;
    }
    false
}

#[test]
fn live_network_collects_tcp_udp_unix() {
    if skip_if_proc_unavailable() {
        return;
    }

    let mut harness = LiveHarness::new().expect("harness init");
    let _tcp_port = harness.open_tcp_connection().expect("open tcp");
    let _udp_port = harness.open_udp_socket().expect("open udp");

    #[cfg(unix)]
    {
        let _unix_path = harness.open_unix_socket().expect("open unix socket");
    }

    let pid = harness.pid();
    let network = collect_network_info(pid).expect("collect_network_info");

    let tcp_listen = network
        .listen_ports
        .iter()
        .any(|p| p.protocol == "tcp" || p.protocol == "tcp6");
    assert!(tcp_listen, "expected at least one tcp listen port");

    let tcp_established = network
        .tcp_connections
        .iter()
        .any(|c| c.state.is_active());
    assert!(tcp_established, "expected an active tcp connection");

    let udp_present = network.udp_sockets.iter().any(|s| s.local_port > 0);
    assert!(udp_present, "expected at least one udp socket");

    #[cfg(unix)]
    {
        let unix_present = network.unix_sockets.iter().any(|s| s.inode > 0);
        assert!(unix_present, "expected at least one unix socket");
    }
}
