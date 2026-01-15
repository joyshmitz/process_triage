# Plan Decision Theory Mapping (Section 5)

This file maps Plan Section 5 (Decision Theory and Optimal Stopping) to canonical beads so execution work does not require re-opening the full plan document.

## Core Principle

Inference answers "what is it?"; decision theory answers "what should we do *now*, given costs and risk?"

---

## Plan 5.1 - Expected Loss Decision

Plan requirement: minimize Bayes risk.
- `a* = argmin_a Sum_C L(a,C) P(C|x)`
- Loss matrix is policy-defined (default includes very high cost for killing useful processes).

Canonical beads
- Loss matrix + guardrails schema (`policy.json`): process_triage-bg5
- Expected loss computation + action selection: process_triage-d88

Notes
- Zombies can't be killed directly; the action set needs "resolve zombie" semantics (Plan Section 6).

---

## Plan 5.2 - Sequential Probability Ratio Test (SPRT)

Plan requirement: use the loss matrix to derive an odds threshold boundary; online accumulation yields an SPRT-like stopping rule.

Canonical beads
- Loss-derived posterior-odds boundary + threshold comparison: process_triage-d88
- Sequential stopping rules for evidence gathering: process_triage-of3n

---

## Plan 5.3 - Value of Information (VOI)

Plan requirement: probe only when expected loss reduction exceeds measurement cost.

Canonical beads
- VOI computation: process_triage-brh7
- Probe budgeting / scheduling policy: process_triage-p15.2

---

## Plan 5.4 - Queueing-theoretic Threshold Adjustment

Plan requirement: adjust aggressiveness based on system load/queueing pressure.

Canonical beads
- Load-aware threshold adjustment (Erlang-C/queuing): process_triage-p15.1

---

## Plan 5.5 - Dependency-Weighted Loss

Plan requirement: scale kill cost by dependency impact (blast radius / live sockets / open files).

Canonical beads
- Dependency impact score features: process_triage-cfon.5
- Dependency-weighted loss scaling: process_triage-un6

---

## Plan 5.6 - Causal Action Selection

Plan requirement: pick action by estimated recovery probability under intervention.

Canonical beads
- Causal action selection model `P(recover | do(a))` (Beta-Bernoulli): process_triage-p15.4

---

## Plan 5.7 - Belief-State Policy (Myopic POMDP)

Plan requirement: choose action minimizing loss under belief state `b_t` (with safety constraints).

Canonical beads
- Belief-state update utilities: process_triage-nao.16
- Myopic belief-state policy under safety constraints: process_triage-p15.5

---

## Plan 5.8 - FDR-Gated Kill Set Selection

Plan requirement: control many-process safety; default conservative under dependence.

Canonical beads
- FDR control via e-values and BH/BY: process_triage-sqe

---

## Plan 5.9 - Budgeted Instrumentation Policy (Whittle / VOI)

Plan requirement: allocate expensive probes under overhead constraints.

Canonical beads
- Probe scheduling policy (VOI/Whittle): process_triage-p15.2
- Submodular probe selection utilities: process_triage-p15.3

---

## Plan 5.10 - Active Sensing Action Selection

Plan requirement: measurement and action selection jointly optimized (VOI-aware).

Canonical beads
- Active sensing policy: process_triage-p15.2

---

## Plan 5.11 - Online FDR Risk Budget (Alpha-Investing)

Plan requirement: maintain long-run safety budget across repeated scans.

Canonical beads
- Alpha-investing online safety budget: process_triage-cpm

---

## Plan 5.12 - DRO / Worst-Case Expected Loss

Plan requirement: tighten kill thresholds under drift/misspecification.

Canonical beads
- DRO / worst-case expected loss gating: process_triage-6a1
- PPC/drift detectors that trigger DRO/robust modes: process_triage-0uy, process_triage-9kk3

---

## Plan 5.13 - Submodular Probe Set Selection

Plan requirement: pick overlapping probes near-optimally under overhead.

Canonical beads
- Submodular probe selection: process_triage-p15.3

---

## Plan 5.14 - Goal-Oriented Resource Recovery Optimization

Plan requirement: explicit goals like "free 4GB RAM"; optimize plan vs risk.

Canonical beads
- Goal-oriented optimization epic: process_triage-uiq

---

## Plan 5.15 - Differential Session Comparison

Plan requirement: delta-first output and delta-first deep scanning.

Canonical beads
- Differential/resumable sessions epic: process_triage-9k8
- Differential scanning implementation: process_triage-9k8.2
- Agent diff command surface: process_triage-gbq

---

## Plan 5.16 - Fleet-Wide Decision Coordination

Plan requirement: fleet-wide safety invariants, pooled FDR where possible, correlation detection.

Canonical beads
- Fleet mode epic: process_triage-8t1

---

## Coverage checklist (Plan Section 5)

- [x] 5.1-5.16 are mapped above to canonical beads.
- [x] All decision outputs are ledgered (why) and policy-gated (safety).

## Acceptance criteria

- [x] Canonical mapping remains accurate (no missing plan subsection).
- [x] All referenced canonical bead IDs exist and are the intended owners.
- [x] Coverage checklist items in this bead are satisfied.
