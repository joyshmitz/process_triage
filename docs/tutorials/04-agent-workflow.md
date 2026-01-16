# Tutorial 04: Agent Workflow (Plan, Explain, Apply)

Goal: Run pt in a structured, automation-friendly way without taking unsafe actions.

## Current (implemented) workflow

Plan-only:

```bash
pt robot plan --format json > /tmp/pt-plan.json
```

Explain a candidate:

```bash
pt robot explain --pid <pid> --format json > /tmp/pt-explain.json
```

Apply (manual decision):

```bash
# Example only. Review evidence before applying.
pt robot apply --pids <pid> --yes --format json
```

## Planned session-based workflow

Session lifecycle is described in the agent contract and will enable resumable workflows:

```bash
# Planned interface (may not be implemented yet)
SESSION=$(pt agent plan --format json | jq -r .session_id)
pt agent explain --session "$SESSION" --pid <pid>
pt agent apply --session "$SESSION" --recommended --yes
pt agent verify --session "$SESSION"
pt agent diff --session "$SESSION" --vs <prior-session>
```

If the session-based commands are not available, keep using plan/explain/apply without sessions.
