# Process Triage DuckDB Views

Standard SQL views and macros for querying the process triage telemetry lake.

## Quick Start

```sql
-- Set the telemetry data directory
SET pt_data_dir = '/home/user/.local/share/process_triage/telemetry';

-- Load all views
.read sql/duckdb/pt_views.sql

-- Query recent runs
SELECT * FROM pt.runs ORDER BY started_at DESC LIMIT 10;

-- Get candidates for a session
SELECT * FROM pt.candidates_for('session-123');
```

## Installation

Copy the `sql/duckdb/` directory to your project or reference it directly:

```bash
# From command line
duckdb -c "SET pt_data_dir = '/path/to/telemetry'; .read /path/to/sql/duckdb/pt_views.sql"

# Or interactively
duckdb
D SET pt_data_dir = '/path/to/telemetry';
D .read sql/duckdb/pt_views.sql
```

## Available Views

### Session / Run Introspection

| View | Description |
|------|-------------|
| `pt.runs` | Normalized runs with computed fields (candidate_rate_pct, kill_rate_pct) |
| `pt.latest_runs` | Most recent run per host |
| `pt.run_history` | Runs with trend indicators (prev_candidates, candidate_delta) |
| `pt.daily_summary` | Daily aggregated statistics |

### Candidate / Decision Analysis

| View | Description |
|------|-------------|
| `pt.candidates` | Combined features + inference for all candidates |
| `pt.recommendations` | Kill/mitigate recommendations with outcome data |
| `pt.evidence_terms` | Unpivoted evidence factors for explainability |
| `pt.candidate_summary` | Aggregate stats per session |
| `pt.proc_type_breakdown` | Analysis by process type |

### Safety & Governance

| View | Description |
|------|-------------|
| `pt.policy_blocks` | Actions blocked by safety gates |
| `pt.gate_summary` | Aggregate gate block statistics |
| `pt.fdr_status` | False Discovery Rate tracking |
| `pt.alpha_investing_status` | Risk budget over time |
| `pt.safety_audit` | Safety-relevant audit events |
| `pt.protected_processes` | Processes protected from action |

### Calibration / Shadow Mode

| View | Description |
|------|-------------|
| `pt.shadow_labels` | User feedback labels for calibration |
| `pt.calibration_bins` | Binned data for reliability diagrams |
| `pt.false_kill_rate` | FKR with Wilson score confidence bounds |
| `pt.accuracy_metrics` | Precision, recall, specificity, F1 |

### Drift / Misspecification Detection

| View | Description |
|------|-------------|
| `pt.ppc_flags` | Posterior Predictive Check flags |
| `pt.drift_indicators` | Time-series drift detection |
| `pt.feature_drift` | Feature distribution changes |
| `pt.anomaly_sessions` | Sessions with warning flags |

### Fleet / Multi-Host

| View | Description |
|------|-------------|
| `pt.fleet_summary` | Overview of all hosts |
| `pt.fleet_rollup` | Daily aggregated fleet statistics |
| `pt.cross_host_patterns` | Patterns appearing across hosts |
| `pt.fleet_comparison` | Compare metrics across hosts |
| `pt.fleet_outliers` | Hosts with anomalous behavior |
| `pt.recurring_offenders` | Processes repeatedly flagged |

## Available Macros

Macros provide parameterized queries:

```sql
-- Get latest run for a specific host
SELECT * FROM pt.latest_run('my-host');

-- Get run summary
SELECT * FROM pt.run_summary('session-123');

-- Get candidates for a session
SELECT * FROM pt.candidates_for('session-123');

-- Get top evidence terms for a process
SELECT * FROM pt.top_terms('session-123', 1234, 'start-id');

-- Get policy blocks for a session
SELECT * FROM pt.policy_blocks_for('session-123');

-- Get FDR status
SELECT * FROM pt.fdr_status_for('session-123');

-- Get alpha investing status (host, window in days)
SELECT * FROM pt.alpha_investing_for('my-host', 30);

-- Get shadow labels
SELECT * FROM pt.shadow_labels_for('my-host', 30);

-- Get calibration curve data
SELECT * FROM pt.calibration_curve('my-host', 30);

-- Get false kill rate bounds
SELECT * FROM pt.false_kill_rate_bounds('my-host', 30);

-- Get PPC flags for a session
SELECT * FROM pt.ppc_flags_for('session-123');

-- Get drift flags
SELECT * FROM pt.drift_flags('my-host', 30);

-- Get fleet rollup
SELECT * FROM pt.fleet_rollup_for(7);

-- Get cross-host patterns
SELECT * FROM pt.cross_host_patterns_for(30);
```

## Example Queries

### Find processes with highest abandonment probability

```sql
SELECT
    cmd_pattern,
    pid,
    p_abandoned,
    score,
    recommendation
FROM pt.candidates
WHERE session_id = (SELECT session_id FROM pt.latest_runs LIMIT 1)
ORDER BY p_abandoned DESC
LIMIT 10;
```

### Check model calibration

```sql
SELECT
    bin,
    n,
    predicted,
    observed,
    ABS(predicted - observed) AS calibration_error
FROM pt.calibration_curve('my-host', 30)
ORDER BY bin;
```

### Find recurring problem processes

```sql
SELECT
    cmd_pattern,
    session_count,
    host_count,
    mean_p_abandoned,
    times_killed
FROM pt.recurring_offenders
WHERE session_count >= 5
ORDER BY session_count DESC;
```

### Track risk budget consumption

```sql
SELECT
    session_id,
    started_at,
    risk_consumed,
    cumulative_risk,
    risk_budget_remaining
FROM pt.alpha_investing_status
WHERE host_id = 'my-host'
ORDER BY started_at DESC
LIMIT 20;
```

## Schema Evolution

Views use additive schema evolution:
- New columns may be added to views
- Existing columns are never removed or renamed
- Column types are never changed incompatibly

## Testing

Run the test script to verify views work with synthetic data:

```bash
duckdb -c ".read sql/duckdb/test_views.sql"
```

## Version History

- **1.0.0** - Initial release with core views and macros
