# Plan Deliverables + Final Safety Statement (Sections 12-13)

> **Bead**: `process_triage-h89.3`
> **Status**: Specification
> **Version**: 1.0.0

## Overview

This document condenses Plan §12 (Deliverables) and §13 (Final Safety Statement) into a checklist that can be validated against implementation work. Each deliverable is a **contract** and should be referenced by any bead that implements it.

---

## Core Deliverables (Must Ship)

- **Wrapper + core**: `pt` bash wrapper (installer/launcher) and `pt-core` Rust monolith (scan/infer/decide/ui).
- **Policies**: `priors.json`, `policy.json`, and versioned redaction/hashing policy used by telemetry.
- **Telemetry lake**: Parquet-first storage (raw + derived + outcomes) with DuckDB views/macros for calibration, PAC-Bayes bounds, FDR, and "why" breakdowns.
- **Agent contract**: `pt agent` plan/explain/apply/sessions/tail/inbox/export/report with stable schemas + automation-friendly exit codes.
- **Bundles + reports**: Shareable `.ptb` session bundles (optional encryption) + single-file HTML report (CDN + SRI, optional `--embed-assets` for offline).
- **Dormant daemon**: `ptd` with systemd/launchd units + inbox UX for pending plans.
- **Docs**: Enhanced README covering math, safety guarantees, telemetry governance, and reproducible analysis workflow.
- **Tests**: Rust unit/integration + wrapper smoke tests (BATS or equivalent).

---

## Agent-Specific Deliverables

- **Supervisor detection module**: Systemd/launchd/pm2/Docker/supervisord/nodemon/tmux/screen detection with supervisor-aware action planning.
- **Pattern/signature library**: Curated signatures + DSL for test/dev/AI/build processes, fast-path inference, and user customization.
- **Trajectory prediction engine**: Memory/CPU trend modeling, time-to-threshold prediction, per-machine baselines.
- **Goal-oriented optimizer**: `--goal "free 4GB RAM"` parsing, candidate scoring, kill set optimization, progress tracking.
- **Genealogy narrative generator**: Process ancestry chains, role annotation, human-readable story output, plus structured JSON.
- **Blast radius analyzer**: Children/ports/files/memory dependency detection, cumulative risk scoring, visualization.
- **What-if explainer**: Flip conditions, delta_p estimates, "what would change my mind" output.
- **Human-friendly summary modes**: Brief/narrative/structured summaries for different contexts.
- **Differential session support**: Baseline tracking, `--since` comparisons, session delta computation.
- **Session resumability**: State persistence, `pt agent apply --session <id> --resume`, context preservation.

---

## Fleet-Specific Deliverables

- **Fleet session manager**: Multi-host session schema, parallel scanning, aggregation, cross-host comparison.
- **Fleet CLI**: `pt agent fleet plan|apply|status` with host-file support.
- **Fleet decision coordinator**: Shared FDR budget, cross-host correlation, coordinated action timing.
- **Learning transfer**: Prior export/import, fleet-wide signature sharing, baseline normalization.
- **Fleet reporting**: Aggregated HTML reports, per-host comparisons, anomaly detection.

---

## Documentation Deliverables

- **Agent integration guide**: Full `pt agent` contract, schema reference, exit codes, error handling, best practices.
- **Fleet operations guide**: Multi-host deployment, SSH config, parallel execution, coordination.
- **Signature authoring guide**: DSL reference, customization, contribution guidelines.

---

## Final Safety Statement (Contract)

**Default behavior**: never auto-kill. The system runs analysis and **requires explicit TUI confirmation** before destructive actions.

**Robot mode**: `--robot` enables non-interactive execution of the pre-toggled recommended plan, but **does not bypass safety gates**. These gates must remain enforced:

- Protected denylist (system services)
- Blast radius limits
- Minimum posterior/confidence thresholds
- Robust Bayes/DRO tightening on drift or mismatch
- FDR / alpha-investing budgets

---

## Acceptance Criteria

- [ ] Every deliverable above maps to at least one implementation bead.
- [ ] Safety statement is reflected in CLI behavior and tests.
- [ ] Any change that violates these guarantees must be rejected or gated.

---

## Notes for Implementers

- Keep the safety statement visible in UX and agent docs.
- Treat the deliverables list as a **ship checklist** for release readiness.
