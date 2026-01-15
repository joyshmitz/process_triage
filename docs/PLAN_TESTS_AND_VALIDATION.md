# Plan Tests and Validation (Section 11)

> **Bead**: `process_triage-h89.13`
> **Status**: Specification
> **Version**: 1.0.0

## Overview

This document translates Plan §11 into a concrete, testable checklist. Every implementation bead that touches inference, safety, telemetry, agent UX, or fleet behavior must reference this checklist and add the appropriate tests.

---

## Core Test Categories

### Math + Inference
- Unit tests: BetaBinomial, Beta posterior utilities, GammaPDF, Bayes factors.
- Deterministic inference: fixed inputs -> fixed outputs.
- Calibration: empirical Bayes hyperparameters.
- Posterior predictive checks (misspecification detection).
- DRO gating: conservative under drift.
- FDR/alpha-investing correctness.
- PAC-Bayes bound reporting on false-kill rate.

### Evidence + Feature Systems
- BOCPD change-point regression tests.
- Hawkes/marked point process fit sanity tests.
- EVT tail-fitting regression tests.
- Periodicity feature regression tests.
- IMM filter regression tests.
- Belief propagation correctness on PPID trees.
- Submodular probe selection monotonicity/approx sanity.
- Sketch/heavy-hitter accuracy vs resource budget.

### Safety + Identity
- `--robot`/`--shadow`/`--dry-run`: no prompts, correct gating, no actions in shadow/dry-run.
- Zombie handling (`Z`) and uninterruptible sleep (`D`) behavior.
- Data-loss gate (open write handles) coverage.
- PID reuse protection via `(pid,start_id,uid)` revalidation.
- Default same-UID enforcement and cross-UID policy blocks.
- Per-user pt lock behavior (manual vs daemon vs agent runs).

### Telemetry + Redaction
- Parquet schema stability and batched write correctness.
- DuckDB views/macros correctness.
- Redaction tests: sensitive strings never persist.
- Bundle/report tests: `.ptb` manifest/checksums and redaction guarantees.
- Report generator output: single HTML file with pinned CDN assets + SRI.
- Offline report: `--embed-assets` yields zero network fetches.

### Agent CLI Contract
- Schema invariants and exit codes.
- Token-efficiency flags: `--compact`, `--fields`, `--only`.
- JSONL progress stream shape and ordering.
- Galaxy-brain ledger matches inference outputs with concrete numbers.

### Daemon + Automation
- Dormant daemon overhead + cooldown/backoff.
- Escalation produces a session + inbox entry.
- Shadow mode metrics: false-kill rate, missed abandonment rate.

---

## Agent + Fleet-Specific Tests

### Supervisor Detection
- Mock systemd/launchd/pm2/Docker environments.
- Supervisor identification and action generation.
- Respawn loop detection across sessions.

### Signatures + Patterns
- Signature matching accuracy.
- Fast-path inference bypass correctness.
- Custom signature loading/merging.
- Signature confidence calibration.

### Trajectory + Goal Optimization
- Memory growth estimation, time-to-threshold calibration.
- Trend analysis correctness with confidence intervals.
- Goal parsing (memory, CPU, ports, composite).
- Kill set optimization for goal progress.
- Infeasible goal handling + shortfall reporting.

### Fleet Mode
- Multi-host session creation.
- Parallel scan aggregation.
- Cross-host correlation + shared FDR budget enforcement.
- Learning transfer export/import.

### Baselines + Drift
- Baseline fitting from history.
- Z-score correctness, cold-start defaults.
- Baseline drift detection.

### Genealogy + Blast Radius + What-If
- Ancestry chain correctness + role annotations.
- Blast radius completeness (children, ports, files) and risk scoring.
- What-if flip condition correctness + delta_p estimation.
- Output schema compliance for each mode.

### Summaries + Deltas + Resumability
- Summary mode output (`--brief`, `--narrative`, structured) correctness.
- Differential session delta computation + `--since` behavior.
- Session resumability (state persistence, resume, context preservation).

---

## Acceptance Criteria

- [ ] Each category above is mapped to at least one test suite (unit/integration/E2E).
- [ ] Safety/robot-mode tests are gating for release builds.
- [ ] Telemetry/redaction tests run in CI on every change.
- [ ] Agent CLI contract tests validate schema versioning and JSONL stream shape.

---

## Notes

Use this checklist as a release-quality gate and as a PR review template for any bead that touches inference, safety, telemetry, or automation.
