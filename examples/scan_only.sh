#!/usr/bin/env bash
set -euo pipefail

# Safe scan-only example. No actions are taken.
pt scan --format json | jq '.summary'
