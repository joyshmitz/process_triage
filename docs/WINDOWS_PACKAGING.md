# Windows Packaging Operations

This document covers the release-time workflow for Windows package publication.

## Channels

- `Scoop`: `process-triage/scoop-bucket`
- `Winget`: dedicated repo/fork path, with optional upstream PR to `microsoft/winget-pkgs`
- `Chocolatey`: no automated publication path yet (manual/issue-tracked until parity work is scheduled)

## Automation Flow

`update-packages.yml` performs:

1. Generate package artifacts from release checksums (`pt.json`, optional Winget manifests).
2. Validate manifests and URLs.
3. Open PRs to Homebrew and Scoop repos.
4. If Winget manifests were generated, open a PR to the configured Winget repo.

## Winget Requirements

- Windows `x64` asset is required whenever Winget manifests are generated.
- Windows `arm64` is optional. If present in the manifest, URL validation must pass.
- Validation is run in CI before the Winget update job.

## Required Secrets

- `WINGET_PKGS_TOKEN`: token with push/PR permissions for `${WINGET_PKGS_REPO}`.
- `SCOOP_BUCKET_TOKEN`: existing Scoop publication token.
- `HOMEBREW_TAP_TOKEN`: existing Homebrew publication token.

If `WINGET_PKGS_TOKEN` is absent, Winget publication is skipped.

## Winget Repo Target

Workflow env default:

- `WINGET_PKGS_REPO=process-triage/winget-pkgs`

This can be changed to another fork/repo. For direct upstream contribution,
maintainers can set it to `microsoft/winget-pkgs` if token permissions allow.

## Expected Winget Path Layout

For version `X.Y.Z`, manifests are written to:

- `manifests/p/ProcessTriage/pt/X.Y.Z/ProcessTriage.pt.yaml`
- `manifests/p/ProcessTriage/pt/X.Y.Z/ProcessTriage.pt.installer.yaml`
- `manifests/p/ProcessTriage/pt/X.Y.Z/ProcessTriage.pt.locale.en-US.yaml`

## Maintainer Checklist

1. Ensure release includes Windows x64 asset (arm64 optional).
2. Confirm `checksums.sha256` contains Windows entries.
3. Ensure `WINGET_PKGS_TOKEN` is configured.
4. Verify PR created by `Update Package Repos` workflow.
5. If using a fork/dedicated repo, open or update upstream Winget PR as needed.
