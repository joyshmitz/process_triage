# Plan §2 Source Idea Inventory Mapping

> **Bead**: `process_triage-h89.1`
> **Version**: 1.0.0
> **Status**: Draft

This document maps the Plan §2 “Source Idea Inventory” (A–CD) to canonical beads so contributors can locate the implementation thread without reopening the full plan.

**Design stance (from plan)**
- The runtime decision core stays **closed-form Bayesian + decision theory** (conjugate priors, log-domain math; no ML).
- “Advanced” techniques (Hawkes/EVT/copulas/wavelets/etc.) are used as **feature extractors / summaries** feeding deterministic quantities into the closed-form core, and are logged in telemetry + explainability ledger.

---

## A) Basic closed-form Bayesian model (non-ML)
- Core posterior computation `P(C|x)`: `process_triage-e48`, `process_triage-wb3`
- Evidence ledger (per-term contributions): `process_triage-myq`
- Conjugate math primitives (Beta/Gamma/Dirichlet + log-domain stability): `process_triage-iau` (and children)
- CPU tick-delta occupancy features: `process_triage-3ir.1.1`, `process_triage-cfon.1`
- Stable identity tuple + provenance needed by posterior/action safety: `process_triage-cfon.2`
- Command/CWD categories + normalization outputs: `process_triage-g7w`, `process_triage-cfon.3`

## B) Decision rule via expected loss (Bayesian risk)
- Expected loss + loss-derived SPRT-style boundary: `process_triage-d88`
- Decision theory epic: `process_triage-p15`

## C) Survival analysis and hazards
- Gamma hazard updates + survival primitives: `process_triage-22q`
- Time-varying / regime hazard model: `process_triage-y4a`
- Nonparametric hazard (Beta-Stacy) tracked separately: `process_triage-nao.17`

## D) Change-point detection (closed-form)
- BOCPD run-length posterior + regime shift detection: `process_triage-lfrb`
- Prequential/CTW code-length anomaly features (MDL bridge): `process_triage-cfon.7`

## E) Hierarchical priors by command category
- Hierarchical priors + empirical-Bayes shrinkage mechanics: `process_triage-nao.10`

## F) Continuous-time hidden semi-Markov chain
- HSMM feature extractor (Gamma durations): `process_triage-nao.13`

## G) CPU model as Markov-modulated Poisson / Lévy subordinator
- Compound Poisson / Lévy burst summaries: `process_triage-nao.14`

## H) Bayes factors for model selection / MDL bridge
- Bayes factor computation and reporting: `process_triage-0ij`
- Evidence ledger surfacing (top BFs, term contributions): `process_triage-myq`
- MDL/prequential/CTW bridge features: `process_triage-cfon.7`

## I) Optimal stopping + SPRT
- Loss-derived stopping boundary (odds threshold): `process_triage-d88`
- Sequential stopping / probe-until-decision policy: `process_triage-of3n`
- Composite testing extensions: `process_triage-p15.7`
- Anytime-valid martingale/e-process gates: `process_triage-p15.8`
- Time-to-decision bound `T_max`: `process_triage-p15.6`

## J) Queueing theory for system-level cost
- Load-aware threshold adjustment (Erlang-C / queuing): `process_triage-p15.1`

## K) Value of Information
- VOI computation: `process_triage-brh7`
- Budgeted active sensing / scheduling: `process_triage-p15.2`, `process_triage-p15.3`

## L) Robust Bayes (imprecise priors / Safe-Bayes eta tempering)
- Robust Bayes + eta tempering: `process_triage-nao.11`
- Least-favorable / minimax prior gating: `process_triage-nao.20`

## M) Information-theoretic abnormality / large deviations
- KL surprisal + large-deviation bounds features: `process_triage-nao.12`

## N) Wonham filtering (continuous-time partial observability)
- Wonham filtering (and optional scheduling integration): `process_triage-p15.9`

## O) Gittins indices
- Wonham + Gittins / index scheduling: `process_triage-p15.9`

## P) Process genealogy / Bayesian network
- Genealogy priors + orphan Bayes factor framing: `process_triage-nao.15`
- Belief propagation on PPID trees: `process_triage-d7s`
- “Unexpected reparenting” evidence with supervision/session conditioning: `process_triage-cfon.4`
- Agent-facing genealogy narrative output: `process_triage-s8s`, `process_triage-bwn.1`

## Q) Practical enhancements (signals, actions, shadow mode, governance)
- Evidence collection + maximal instrumentation (budgeted): `process_triage-3ir`, `process_triage-71t`
- Feature layer hygiene + provenance: `process_triage-cfon`
- Expanded action space (pause/throttle/renice/cgroups/kill): `process_triage-sj6`
- Shadow mode + calibration loop: `process_triage-21f`
- Telemetry + retention + redaction/hashing: `process_triage-k4yc` (+ `process_triage-k4yc.1`, `process_triage-k4yc.6`)
- Policy engine + guardrails: `process_triage-dvi`
- Supervisor awareness + respawn handling: `process_triage-6l1`, `process_triage-sj6.8`

## S) Causal intervention layer (do-calculus)
- Causal action selection model (Beta-Bernoulli outcomes by action): `process_triage-p15.4`

## T) Coupled process-tree model
- Graph/Laplacian smoothing / coupling priors: `process_triage-nao.9`
- PPID-tree belief propagation: `process_triage-d7s`

## U) POMDP / belief-state decision
- Belief-state update utilities: `process_triage-nao.16`
- Myopic belief-state policy: `process_triage-p15.5`

## V) Generalization guarantees (PAC-Bayes + Bayesian credible bounds)
- Shadow-mode calibration epic: `process_triage-21f`
- Beta posterior credible bound on false-kill rate: `process_triage-21f.1`

## W) Empirical Bayes calibration
- EB/shrinkage mechanics by category: `process_triage-nao.10`
- Shadow-mode hyperparameter refits + calibration reporting: `process_triage-21f`

## X) Minimax / least-favorable priors
- Minimax / least-favorable prior gating: `process_triage-nao.20`

## Y) Time-to-decision bound
- Time-to-decision bound + default-to-pause: `process_triage-p15.6`

## Z) Dependency impact weighting
- Dependency impact features (sockets/clients/open files/service bindings): `process_triage-cfon.5`
- Dependency-weighted loss scaling: `process_triage-un6`

---

## AA) Hawkes processes (self-exciting point processes)
- Hawkes process layer summaries: `process_triage-hxh`

## AB) Marked point processes
- Marked point process summary features (event times + magnitudes): `process_triage-cfon.8`

## AC) Bayesian nonparametric survival (Beta-Stacy)
- Beta-Stacy discrete-time survival model: `process_triage-nao.17`

## AD) Empirical Bayes shrinkage (Efron–Morris)
- EB shrinkage applied to command/category priors: `process_triage-nao.10`

## AE) Bayesian online change-point detection (BOCPD)
- BOCPD run-length posterior: `process_triage-lfrb`

## AF) Robust statistics (Huberization)
- Robust statistics summaries / outlier suppression: `process_triage-nao.8`

## AG) Large deviations / rate functions (Chernoff, Cramer)
- Large-deviation bounds surfaced as evidence features: `process_triage-nao.12`

## AH) POMDP with belief-state approximation
- Belief update utilities + policy layer: `process_triage-nao.16`, `process_triage-p15.5`

## AI) Multivariate Hawkes processes (cross-excitation)
- Multivariate Hawkes cross-excitation summaries: `process_triage-nao.18`

## AJ) Copula models for dependence (Archimedean/vine)
- Copula-based dependence summaries: `process_triage-nao.1`

## AK) Risk-sensitive control / coherent risk measures
- CVaR risk-sensitive decision layer: `process_triage-ctb`

## AL) Bayesian model averaging (BMA)
- Model averaging over inference submodels: `process_triage-nao.7`

## AM) Composite-hypothesis testing (mixture SPRT / GLR)
- Composite testing extensions: `process_triage-p15.7`

## AN) Linear Gaussian state-space (Kalman)
- Kalman smoothing utilities: `process_triage-0io`

## AO) Optimal transport / Wasserstein distances
- 1D Wasserstein drift score: `process_triage-9kk3`

## AP) Martingale concentration / sequential bounds
- Time-uniform martingale gates / confidence sequences: `process_triage-p15.8`
- Martingale deviation feature summaries: `process_triage-cfon.9`

## AQ) Graph signal processing / Laplacian regularization
- Laplacian smoothing priors/features: `process_triage-nao.9`

## AR) Renewal reward / semi-regenerative processes
- Renewal-reward summaries for CPU/IO: `process_triage-nao.21`

## AS) Conformal prediction (distribution-free coverage)
- Conformal prediction intervals/sets: `process_triage-tcf`

## AT) False Discovery Rate (FDR) control across many processes
- e-values → BH/BY selection + FDR rule: `process_triage-sqe`

## AU) Restless bandits / Whittle index policies
- VOI/Whittle probe budgeting policy: `process_triage-p15.2`

## AV) Bayesian optimal experimental design (active sensing)
- Active sensing policy + probe selection utilities: `process_triage-p15.2`, `process_triage-p15.3`

## AW) Extreme value theory (POT/GPD)
- EVT tail modeling summaries: `process_triage-fh0d`

## AX) Streaming sketches / heavy-hitter theory
- Streaming sketches/heavy-hitter summaries: `process_triage-nao.5`

## AY) Belief propagation / message passing on process tree
- Sum-product BP on PPID forests: `process_triage-d7s`

## AZ) Wavelet / spectral periodicity analysis
- Wavelet/spectral periodicity features: `process_triage-nao.2`

## BA) Switching linear dynamical systems (IMM filter)
- Switching state-space (IMM) feature extractor: `process_triage-nao.6`

## BB) Online FDR / alpha-investing for sequential operations
- Alpha-investing budget + reporting: `process_triage-cpm`

## BC) Posterior predictive checks / Bayesian model criticism
- Posterior predictive checks (PPC) + misspecification flags: `process_triage-0uy`

## BD) Distributionally robust decision theory (DRO)
- DRO / worst-case expected-loss gating: `process_triage-6a1`

## BE) Submodular probe selection (near-optimal sensor scheduling)
- Submodular probe set selection: `process_triage-p15.3`

## BF) Telemetry & analytics (Parquet-first + DuckDB query engine)
- Telemetry epic (Parquet-first, DuckDB views/macros): `process_triage-k4yc` (+ `process_triage-5y9`, `process_triage-k4yc.2`)

## BG) Implementation architecture (bash wrapper + monolithic Rust core)
- Boundary/spec: wrapper vs pt-core monolith: `process_triage-kze`
- pt-core bootstrap + CLI skeleton: `process_triage-40mt`
- Packaging/install scaffolding: `process_triage-n0r`, `process_triage-ica`

## BH) Privacy/secrets governance for telemetry
- Redaction/hashing engine: `process_triage-k4yc.1`
- Policy/spec for redaction/hashing: `process_triage-8n3`

## BI) Agent/robot CLI contract (no TUI)
- Contract/spec: `process_triage-jqi`
- Implementation epic (parity layer): `process_triage-bwn`

## BJ) Shareable session bundles + rich HTML reports
- Bundles/reports epic: `process_triage-bra`
- Bundle writer/reader + manifest checksums: `process_triage-k4yc.3`
- Single-file HTML report generator (CDN pinned + SRI, galaxy-brain tab): `process_triage-k4yc.5`
- Optional bundle encryption: `process_triage-k4yc.4`

## BK) Dormant mode (always-on guardian)
- Dormant daemon epic: `process_triage-b4v`

## BL) Anytime-valid inference (e-values / e-processes) for 24/7 monitoring
- Time-uniform martingale/e-process gates: `process_triage-p15.8`
- e-values/BH/BY selection integration: `process_triage-sqe`

## BM) Time-uniform concentration inequalities (modern martingale bounds)
- Confidence sequences / time-uniform bounds: `process_triage-p15.8`, `process_triage-cfon.9`

## BN) Fast optimal-transport drift detection (Sinkhorn divergence)
- Sinkhorn divergence approximation: `process_triage-nao.19`

## BO) Fleet mode architecture (multi-host operation)
- Fleet mode epic: `process_triage-8t1`

## BP) Delta/differential mode (session comparison)
- Differential + resumable sessions epic: `process_triage-9k8`

## BQ) Goal-oriented resource recovery optimization
- Goal-oriented optimization epic: `process_triage-uiq`

## BR) Pattern/signature library (known-pattern fast path)
- Pattern/signature library epic: `process_triage-79x`

## BS) Process genealogy narrative (explain backstory)
- Agent-facing narratives + related UX: `process_triage-s8s`, `process_triage-bwn.1`

## BT) Supervisor-aware action routing
- Supervisor detection epic: `process_triage-6l1`

## BU) Trajectory/predictive analysis
- Trajectory prediction + baselines epic: `process_triage-mpi`

## BV) Blast radius / dependency graph visualization
- Blast radius analyzer (agent-facing): `process_triage-bwn.2`
- Dependency impact feature extraction (for loss scaling): `process_triage-cfon.5`

## BW) Confidence-bounded automation controls
- Robot constraints in policy + enforcement: `process_triage-dvi.2`

## BX) Session resumability and idempotency
- Session continuity / resumable apply: `process_triage-t6lf`, `process_triage-9k8`

## BY) Learning transfer (prior export/import)
- Export priors: `process_triage-iaco`
- Import priors: `process_triage-r2of`

## BZ) Agent-optimized output formats
- Output format spec: `process_triage-3mi`
- Agent summary modes: `process_triage-s8s`, `process_triage-bwn.4`

## CA) Watch/alert mode for agents
- Agent watch command: `process_triage-pwjm`

## CB) “What would change your mind” explanations
- Flip-conditions / what-if explainer: `process_triage-bwn.3`
- VOI support: `process_triage-brh7`

## CC) Per-machine learned baselines
- Per-host baseline modeling + anomalies: `process_triage-mpi`

## CD) Use-case interpretation of observed processes
- Applied interpretation examples (fixtures/regression intent): `process_triage-h89.4`

---

## Maintenance Notes
- If a future change introduces a new plan idea, add it here and create (or expand) the corresponding bead(s) with explicit rationale.
- Avoid pointing to closed duplicates/superseded beads.
