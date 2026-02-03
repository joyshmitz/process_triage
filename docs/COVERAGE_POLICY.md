# Coverage Policy

## Purpose
Coverage reports provide visibility into how much of the system is exercised in tests and protect against regressions.

## Tooling
We use `cargo llvm-cov` for workspace coverage.

Local run:
```bash
scripts/coverage.sh
```

Outputs:
- `coverage/coverage.lcov`
- `coverage/html/`
- `coverage/coverage.json`
- `coverage/coverage_summary.json`

## Thresholds
Thresholds are defined in `coverage.toml`.

- `global` applies to overall workspace totals.
- `crates.*` applies per crate.
- Set `enforce = false` with a `reason` for explicit exceptions.

## Exceptions
Exceptions must be explicit in `coverage.toml` and include a reason. Do not silently skip enforcement.

## CI
The Coverage workflow runs `scripts/coverage.sh`, enforces thresholds, and uploads artifacts.
