#!/usr/bin/env bash
set -euo pipefail

# Safe plan-only example. No actions are taken.
pt robot plan --format json | jq '.summary'
