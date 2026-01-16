//! No-mock integration tests for progress event JSONL output.
//!
//! These tests validate:
//! - ProgressEvent JSONL schema (required/optional fields)
//! - LogEvent JSONL schema
//! - JsonlWriter produces valid JSONL output
//! - EventBus broadcasting and subscription
//! - SessionEmitter session ID attachment
//!
//! See: process_triage-aii.7.6

use chrono::{DateTime, Utc};
use pt_core::events::{
    event_names, EventBus, FanoutEmitter, JsonlWriter, Phase, ProgressEmitter,
    ProgressEvent, SessionEmitter,
};
use pt_core::logging::events::{event_names as log_event_names, Level, LogContext, LogEvent, Stage};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::{Arc, Mutex};
use tempfile::tempdir;

// ============================================================================
// Test Helpers
// ============================================================================

/// Validate that a JSON string is valid ProgressEvent JSONL.
fn validate_progress_event_schema(json: &str) -> Result<(), String> {
    let value: serde_json::Value =
        serde_json::from_str(json).map_err(|e| format!("Invalid JSON: {}", e))?;

    let obj = value
        .as_object()
        .ok_or_else(|| "Expected JSON object".to_string())?;

    // Required fields for ProgressEvent
    let required_fields = ["event", "timestamp", "phase"];

    for field in required_fields {
        if !obj.contains_key(field) {
            return Err(format!("Missing required field: {}", field));
        }
    }

    // Validate timestamp is ISO-8601
    if let Some(ts) = obj.get("timestamp").and_then(|v| v.as_str()) {
        DateTime::parse_from_rfc3339(ts)
            .map_err(|e| format!("Invalid timestamp format: {}", e))?;
    }

    // Validate phase is a known value
    if let Some(phase) = obj.get("phase").and_then(|v| v.as_str()) {
        let valid_phases = [
            "session",
            "quick_scan",
            "deep_scan",
            "infer",
            "decide",
            "plan",
            "apply",
            "ui",
            "verify",
            "report",
            "bundle",
        ];
        if !valid_phases.contains(&phase) {
            return Err(format!("Unknown phase: {}", phase));
        }
    }

    Ok(())
}

/// Validate that a JSON string is valid LogEvent JSONL.
fn validate_log_event_schema(json: &str) -> Result<(), String> {
    let value: serde_json::Value =
        serde_json::from_str(json).map_err(|e| format!("Invalid JSON: {}", e))?;

    let obj = value
        .as_object()
        .ok_or_else(|| "Expected JSON object".to_string())?;

    // Required fields for LogEvent
    let required_fields = ["ts", "level", "event", "run_id", "stage", "host_id", "message"];

    for field in required_fields {
        if !obj.contains_key(field) {
            return Err(format!("Missing required field: {}", field));
        }
    }

    // Validate timestamp is ISO-8601
    if let Some(ts) = obj.get("ts").and_then(|v| v.as_str()) {
        DateTime::parse_from_rfc3339(ts)
            .map_err(|e| format!("Invalid timestamp format: {}", e))?;
    }

    // Validate level is a known value
    if let Some(level) = obj.get("level").and_then(|v| v.as_str()) {
        let valid_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_levels.contains(&level) {
            return Err(format!("Unknown level: {}", level));
        }
    }

    // Validate stage is a known value
    if let Some(stage) = obj.get("stage").and_then(|v| v.as_str()) {
        let valid_stages = [
            "init", "scan", "infer", "decide", "ui", "apply", "verify", "report", "bundle", "daemon",
        ];
        if !valid_stages.contains(&stage) {
            return Err(format!("Unknown stage: {}", stage));
        }
    }

    Ok(())
}

/// Read and validate all events in a JSONL file.
fn validate_progress_jsonl_file(path: &Path) -> Result<Vec<ProgressEvent>, String> {
    let file = fs::File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
    let reader = BufReader::new(file);
    let mut events = Vec::new();

    for (line_num, line_result) in reader.lines().enumerate() {
        let line = line_result.map_err(|e| format!("Read error at line {}: {}", line_num + 1, e))?;

        if line.trim().is_empty() {
            continue;
        }

        // First validate schema
        validate_progress_event_schema(&line)
            .map_err(|e| format!("Schema validation failed at line {}: {}", line_num + 1, e))?;

        // Then parse into struct
        let event: ProgressEvent = serde_json::from_str(&line)
            .map_err(|e| format!("Parse error at line {}: {}", line_num + 1, e))?;

        events.push(event);
    }

    Ok(events)
}

// ============================================================================
// ProgressEvent JSONL Schema Tests
// ============================================================================

#[test]
fn test_progress_event_required_fields() {
    let event = ProgressEvent::new(event_names::SESSION_STARTED, Phase::Session);
    let json = event.to_jsonl();

    validate_progress_event_schema(&json).expect("schema validation");

    // Verify required fields are present
    assert!(json.contains("\"event\":\"session_started\""));
    assert!(json.contains("\"phase\":\"session\""));
    assert!(json.contains("\"timestamp\""));

    eprintln!("[INFO] progress_event_required_fields passed");
}

#[test]
fn test_progress_event_all_phases() {
    let phases = vec![
        (Phase::Session, "session"),
        (Phase::QuickScan, "quick_scan"),
        (Phase::DeepScan, "deep_scan"),
        (Phase::Infer, "infer"),
        (Phase::Decide, "decide"),
        (Phase::Plan, "plan"),
        (Phase::Apply, "apply"),
        (Phase::Ui, "ui"),
        (Phase::Verify, "verify"),
        (Phase::Report, "report"),
        (Phase::Bundle, "bundle"),
    ];

    for (phase, expected_str) in phases {
        let event = ProgressEvent::new("test_event", phase);
        let json = event.to_jsonl();

        validate_progress_event_schema(&json)
            .unwrap_or_else(|e| panic!("Schema validation failed for {:?}: {}", phase, e));

        assert!(
            json.contains(&format!("\"phase\":\"{}\"", expected_str)),
            "Phase {:?} should serialize to \"{}\"",
            phase,
            expected_str
        );
    }

    eprintln!("[INFO] All phases serialize correctly");
}

#[test]
fn test_progress_event_optional_fields() {
    let event = ProgressEvent::new(event_names::QUICK_SCAN_PROGRESS, Phase::QuickScan)
        .with_session_id("sess-abc-123")
        .with_progress(50, Some(100))
        .with_elapsed_ms(1234)
        .with_detail("pids_scanned", 50)
        .with_detail("memory_used_mb", 128);

    let json = event.to_jsonl();
    validate_progress_event_schema(&json).expect("schema validation");

    // Verify optional fields
    assert!(json.contains("\"session_id\":\"sess-abc-123\""));
    assert!(json.contains("\"progress\":{\"current\":50,\"total\":100}"));
    assert!(json.contains("\"elapsed_ms\":1234"));
    assert!(json.contains("\"pids_scanned\":50"));
    assert!(json.contains("\"memory_used_mb\":128"));

    eprintln!("[INFO] Optional fields serialize correctly");
}

#[test]
fn test_progress_event_roundtrip_fidelity() {
    let original = ProgressEvent::new(event_names::INFERENCE_COMPLETE, Phase::Infer)
        .with_session_id("sess-xyz")
        .with_progress(100, Some(100))
        .with_elapsed_ms(5000)
        .with_detail("processes_classified", 42);

    let json = original.to_jsonl();
    let roundtrip: ProgressEvent = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(roundtrip.event, original.event);
    assert_eq!(roundtrip.phase, original.phase);
    assert_eq!(roundtrip.session_id, original.session_id);
    assert_eq!(roundtrip.progress, original.progress);
    assert_eq!(roundtrip.elapsed_ms, original.elapsed_ms);

    // Timestamp should be within 1 second
    let time_diff = (roundtrip.timestamp - original.timestamp).num_seconds().abs();
    assert!(time_diff <= 1, "Timestamp drift: {} seconds", time_diff);

    eprintln!("[INFO] Roundtrip fidelity verified");
}

#[test]
fn test_progress_event_empty_details_omitted() {
    let event = ProgressEvent::new(event_names::SESSION_ENDED, Phase::Session);
    let json = event.to_jsonl();

    // Empty details should be omitted (skip_serializing_if)
    assert!(
        !json.contains("\"details\":{}"),
        "Empty details should be omitted from JSON"
    );

    eprintln!("[INFO] Empty details correctly omitted");
}

// ============================================================================
// LogEvent JSONL Schema Tests
// ============================================================================

#[test]
fn test_log_event_required_fields() {
    let event = LogEvent::new(
        Level::Info,
        log_event_names::RUN_STARTED,
        "run-abc-123",
        "host-xyz",
        Stage::Init,
        "Starting process triage run",
    );

    let json = event.to_jsonl();
    validate_log_event_schema(&json).expect("schema validation");

    assert!(json.contains("\"level\":\"info\""));
    assert!(json.contains("\"event\":\"run.started\""));
    assert!(json.contains("\"run_id\":\"run-abc-123\""));
    assert!(json.contains("\"host_id\":\"host-xyz\""));
    assert!(json.contains("\"stage\":\"init\""));
    assert!(json.contains("\"message\":\"Starting process triage run\""));

    eprintln!("[INFO] log_event_required_fields passed");
}

#[test]
fn test_log_event_all_levels() {
    let levels = vec![
        (Level::Trace, "trace"),
        (Level::Debug, "debug"),
        (Level::Info, "info"),
        (Level::Warn, "warn"),
        (Level::Error, "error"),
    ];

    for (level, expected_str) in levels {
        let event = LogEvent::new(level, "test.event", "run-1", "host-1", Stage::Scan, "Test");
        let json = event.to_jsonl();

        validate_log_event_schema(&json)
            .unwrap_or_else(|e| panic!("Schema validation failed for {:?}: {}", level, e));

        assert!(
            json.contains(&format!("\"level\":\"{}\"", expected_str)),
            "Level {:?} should serialize to \"{}\"",
            level,
            expected_str
        );
    }

    eprintln!("[INFO] All log levels serialize correctly");
}

#[test]
fn test_log_event_all_stages() {
    let stages = vec![
        (Stage::Init, "init"),
        (Stage::Scan, "scan"),
        (Stage::Infer, "infer"),
        (Stage::Decide, "decide"),
        (Stage::Ui, "ui"),
        (Stage::Apply, "apply"),
        (Stage::Verify, "verify"),
        (Stage::Report, "report"),
        (Stage::Bundle, "bundle"),
        (Stage::Daemon, "daemon"),
    ];

    for (stage, expected_str) in stages {
        let event = LogEvent::new(Level::Info, "test.event", "run-1", "host-1", stage, "Test");
        let json = event.to_jsonl();

        validate_log_event_schema(&json)
            .unwrap_or_else(|e| panic!("Schema validation failed for {:?}: {}", stage, e));

        assert!(
            json.contains(&format!("\"stage\":\"{}\"", expected_str)),
            "Stage {:?} should serialize to \"{}\"",
            stage,
            expected_str
        );
    }

    eprintln!("[INFO] All log stages serialize correctly");
}

#[test]
fn test_log_event_optional_fields() {
    let event = LogEvent::new(
        Level::Debug,
        log_event_names::INFER_PROC_DONE,
        "run-xyz",
        "host-abc",
        Stage::Infer,
        "Inference complete for process",
    )
    .with_session_id("pt-20260116-123456-wxyz")
    .with_process(1234, "boot-abc:12345:1234")
    .with_field("posterior_useful", 0.85)
    .with_field("classification", "useful");

    let json = event.to_jsonl();
    validate_log_event_schema(&json).expect("schema validation");

    assert!(json.contains("\"session_id\":\"pt-20260116-123456-wxyz\""));
    assert!(json.contains("\"pid\":1234"));
    assert!(json.contains("\"start_id\":\"boot-abc:12345:1234\""));
    assert!(json.contains("\"posterior_useful\":0.85"));
    assert!(json.contains("\"classification\":\"useful\""));

    eprintln!("[INFO] Log event optional fields serialize correctly");
}

#[test]
fn test_log_context_event_generation() {
    let ctx = LogContext::new("run-context-test", "host-context-test")
        .with_session_id("sess-ctx-123");

    let info_event = ctx.info(log_event_names::SCAN_STARTED, Stage::Scan, "Beginning scan");
    let debug_event = ctx.debug(log_event_names::SCAN_SAMPLED, Stage::Scan, "Sampled process");
    let warn_event = ctx.warn(log_event_names::CONFIG_ERROR, Stage::Init, "Config warning");
    let error_event = ctx.error(log_event_names::INTERNAL_ERROR, Stage::Apply, "Action failed");

    for (event, expected_level) in [
        (&info_event, "info"),
        (&debug_event, "debug"),
        (&warn_event, "warn"),
        (&error_event, "error"),
    ] {
        let json = event.to_jsonl();
        validate_log_event_schema(&json).expect("schema validation");

        assert!(json.contains(&format!("\"level\":\"{}\"", expected_level)));
        assert!(json.contains("\"run_id\":\"run-context-test\""));
        assert!(json.contains("\"host_id\":\"host-context-test\""));
        assert!(json.contains("\"session_id\":\"sess-ctx-123\""));
    }

    eprintln!("[INFO] LogContext generates valid events");
}

// ============================================================================
// JsonlWriter Tests
// ============================================================================

#[test]
fn test_jsonl_writer_produces_valid_output() {
    let dir = tempdir().expect("tempdir");
    let output_path = dir.path().join("progress.jsonl");

    // Create JsonlWriter with a file
    let file = fs::File::create(&output_path).expect("create file");
    let writer = JsonlWriter::new(file);

    // Emit several events
    let events = vec![
        ProgressEvent::new(event_names::SESSION_STARTED, Phase::Session)
            .with_session_id("sess-writer-test"),
        ProgressEvent::new(event_names::QUICK_SCAN_STARTED, Phase::QuickScan)
            .with_session_id("sess-writer-test"),
        ProgressEvent::new(event_names::QUICK_SCAN_PROGRESS, Phase::QuickScan)
            .with_session_id("sess-writer-test")
            .with_progress(50, Some(100)),
        ProgressEvent::new(event_names::QUICK_SCAN_COMPLETE, Phase::QuickScan)
            .with_session_id("sess-writer-test")
            .with_elapsed_ms(1500),
    ];

    for event in &events {
        writer.emit(event.clone());
    }

    // Force file close by dropping writer
    drop(writer);

    // Validate the output file
    let validated_events =
        validate_progress_jsonl_file(&output_path).expect("JSONL file should be valid");

    assert_eq!(
        validated_events.len(),
        events.len(),
        "All events should be written"
    );

    // Verify each line is valid and contains expected event
    let content = fs::read_to_string(&output_path).expect("read file");
    let lines: Vec<_> = content.lines().collect();
    assert_eq!(lines.len(), events.len());

    assert!(lines[0].contains("session_started"));
    assert!(lines[1].contains("quick_scan_started"));
    assert!(lines[2].contains("quick_scan_progress"));
    assert!(lines[3].contains("quick_scan_complete"));

    eprintln!("[INFO] JsonlWriter produces valid JSONL output");
    eprintln!("  Output file: {}", output_path.display());
    eprintln!("  Events written: {}", validated_events.len());
}

#[test]
fn test_jsonl_writer_no_trailing_comma() {
    let dir = tempdir().expect("tempdir");
    let output_path = dir.path().join("no_trailing.jsonl");

    let file = fs::File::create(&output_path).expect("create file");
    let writer = JsonlWriter::new(file);

    writer.emit(ProgressEvent::new(event_names::SESSION_STARTED, Phase::Session));
    writer.emit(ProgressEvent::new(event_names::SESSION_ENDED, Phase::Session));

    drop(writer);

    // Read raw content and verify no trailing commas or array brackets
    let content = fs::read_to_string(&output_path).expect("read file");

    assert!(
        !content.contains("["),
        "JSONL should not contain array brackets"
    );
    assert!(
        !content.contains("],"),
        "JSONL should not contain array brackets"
    );

    // Each line should be a valid JSON object
    for (i, line) in content.lines().enumerate() {
        let parsed: serde_json::Value = serde_json::from_str(line)
            .unwrap_or_else(|e| panic!("Line {} is not valid JSON: {}", i + 1, e));
        assert!(
            parsed.is_object(),
            "Line {} should be a JSON object",
            i + 1
        );
        // Should not end with comma
        assert!(
            !line.trim().ends_with(','),
            "Line {} should not end with comma",
            i + 1
        );
    }

    eprintln!("[INFO] JSONL output has no trailing commas or array syntax");
}

// ============================================================================
// EventBus Tests
// ============================================================================

#[test]
fn test_event_bus_broadcast() {
    let bus = EventBus::new();

    // Create multiple subscribers
    let rx1 = bus.subscribe();
    let rx2 = bus.subscribe();

    // Emit an event
    bus.emit(ProgressEvent::new(
        event_names::INFERENCE_STARTED,
        Phase::Infer,
    ));

    // Both subscribers should receive it
    let received1 = rx1.recv().expect("rx1 should receive");
    let received2 = rx2.recv().expect("rx2 should receive");

    assert_eq!(received1.event, event_names::INFERENCE_STARTED);
    assert_eq!(received2.event, event_names::INFERENCE_STARTED);

    eprintln!("[INFO] EventBus broadcasts to all subscribers");
}

#[test]
fn test_event_bus_multiple_events() {
    let bus = EventBus::new();
    let rx = bus.subscribe();

    let event_names_list = [
        event_names::SESSION_STARTED,
        event_names::QUICK_SCAN_STARTED,
        event_names::QUICK_SCAN_COMPLETE,
        event_names::INFERENCE_STARTED,
        event_names::INFERENCE_COMPLETE,
        event_names::DECISION_COMPLETE,
        event_names::SESSION_ENDED,
    ];

    for name in event_names_list {
        bus.emit(ProgressEvent::new(name, Phase::Session));
    }

    // Receive all events
    let mut received = Vec::new();
    for _ in 0..event_names_list.len() {
        received.push(rx.recv().expect("should receive event"));
    }

    // Verify order is preserved
    for (i, name) in event_names_list.iter().enumerate() {
        assert_eq!(received[i].event, *name, "Event {} should match", i);
    }

    eprintln!("[INFO] EventBus preserves event order");
}

// ============================================================================
// SessionEmitter Tests
// ============================================================================

#[test]
fn test_session_emitter_attaches_session_id() {
    struct CaptureEmitter {
        captured: Mutex<Vec<ProgressEvent>>,
    }

    impl CaptureEmitter {
        fn new() -> Self {
            Self {
                captured: Mutex::new(Vec::new()),
            }
        }
    }

    impl ProgressEmitter for CaptureEmitter {
        fn emit(&self, event: ProgressEvent) {
            self.captured.lock().unwrap().push(event);
        }
    }

    let capture = Arc::new(CaptureEmitter::new());
    let session_emitter = SessionEmitter::new("sess-auto-attach", capture.clone());

    // Emit event without session_id - should get attached
    session_emitter.emit(ProgressEvent::new(event_names::QUICK_SCAN_STARTED, Phase::QuickScan));

    // Emit event with session_id - should be preserved
    session_emitter.emit(
        ProgressEvent::new(event_names::QUICK_SCAN_COMPLETE, Phase::QuickScan)
            .with_session_id("sess-explicit"),
    );

    let captured = capture.captured.lock().unwrap();
    assert_eq!(captured.len(), 2);

    // First event should have auto-attached session_id
    assert_eq!(
        captured[0].session_id.as_deref(),
        Some("sess-auto-attach")
    );

    // Second event should preserve explicit session_id
    assert_eq!(
        captured[1].session_id.as_deref(),
        Some("sess-explicit")
    );

    eprintln!("[INFO] SessionEmitter correctly attaches session IDs");
}

// ============================================================================
// FanoutEmitter Tests
// ============================================================================

#[test]
fn test_fanout_emitter_multiple_outputs() {
    let dir = tempdir().expect("tempdir");

    // Create two file outputs
    let file1_path = dir.path().join("output1.jsonl");
    let file2_path = dir.path().join("output2.jsonl");

    let file1 = fs::File::create(&file1_path).expect("create file1");
    let file2 = fs::File::create(&file2_path).expect("create file2");

    let writer1 = Arc::new(JsonlWriter::new(file1));
    let writer2 = Arc::new(JsonlWriter::new(file2));

    let fanout = FanoutEmitter::new(vec![writer1.clone(), writer2.clone()]);

    // Emit events through fanout
    fanout.emit(ProgressEvent::new(event_names::SESSION_STARTED, Phase::Session));
    fanout.emit(ProgressEvent::new(event_names::SESSION_ENDED, Phase::Session));

    // Force files to close
    drop(fanout);
    drop(writer1);
    drop(writer2);

    // Both files should have the same content
    let content1 = fs::read_to_string(&file1_path).expect("read file1");
    let content2 = fs::read_to_string(&file2_path).expect("read file2");

    assert_eq!(content1.lines().count(), 2);
    assert_eq!(content2.lines().count(), 2);

    // Validate both files
    validate_progress_jsonl_file(&file1_path).expect("file1 valid");
    validate_progress_jsonl_file(&file2_path).expect("file2 valid");

    eprintln!("[INFO] FanoutEmitter writes to multiple outputs");
}

// ============================================================================
// CI Artifact Tests
// ============================================================================

#[test]
fn test_progress_events_ci_artifacts() {
    let dir = tempdir().expect("tempdir");
    let artifacts_dir = dir.path().join("ci_artifacts");
    fs::create_dir_all(&artifacts_dir).expect("create artifacts dir");

    let output_path = artifacts_dir.join("progress_events.jsonl");
    let file = fs::File::create(&output_path).expect("create file");
    let writer = JsonlWriter::new(file);

    // Simulate a complete session lifecycle
    let session_id = format!("pt-{}", Utc::now().format("%Y%m%d-%H%M%S"));

    let lifecycle_events = vec![
        ProgressEvent::new(event_names::SESSION_STARTED, Phase::Session)
            .with_session_id(&session_id),
        ProgressEvent::new(event_names::QUICK_SCAN_STARTED, Phase::QuickScan)
            .with_session_id(&session_id),
        ProgressEvent::new(event_names::QUICK_SCAN_PROGRESS, Phase::QuickScan)
            .with_session_id(&session_id)
            .with_progress(25, Some(100)),
        ProgressEvent::new(event_names::QUICK_SCAN_PROGRESS, Phase::QuickScan)
            .with_session_id(&session_id)
            .with_progress(50, Some(100)),
        ProgressEvent::new(event_names::QUICK_SCAN_PROGRESS, Phase::QuickScan)
            .with_session_id(&session_id)
            .with_progress(75, Some(100)),
        ProgressEvent::new(event_names::QUICK_SCAN_COMPLETE, Phase::QuickScan)
            .with_session_id(&session_id)
            .with_progress(100, Some(100))
            .with_elapsed_ms(2500),
        ProgressEvent::new(event_names::INFERENCE_STARTED, Phase::Infer)
            .with_session_id(&session_id),
        ProgressEvent::new(event_names::INFERENCE_COMPLETE, Phase::Infer)
            .with_session_id(&session_id)
            .with_elapsed_ms(1500)
            .with_detail("processes_classified", 42),
        ProgressEvent::new(event_names::DECISION_STARTED, Phase::Decide)
            .with_session_id(&session_id),
        ProgressEvent::new(event_names::DECISION_COMPLETE, Phase::Decide)
            .with_session_id(&session_id)
            .with_elapsed_ms(500)
            .with_detail("candidates", 5),
        ProgressEvent::new(event_names::PLAN_READY, Phase::Plan)
            .with_session_id(&session_id)
            .with_detail("actions", 3),
        ProgressEvent::new(event_names::SESSION_ENDED, Phase::Session)
            .with_session_id(&session_id)
            .with_elapsed_ms(5000),
    ];

    for event in &lifecycle_events {
        writer.emit(event.clone());
    }

    drop(writer);

    // Validate the artifact
    let validated_events =
        validate_progress_jsonl_file(&output_path).expect("JSONL file should be valid");

    assert_eq!(validated_events.len(), lifecycle_events.len());

    // All events should have the same session_id
    for event in &validated_events {
        assert_eq!(
            event.session_id.as_deref(),
            Some(session_id.as_str()),
            "All events should have session_id"
        );
    }

    // Output artifact summary for CI systems
    eprintln!("\n=== CI ARTIFACT SUMMARY (Progress Events) ===");
    eprintln!("Artifacts directory: {}", artifacts_dir.display());
    eprintln!("Output file: {}", output_path.display());
    eprintln!("Session ID: {}", session_id);
    eprintln!("Events recorded: {}", validated_events.len());

    let metadata = fs::metadata(&output_path).expect("metadata");
    eprintln!("File size: {} bytes", metadata.len());

    // List event types
    eprintln!("Event sequence:");
    for (i, event) in validated_events.iter().enumerate() {
        eprintln!("  {}. {} ({:?})", i + 1, event.event, event.phase);
    }
    eprintln!("=== END ARTIFACT SUMMARY ===\n");
}

// ============================================================================
// Event Names Consistency Tests
// ============================================================================

#[test]
fn test_event_names_constants() {
    // Verify progress event name constants
    assert_eq!(event_names::SESSION_STARTED, "session_started");
    assert_eq!(event_names::SESSION_ENDED, "session_ended");
    assert_eq!(event_names::QUICK_SCAN_STARTED, "quick_scan_started");
    assert_eq!(event_names::QUICK_SCAN_PROGRESS, "quick_scan_progress");
    assert_eq!(event_names::QUICK_SCAN_COMPLETE, "quick_scan_complete");
    assert_eq!(event_names::DEEP_SCAN_STARTED, "deep_scan_started");
    assert_eq!(event_names::DEEP_SCAN_PROGRESS, "deep_scan_progress");
    assert_eq!(event_names::DEEP_SCAN_COMPLETE, "deep_scan_complete");
    assert_eq!(event_names::INFERENCE_STARTED, "inference_started");
    assert_eq!(event_names::INFERENCE_PROGRESS, "inference_progress");
    assert_eq!(event_names::INFERENCE_COMPLETE, "inference_complete");
    assert_eq!(event_names::DECISION_STARTED, "decision_started");
    assert_eq!(event_names::DECISION_COMPLETE, "decision_complete");
    assert_eq!(event_names::ACTION_STARTED, "action_started");
    assert_eq!(event_names::ACTION_COMPLETE, "action_complete");
    assert_eq!(event_names::ACTION_FAILED, "action_failed");
    assert_eq!(event_names::PLAN_READY, "plan_ready");

    eprintln!("[INFO] Progress event name constants verified");
}

#[test]
fn test_log_event_names_constants() {
    // Verify log event name constants
    assert_eq!(log_event_names::RUN_STARTED, "run.started");
    assert_eq!(log_event_names::RUN_FINISHED, "run.finished");
    assert_eq!(log_event_names::SCAN_STARTED, "scan.started");
    assert_eq!(log_event_names::SCAN_SAMPLED, "scan.sampled");
    assert_eq!(log_event_names::SCAN_FINISHED, "scan.finished");
    assert_eq!(log_event_names::INFER_STARTED, "infer.started");
    assert_eq!(log_event_names::INFER_PROC_DONE, "infer.proc_done");
    assert_eq!(log_event_names::INFER_FINISHED, "infer.finished");
    assert_eq!(log_event_names::DECIDE_STARTED, "decide.started");
    assert_eq!(log_event_names::DECIDE_BLOCKED_BY_POLICY, "decide.blocked_by_policy");
    assert_eq!(log_event_names::DECIDE_RECOMMENDED_ACTION, "decide.recommended_action");
    assert_eq!(log_event_names::DECIDE_FINISHED, "decide.finished");
    assert_eq!(log_event_names::APPLY_STARTED, "apply.started");
    assert_eq!(log_event_names::APPLY_INTENT_LOGGED, "apply.intent_logged");
    assert_eq!(log_event_names::APPLY_ACTION_ATTEMPTED, "apply.action_attempted");
    assert_eq!(log_event_names::APPLY_ACTION_RESULT, "apply.action_result");
    assert_eq!(log_event_names::APPLY_FINISHED, "apply.finished");
    assert_eq!(log_event_names::CONFIG_LOADED, "config.loaded");
    assert_eq!(log_event_names::INTERNAL_ERROR, "internal_error");

    eprintln!("[INFO] Log event name constants verified");
}
