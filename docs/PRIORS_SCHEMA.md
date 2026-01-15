# Priors Schema Specification

**Version**: 1.0.0
**Status**: Draft
**Bead**: process_triage-2f3

---

## 1. Overview

The `priors.json` file contains all Bayesian hyperparameters used by the Process Triage inference engine. These priors encode our initial beliefs about process behavior before observing any data, and are updated as the system learns from user decisions.

**Design Principles**:
1. **Conjugate families only**: All priors use Beta, Gamma, or Dirichlet distributions to enable closed-form posterior updates
2. **Rate parameterization for Gamma**: Gamma distributions use shape/rate (not shape/scale) to match the plan specification
3. **Human-editable**: JSON format with documentation fields for customization
4. **Versioned**: Schema version field for forward compatibility

---

## 2. Four-State Classification Model

Process Triage classifies processes into four states:

| State | Key (snake_case) | Description |
|-------|------------------|-------------|
| **Useful** | `useful` | Process doing productive work the user cares about |
| **Useful-but-bad** | `useful_bad` | Process running but stuck, leaking, or misbehaving |
| **Abandoned** | `abandoned` | Process was once useful but no longer needed |
| **Zombie** | `zombie` | Process terminated but not reaped (state = 'Z') |

---

## 3. Distribution Parameterization

### 3.1 Beta Distribution

Used for Bernoulli features (binary outcomes):

```json
{
  "alpha": 5.0,
  "beta": 2.0
}
```

- **Mean**: `alpha / (alpha + beta)`
- **Concentration**: Higher `alpha + beta` = more confident prior
- **Example**: `Beta(5, 2)` has mean 0.71 (high probability)

### 3.2 Gamma Distribution

Used for positive continuous quantities (hazard rates, durations):

```json
{
  "shape": 2.0,
  "rate": 0.01
}
```

- **Mean**: `shape / rate`
- **Variance**: `shape / rate^2`
- **Note**: We use **rate** parameterization, not scale. Rate = 1/scale.

### 3.3 Dirichlet Distribution

Used for categorical features (multinomial outcomes):

```json
{
  "alpha": [2.0, 5.0, 3.0, 1.0]
}
```

- **Mean** for category i: `alpha[i] / sum(alpha)`
- **Concentration**: Higher sum = more confident prior

---

## 4. Per-Class Priors

Each class has the following Bayesian hyperparameters:

### 4.1 Class Prior Probability

```json
"prior_prob": 0.70
```

The prior probability P(C) for this class. All class priors should sum to 1.0.

### 4.2 CPU Occupancy (Beta-Binomial)

```json
"cpu_beta": { "alpha": 5.0, "beta": 3.0 }
```

Prior for CPU occupancy rate `p_u|C ~ Beta(alpha, beta)`.
- **Useful**: Higher alpha (processes actively using CPU)
- **Abandoned**: Lower alpha (processes idle)

### 4.3 Runtime (Gamma)

```json
"runtime_gamma": { "shape": 2.0, "rate": 0.0001 }
```

Prior for runtime `t|C ~ Gamma(shape, rate)`.
- **Note**: Use either runtime_gamma OR survival/hazard modeling, not both.

### 4.4 Orphan Status (Beta-Bernoulli)

```json
"orphan_beta": { "alpha": 1.0, "beta": 20.0 }
```

Prior for PPID=1 probability `p_o|C ~ Beta(alpha, beta)`.
- **Abandoned**: Higher alpha (often reparented to init)
- **Useful**: Lower alpha (has live parent)

### 4.5 TTY Attachment (Beta-Bernoulli)

```json
"tty_beta": { "alpha": 8.0, "beta": 2.0 }
```

Prior for TTY attachment probability `q|C ~ Beta(alpha, beta)`.
- **Useful**: Higher alpha (attached to terminal)
- **Abandoned**: Lower alpha (detached)

### 4.6 Network Activity (Beta-Bernoulli)

```json
"net_beta": { "alpha": 5.0, "beta": 3.0 }
```

Prior for network activity probability `r|C ~ Beta(alpha, beta)`.

### 4.7 I/O Activity (Beta-Bernoulli)

```json
"io_active_beta": { "alpha": 6.0, "beta": 2.0 }
```

Prior for I/O activity probability.

### 4.8 Competing Hazards

```json
"competing_hazards": {
  "finish": { "shape": 2.0, "rate": 5.0 },
  "abandon": { "shape": 1.0, "rate": 100.0 },
  "degrade": { "shape": 1.0, "rate": 50.0 }
}
```

Per-class hazard rates for different state transitions:
- `finish`: Rate of normal termination
- `abandon`: Rate of becoming abandoned
- `degrade`: Rate of becoming useful_bad

---

## 5. Hazard Regimes

Piecewise-constant hazard regimes model how hazard rates change when specific events occur:

```json
{
  "name": "tty_lost",
  "description": "Process lost its controlling TTY",
  "gamma": { "shape": 3.0, "rate": 5.0 },
  "trigger_conditions": ["tty changed from pts/* to ?"]
}
```

Standard regimes:
- `baseline`: Normal operation
- `tty_lost`: TTY detached
- `orphaned`: Reparented to init
- `io_flatline`: I/O completely idle
- `cpu_stalled`: CPU time not advancing
- `memory_leak`: RSS growing unbounded

---

## 6. Change-Point Detection

Priors for detecting activity changes over time:

```json
"change_point": {
  "p_before": { "alpha": 5.0, "beta": 2.0 },
  "p_after": { "alpha": 1.0, "beta": 5.0 },
  "tau_geometric_p": 0.01
}
```

- `p_before`: Activity rate before change point
- `p_after`: Activity rate after change point
- `tau_geometric_p`: Geometric prior on change-point location

---

## 7. Causal Intervention Priors

Priors for action outcomes (do-calculus models):

```json
"causal_interventions": {
  "kill": {
    "useful": { "alpha": 1.0, "beta": 20.0 },
    "abandoned": { "alpha": 9.0, "beta": 1.0 }
  }
}
```

Each action has per-class Beta priors for `P(recover | do(action), class)`.

---

## 8. Command Categories

Dirichlet priors for command type classification:

```json
"command_categories": {
  "category_names": ["test_runner", "dev_server", "build_tool", ...],
  "useful": { "alpha": [2.0, 5.0, 3.0, ...] },
  "abandoned": { "alpha": [8.0, 6.0, 5.0, ...] }
}
```

The alpha vector indices correspond to `category_names` in order.

---

## 9. Process State Flags

Dirichlet priors for OS process state (R/S/D/Z/T):

```json
"state_flags": {
  "flag_names": ["R", "S", "D", "Z", "T", "t", "X"],
  "useful": { "alpha": [5.0, 8.0, 2.0, 0.1, 1.0, 0.5, 0.1] },
  "zombie": { "alpha": [0.1, 0.1, 0.5, 10.0, 0.1, 0.1, 1.0] }
}
```

---

## 10. Robust Bayes Settings

Imprecise priors and Safe Bayes tempering for robustness:

```json
"robust_bayes": {
  "class_prior_bounds": {
    "useful": { "lower": 0.50, "upper": 0.85 }
  },
  "safe_bayes_eta": 1.0,
  "auto_eta_enabled": true
}
```

- `class_prior_bounds`: Credal set bounds for each class
- `safe_bayes_eta`: Learning rate (1.0 = standard Bayes, <1.0 = tempered)
- `auto_eta_enabled`: Auto-adjust eta based on prediction performance

---

## 11. Error Rate Tracking

Beta priors for false-kill/false-spare rate estimation:

```json
"error_rate": {
  "false_kill": { "alpha": 1.0, "beta": 99.0 },
  "false_spare": { "alpha": 5.0, "beta": 95.0 }
}
```

Initial priors encode belief that false-kill rate is ~1% and false-spare rate is ~5%.

---

## 12. BOCPD Settings

Bayesian Online Change-Point Detection configuration:

```json
"bocpd": {
  "hazard_lambda": 0.01,
  "min_run_length": 10
}
```

- `hazard_lambda`: 1 / expected_run_length between change points
- `min_run_length`: Minimum samples before considering a change point

---

## 13. File Locations

| File | Purpose |
|------|---------|
| `~/.config/process_triage/priors.json` | User's customized priors |
| Built-in defaults | Used when no user file exists |
| Learned updates | Written after shadow-mode calibration |

---

## 14. Customization Guide

### Adjusting for Your Workload

**If you run many long-lived dev servers**:
- Increase `useful.runtime_gamma.rate` (lower mean expected runtime)
- Increase `abandoned.orphan_beta.beta` (more confidence that orphans are abandoned)

**If you run many test suites**:
- Increase `abandoned.prior_prob` slightly
- Adjust `command_categories.abandoned` alpha for test_runner

**For CI/CD environments**:
- Lower `useful.prior_prob` (most processes should complete)
- Increase `abandoned.prior_prob`

### Fleet Prior Sharing

Use `host_profile` to tag priors for different machine types:

```json
{
  "host_profile": "devbox",
  "description": "Priors learned from developer workstations"
}
```

Export and import priors between machines:
```bash
pt agent export-priors --host-profile devbox --out priors.json
pt agent import-priors --from priors.json --merge
```

---

## 15. References

- PLAN Section 4.2: Priors and Likelihoods (Conjugate)
- PLAN Section 4.5: Semi-Markov and Competing Hazards
- PLAN Section 4.7: Change-Point Detection
- PLAN Section 4.9: Robust Bayes (Imprecise Priors)
- PLAN Section 4.10: Causal Intervention Models
- PLAN Section 4.15: Bayesian Credible Bounds
- PLAN Section 4.16: Empirical Bayes Hyperparameter Calibration

---

## 16. Schema Files

- JSON Schema: `specs/schemas/priors.schema.json`
- Default values: `specs/schemas/priors.default.json`
