# Galaxy-Brain Mode Contract

> **Bead**: `process_triage-8f6`
> **Status**: Specification
> **Version**: 1.0.0

## Overview

Galaxy-brain mode is the "alien artifact" transparency feature that exposes the full mathematical derivation behind every recommendation. It serves dual purposes:

1. **Educational/Fun**: Users can see "the scary math" with real numbers
2. **Debuggable**: Developers and power users can verify inference correctness

### Design Philosophy

- **Concrete over abstract**: Show equations AND substituted values
- **Intuitive summaries**: Every card has a one-line plain-English explanation
- **Consistent across surfaces**: Same data model for TUI, CLI, and reports
- **Regenerable**: Can reconstruct from stored inference artifacts

---

## Activation

### TUI
- Press `g` to toggle galaxy-brain mode
- Visual indicator shows when active
- Cards appear in detail pane alongside evidence

### CLI
```bash
pt-core run --galaxy-brain
pt-core agent explain --pid 1234 --galaxy-brain
pt-core agent plan --galaxy-brain
```

### Reports
- Separate "Math" tab in HTML report
- Toggle visibility via checkbox
- All cards rendered with LaTeX equations

---

## Card Schema

### JSON Schema

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "galaxy_brain.schema.json",
  "title": "Galaxy-Brain Card Schema",
  "type": "object",
  "required": ["schema_version", "pid", "cards"],
  "properties": {
    "schema_version": {
      "type": "string",
      "const": "1.0.0"
    },
    "pid": {
      "type": "integer",
      "description": "Process ID this analysis applies to"
    },
    "generated_at": {
      "type": "string",
      "format": "date-time"
    },
    "cards": {
      "type": "array",
      "items": { "$ref": "#/$defs/card" }
    }
  },
  "$defs": {
    "card": {
      "type": "object",
      "required": ["id", "title", "equations", "values", "intuition"],
      "properties": {
        "id": {
          "type": "string",
          "enum": [
            "posterior_core",
            "hazard_time_varying",
            "conformal_interval",
            "conformal_class",
            "e_fdr",
            "alpha_investing",
            "voi"
          ],
          "description": "Stable card identifier"
        },
        "title": {
          "type": "string",
          "description": "Human-readable card title"
        },
        "equations": {
          "type": "array",
          "items": {
            "type": "object",
            "required": ["latex", "label"],
            "properties": {
              "latex": {
                "type": "string",
                "description": "LaTeX equation (KaTeX compatible)"
              },
              "label": {
                "type": "string",
                "description": "Short label for the equation"
              },
              "substituted": {
                "type": "string",
                "description": "Equation with values substituted"
              }
            }
          },
          "description": "Mathematical equations in LaTeX format"
        },
        "values": {
          "type": "object",
          "additionalProperties": {
            "oneOf": [
              { "type": "number" },
              { "type": "string" },
              { "type": "array", "items": { "type": "number" } }
            ]
          },
          "description": "Computed values referenced in equations"
        },
        "intuition": {
          "type": "string",
          "description": "One-line plain-English explanation"
        },
        "details": {
          "type": "string",
          "description": "Extended explanation (optional)"
        },
        "collapse_default": {
          "type": "boolean",
          "default": false,
          "description": "Whether to collapse this card by default in TUI"
        }
      }
    }
  }
}
```

---

## Standard Card Definitions

### 1. Posterior Core (`posterior_core`)

**Purpose**: Show the main Bayesian posterior computation

**Equations**:
```latex
\log P(C|x) = \log P(C) + \sum_i \log P(x_i|C) - \log P(x)
```

**Values**:
| Key | Type | Description |
|-----|------|-------------|
| `log_prior_useful` | float | log P(useful) |
| `log_prior_useful_bad` | float | log P(useful_bad) |
| `log_prior_abandoned` | float | log P(abandoned) |
| `log_prior_zombie` | float | log P(zombie) |
| `log_likelihood_useful` | float | log P(x\|useful) |
| `log_likelihood_useful_bad` | float | log P(x\|useful_bad) |
| `log_likelihood_abandoned` | float | log P(x\|abandoned) |
| `log_likelihood_zombie` | float | log P(x\|zombie) |
| `log_evidence` | float | log P(x) |
| `posterior_useful` | float | P(useful\|x) |
| `posterior_useful_bad` | float | P(useful_bad\|x) |
| `posterior_abandoned` | float | P(abandoned\|x) |
| `posterior_zombie` | float | P(zombie\|x) |
| `evidence_terms` | array | Individual evidence contributions |

**Example**:
```json
{
  "id": "posterior_core",
  "title": "Posterior Core",
  "equations": [
    {
      "latex": "\\log P(C|x) = \\log P(C) + \\sum_i \\log P(x_i|C) - \\log P(x)",
      "label": "Bayes' Rule (log form)",
      "substituted": "\\log P(\\text{aband}|x) = -2.11 + (-3.45) - (-4.89) = -0.67"
    },
    {
      "latex": "P(\\text{abandoned}|x) = \\frac{e^{-0.67}}{\\sum_C e^{\\log P(C|x)}}",
      "label": "Normalized posterior",
      "substituted": "P(\\text{abandoned}|x) = 0.98"
    }
  ],
  "values": {
    "log_prior_abandoned": -2.11,
    "log_likelihood_abandoned": -3.45,
    "log_evidence": -4.89,
    "posterior_abandoned": 0.98,
    "evidence_terms": [
      {"name": "cpu_low", "contribution": 0.82},
      {"name": "tty_detached", "contribution": 0.65},
      {"name": "runtime_long", "contribution": 0.43}
    ]
  },
  "intuition": "CPU+TTY evidence dominate; 98% confidence this is abandoned."
}
```

---

### 2. Time-Varying Hazard (`hazard_time_varying`)

**Purpose**: Show regime-based hazard rate analysis

**Equations**:
```latex
\lambda_r | \text{data} \sim \text{Gamma}(\alpha + n_r, \beta + T_r)
```

**Values**:
| Key | Type | Description |
|-----|------|-------------|
| `regimes` | array | Active hazard regimes |
| `base_hazard` | float | Base hazard rate |
| `regime_hazards` | object | Per-regime hazard posteriors |
| `survival_prob` | float | P(still alive at time t) |

**Example**:
```json
{
  "id": "hazard_time_varying",
  "title": "Time-Varying Hazard",
  "equations": [
    {
      "latex": "\\lambda_r | \\text{data} \\sim \\text{Gamma}(\\alpha + n_r, \\beta + T_r)",
      "label": "Posterior hazard rate",
      "substituted": "\\lambda_{\\text{orphan}} \\sim \\text{Gamma}(3.2, 1.8)"
    },
    {
      "latex": "S(t) = \\exp\\left(-\\int_0^t \\lambda(s)\\,ds\\right)",
      "label": "Survival function"
    }
  ],
  "values": {
    "regimes": ["orphaned", "tty_lost"],
    "base_hazard": 0.001,
    "regime_hazards": {
      "orphaned": {"alpha": 3.2, "beta": 1.8, "mean": 1.78},
      "tty_lost": {"alpha": 2.5, "beta": 2.0, "mean": 1.25}
    },
    "survival_prob": 0.12
  },
  "intuition": "Orphan+TTY regimes active; only 12% chance still useful."
}
```

---

### 3. Conformal Interval (`conformal_interval`)

**Purpose**: Show prediction intervals for runtime/resource usage

**Equations**:
```latex
\hat{C}_\alpha = [\hat{y} - q_{1-\alpha}(|R|), \hat{y} + q_{1-\alpha}(|R|)]
```

**Values**:
| Key | Type | Description |
|-----|------|-------------|
| `coverage` | float | Target coverage level (e.g., 0.95) |
| `runtime_interval` | [float, float] | Predicted runtime bounds |
| `cpu_interval` | [float, float] | Predicted CPU bounds |
| `rss_interval` | [float, float] | Predicted memory bounds |
| `calibration_n` | int | Calibration set size |

**Example**:
```json
{
  "id": "conformal_interval",
  "title": "Conformal Prediction Intervals",
  "equations": [
    {
      "latex": "\\hat{C}_{0.95} = [\\hat{y} - q_{0.95}, \\hat{y} + q_{0.95}]",
      "label": "95% prediction interval"
    }
  ],
  "values": {
    "coverage": 0.95,
    "runtime_interval": [259200, 432000],
    "cpu_interval": [0.0, 0.8],
    "rss_interval": [1800, 2400],
    "calibration_n": 127
  },
  "intuition": "95% confident runtime will be 3-5 days; CPU stays below 1%."
}
```

---

### 4. Conformal Class Set (`conformal_class`)

**Purpose**: Show classification prediction sets with p-values

**Equations**:
```latex
\Gamma_\alpha = \{c : p_c \geq \alpha\}
```

**Values**:
| Key | Type | Description |
|-----|------|-------------|
| `alpha` | float | Significance level |
| `p_values` | object | Per-class p-values |
| `prediction_set` | array | Classes in prediction set |
| `set_size` | int | Size of prediction set |

**Example**:
```json
{
  "id": "conformal_class",
  "title": "Conformal Classification",
  "equations": [
    {
      "latex": "\\Gamma_{0.05} = \\{c : p_c \\geq 0.05\\}",
      "label": "95% prediction set"
    }
  ],
  "values": {
    "alpha": 0.05,
    "p_values": {
      "useful": 0.02,
      "useful_bad": 0.08,
      "abandoned": 0.89,
      "zombie": 0.15
    },
    "prediction_set": ["useful_bad", "abandoned", "zombie"],
    "set_size": 3
  },
  "intuition": "Can't rule out useful_bad or zombie at 95%; abandoned most likely."
}
```

---

### 5. e-Values and e-FDR (`e_fdr`)

**Purpose**: Show anytime-valid multiple testing control

**Equations**:
```latex
e_i = \frac{P(x_i|H_1)}{P(x_i|H_0)}, \quad \text{eFDR} = \frac{\sum_{i \in R} 1/e_i}{|R|}
```

**Values**:
| Key | Type | Description |
|-----|------|-------------|
| `e_value` | float | e-value for this process |
| `e_threshold` | float | Rejection threshold |
| `rejected` | bool | Whether H0 is rejected |
| `efdr_estimate` | float | Estimated e-FDR |
| `n_discoveries` | int | Number of discoveries so far |

**Example**:
```json
{
  "id": "e_fdr",
  "title": "Anytime-Valid FDR Control",
  "equations": [
    {
      "latex": "e_i = \\frac{P(x_i|H_1)}{P(x_i|H_0)}",
      "label": "e-value (likelihood ratio)"
    },
    {
      "latex": "\\widehat{\\text{eFDR}} = \\frac{\\sum_{i \\in R} 1/e_i}{|R|}",
      "label": "e-FDR estimator"
    }
  ],
  "values": {
    "e_value": 48.2,
    "e_threshold": 20.0,
    "rejected": true,
    "efdr_estimate": 0.03,
    "n_discoveries": 5
  },
  "intuition": "e-value 48.2 > 20 threshold; reject H0 (useful) at ~3% FDR."
}
```

---

### 6. Alpha-Investing (`alpha_investing`)

**Purpose**: Show online testing budget state

**Equations**:
```latex
W(t+1) = \begin{cases} W(t) - \alpha_t + \omega & \text{if reject} \\ W(t) - \alpha_t & \text{otherwise} \end{cases}
```

**Values**:
| Key | Type | Description |
|-----|------|-------------|
| `current_wealth` | float | Current alpha budget |
| `initial_wealth` | float | Starting budget |
| `alpha_spent` | float | Alpha spent this test |
| `reward_if_reject` | float | Budget gained on rejection |
| `tests_run` | int | Total tests conducted |
| `rejections` | int | Total rejections |

**Example**:
```json
{
  "id": "alpha_investing",
  "title": "Alpha-Investing Budget",
  "equations": [
    {
      "latex": "W(t+1) = W(t) - \\alpha_t + \\omega \\cdot \\mathbf{1}_{\\text{reject}}",
      "label": "Wealth update rule"
    }
  ],
  "values": {
    "current_wealth": 0.042,
    "initial_wealth": 0.050,
    "alpha_spent": 0.003,
    "reward_if_reject": 0.001,
    "tests_run": 12,
    "rejections": 5
  },
  "intuition": "Budget at 4.2%; can afford ~14 more tests at current spending."
}
```

---

### 7. Value of Information (`voi`)

**Purpose**: Show what additional evidence would change the decision

**Equations**:
```latex
\text{VOI}(E) = \mathbb{E}_{E}[\max_a U(a|x,E)] - \max_a U(a|x)
```

**Values**:
| Key | Type | Description |
|-----|------|-------------|
| `current_utility` | float | Utility of current best action |
| `probe_voi` | object | VOI per potential probe |
| `best_probe` | string | Highest VOI probe |
| `best_probe_voi` | float | VOI of best probe |
| `would_change` | bool | Whether best probe might change decision |

**Example**:
```json
{
  "id": "voi",
  "title": "Value of Information",
  "equations": [
    {
      "latex": "\\text{VOI}(E) = \\mathbb{E}_{E}[\\max_a U(a|x,E)] - \\max_a U(a|x)",
      "label": "Expected value of information"
    }
  ],
  "values": {
    "current_utility": 0.87,
    "probe_voi": {
      "strace": 0.12,
      "lsof": 0.08,
      "perf": 0.03,
      "network": 0.02
    },
    "best_probe": "strace",
    "best_probe_voi": 0.12,
    "would_change": true
  },
  "intuition": "strace could shift decision; might find active syscalls."
}
```

---

## TUI Rendering Specification

### Layout

```
┌────────────────────────────────────────────────────────────────────┐
│ 🧠 GALAXY-BRAIN MODE                                    [g] toggle │
├────────────────────────────────────────────────────────────────────┤
│                                                                    │
│ ┌─ Posterior Core ──────────────────────────────────────────────┐ │
│ │                                                                │ │
│ │  log P(C|x) = log P(C) + Σᵢ log P(xᵢ|C) − log P(x)           │ │
│ │                                                                │ │
│ │  log P(aband|x) = -2.11 + (-3.45) - (-4.89) = -0.67          │ │
│ │  → P(abandoned|x) = 0.98                                      │ │
│ │                                                                │ │
│ │  💡 CPU+TTY evidence dominate; 98% confidence abandoned.      │ │
│ └────────────────────────────────────────────────────────────────┘ │
│                                                                    │
│ ┌─ Time-Varying Hazard ─────────────────────────────────────────┐ │
│ │                                                                │ │
│ │  λᵣ | data ~ Gamma(α + nᵣ, β + Tᵣ)                           │ │
│ │  λ_orphan ~ Gamma(3.2, 1.8), mean = 1.78                      │ │
│ │  S(t) = 0.12                                                  │ │
│ │                                                                │ │
│ │  💡 Orphan+TTY regimes active; 12% survival probability.      │ │
│ └────────────────────────────────────────────────────────────────┘ │
│                                                                    │
│ [↑↓] Navigate cards  [Enter] Expand  [c] Copy LaTeX  [g] Close   │
└────────────────────────────────────────────────────────────────────┘
```

### Unicode Math Fallback

For terminals without LaTeX rendering, use Unicode approximations:

| LaTeX | Unicode |
|-------|---------|
| `\sum` | Σ |
| `\prod` | Π |
| `\alpha` | α |
| `\beta` | β |
| `\lambda` | λ |
| `\log` | log |
| `\exp` | exp |
| `\frac{a}{b}` | a/b |
| `\sqrt{x}` | √x |
| `\infty` | ∞ |
| `\leq` | ≤ |
| `\geq` | ≥ |
| `\neq` | ≠ |
| `\sim` | ~ |
| `\in` | ∈ |
| `\mathbb{E}` | 𝔼 |
| `\mathbf{1}` | 𝟙 |

### Card Keyboard Navigation

| Key | Action |
|-----|--------|
| `g` | Toggle galaxy-brain mode |
| `↑/↓` | Navigate between cards |
| `Enter` | Expand/collapse card |
| `c` | Copy card LaTeX to clipboard |
| `j` | Copy card JSON to clipboard |
| `Tab` | Focus next card |

---

## CLI Output Format

### JSON Output (default)

```bash
pt-core agent explain --pid 1234 --galaxy-brain --format json
```

```json
{
  "schema_version": "1.0.0",
  "session_id": "pt-20260115-143022-a7xq",
  "pid": 1234,
  "galaxy_brain": {
    "enabled": true,
    "cards": [
      {
        "id": "posterior_core",
        "title": "Posterior Core",
        "equations": [...],
        "values": {...},
        "intuition": "..."
      },
      ...
    ]
  },
  "recommendation": "kill",
  "posterior_abandoned": 0.98
}
```

### Markdown Output

```bash
pt-core agent explain --pid 1234 --galaxy-brain --format md
```

```markdown
## Galaxy-Brain Analysis: PID 1234

### Posterior Core

$$\log P(C|x) = \log P(C) + \sum_i \log P(x_i|C) - \log P(x)$$

**Values**:
- log P(abandoned|x) = -2.11 + (-3.45) - (-4.89) = **-0.67**
- P(abandoned|x) = **0.98**

> 💡 CPU+TTY evidence dominate; 98% confidence this is abandoned.

---

### Time-Varying Hazard
...
```

---

## HTML Report Tab

### Tab Structure

```html
<div class="report-tabs">
  <button class="tab active" data-tab="summary">Summary</button>
  <button class="tab" data-tab="evidence">Evidence</button>
  <button class="tab" data-tab="math">🧠 Math</button>
  <button class="tab" data-tab="timeline">Timeline</button>
</div>

<div class="tab-content" id="math">
  <div class="galaxy-brain-cards">
    <!-- Cards rendered here with KaTeX -->
  </div>
</div>
```

### KaTeX Rendering

```html
<script src="https://cdn.jsdelivr.net/npm/katex@0.16.0/dist/katex.min.js"></script>
<script>
  document.querySelectorAll('.equation').forEach(el => {
    katex.render(el.dataset.latex, el, { throwOnError: false });
  });
</script>
```

### SRI Integrity

All CDN assets use Subresource Integrity (SRI) for `file://` compatibility:

```html
<link rel="stylesheet"
      href="https://cdn.jsdelivr.net/npm/katex@0.16.0/dist/katex.min.css"
      integrity="sha384-..."
      crossorigin="anonymous">
```

---

## Caching Strategy

Galaxy-brain cards are computationally derived from inference results. To enable responsive TUI toggling:

1. **Pre-compute on inference**: Generate all cards during inference phase
2. **Store in session**: Write `galaxy_brain.json` to session directory
3. **Lazy load in TUI**: Only render visible cards
4. **Memory cache**: Keep recently viewed cards in memory

### Cache Invalidation

Cards are regenerated when:
- Inference results change
- Card schema version changes
- User requests refresh (`Shift+G`)

---

## Acceptance Criteria

- [ ] All 7 standard cards are defined with stable IDs
- [ ] JSON schema validates all card structures
- [ ] TUI renders cards with Unicode fallback
- [ ] CLI outputs cards in JSON and Markdown formats
- [ ] HTML reports render cards with KaTeX
- [ ] Cards are cached for responsive toggling
- [ ] `g` keybinding toggles mode in TUI

---

## Test Plan

### Unit Tests
- Card schema validation
- Unicode fallback rendering
- LaTeX to Unicode conversion

### Integration Tests
- TUI toggle behavior
- CLI output format correctness
- Report tab rendering

### E2E Tests
- Full inference → galaxy-brain flow
- Card content accuracy vs manual computation
- Cross-browser KaTeX rendering

---

## References

- CLI Specification: `docs/CLI_SPECIFICATION.md` (galaxy-brain flag)
- UX Mapping: `docs/PLAN_UX_EXPLAINABILITY_MAPPING.md` (Section 7.8)
- Golden Path UX: `docs/GOLDEN_PATH_UX.md` (TUI layout)
- Priors Schema: `docs/schemas/priors.schema.json` (distributions)
