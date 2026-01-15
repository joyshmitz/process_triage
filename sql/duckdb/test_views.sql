-- Process Triage DuckDB Views Test Script
--
-- This script tests the view definitions with synthetic fixture data.
-- Run with: duckdb -c ".read sql/duckdb/test_views.sql"
--
-- Tests verify:
-- 1. All views can be created without errors
-- 2. Views return expected column structure
-- 3. Macros execute without errors
-- 4. Basic invariants hold

-- Create test schema
CREATE SCHEMA IF NOT EXISTS test_pt;

-- Create synthetic test data directly (in-memory)
CREATE OR REPLACE VIEW test_pt.runs AS
SELECT
    'session-001' AS session_id,
    'host-abc' AS host_id,
    'testhost' AS hostname,
    'testuser' AS username,
    1000 AS uid,
    'interactive' AS mode,
    true AS deep_scan,
    TIMESTAMP '2024-01-15 10:00:00' AS started_at,
    TIMESTAMP '2024-01-15 10:05:00' AS ended_at,
    300000 AS duration_ms,
    'completed' AS state,
    100 AS processes_scanned,
    10 AS candidates_found,
    5 AS kills_attempted,
    4 AS kills_successful,
    5 AS spares,
    '0.1.0' AS pt_version,
    '0.1.0' AS pt_core_version,
    '1' AS schema_version,
    NULL AS capabilities_hash,
    NULL AS config_snapshot,
    'linux' AS os_family,
    'Ubuntu 22.04' AS os_version,
    '6.5.0' AS kernel_version,
    'x86_64' AS arch,
    8 AS cores,
    17179869184 AS memory_bytes
UNION ALL
SELECT
    'session-002', 'host-abc', 'testhost', 'testuser', 1000, 'interactive', true,
    TIMESTAMP '2024-01-15 11:00:00', TIMESTAMP '2024-01-15 11:03:00', 180000, 'completed',
    95, 8, 4, 4, 4, '0.1.0', '0.1.0', '1', NULL, NULL,
    'linux', 'Ubuntu 22.04', '6.5.0', 'x86_64', 8, 17179869184;

CREATE OR REPLACE VIEW test_pt.proc_samples AS
SELECT
    'session-001' AS session_id,
    TIMESTAMP '2024-01-15 10:01:00' AS sample_ts,
    1 AS sample_seq,
    1234 AS pid,
    1 AS ppid,
    1234 AS pgid,
    1234 AS sid,
    1000 AS uid,
    1000 AS euid,
    12345678 AS start_time_boot,
    'start-001' AS start_id,
    3600 AS age_s,
    'test-process' AS cmd,
    'test-process --arg1' AS cmdline,
    'hash123' AS cmdline_hash,
    '/usr/bin/test' AS exe,
    '/home/user' AS cwd,
    NULL AS tty,
    'S' AS state,
    1000 AS utime_ticks,
    500 AS stime_ticks,
    0 AS cutime_ticks,
    0 AS cstime_ticks,
    52428800 AS rss_bytes,
    104857600 AS vsize_bytes,
    10485760 AS shared_bytes,
    1048576 AS text_bytes,
    5242880 AS data_bytes,
    0 AS nice,
    20 AS priority,
    1 AS num_threads,
    5.5 AS cpu_percent,
    3.2 AS mem_percent,
    1048576 AS io_read_bytes,
    524288 AS io_write_bytes,
    100 AS io_read_ops,
    50 AS io_write_ops,
    10 AS voluntary_ctxt_switches,
    5 AS nonvoluntary_ctxt_switches,
    NULL AS wchan,
    0 AS oom_score,
    0 AS oom_score_adj,
    NULL AS cgroup_path,
    NULL AS systemd_unit,
    NULL AS container_id,
    NULL AS ns_pid,
    NULL AS ns_mnt,
    10 AS fd_count,
    0 AS tcp_listen_count,
    2 AS tcp_estab_count,
    0 AS child_count;

CREATE OR REPLACE VIEW test_pt.proc_features AS
SELECT
    'session-001' AS session_id,
    1234 AS pid,
    'start-001' AS start_id,
    TIMESTAMP '2024-01-15 10:01:00' AS feature_ts,
    'user_app' AS proc_type,
    0.85 AS proc_type_conf,
    3600 AS age_s,
    0.5 AS age_ratio,
    'hours' AS age_bucket,
    5.5 AS cpu_pct_instant,
    4.2 AS cpu_pct_avg,
    1500 AS cpu_delta_ticks,
    0.05 AS cpu_utilization,
    false AS cpu_stalled,
    false AS cpu_spinning,
    50.0 AS mem_mb,
    3.2 AS mem_pct,
    0.01 AS mem_growth_rate,
    'medium' AS mem_bucket,
    1024.0 AS io_read_rate,
    512.0 AS io_write_rate,
    true AS io_active,
    false AS io_idle,
    true AS is_orphan,
    false AS is_zombie,
    false AS is_stopped,
    true AS is_sleeping,
    false AS is_running,
    false AS has_tty,
    NULL AS tty_active,
    NULL AS tty_dead,
    true AS has_network,
    false AS is_listener,
    false AS has_children,
    NULL AS children_active,
    'test-process' AS cmd_pattern,
    'development' AS cmd_category,
    false AS is_protected,
    NULL AS prior_decision,
    NULL AS prior_decision_count;

CREATE OR REPLACE VIEW test_pt.proc_inference AS
SELECT
    'session-001' AS session_id,
    1234 AS pid,
    'start-001' AS start_id,
    TIMESTAMP '2024-01-15 10:01:00' AS inference_ts,
    0.75 AS p_abandoned,
    0.20 AS p_legitimate,
    0.05 AS p_uncertain,
    1.2 AS log_bayes_factor,
    'substantial' AS bayes_factor_interpretation,
    0.72 AS score,
    'medium' AS confidence,
    'kill' AS recommendation,
    0.1 AS evidence_prior,
    0.2 AS evidence_age,
    0.15 AS evidence_cpu,
    0.1 AS evidence_memory,
    0.05 AS evidence_io,
    0.08 AS evidence_state,
    0.02 AS evidence_network,
    0.01 AS evidence_children,
    0.01 AS evidence_history,
    NULL AS evidence_deep,
    '["orphan", "idle", "old"]' AS evidence_tags_json,
    NULL AS evidence_ledger_json,
    true AS passed_safety_gates,
    NULL AS blocked_by_gate,
    NULL AS safety_gate_details;

CREATE OR REPLACE VIEW test_pt.outcomes AS
SELECT
    'session-001' AS session_id,
    TIMESTAMP '2024-01-15 10:02:00' AS outcome_ts,
    1234 AS pid,
    'start-001' AS start_id,
    'kill' AS recommendation,
    'kill' AS decision,
    'user' AS decision_source,
    'SIGTERM' AS action_type,
    true AS action_attempted,
    true AS action_successful,
    'SIGTERM' AS signal_sent,
    'terminated' AS signal_response,
    true AS verified_identity,
    1234 AS pid_at_action,
    true AS start_id_matched,
    'terminated' AS process_state_after,
    52428800 AS memory_freed_bytes,
    NULL AS error_message,
    'correct_kill' AS user_feedback,
    TIMESTAMP '2024-01-15 10:10:00' AS feedback_ts,
    'Process was indeed abandoned' AS feedback_note,
    'test-process' AS cmd,
    'hash123' AS cmdline_hash,
    0.72 AS score,
    'user_app' AS proc_type;

CREATE OR REPLACE VIEW test_pt.audit AS
SELECT
    TIMESTAMP '2024-01-15 10:02:00' AS audit_ts,
    'session-001' AS session_id,
    'kill_succeeded' AS event_type,
    'info' AS severity,
    'system' AS actor,
    1234 AS target_pid,
    'start-001' AS target_start_id,
    'Process terminated successfully' AS message,
    '{"signal": "SIGTERM"}' AS details_json,
    'host-abc' AS host_id;

-- Override pt schema views with test data
CREATE SCHEMA IF NOT EXISTS pt;

CREATE OR REPLACE MACRO pt.table_path(table_name) AS
    'nonexistent_' || table_name;  -- Will cause error if raw views are accessed

CREATE OR REPLACE VIEW pt.raw_runs AS SELECT * FROM test_pt.runs;
CREATE OR REPLACE VIEW pt.raw_proc_samples AS SELECT * FROM test_pt.proc_samples;
CREATE OR REPLACE VIEW pt.raw_proc_features AS SELECT * FROM test_pt.proc_features;
CREATE OR REPLACE VIEW pt.raw_proc_inference AS SELECT * FROM test_pt.proc_inference;
CREATE OR REPLACE VIEW pt.raw_outcomes AS SELECT * FROM test_pt.outcomes;
CREATE OR REPLACE VIEW pt.raw_audit AS SELECT * FROM test_pt.audit;

-- Now load the views (they will use our test data)
.read sql/duckdb/10_session_views.sql
.read sql/duckdb/20_candidate_views.sql
.read sql/duckdb/30_safety_views.sql
.read sql/duckdb/40_calibration_views.sql
.read sql/duckdb/50_drift_views.sql
.read sql/duckdb/60_fleet_views.sql

-- ============================================================================
-- TESTS
-- ============================================================================

-- Test 1: Session views return data
SELECT 'TEST 1: pt.runs' AS test;
SELECT COUNT(*) AS row_count FROM pt.runs;
SELECT CASE WHEN COUNT(*) = 2 THEN 'PASS' ELSE 'FAIL' END AS result FROM pt.runs;

-- Test 2: Latest runs view
SELECT 'TEST 2: pt.latest_runs' AS test;
SELECT COUNT(*) AS row_count FROM pt.latest_runs;
SELECT CASE WHEN COUNT(*) = 1 THEN 'PASS' ELSE 'FAIL' END AS result FROM pt.latest_runs;

-- Test 3: Candidates view
SELECT 'TEST 3: pt.candidates' AS test;
SELECT COUNT(*) AS row_count FROM pt.candidates;
SELECT CASE WHEN COUNT(*) >= 1 THEN 'PASS' ELSE 'FAIL' END AS result FROM pt.candidates;

-- Test 4: Recommendations view
SELECT 'TEST 4: pt.recommendations' AS test;
SELECT COUNT(*) AS row_count FROM pt.recommendations;

-- Test 5: Candidate summary
SELECT 'TEST 5: pt.candidate_summary' AS test;
SELECT * FROM pt.candidate_summary;

-- Test 6: FDR status
SELECT 'TEST 6: pt.fdr_status' AS test;
SELECT * FROM pt.fdr_status;

-- Test 7: Shadow labels
SELECT 'TEST 7: pt.shadow_labels' AS test;
SELECT COUNT(*) AS labeled_count FROM pt.shadow_labels;

-- Test 8: Accuracy metrics
SELECT 'TEST 8: pt.accuracy_metrics' AS test;
SELECT * FROM pt.accuracy_metrics;

-- Test 9: Fleet summary
SELECT 'TEST 9: pt.fleet_summary' AS test;
SELECT * FROM pt.fleet_summary;

-- Test 10: Run history
SELECT 'TEST 10: pt.run_history' AS test;
SELECT session_id, candidate_delta, run_rank FROM pt.run_history;

-- Test 11: Macro - latest_run
SELECT 'TEST 11: pt.latest_run macro' AS test;
SELECT * FROM pt.latest_run('host-abc');

-- Test 12: Macro - candidates_for
SELECT 'TEST 12: pt.candidates_for macro' AS test;
SELECT pid, score, recommendation FROM pt.candidates_for('session-001');

-- Test 13: Evidence terms
SELECT 'TEST 13: pt.evidence_terms' AS test;
SELECT session_id, pid, term_data FROM pt.evidence_terms LIMIT 5;

-- Test 14: Policy blocks (should be empty with test data)
SELECT 'TEST 14: pt.policy_blocks' AS test;
SELECT COUNT(*) AS blocked_count FROM pt.policy_blocks;

-- Test 15: Daily summary
SELECT 'TEST 15: pt.daily_summary' AS test;
SELECT * FROM pt.daily_summary;

-- Final summary
SELECT '=== ALL TESTS COMPLETED ===' AS status;
SELECT 'Views loaded and queryable with test data' AS summary;
