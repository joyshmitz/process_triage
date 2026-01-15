# Plan Phased Implementation Mapping (Section 10)

This file maps Plan Section 10 (Phased Implementation, Phases 1-16) to canonical epics so execution work can navigate sequencing without re-reading the full plan document.

---

## Phase 1: Spec and Config

Canonical epic: process_triage-2l3

Key phase-1 child beads (must exist before implementation):
- Packaging boundary: process_triage-kze
- pt-core CLI surface + stable formats: process_triage-3mi
- Session model/artifact layout: process_triage-qje
- Priors schema: process_triage-2f3
- Policy schema (loss + guardrails): process_triage-bg5
- Telemetry schema + partitioning: process_triage-4r8
- Redaction/hashing policy: process_triage-8n3
- Capabilities cache schema: process_triage-agz
- Agent/robot contract: process_triage-jqi
- Bundle/report spec: process_triage-2ws
- Dormant daemon spec: process_triage-2kz
- Golden path UX spec: process_triage-6rf
- Galaxy-brain contract: process_triage-8f6

---

## Phase 2: Math Utilities

Canonical epic: process_triage-iau

Key child beads:
- Numerical stability primitives: process_triage-00b
- Beta conjugate: process_triage-rqn
- Binomial: process_triage-3ot
- Bernoulli: process_triage-m99
- Dirichlet-Multinomial: process_triage-5s5
- Gamma: process_triage-22q

---

## Phase 3: Evidence Collection

Canonical epic: process_triage-3ir

Key child beads:
- Quick scan: process_triage-d31
- Deep scan: process_triage-cki
- Tool runner (timeouts/caps/backpressure): process_triage-71t

---

## Phase 3a: Tooling Install Strategy

Canonical bead: process_triage-167

Key supporting epics:
- Installation infrastructure (wrapper install/upgrade/dep mgmt): process_triage-n0r

Notes
- Maximal instrumentation by default - install probes/tools aggressively for maximum telemetry coverage.

---

## Phase 4: Inference Integration

Canonical epic: process_triage-nao

Key child beads:
- Bayes factors: process_triage-0ij
- Core posterior P(C|x): process_triage-e48
- Evidence ledger generation: process_triage-myq

---

## Phase 5: Decision Theory

Canonical epic: process_triage-p15

Key child beads:
- Expected loss + SPRT boundary: process_triage-d88
- FDR control: process_triage-sqe
- Alpha-investing: process_triage-cpm

---

## Phase 6: Action Tray

Canonical epic: process_triage-sj6

Key child beads:
- Action plan generation: process_triage-1t1
- Staged execution: process_triage-kyl
- Pause/resume: process_triage-sj6.2
- Kill (TERM -> KILL): process_triage-sj6.3
- Renice: process_triage-sj6.4
- Cgroup freeze: process_triage-sj6.5
- Cgroup CPU throttle: process_triage-sj6.6

---

## Phase 7: UX Refinement

Canonical epic: process_triage-2ka

---

## Phase 8: Safety and Policy

Canonical epic: process_triage-dvi

---

## Phase 9: Shadow Mode and Calibration

Canonical epic: process_triage-21f

---

## Phase 10: Supervisor Detection

Canonical epic: process_triage-6l1

Notes
- Supervisor-aware actions (systemd/launchd/supervisord/pm2/docker/containerd/nodemon/tmux/screen)
- Respawn loop detection

---

## Phase 11: Pattern/Signature Library

Canonical epic: process_triage-79x

---

## Phase 12: Trajectory Prediction

Canonical epic: process_triage-mpi

Notes
- Time-to-threshold prediction
- Baseline establishment

---

## Phase 13: Goal-Oriented Optimization

Canonical epic: process_triage-uiq

Notes
- Explicit goals like "free 4GB RAM"
- Optimize plan vs risk

---

## Phase 14: Fleet Mode

Canonical epic: process_triage-8t1

Notes
- Multi-host coordination
- Pooled FDR where possible
- Correlation detection across fleet

---

## Phase 15: Enhanced UX for Agents

Canonical epic: process_triage-s8s

---

## Phase 16: Differential and Resumable Sessions

Canonical epic: process_triage-9k8

---

## Cross-Cutting Epics

These are not phases but are required to ship:

- pt-core Rust bootstrap: process_triage-40mt
- Telemetry lake + reporting: process_triage-k4yc
- Bundles + HTML reports: process_triage-bra
- Testing infrastructure: process_triage-aii
- Release/packaging/docs: process_triage-ica
- CI/CD: process_triage-68c
- Installation infrastructure: process_triage-n0r
- Self-update mechanism: process_triage-097
- Documentation: process_triage-aip

---

## Dependency Structure

Intended macro ordering (leaf beads may be executable earlier if they depend only on specs):

```
Phase 1 (Spec)
    |
    +---> Phase 2 (Math) + Phase 3 (Collection)
                    |
                    v
              Phase 4 (Inference)
                    |
                    v
              Phase 5 (Decision)
                    |
                    v
              Phase 6 (Action)
                    |
                    v
              Phase 7 (UX)
                    |
    +---------------+---------------+
    |               |               |
    v               v               v
Phase 8         Phase 9         Phase 10
(Safety)        (Shadow)        (Supervisor)
    |               |               |
    +-------+-------+-------+-------+
            |
            v
    "Safe Robot Mode" gate
            |
            +---> Phases 11-16
```

Phase gates:
- Phase 1 blocks Phase 2 and Phase 3
- Phase 2 + Phase 3 block Phase 4
- Phase 4 blocks Phase 5
- Phase 5 blocks Phase 6
- Phase 6 blocks Phase 7
- Phase 7 + Phase 8 + Phase 9 collectively gate "safe robot mode"

---

## Coverage checklist (Plan Section 10)

- [x] All Phases 1-16 have canonical epics.
- [x] Cross-cutting epics are explicitly listed.
- [x] Macro dependency ordering is documented.

## Acceptance criteria

- [x] Canonical mapping remains accurate (no missing plan subsection).
- [x] All referenced canonical bead IDs exist and are the intended owners.
- [x] Coverage checklist items in this bead are satisfied.
