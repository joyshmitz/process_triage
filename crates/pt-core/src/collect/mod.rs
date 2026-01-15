//! Process collection and scanning.
//!
//! This module provides the evidence collection layer for process triage:
//! - Quick scan via ps parsing (fast, universal)
//! - Deep scan via /proc inspection (detailed, Linux-only)
//! - Network connection collection
//! - Tool runner for safe external command execution
//!
//! The collection layer produces structured records that feed into the
//! inference engine for classification.

#[cfg(target_os = "linux")]
mod deep_scan;
#[cfg(target_os = "linux")]
mod network;
#[cfg(target_os = "linux")]
mod proc_parsers;
mod quick_scan;
pub mod tool_runner;
mod types;

#[cfg(target_os = "linux")]
pub use deep_scan::{
    deep_scan, DeepScanError, DeepScanMetadata, DeepScanOptions, DeepScanRecord, DeepScanResult,
};
#[cfg(target_os = "linux")]
pub use network::{
    collect_network_info, ListenPort, NetworkInfo, SocketCounts, TcpConnection, TcpState,
    UdpSocket, UnixSocket, UnixSocketState, UnixSocketType,
};
#[cfg(target_os = "linux")]
pub use proc_parsers::{
    CgroupInfo, CriticalFile, CriticalFileCategory, FdInfo, FdType, IoStats, MemStats, OpenFile,
    OpenMode, SchedInfo, SchedStats,
};
pub use quick_scan::{quick_scan, QuickScanError, QuickScanOptions};
pub use tool_runner::{
    run_tool, run_tools_parallel, ToolConfig, ToolError, ToolOutput, ToolRunner,
    ToolRunnerBuilder, ToolSpec, DEFAULT_BUDGET_MS, DEFAULT_MAX_OUTPUT_BYTES,
    DEFAULT_MAX_PARALLEL, DEFAULT_TIMEOUT_SECS,
};
pub use types::{ProcessRecord, ProcessState, ScanResult, ScanMetadata};
