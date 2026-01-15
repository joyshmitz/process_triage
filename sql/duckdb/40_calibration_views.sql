-- Process Triage: Calibration / Shadow Mode Views
--
-- Views for analyzing model calibration, shadow mode results, and accuracy metrics.

-- pt_shadow_labels: User feedback and labels for calibration
CREATE OR REPLACE VIEW pt.shadow_labels AS
SELECT
    o.session_id,
    o.outcome_ts,
    o.pid,
    o.start_id,
    o.cmd,
    o.proc_type,
    o.recommendation,
    o.decision,
    o.decision_source,
    o.user_feedback,
    o.feedback_ts,
    o.feedback_note,
    o.score,
    i.p_abandoned,
    i.confidence,
    r.host_id,
    r.hostname,
    -- Label interpretation
    CASE
        WHEN o.user_feedback = 'correct_kill' THEN 'true_positive'
        WHEN o.user_feedback = 'incorrect_kill' OR o.user_feedback = 'false_positive' THEN 'false_positive'
        WHEN o.user_feedback = 'correct_spare' THEN 'true_negative'
        WHEN o.user_feedback = 'incorrect_spare' OR o.user_feedback = 'false_negative' THEN 'false_negative'
        WHEN o.user_feedback = 'uncertain' THEN 'uncertain'
        ELSE NULL
    END AS label_type
FROM pt.raw_outcomes o
LEFT JOIN pt.raw_proc_inference i
    ON o.session_id = i.session_id
    AND o.pid = i.pid
    AND o.start_id = i.start_id
LEFT JOIN pt.raw_runs r ON o.session_id = r.session_id
WHERE o.user_feedback IS NOT NULL;

-- Macro: Get shadow labels for a host within time window
CREATE OR REPLACE MACRO pt.shadow_labels_for(host, window_days) AS
    TABLE SELECT * FROM pt.shadow_labels
    WHERE host_id = host
      AND outcome_ts >= CURRENT_TIMESTAMP - INTERVAL (window_days) DAY
    ORDER BY outcome_ts DESC;

-- pt_calibration_bins: Binned calibration data for reliability diagrams
CREATE OR REPLACE VIEW pt.calibration_bins AS
WITH binned AS (
    SELECT
        r.host_id,
        -- Bin by predicted probability
        FLOOR(i.p_abandoned * 10) / 10 AS prob_bin,
        i.p_abandoned,
        o.user_feedback,
        CASE
            WHEN o.user_feedback IN ('correct_kill', 'true_positive') THEN 1
            WHEN o.user_feedback IN ('incorrect_kill', 'false_positive') THEN 0
            WHEN o.user_feedback IN ('correct_spare', 'true_negative') THEN 0
            WHEN o.user_feedback IN ('incorrect_spare', 'false_negative') THEN 1
            ELSE NULL
        END AS actual_abandoned
    FROM pt.raw_proc_inference i
    JOIN pt.raw_outcomes o
        ON i.session_id = o.session_id
        AND i.pid = o.pid
        AND i.start_id = o.start_id
    JOIN pt.raw_runs r ON i.session_id = r.session_id
    WHERE o.user_feedback IS NOT NULL
      AND o.user_feedback NOT IN ('uncertain', 'unknown')
)
SELECT
    host_id,
    prob_bin,
    COUNT(*) AS bin_count,
    AVG(p_abandoned) AS mean_predicted,
    AVG(actual_abandoned) AS mean_actual,
    -- Calibration error for this bin
    ABS(AVG(p_abandoned) - AVG(actual_abandoned)) AS calibration_error,
    STDDEV(p_abandoned) AS predicted_stddev
FROM binned
WHERE actual_abandoned IS NOT NULL
GROUP BY host_id, prob_bin
ORDER BY host_id, prob_bin;

-- Macro: Get calibration curve data for a host
CREATE OR REPLACE MACRO pt.calibration_curve(host, window_days) AS
    TABLE WITH labeled AS (
        SELECT
            i.p_abandoned,
            CASE
                WHEN o.user_feedback IN ('correct_kill', 'true_positive') THEN 1.0
                WHEN o.user_feedback IN ('incorrect_kill', 'false_positive') THEN 0.0
                WHEN o.user_feedback IN ('correct_spare', 'true_negative') THEN 0.0
                WHEN o.user_feedback IN ('incorrect_spare', 'false_negative') THEN 1.0
            END AS actual
        FROM pt.raw_proc_inference i
        JOIN pt.raw_outcomes o
            ON i.session_id = o.session_id AND i.pid = o.pid AND i.start_id = o.start_id
        JOIN pt.raw_runs r ON i.session_id = r.session_id
        WHERE r.host_id = host
          AND o.outcome_ts >= CURRENT_TIMESTAMP - INTERVAL (window_days) DAY
          AND o.user_feedback IS NOT NULL
          AND o.user_feedback NOT IN ('uncertain', 'unknown')
    )
    SELECT
        FLOOR(p_abandoned * 10) / 10 AS bin,
        COUNT(*) AS n,
        AVG(p_abandoned) AS predicted,
        AVG(actual) AS observed
    FROM labeled
    WHERE actual IS NOT NULL
    GROUP BY FLOOR(p_abandoned * 10) / 10
    ORDER BY bin;

-- pt_false_kill_rate: Estimated false kill rates with bounds
CREATE OR REPLACE VIEW pt.false_kill_rate AS
WITH labeled_kills AS (
    SELECT
        r.host_id,
        DATE_TRUNC('day', o.outcome_ts) AS day,
        COUNT(*) AS total_kills,
        COUNT(*) FILTER (
            WHERE o.user_feedback IN ('incorrect_kill', 'false_positive')
        ) AS false_kills,
        COUNT(*) FILTER (
            WHERE o.user_feedback IN ('correct_kill', 'true_positive')
        ) AS true_kills,
        COUNT(*) FILTER (
            WHERE o.user_feedback IS NOT NULL
        ) AS labeled_kills
    FROM pt.raw_outcomes o
    JOIN pt.raw_runs r ON o.session_id = r.session_id
    WHERE o.action_attempted = true
    GROUP BY r.host_id, DATE_TRUNC('day', o.outcome_ts)
)
SELECT
    host_id,
    day,
    total_kills,
    labeled_kills,
    false_kills,
    true_kills,
    -- Point estimate
    CASE
        WHEN labeled_kills > 0
        THEN ROUND(100.0 * false_kills / labeled_kills, 2)
        ELSE NULL
    END AS false_kill_rate_pct,
    -- Wilson score interval (approximate 95% CI)
    CASE
        WHEN labeled_kills >= 10 THEN
            ROUND(100.0 * (
                (false_kills + 1.96*1.96/2) / (labeled_kills + 1.96*1.96)
                - 1.96 * SQRT((false_kills * (labeled_kills - false_kills) / labeled_kills + 1.96*1.96/4) / (labeled_kills + 1.96*1.96))
            ), 2)
        ELSE NULL
    END AS fkr_lower_95,
    CASE
        WHEN labeled_kills >= 10 THEN
            ROUND(100.0 * (
                (false_kills + 1.96*1.96/2) / (labeled_kills + 1.96*1.96)
                + 1.96 * SQRT((false_kills * (labeled_kills - false_kills) / labeled_kills + 1.96*1.96/4) / (labeled_kills + 1.96*1.96))
            ), 2)
        ELSE NULL
    END AS fkr_upper_95
FROM labeled_kills
ORDER BY host_id, day DESC;

-- Macro: Get false kill rate bounds for a host
CREATE OR REPLACE MACRO pt.false_kill_rate_bounds(host, window_days) AS
    TABLE SELECT
        host_id,
        SUM(total_kills) AS total_kills,
        SUM(labeled_kills) AS labeled_kills,
        SUM(false_kills) AS false_kills,
        SUM(true_kills) AS true_kills,
        CASE
            WHEN SUM(labeled_kills) > 0
            THEN ROUND(100.0 * SUM(false_kills) / SUM(labeled_kills), 2)
            ELSE NULL
        END AS overall_fkr_pct
    FROM pt.false_kill_rate
    WHERE host_id = host
      AND day >= CURRENT_DATE - INTERVAL (window_days) DAY
    GROUP BY host_id;

-- pt_accuracy_metrics: Comprehensive accuracy statistics
CREATE OR REPLACE VIEW pt.accuracy_metrics AS
WITH labeled AS (
    SELECT
        r.host_id,
        i.recommendation,
        o.user_feedback,
        CASE
            WHEN i.recommendation = 'kill' AND o.user_feedback IN ('correct_kill', 'true_positive') THEN 'TP'
            WHEN i.recommendation = 'kill' AND o.user_feedback IN ('incorrect_kill', 'false_positive') THEN 'FP'
            WHEN i.recommendation = 'spare' AND o.user_feedback IN ('correct_spare', 'true_negative') THEN 'TN'
            WHEN i.recommendation = 'spare' AND o.user_feedback IN ('incorrect_spare', 'false_negative') THEN 'FN'
        END AS confusion_class
    FROM pt.raw_proc_inference i
    JOIN pt.raw_outcomes o
        ON i.session_id = o.session_id AND i.pid = o.pid AND i.start_id = o.start_id
    JOIN pt.raw_runs r ON i.session_id = r.session_id
    WHERE o.user_feedback IS NOT NULL
      AND o.user_feedback NOT IN ('uncertain', 'unknown')
)
SELECT
    host_id,
    COUNT(*) FILTER (WHERE confusion_class = 'TP') AS true_positives,
    COUNT(*) FILTER (WHERE confusion_class = 'FP') AS false_positives,
    COUNT(*) FILTER (WHERE confusion_class = 'TN') AS true_negatives,
    COUNT(*) FILTER (WHERE confusion_class = 'FN') AS false_negatives,
    -- Precision = TP / (TP + FP)
    ROUND(
        COUNT(*) FILTER (WHERE confusion_class = 'TP')::FLOAT /
        NULLIF(COUNT(*) FILTER (WHERE confusion_class IN ('TP', 'FP')), 0),
        4
    ) AS precision,
    -- Recall = TP / (TP + FN)
    ROUND(
        COUNT(*) FILTER (WHERE confusion_class = 'TP')::FLOAT /
        NULLIF(COUNT(*) FILTER (WHERE confusion_class IN ('TP', 'FN')), 0),
        4
    ) AS recall,
    -- Specificity = TN / (TN + FP)
    ROUND(
        COUNT(*) FILTER (WHERE confusion_class = 'TN')::FLOAT /
        NULLIF(COUNT(*) FILTER (WHERE confusion_class IN ('TN', 'FP')), 0),
        4
    ) AS specificity,
    -- F1 = 2 * precision * recall / (precision + recall)
    ROUND(
        2.0 * COUNT(*) FILTER (WHERE confusion_class = 'TP')::FLOAT /
        NULLIF(
            2 * COUNT(*) FILTER (WHERE confusion_class = 'TP') +
            COUNT(*) FILTER (WHERE confusion_class = 'FP') +
            COUNT(*) FILTER (WHERE confusion_class = 'FN'),
            0
        ),
        4
    ) AS f1_score
FROM labeled
WHERE confusion_class IS NOT NULL
GROUP BY host_id;
