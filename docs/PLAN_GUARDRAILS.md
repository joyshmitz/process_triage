# Plan Guardrails: Pitfalls to Avoid (Section 8.1)

> **Bead**: `process_triage-h89.2`
> **Status**: Specification
> **Version**: 1.0.0

## Overview

This document is a single, self-contained checklist of the **top pitfalls** from Plan §8.1 and the **guardrails** required to keep Process Triage safe, reliable, and "premium alien artifact" quality. Every implementation bead that touches these areas must **explicitly reference** this checklist and provide enforcement points and tests.

## Guardrail Checklist (Must-Haves)

### 1) Sudo + installs (Never block automation)

**Pitfall**: Hanging on prompts or blocking automation.

**Guardrails**
- Use non-interactive elevation (`sudo -n`) only; never prompt in robot/agent mode.
- Always emit a capabilities report (available vs missing tools, with reasons).
- Agent/robot modes **must never prompt** (stdin may be closed).

**Primary enforcement beads**
- Tool runner + caps: `process_triage-71t`
- Capability detection/cache: `process_triage-qa9` (+ Phase 1 schema `process_triage-agz`)
- Tool install strategy: `process_triage-167`
- Robot/no-prompt tests: `process_triage-y3ao`

### 2) Too much raw data (Redaction + caps)

**Pitfall**: Dumping sensitive or high-volume raw traces into logs/telemetry; uncontrolled disk growth.

**Guardrails**
- Redact/hash **before** persistence (default-on).
- Enforce strict caps on high-volume traces.
- Retain summaries/ledgers longer than raw traces.
- Explicit retention events; no silent deletion.

**Primary enforcement beads**
- Redaction/hashing: `process_triage-k4yc.1`
- Retention policy: `process_triage-k4yc.6`

### 3) Robot mode safety (Never a foot-gun)

**Pitfall**: `--robot` becomes unsafe or bypasses protections.

**Guardrails**
- `--robot` must be explicit.
- Always enforce: protected denylist, blast radius limits, min posterior/confidence.
- Apply robust Bayes/DRO tightening on drift or mismatch.
- Enforce FDR/alpha-investing budgets.

**Primary enforcement beads**
- Policy enforcement: `process_triage-dvi`
- Robot constraints: `process_triage-dvi.2`
- DRO gate: `process_triage-6a1`
- FDR + alpha budgets: `process_triage-sqe`, `process_triage-cpm`

### 4) UI overload (Progressive disclosure)

**Pitfall**: Overwhelming the user with math or verbose output by default.

**Guardrails**
- Provide a one-line, human-readable "why" by default.
- Evidence ledger and galaxy-brain views are **on-demand** only.

**Primary enforcement beads**
- Premium TUI/progressive disclosure: `process_triage-2ka`, `process_triage-t65l`
- Evidence ledger: `process_triage-myq`, `process_triage-03n`

### 5) Tool becomes the hog (Performance budgets)

**Pitfall**: The triage tool itself causes resource pressure.

**Guardrails**
- Hard overhead budgets and cooldowns.
- VOI-driven probe selection (no blanket deep probing).
- Dormant daemon is safe-by-default; no surprise actions.

**Primary enforcement beads**
- Performance/scalability: `process_triage-dki`
- VOI/budgeted probing: `process_triage-brh7`, `process_triage-p15.2`
- Dormant daemon: `process_triage-b4v`

---

## Enforcement Checklist (Per Bead)

Any bead that touches these areas must explicitly answer:

- Does this change introduce new prompts, blocking, or elevation? If yes, prove non-blocking behavior.
- Does this change write or persist sensitive data? If yes, show redaction/hashing and caps.
- Does this change alter robot behavior? If yes, show safety gates and budgets remain enforced.
- Does this change add UI verbosity? If yes, default remains minimal and progressive disclosure applies.
- Does this change increase runtime overhead? If yes, show budgets, timeouts, and cooldowns.

---

## Acceptance Criteria

- [ ] Each pitfall above has **explicit enforcement points** in code.
- [ ] Each guardrail has a **test or validation** (unit/integration/E2E).
- [ ] Any new behavior that violates guardrails is **blocked** until mitigated.

---

## Audit Record

This document is the canonical reference for §8.1 guardrails. Any future spec or implementation doc should link to `docs/PLAN_GUARDRAILS.md` when addressing safety, telemetry, robot mode, UI disclosure, or performance budgets.
