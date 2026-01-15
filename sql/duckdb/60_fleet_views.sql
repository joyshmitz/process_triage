-- Process Triage: Fleet / Multi-Host Views
--
-- Views for aggregating and comparing data across multiple hosts.

-- pt_fleet_summary: Overview of all hosts in the fleet
CREATE OR REPLACE VIEW pt.fleet_summary AS
SELECT
    host_id,
    MAX(hostname) AS hostname,
    COUNT(*) AS total_runs,
    MAX(started_at) AS last_run,
    MIN(started_at) AS first_run,
    SUM(processes_scanned) AS total_processes_scanned,
    SUM(candidates_found) AS total_candidates,
    SUM(kills_successful) AS total_kills,
    AVG(candidate_rate_pct) AS avg_candidate_rate,
    AVG(kill_success_rate_pct) AS avg_kill_success_rate,
    MAX(os_family) AS os_family,
    MAX(arch) AS arch,
    MAX(cores) AS cores,
    MAX(memory_bytes) AS memory_bytes
FROM pt.runs
GROUP BY host_id;

-- pt_fleet_rollup: Aggregated fleet statistics within time window
CREATE OR REPLACE VIEW pt.fleet_rollup AS
WITH daily AS (
    SELECT
        DATE_TRUNC('day', started_at) AS day,
        COUNT(DISTINCT host_id) AS active_hosts,
        COUNT(*) AS total_runs,
        SUM(processes_scanned) AS processes_scanned,
        SUM(candidates_found) AS candidates_found,
        SUM(kills_attempted) AS kills_attempted,
        SUM(kills_successful) AS kills_successful,
        AVG(duration_ms) AS avg_duration_ms
    FROM pt.runs
    GROUP BY DATE_TRUNC('day', started_at)
)
SELECT
    day,
    active_hosts,
    total_runs,
    processes_scanned,
    candidates_found,
    kills_attempted,
    kills_successful,
    avg_duration_ms,
    ROUND(100.0 * candidates_found / NULLIF(processes_scanned, 0), 2) AS candidate_rate_pct,
    ROUND(100.0 * kills_successful / NULLIF(kills_attempted, 0), 2) AS kill_success_rate_pct
FROM daily
ORDER BY day DESC;

-- Macro: Get fleet rollup for a time window
CREATE OR REPLACE MACRO pt.fleet_rollup_for(window_days) AS
    TABLE SELECT * FROM pt.fleet_rollup
    WHERE day >= CURRENT_DATE - INTERVAL (window_days) DAY
    ORDER BY day DESC;

-- pt_cross_host_patterns: Patterns that appear across multiple hosts
CREATE OR REPLACE VIEW pt.cross_host_patterns AS
WITH pattern_hosts AS (
    SELECT
        f.cmd_pattern,
        f.cmd_category,
        f.proc_type,
        r.host_id,
        COUNT(*) AS occurrences,
        AVG(i.p_abandoned) AS mean_p_abandoned,
        AVG(i.score) AS mean_score,
        COUNT(*) FILTER (WHERE i.recommendation = 'kill') AS kill_recommendations
    FROM pt.raw_proc_features f
    JOIN pt.raw_proc_inference i
        ON f.session_id = i.session_id
        AND f.pid = i.pid
        AND f.start_id = i.start_id
    JOIN pt.raw_runs r ON f.session_id = r.session_id
    GROUP BY f.cmd_pattern, f.cmd_category, f.proc_type, r.host_id
)
SELECT
    cmd_pattern,
    cmd_category,
    proc_type,
    COUNT(DISTINCT host_id) AS host_count,
    SUM(occurrences) AS total_occurrences,
    AVG(mean_p_abandoned) AS overall_mean_p,
    AVG(mean_score) AS overall_mean_score,
    SUM(kill_recommendations) AS total_kill_recommendations,
    -- List of hosts where this pattern appears
    ARRAY_AGG(DISTINCT host_id) AS hosts
FROM pattern_hosts
GROUP BY cmd_pattern, cmd_category, proc_type
HAVING COUNT(DISTINCT host_id) > 1
ORDER BY host_count DESC, total_occurrences DESC;

-- Macro: Get cross-host patterns for a time window
CREATE OR REPLACE MACRO pt.cross_host_patterns_for(window_days) AS
    TABLE WITH recent AS (
        SELECT
            f.cmd_pattern,
            f.cmd_category,
            f.proc_type,
            r.host_id,
            i.p_abandoned,
            i.score,
            i.recommendation
        FROM pt.raw_proc_features f
        JOIN pt.raw_proc_inference i
            ON f.session_id = i.session_id AND f.pid = i.pid AND f.start_id = i.start_id
        JOIN pt.raw_runs r ON f.session_id = r.session_id
        WHERE r.started_at >= CURRENT_TIMESTAMP - INTERVAL (window_days) DAY
    )
    SELECT
        cmd_pattern,
        cmd_category,
        proc_type,
        COUNT(DISTINCT host_id) AS host_count,
        COUNT(*) AS total_occurrences,
        AVG(p_abandoned) AS mean_p,
        COUNT(*) FILTER (WHERE recommendation = 'kill') AS kill_count,
        ARRAY_AGG(DISTINCT host_id) AS hosts
    FROM recent
    GROUP BY cmd_pattern, cmd_category, proc_type
    HAVING COUNT(DISTINCT host_id) > 1
    ORDER BY host_count DESC, total_occurrences DESC;

-- pt_fleet_comparison: Compare metrics across hosts
CREATE OR REPLACE VIEW pt.fleet_comparison AS
WITH host_metrics AS (
    SELECT
        host_id,
        COUNT(*) AS run_count,
        AVG(candidate_rate_pct) AS avg_candidate_rate,
        AVG(kill_rate_pct) AS avg_kill_rate,
        SUM(kills_successful) AS total_kills,
        AVG(duration_ms) AS avg_duration
    FROM pt.runs
    GROUP BY host_id
),
fleet_avg AS (
    SELECT
        AVG(avg_candidate_rate) AS fleet_candidate_rate,
        AVG(avg_kill_rate) AS fleet_kill_rate,
        AVG(avg_duration) AS fleet_duration
    FROM host_metrics
)
SELECT
    h.host_id,
    h.run_count,
    h.avg_candidate_rate,
    h.avg_kill_rate,
    h.total_kills,
    h.avg_duration,
    -- Comparison to fleet average
    h.avg_candidate_rate - f.fleet_candidate_rate AS candidate_rate_vs_fleet,
    h.avg_kill_rate - f.fleet_kill_rate AS kill_rate_vs_fleet,
    h.avg_duration - f.fleet_duration AS duration_vs_fleet,
    -- Percentile within fleet
    PERCENT_RANK() OVER (ORDER BY h.avg_candidate_rate) AS candidate_rate_percentile,
    PERCENT_RANK() OVER (ORDER BY h.total_kills) AS total_kills_percentile
FROM host_metrics h
CROSS JOIN fleet_avg f
ORDER BY h.total_kills DESC;

-- pt_fleet_outliers: Hosts with anomalous behavior
CREATE OR REPLACE VIEW pt.fleet_outliers AS
WITH host_stats AS (
    SELECT
        host_id,
        AVG(candidate_rate_pct) AS avg_candidate_rate,
        AVG(kills_successful::FLOAT / NULLIF(processes_scanned, 0)) AS kill_ratio,
        COUNT(*) AS run_count
    FROM pt.runs
    GROUP BY host_id
    HAVING COUNT(*) >= 3  -- Need minimum data points
),
fleet_stats AS (
    SELECT
        AVG(avg_candidate_rate) AS mean_candidate_rate,
        STDDEV(avg_candidate_rate) AS std_candidate_rate,
        AVG(kill_ratio) AS mean_kill_ratio,
        STDDEV(kill_ratio) AS std_kill_ratio
    FROM host_stats
)
SELECT
    h.host_id,
    h.avg_candidate_rate,
    h.kill_ratio,
    h.run_count,
    f.mean_candidate_rate,
    f.std_candidate_rate,
    -- Z-scores
    CASE
        WHEN f.std_candidate_rate > 0
        THEN (h.avg_candidate_rate - f.mean_candidate_rate) / f.std_candidate_rate
        ELSE 0
    END AS candidate_rate_zscore,
    CASE
        WHEN f.std_kill_ratio > 0
        THEN (h.kill_ratio - f.mean_kill_ratio) / f.std_kill_ratio
        ELSE 0
    END AS kill_ratio_zscore,
    -- Outlier flags
    CASE
        WHEN f.std_candidate_rate > 0 AND
             ABS(h.avg_candidate_rate - f.mean_candidate_rate) > 2 * f.std_candidate_rate
        THEN true ELSE false
    END AS is_candidate_rate_outlier,
    CASE
        WHEN f.std_kill_ratio > 0 AND
             ABS(h.kill_ratio - f.mean_kill_ratio) > 2 * f.std_kill_ratio
        THEN true ELSE false
    END AS is_kill_ratio_outlier
FROM host_stats h
CROSS JOIN fleet_stats f
WHERE f.std_candidate_rate > 0
ORDER BY ABS((h.avg_candidate_rate - f.mean_candidate_rate) / f.std_candidate_rate) DESC;

-- pt_recurring_offenders: Processes that repeatedly become candidates across hosts/sessions
CREATE OR REPLACE VIEW pt.recurring_offenders AS
SELECT
    f.cmd_pattern,
    f.cmd_category,
    COUNT(DISTINCT r.host_id) AS host_count,
    COUNT(DISTINCT f.session_id) AS session_count,
    COUNT(*) AS total_occurrences,
    AVG(i.p_abandoned) AS mean_p_abandoned,
    AVG(i.score) AS mean_score,
    SUM(CASE WHEN o.action_successful THEN 1 ELSE 0 END) AS times_killed,
    MAX(r.started_at) AS last_seen
FROM pt.raw_proc_features f
JOIN pt.raw_proc_inference i
    ON f.session_id = i.session_id AND f.pid = i.pid AND f.start_id = i.start_id
JOIN pt.raw_runs r ON f.session_id = r.session_id
LEFT JOIN pt.raw_outcomes o
    ON f.session_id = o.session_id AND f.pid = o.pid AND f.start_id = o.start_id
WHERE i.recommendation = 'kill'
GROUP BY f.cmd_pattern, f.cmd_category
HAVING COUNT(DISTINCT f.session_id) >= 3
ORDER BY session_count DESC, total_occurrences DESC;
