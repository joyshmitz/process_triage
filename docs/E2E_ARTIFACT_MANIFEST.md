# E2E Artifact Manifest

The E2E artifact manifest is a versioned JSON file that describes a single test run and
all artifacts produced (logs, snapshots, plans, telemetry, reports). It provides a
stable, machine-readable contract for CI uploads and validation.

## Schema

Schema location:
- `specs/schemas/e2e-artifact-manifest.schema.json`

Versioning policy:
- `schema_version` must be semver.
- Major version `1` is the only supported major in the validator.
- Minor/patch bumps are allowed for backward-compatible additions.

## Required Fields (Top Level)

- `schema_version`: Semver string (major 1).
- `run_id`: Unique E2E run identifier.
- `suite`: E2E suite name (tui, install, daemon, bundle-report, etc.).
- `test_id`: Test case/workflow identifier.
- `timestamp`: RFC3339 timestamp.
- `env`: Execution environment (os/arch/kernel/ci provider + optional metadata).
- `commands[]`: Commands executed (argv, exit_code, duration_ms).
- `logs[]`: Log files with sha256 + size.
- `artifacts[]`: Additional outputs with sha256 + size.
- `metrics`: Timing + counts + flake retry counts.
- `manifest_sha256`: SHA-256 of the canonical manifest JSON with this field removed.

## Hashing Rules

`manifest_sha256` is computed over canonical JSON:
- Remove the `manifest_sha256` field.
- Serialize with `sort_keys=true` and separators `,` and `:`.
- Hash UTF-8 bytes with SHA-256.

Each entry in `logs[]` and `artifacts[]` must include:
- `path` (relative to manifest location unless absolute)
- `sha256` of file bytes
- `bytes` (file size)

## Run ID Correlation

JSONL logs must include a `run_id` field matching the manifest `run_id` so
artifacts can be correlated across systems.

## Validation

Validator CLI:
- `scripts/validate_e2e_manifest.py <path/to/manifest.json>`

Validation includes:
- Required field checks
- Schema validation (if `jsonschema` is available)
- File existence + sha256/byte verification
- Redaction profile enforcement for sensitive artifact kinds

## Examples

Example manifests and sample artifacts:
- `test/fixtures/manifest_examples/tui/manifest.json`
- `test/fixtures/manifest_examples/install/manifest.json`
- `test/fixtures/manifest_examples/daemon/manifest.json`
- `test/fixtures/manifest_examples/bundle_report/manifest.json`
