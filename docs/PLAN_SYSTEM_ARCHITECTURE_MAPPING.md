# Plan System Architecture Mapping (Section 3)

This file maps Plan Section 3 (System Architecture) to canonical beads.
It exists so implementation and review do not require re-opening
PLAN_TO_MAKE_PROCESS_TRIAGE_INTO_AN_ALIEN_TECHNOLOGY_ARTIFACT.md.

## Non-negotiable design constraints
- Keep `pt` (bash) as a thin wrapper/installer/capability launcher.
- Keep `pt-core` (Rust) as the monolith for scan, infer, decide, UI, telemetry, agent.
- Default behavior is conservative: no auto-kill without explicit confirmation.
- Same-UID actions by default; cross-UID requires explicit policy + privilege.
- Strong safety: identity revalidation, locks, staged actions, audit logs.

## Mapping (Plan Section 3 -> canonical beads)

### 3.0 Execution and packaging architecture
Plan requirement: two-layer packaging and stable CLI surfaces with safety gates.

Canonical beads
- Wrapper/monolith boundary: process_triage-kze, process_triage-40mt, process_triage-n0r
- CLI surface + exit codes: process_triage-3mi, process_triage-40mt.2
- Locking + identity/privilege contracts: process_triage-o8m, process_triage-cfon.2

---

### 3.1 Data collection layer
Plan requirement: staged quick scan -> targeted deep scan with provenance and budgets.

Canonical beads
- Collection epic: process_triage-3ir
- Quick scan: process_triage-d31
- Deep scan: process_triage-cki
- Tool runner (timeouts/caps/backpressure): process_triage-71t
- Progress events (JSONL stage stream): process_triage-f5o
- Capability detection + cache: process_triage-qa9 (schema: process_triage-agz)
- Maximal tool install strategy: process_triage-167

---

### 3.2 Feature layer (deterministic features + provenance)
Plan requirement: stable identity tuple and deterministic derived features with provenance.

Canonical beads
- Feature layer epic: process_triage-cfon
- Stable identity features: process_triage-cfon.2
- CPU tick-delta + n_eff: process_triage-3ir.1.1, process_triage-cfon.1
- Orphan conditioning and supervision context: process_triage-cfon.4

---

### 3.3 Telemetry and analytics (Parquet + DuckDB)
Plan requirement: Parquet-first append-only telemetry with DuckDB views and retention.

Canonical beads
- Telemetry epic: process_triage-k4yc
- Arrow/Parquet schemas + writer: process_triage-5y9
- DuckDB views/macros: process_triage-k4yc.2
- Retention policy + explicit events: process_triage-k4yc.6

---

### 3.4 Redaction, hashing, data governance
Plan requirement: redact/hash before persistence, preserve analytic utility.

Canonical beads
- Redaction policy spec: process_triage-8n3
- Redaction/hashing engine: process_triage-k4yc.1
- Redaction tests: process_triage-8t2k

---

### 3.5 Agent/robot CLI contract (no TUI)
Plan requirement: stable schemas + token-efficient output modes and exit codes.

Canonical beads
- Contract spec: process_triage-jqi
- Agent parity implementation epic: process_triage-bwn
- Contract tests: process_triage-5q2m

---

### 3.5.1 Session continuity (agents)
Plan requirement: durable sessions, resumable workflows, idempotent apply.

Canonical beads
- Session model/layout spec: process_triage-qje
- Session continuity + resumption: process_triage-t6lf
- Differential/resumable sessions epic: process_triage-9k8

---

### 3.6 Session bundles and HTML reports
Plan requirement: .ptb bundles and standalone HTML reports (CDN pinned, offline option).

Canonical beads
- Bundle/report spec: process_triage-2ws
- Bundle writer/reader: process_triage-k4yc.3
- Report generator: process_triage-k4yc.5
- Bundles/reports epic: process_triage-bra
- Tests: process_triage-j47h (report), process_triage-aii.2 (bundle)

---

### 3.7 Dormant mode (always-on guardian)
Plan requirement: low-overhead daemon with triggers and inbox escalation.

Canonical beads
- Dormant mode epic: process_triage-b4v
- Daemon core loop + trigger/escalation: process_triage-nh7p
- Notifications/escalation ladder: process_triage-1k6, process_triage-a3h0

---

### 3.8 Fleet mode architecture
Plan requirement: multi-host sessions and aggregated reporting.

Canonical beads
- Fleet mode epic: process_triage-8t1

---

### 3.9 Pattern/signature library
Plan requirement: signature schema, matching engine, and signature CLI.

Canonical beads
- Signature library epic: process_triage-79x

---

## Acceptance checklist
- Every Plan Section 3 subsection (3.0-3.9) mapped to canonical beads.
- Safety/UX invariants (lock, same-UID default, file:// reports) have explicit beads.
