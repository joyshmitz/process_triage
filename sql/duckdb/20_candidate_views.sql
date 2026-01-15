-- Process Triage: Candidate / Decision Summary Views
--
-- Views for analyzing process candidates, inference results, and decisions.

-- pt_candidates: Combined view of candidates with features and inference
CREATE OR REPLACE VIEW pt.candidates AS
SELECT
    i.session_id,
    i.pid,
    i.start_id,
    i.inference_ts,
    -- Identity from features
    f.proc_type,
    f.cmd_pattern,
    f.cmd_category,
    f.age_s,
    f.age_bucket,
    -- State
    f.is_orphan,
    f.is_zombie,
    f.has_tty,
    f.has_network,
    f.has_children,
    f.is_protected,
    -- Resources
    f.cpu_pct_instant,
    f.cpu_pct_avg,
    f.mem_mb,
    f.mem_pct,
    -- Inference
    i.p_abandoned,
    i.p_legitimate,
    i.p_uncertain,
    i.log_bayes_factor,
    i.bayes_factor_interpretation,
    i.score,
    i.confidence,
    i.recommendation,
    -- Evidence breakdown
    i.evidence_prior,
    i.evidence_age,
    i.evidence_cpu,
    i.evidence_memory,
    i.evidence_io,
    i.evidence_state,
    i.evidence_network,
    i.evidence_children,
    i.evidence_history,
    i.evidence_deep,
    -- Safety
    i.passed_safety_gates,
    i.blocked_by_gate,
    -- Computed fields
    CASE
        WHEN i.recommendation = 'kill' AND i.passed_safety_gates THEN 'actionable_kill'
        WHEN i.recommendation = 'kill' AND NOT i.passed_safety_gates THEN 'blocked_kill'
        WHEN i.recommendation = 'spare' THEN 'spare'
        ELSE 'uncertain'
    END AS action_status,
    -- Rank within session
    ROW_NUMBER() OVER (PARTITION BY i.session_id ORDER BY i.score DESC) AS score_rank
FROM pt.raw_proc_inference i
LEFT JOIN pt.raw_proc_features f
    ON i.session_id = f.session_id
    AND i.pid = f.pid
    AND i.start_id = f.start_id;

-- Macro: Get candidates for a specific session
CREATE OR REPLACE MACRO pt.candidates_for(sid) AS
    TABLE SELECT * FROM pt.candidates
    WHERE session_id = sid
    ORDER BY score DESC;

-- pt_recommendations: Actionable recommendations with decision context
CREATE OR REPLACE VIEW pt.recommendations AS
SELECT
    c.*,
    o.decision,
    o.decision_source,
    o.action_type,
    o.action_attempted,
    o.action_successful,
    o.user_feedback,
    o.memory_freed_bytes,
    CASE
        WHEN o.action_successful = true THEN 'executed_success'
        WHEN o.action_successful = false THEN 'executed_failed'
        WHEN o.action_attempted = true THEN 'attempted'
        WHEN o.decision = 'skip' THEN 'skipped'
        WHEN o.decision = 'spare' THEN 'spared'
        ELSE 'pending'
    END AS execution_status
FROM pt.candidates c
LEFT JOIN pt.raw_outcomes o
    ON c.session_id = o.session_id
    AND c.pid = o.pid
    AND c.start_id = o.start_id
WHERE c.recommendation IN ('kill', 'mitigate');

-- Macro: Get recommendations for a session
CREATE OR REPLACE MACRO pt.recommendations_for(sid) AS
    TABLE SELECT * FROM pt.recommendations
    WHERE session_id = sid
    ORDER BY score DESC;

-- pt_top_terms: Evidence breakdown for explainability
-- Shows which evidence factors contributed most to the score
CREATE OR REPLACE VIEW pt.evidence_terms AS
SELECT
    session_id,
    pid,
    start_id,
    score,
    recommendation,
    -- Unpivot evidence factors into rows
    UNNEST([
        {'term': 'prior', 'weight': evidence_prior},
        {'term': 'age', 'weight': evidence_age},
        {'term': 'cpu', 'weight': evidence_cpu},
        {'term': 'memory', 'weight': evidence_memory},
        {'term': 'io', 'weight': evidence_io},
        {'term': 'state', 'weight': evidence_state},
        {'term': 'network', 'weight': evidence_network},
        {'term': 'children', 'weight': evidence_children},
        {'term': 'history', 'weight': evidence_history},
        {'term': 'deep', 'weight': COALESCE(evidence_deep, 0)}
    ]) AS term_data
FROM pt.raw_proc_inference;

-- Macro: Get top evidence terms for a specific process
CREATE OR REPLACE MACRO pt.top_terms(sid, p, s_id) AS
    TABLE SELECT
        term_data.term AS evidence_term,
        term_data.weight AS evidence_weight,
        CASE
            WHEN term_data.weight > 0.5 THEN 'strong_positive'
            WHEN term_data.weight > 0.2 THEN 'moderate_positive'
            WHEN term_data.weight > 0 THEN 'weak_positive'
            WHEN term_data.weight > -0.2 THEN 'weak_negative'
            WHEN term_data.weight > -0.5 THEN 'moderate_negative'
            ELSE 'strong_negative'
        END AS interpretation
    FROM pt.evidence_terms
    WHERE session_id = sid AND pid = p AND start_id = s_id
    ORDER BY ABS(term_data.weight) DESC;

-- pt_candidate_summary: Aggregate candidate statistics per session
CREATE OR REPLACE VIEW pt.candidate_summary AS
SELECT
    session_id,
    COUNT(*) AS total_candidates,
    COUNT(*) FILTER (WHERE recommendation = 'kill') AS kill_candidates,
    COUNT(*) FILTER (WHERE recommendation = 'spare') AS spare_candidates,
    COUNT(*) FILTER (WHERE recommendation = 'kill' AND passed_safety_gates) AS actionable_kills,
    COUNT(*) FILTER (WHERE NOT passed_safety_gates) AS blocked_by_gates,
    AVG(score) AS avg_score,
    MAX(score) AS max_score,
    MIN(score) AS min_score,
    AVG(p_abandoned) AS avg_p_abandoned,
    -- Category breakdown
    COUNT(*) FILTER (WHERE cmd_category = 'system') AS system_count,
    COUNT(*) FILTER (WHERE cmd_category = 'user_app') AS user_app_count,
    COUNT(*) FILTER (WHERE cmd_category = 'development') AS dev_count,
    COUNT(*) FILTER (WHERE cmd_category = 'service') AS service_count,
    COUNT(*) FILTER (WHERE is_orphan) AS orphan_count,
    COUNT(*) FILTER (WHERE is_zombie) AS zombie_count
FROM pt.candidates
GROUP BY session_id;

-- pt_proc_type_breakdown: Analysis by process type
CREATE OR REPLACE VIEW pt.proc_type_breakdown AS
SELECT
    session_id,
    proc_type,
    COUNT(*) AS count,
    AVG(score) AS avg_score,
    AVG(p_abandoned) AS avg_p_abandoned,
    COUNT(*) FILTER (WHERE recommendation = 'kill') AS kill_count,
    COUNT(*) FILTER (WHERE is_protected) AS protected_count
FROM pt.candidates
GROUP BY session_id, proc_type
ORDER BY session_id, count DESC;
