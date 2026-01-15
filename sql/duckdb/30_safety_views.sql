-- Process Triage: Safety & Governance Views
--
-- Views for analyzing safety gates, policy enforcement, and risk controls.

-- pt_policy_blocks: Details of what gates blocked actions
CREATE OR REPLACE VIEW pt.policy_blocks AS
SELECT
    i.session_id,
    i.pid,
    i.start_id,
    i.inference_ts,
    i.recommendation,
    i.score,
    i.confidence,
    i.blocked_by_gate,
    i.safety_gate_details,
    f.cmd_pattern,
    f.cmd_category,
    f.proc_type,
    f.is_protected,
    f.has_network,
    f.has_children,
    f.has_tty,
    -- Parse gate details if available
    CASE
        WHEN i.blocked_by_gate LIKE '%protected%' THEN 'protected_process'
        WHEN i.blocked_by_gate LIKE '%confidence%' THEN 'low_confidence'
        WHEN i.blocked_by_gate LIKE '%threshold%' THEN 'below_threshold'
        WHEN i.blocked_by_gate LIKE '%fdr%' OR i.blocked_by_gate LIKE '%FDR%' THEN 'fdr_budget'
        WHEN i.blocked_by_gate LIKE '%alpha%' THEN 'alpha_investing'
        WHEN i.blocked_by_gate LIKE '%children%' THEN 'has_children'
        WHEN i.blocked_by_gate LIKE '%network%' THEN 'network_active'
        WHEN i.blocked_by_gate LIKE '%tty%' THEN 'tty_active'
        WHEN i.blocked_by_gate LIKE '%root%' OR i.blocked_by_gate LIKE '%uid%' THEN 'privileged_user'
        ELSE 'other'
    END AS block_category
FROM pt.raw_proc_inference i
LEFT JOIN pt.raw_proc_features f
    ON i.session_id = f.session_id
    AND i.pid = f.pid
    AND i.start_id = f.start_id
WHERE NOT i.passed_safety_gates;

-- Macro: Get policy blocks for a session
CREATE OR REPLACE MACRO pt.policy_blocks_for(sid) AS
    TABLE SELECT * FROM pt.policy_blocks
    WHERE session_id = sid
    ORDER BY score DESC;

-- pt_gate_summary: Aggregate gate block statistics
CREATE OR REPLACE VIEW pt.gate_summary AS
SELECT
    session_id,
    blocked_by_gate,
    COUNT(*) AS block_count,
    AVG(score) AS avg_blocked_score,
    MAX(score) AS max_blocked_score
FROM pt.policy_blocks
GROUP BY session_id, blocked_by_gate
ORDER BY session_id, block_count DESC;

-- pt_fdr_status: False Discovery Rate budget tracking
-- Tracks e-values and FDR thresholds for selected recommendations
CREATE OR REPLACE VIEW pt.fdr_status AS
SELECT
    i.session_id,
    COUNT(*) AS total_candidates,
    COUNT(*) FILTER (WHERE i.recommendation = 'kill' AND i.passed_safety_gates) AS selected_count,
    AVG(i.p_abandoned) FILTER (WHERE i.recommendation = 'kill' AND i.passed_safety_gates) AS avg_p_abandoned_selected,
    -- Estimated FDR = avg(1 - p_abandoned) for selected set
    AVG(1 - i.p_abandoned) FILTER (WHERE i.recommendation = 'kill' AND i.passed_safety_gates) AS estimated_fdr,
    -- Conservative bound using Benjamini-Hochberg concept
    MAX(i.score) FILTER (WHERE i.recommendation = 'kill' AND i.passed_safety_gates) AS max_score_selected,
    MIN(i.score) FILTER (WHERE i.recommendation = 'kill' AND i.passed_safety_gates) AS min_score_selected,
    -- E-value summary (log Bayes factor interpreted as e-value)
    AVG(i.log_bayes_factor) FILTER (WHERE i.recommendation = 'kill') AS avg_log_bf_kills,
    SUM(i.log_bayes_factor) FILTER (WHERE i.recommendation = 'kill' AND i.passed_safety_gates) AS total_log_bf_selected
FROM pt.raw_proc_inference i
GROUP BY i.session_id;

-- Macro: Get FDR status for a session
CREATE OR REPLACE MACRO pt.fdr_status_for(sid) AS
    TABLE SELECT * FROM pt.fdr_status WHERE session_id = sid;

-- pt_alpha_investing_status: Risk budget over time
-- Tracks cumulative risk budget consumption across sessions
CREATE OR REPLACE VIEW pt.alpha_investing_status AS
WITH session_risk AS (
    SELECT
        r.host_id,
        r.session_id,
        r.started_at,
        -- Risk consumed = actions taken weighted by (1 - confidence)
        COALESCE(SUM(
            CASE WHEN o.action_attempted THEN
                CASE i.confidence
                    WHEN 'high' THEN 0.01
                    WHEN 'medium' THEN 0.05
                    WHEN 'low' THEN 0.1
                    ELSE 0.05
                END
            ELSE 0 END
        ), 0) AS risk_consumed,
        COUNT(*) FILTER (WHERE o.action_attempted) AS actions_taken,
        COUNT(*) FILTER (WHERE o.action_successful) AS actions_succeeded
    FROM pt.raw_runs r
    LEFT JOIN pt.raw_proc_inference i ON r.session_id = i.session_id
    LEFT JOIN pt.raw_outcomes o ON i.session_id = o.session_id
        AND i.pid = o.pid AND i.start_id = o.start_id
    GROUP BY r.host_id, r.session_id, r.started_at
)
SELECT
    host_id,
    session_id,
    started_at,
    risk_consumed,
    actions_taken,
    actions_succeeded,
    -- Cumulative risk over time for this host
    SUM(risk_consumed) OVER (
        PARTITION BY host_id
        ORDER BY started_at
        ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
    ) AS cumulative_risk,
    -- Risk budget remaining (assuming starting budget of 1.0)
    1.0 - SUM(risk_consumed) OVER (
        PARTITION BY host_id
        ORDER BY started_at
        ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
    ) AS risk_budget_remaining
FROM session_risk
ORDER BY host_id, started_at;

-- Macro: Get alpha investing status for a host within time window
CREATE OR REPLACE MACRO pt.alpha_investing_for(host, window_days) AS
    TABLE SELECT * FROM pt.alpha_investing_status
    WHERE host_id = host
      AND started_at >= CURRENT_TIMESTAMP - INTERVAL (window_days) DAY
    ORDER BY started_at DESC;

-- pt_safety_audit: Audit trail for safety-relevant events
CREATE OR REPLACE VIEW pt.safety_audit AS
SELECT
    audit_ts,
    session_id,
    event_type,
    severity,
    actor,
    target_pid,
    target_start_id,
    message,
    details_json,
    host_id
FROM pt.raw_audit
WHERE event_type IN (
    'gate_block',
    'policy_override',
    'kill_attempted',
    'kill_succeeded',
    'kill_failed',
    'identity_mismatch',
    'verification_failed',
    'budget_exceeded'
)
ORDER BY audit_ts DESC;

-- pt_protected_processes: Processes that were protected from action
CREATE OR REPLACE VIEW pt.protected_processes AS
SELECT
    c.session_id,
    c.pid,
    c.start_id,
    c.cmd_pattern,
    c.cmd_category,
    c.proc_type,
    c.score,
    c.recommendation,
    c.blocked_by_gate,
    c.is_protected
FROM pt.candidates c
WHERE c.is_protected = true
   OR c.blocked_by_gate IS NOT NULL;
