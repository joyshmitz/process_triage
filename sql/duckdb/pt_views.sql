-- Process Triage DuckDB Views and Macros
-- Master loader file - loads all view definitions
--
-- Version: 1.0.0
-- Schema Version: 1
--
-- Usage:
--   1. Set the data directory:
--      SET pt_data_dir = '/path/to/telemetry';
--
--   2. Load all views:
--      .read sql/duckdb/pt_views.sql
--
--   3. Query using pt.* views and macros:
--      SELECT * FROM pt.runs LIMIT 10;
--      SELECT * FROM pt.latest_run('my-host');
--      SELECT * FROM pt.candidates_for('session-123');
--
-- Available Views:
--   Session/Run:
--     - pt.runs              : Normalized runs with computed fields
--     - pt.latest_runs       : Most recent run per host
--     - pt.run_history       : Runs with trend indicators
--     - pt.daily_summary     : Daily aggregated statistics
--
--   Candidates/Decisions:
--     - pt.candidates        : Processes with features and inference
--     - pt.recommendations   : Actionable kill recommendations
--     - pt.evidence_terms    : Unpivoted evidence factors
--     - pt.candidate_summary : Aggregate stats per session
--     - pt.proc_type_breakdown: Analysis by process type
--
--   Safety/Governance:
--     - pt.policy_blocks     : Actions blocked by safety gates
--     - pt.gate_summary      : Gate block statistics
--     - pt.fdr_status        : False discovery rate tracking
--     - pt.alpha_investing_status: Risk budget over time
--     - pt.safety_audit      : Safety-relevant audit events
--     - pt.protected_processes: Protected process list
--
--   Calibration:
--     - pt.shadow_labels     : User feedback labels
--     - pt.calibration_bins  : Binned calibration data
--     - pt.false_kill_rate   : FKR with confidence bounds
--     - pt.accuracy_metrics  : Precision/recall/F1
--
--   Drift/Misspecification:
--     - pt.ppc_flags         : Posterior predictive check flags
--     - pt.drift_indicators  : Time-series drift detection
--     - pt.feature_drift     : Feature distribution changes
--     - pt.anomaly_sessions  : Sessions with flags
--
--   Fleet/Multi-Host:
--     - pt.fleet_summary     : Overview of all hosts
--     - pt.fleet_rollup      : Daily aggregated fleet stats
--     - pt.cross_host_patterns: Patterns across hosts
--     - pt.fleet_comparison  : Host comparison metrics
--     - pt.fleet_outliers    : Anomalous hosts
--     - pt.recurring_offenders: Repeated candidates
--
-- Available Macros (parameterized queries):
--   - pt.latest_run(host_id)
--   - pt.run_summary(session_id)
--   - pt.candidates_for(session_id)
--   - pt.recommendations_for(session_id)
--   - pt.top_terms(session_id, pid, start_id)
--   - pt.policy_blocks_for(session_id)
--   - pt.fdr_status_for(session_id)
--   - pt.alpha_investing_for(host_id, window_days)
--   - pt.shadow_labels_for(host_id, window_days)
--   - pt.calibration_curve(host_id, window_days)
--   - pt.false_kill_rate_bounds(host_id, window_days)
--   - pt.ppc_flags_for(session_id)
--   - pt.drift_flags(host_id, window_days)
--   - pt.fleet_rollup_for(window_days)
--   - pt.cross_host_patterns_for(window_days)

-- Load all component files
.read sql/duckdb/00_init.sql
.read sql/duckdb/10_session_views.sql
.read sql/duckdb/20_candidate_views.sql
.read sql/duckdb/30_safety_views.sql
.read sql/duckdb/40_calibration_views.sql
.read sql/duckdb/50_drift_views.sql
.read sql/duckdb/60_fleet_views.sql

-- Confirmation message
SELECT 'Process Triage views loaded successfully. Use pt.* to access views.' AS status;
