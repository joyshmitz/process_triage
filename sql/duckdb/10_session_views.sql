-- Process Triage: Session / Run Introspection Views
--
-- Views for understanding session metadata, history, and run summaries.

-- pt_runs: Normalized runs table with computed fields
CREATE OR REPLACE VIEW pt.runs AS
SELECT
    session_id,
    host_id,
    hostname,
    username,
    uid,
    mode,
    deep_scan,
    started_at,
    ended_at,
    duration_ms,
    state,
    processes_scanned,
    candidates_found,
    kills_attempted,
    kills_successful,
    spares,
    pt_version,
    pt_core_version,
    schema_version,
    os_family,
    os_version,
    kernel_version,
    arch,
    cores,
    memory_bytes,
    -- Computed fields
    CASE
        WHEN processes_scanned > 0
        THEN ROUND(100.0 * candidates_found / processes_scanned, 2)
        ELSE 0
    END AS candidate_rate_pct,
    CASE
        WHEN candidates_found > 0
        THEN ROUND(100.0 * kills_attempted / candidates_found, 2)
        ELSE 0
    END AS kill_rate_pct,
    CASE
        WHEN kills_attempted > 0
        THEN ROUND(100.0 * kills_successful / kills_attempted, 2)
        ELSE NULL
    END AS kill_success_rate_pct,
    DATE_TRUNC('day', started_at) AS run_date,
    DATE_TRUNC('hour', started_at) AS run_hour
FROM pt.raw_runs;

-- pt_latest_run: Most recent run per host
CREATE OR REPLACE VIEW pt.latest_runs AS
SELECT DISTINCT ON (host_id) *
FROM pt.runs
ORDER BY host_id, started_at DESC;

-- Macro: Get latest run for a specific host
CREATE OR REPLACE MACRO pt.latest_run(host) AS
    TABLE SELECT * FROM pt.runs
    WHERE host_id = host
    ORDER BY started_at DESC
    LIMIT 1;

-- Macro: Get run summary for a session
CREATE OR REPLACE MACRO pt.run_summary(sid) AS
    TABLE SELECT
        r.session_id,
        r.host_id,
        r.hostname,
        r.mode,
        r.started_at,
        r.ended_at,
        r.duration_ms,
        r.state,
        r.processes_scanned,
        r.candidates_found,
        r.kills_attempted,
        r.kills_successful,
        r.spares,
        r.candidate_rate_pct,
        r.kill_rate_pct,
        r.kill_success_rate_pct,
        -- Aggregate from inference
        (SELECT COUNT(*) FROM pt.raw_proc_inference i WHERE i.session_id = r.session_id AND i.recommendation = 'kill') AS kill_recommendations,
        (SELECT COUNT(*) FROM pt.raw_proc_inference i WHERE i.session_id = r.session_id AND i.recommendation = 'spare') AS spare_recommendations,
        (SELECT COUNT(*) FROM pt.raw_proc_inference i WHERE i.session_id = r.session_id AND NOT i.passed_safety_gates) AS blocked_by_gates,
        -- Aggregate from outcomes
        (SELECT COUNT(*) FROM pt.raw_outcomes o WHERE o.session_id = r.session_id AND o.user_feedback IS NOT NULL) AS feedback_count
    FROM pt.runs r
    WHERE r.session_id = sid;

-- pt_run_history: Recent runs with trend indicators
CREATE OR REPLACE VIEW pt.run_history AS
SELECT
    r.*,
    LAG(candidates_found) OVER (PARTITION BY host_id ORDER BY started_at) AS prev_candidates,
    LAG(kills_successful) OVER (PARTITION BY host_id ORDER BY started_at) AS prev_kills,
    candidates_found - COALESCE(LAG(candidates_found) OVER (PARTITION BY host_id ORDER BY started_at), candidates_found) AS candidate_delta,
    ROW_NUMBER() OVER (PARTITION BY host_id ORDER BY started_at DESC) AS run_rank
FROM pt.runs r;

-- pt_daily_summary: Aggregated daily statistics
CREATE OR REPLACE VIEW pt.daily_summary AS
SELECT
    host_id,
    run_date,
    COUNT(*) AS run_count,
    SUM(processes_scanned) AS total_processes_scanned,
    SUM(candidates_found) AS total_candidates,
    SUM(kills_attempted) AS total_kill_attempts,
    SUM(kills_successful) AS total_kills,
    SUM(spares) AS total_spares,
    AVG(duration_ms) AS avg_duration_ms,
    MAX(duration_ms) AS max_duration_ms,
    AVG(candidate_rate_pct) AS avg_candidate_rate_pct
FROM pt.runs
GROUP BY host_id, run_date
ORDER BY host_id, run_date DESC;
