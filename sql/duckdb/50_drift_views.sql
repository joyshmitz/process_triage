-- Process Triage: Drift / Misspecification Views
--
-- Views for detecting model drift, distribution shifts, and potential misspecification.

-- pt_ppc_flags: Posterior Predictive Check flags per session
-- Identifies sessions where model predictions deviate from expectations
CREATE OR REPLACE VIEW pt.ppc_flags AS
WITH session_stats AS (
    SELECT
        i.session_id,
        r.host_id,
        r.started_at,
        COUNT(*) AS total_candidates,
        AVG(i.p_abandoned) AS mean_p_abandoned,
        STDDEV(i.p_abandoned) AS std_p_abandoned,
        AVG(i.score) AS mean_score,
        STDDEV(i.score) AS std_score,
        -- Distribution shape indicators
        PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY i.p_abandoned) AS median_p_abandoned,
        PERCENTILE_CONT(0.25) WITHIN GROUP (ORDER BY i.p_abandoned) AS q1_p_abandoned,
        PERCENTILE_CONT(0.75) WITHIN GROUP (ORDER BY i.p_abandoned) AS q3_p_abandoned,
        -- Extreme value counts
        COUNT(*) FILTER (WHERE i.p_abandoned > 0.9) AS high_confidence_count,
        COUNT(*) FILTER (WHERE i.p_abandoned < 0.1) AS low_confidence_count,
        COUNT(*) FILTER (WHERE i.p_abandoned BETWEEN 0.4 AND 0.6) AS uncertain_count
    FROM pt.raw_proc_inference i
    JOIN pt.raw_runs r ON i.session_id = r.session_id
    GROUP BY i.session_id, r.host_id, r.started_at
),
host_baseline AS (
    SELECT
        host_id,
        AVG(mean_p_abandoned) AS baseline_mean_p,
        STDDEV(mean_p_abandoned) AS baseline_std_p,
        AVG(mean_score) AS baseline_mean_score,
        AVG(total_candidates) AS baseline_candidate_count
    FROM session_stats
    GROUP BY host_id
)
SELECT
    s.session_id,
    s.host_id,
    s.started_at,
    s.total_candidates,
    s.mean_p_abandoned,
    s.std_p_abandoned,
    s.mean_score,
    s.high_confidence_count,
    s.uncertain_count,
    b.baseline_mean_p,
    b.baseline_std_p,
    -- PPC flags
    CASE
        WHEN s.uncertain_count > s.total_candidates * 0.5
        THEN 'high_uncertainty'
        ELSE NULL
    END AS uncertainty_flag,
    CASE
        WHEN b.baseline_std_p > 0 AND ABS(s.mean_p_abandoned - b.baseline_mean_p) > 2 * b.baseline_std_p
        THEN 'distribution_shift'
        ELSE NULL
    END AS distribution_flag,
    CASE
        WHEN s.std_p_abandoned < 0.05 AND s.total_candidates > 10
        THEN 'low_variance'
        ELSE NULL
    END AS variance_flag,
    CASE
        WHEN s.total_candidates > b.baseline_candidate_count * 2
        THEN 'candidate_spike'
        WHEN s.total_candidates < b.baseline_candidate_count * 0.5 AND b.baseline_candidate_count > 10
        THEN 'candidate_drop'
        ELSE NULL
    END AS volume_flag
FROM session_stats s
LEFT JOIN host_baseline b ON s.host_id = b.host_id;

-- Macro: Get PPC flags for a session
CREATE OR REPLACE MACRO pt.ppc_flags_for(sid) AS
    TABLE SELECT * FROM pt.ppc_flags WHERE session_id = sid;

-- pt_drift_indicators: Time-series drift detection across sessions
CREATE OR REPLACE VIEW pt.drift_indicators AS
WITH session_metrics AS (
    SELECT
        r.host_id,
        r.session_id,
        r.started_at,
        DATE_TRUNC('day', r.started_at) AS day,
        AVG(i.p_abandoned) AS mean_p,
        AVG(i.score) AS mean_score,
        COUNT(*) AS candidate_count,
        COUNT(*) FILTER (WHERE i.recommendation = 'kill') AS kill_count
    FROM pt.raw_runs r
    JOIN pt.raw_proc_inference i ON r.session_id = i.session_id
    GROUP BY r.host_id, r.session_id, r.started_at
)
SELECT
    host_id,
    session_id,
    started_at,
    day,
    mean_p,
    mean_score,
    candidate_count,
    kill_count,
    -- Rolling averages (7-day window)
    AVG(mean_p) OVER (
        PARTITION BY host_id
        ORDER BY started_at
        RANGE BETWEEN INTERVAL '7 days' PRECEDING AND CURRENT ROW
    ) AS rolling_mean_p,
    AVG(mean_score) OVER (
        PARTITION BY host_id
        ORDER BY started_at
        RANGE BETWEEN INTERVAL '7 days' PRECEDING AND CURRENT ROW
    ) AS rolling_mean_score,
    -- Deviation from rolling average
    mean_p - AVG(mean_p) OVER (
        PARTITION BY host_id
        ORDER BY started_at
        RANGE BETWEEN INTERVAL '7 days' PRECEDING AND CURRENT ROW
    ) AS p_deviation,
    -- CUSUM-style cumulative deviation
    SUM(mean_p - 0.5) OVER (
        PARTITION BY host_id
        ORDER BY started_at
        ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
    ) AS cusum_p
FROM session_metrics
ORDER BY host_id, started_at;

-- Macro: Get drift flags for a host within time window
CREATE OR REPLACE MACRO pt.drift_flags(host, window_days) AS
    TABLE WITH recent AS (
        SELECT * FROM pt.drift_indicators
        WHERE host_id = host
          AND started_at >= CURRENT_TIMESTAMP - INTERVAL (window_days) DAY
    )
    SELECT
        host_id,
        -- Summary statistics
        COUNT(*) AS session_count,
        AVG(mean_p) AS overall_mean_p,
        STDDEV(mean_p) AS overall_std_p,
        -- Drift indicators
        MAX(ABS(p_deviation)) AS max_deviation,
        MAX(cusum_p) AS max_cusum,
        MIN(cusum_p) AS min_cusum,
        -- Trend direction
        CASE
            WHEN REGR_SLOPE(mean_p, EXTRACT(EPOCH FROM started_at)) > 0.0001 THEN 'increasing'
            WHEN REGR_SLOPE(mean_p, EXTRACT(EPOCH FROM started_at)) < -0.0001 THEN 'decreasing'
            ELSE 'stable'
        END AS p_trend
    FROM recent
    GROUP BY host_id;

-- pt_feature_drift: Feature distribution changes over time
CREATE OR REPLACE VIEW pt.feature_drift AS
WITH daily_features AS (
    SELECT
        r.host_id,
        DATE_TRUNC('day', f.feature_ts) AS day,
        AVG(f.cpu_pct_instant) AS mean_cpu,
        AVG(f.mem_mb) AS mean_mem,
        AVG(f.age_s) AS mean_age,
        COUNT(*) FILTER (WHERE f.is_orphan) AS orphan_count,
        COUNT(*) FILTER (WHERE f.is_zombie) AS zombie_count,
        COUNT(*) AS total_count
    FROM pt.raw_proc_features f
    JOIN pt.raw_runs r ON f.session_id = r.session_id
    GROUP BY r.host_id, DATE_TRUNC('day', f.feature_ts)
)
SELECT
    host_id,
    day,
    mean_cpu,
    mean_mem,
    mean_age,
    orphan_count,
    zombie_count,
    total_count,
    -- Compare to 7-day rolling average
    mean_cpu - AVG(mean_cpu) OVER (
        PARTITION BY host_id ORDER BY day
        ROWS BETWEEN 7 PRECEDING AND 1 PRECEDING
    ) AS cpu_change,
    mean_mem - AVG(mean_mem) OVER (
        PARTITION BY host_id ORDER BY day
        ROWS BETWEEN 7 PRECEDING AND 1 PRECEDING
    ) AS mem_change
FROM daily_features
ORDER BY host_id, day DESC;

-- pt_anomaly_sessions: Sessions flagged with potential issues
CREATE OR REPLACE VIEW pt.anomaly_sessions AS
SELECT
    p.session_id,
    p.host_id,
    p.started_at,
    p.total_candidates,
    -- Collect all flags
    ARRAY_AGG(DISTINCT flag) FILTER (WHERE flag IS NOT NULL) AS flags,
    COUNT(DISTINCT flag) FILTER (WHERE flag IS NOT NULL) AS flag_count
FROM pt.ppc_flags p
CROSS JOIN LATERAL (
    VALUES
        (p.uncertainty_flag),
        (p.distribution_flag),
        (p.variance_flag),
        (p.volume_flag)
) AS t(flag)
GROUP BY p.session_id, p.host_id, p.started_at, p.total_candidates
HAVING COUNT(DISTINCT flag) FILTER (WHERE flag IS NOT NULL) > 0
ORDER BY p.started_at DESC;
