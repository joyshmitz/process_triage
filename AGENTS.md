# AGENTS.md — Process Triage (pt)

## RULE 1 – ABSOLUTE (DO NOT EVER VIOLATE THIS)

You may NOT delete any file or directory unless I explicitly give the exact command **in this session**.

- This includes files you just created (tests, tmp files, scripts, etc.).
- You do not get to decide that something is "safe" to remove.
- If you think something should be removed, stop and ask. You must receive clear written approval **before** any deletion command is even proposed.

Treat "never delete files without permission" as a hard invariant.

---

## Project Overview

`pt` (Process Triage) is an interactive CLI for identifying and killing abandoned/zombie processes. It uses heuristics to score processes and learns from user decisions.

### Architecture

```
process_triage/
├── pt                  # Main executable (single file)
├── test/
│   └── pt.bats         # BATS test suite
├── README.md
├── AGENTS.md
└── LICENSE
```

### Key Design Decisions

1. **Bash-first**: No compiled dependencies, runs anywhere
2. **gum for UI**: Beautiful, consistent interactive components
3. **Learning memory**: JSON-based decision history with pattern normalization
4. **Safety by default**: Never auto-kills, always confirms, protects system services

---

## Development Guidelines

### Code Style

- Use `shellcheck` for all bash code
- Prefer `[[` over `[` for conditionals
- Quote all variables: `"$var"` not `$var`
- Use `local` for function variables
- Prefer `printf` over `echo` for portable output

### Testing

Tests use BATS (Bash Automated Testing System):

```bash
# Run all tests
bats test/

# Run specific test file
bats test/pt.bats

# Verbose output
bats --verbose-run test/
```

### Adding New Features

1. Write BATS tests first
2. Implement in appropriate lib file
3. Export function if needed in bin/pt
4. Update README if user-facing
5. Run `shellcheck bin/pt lib/*.sh`

---

## pt Quick Reference for AI Agents

Interactive process killer that identifies zombies using heuristics and learns from decisions.

### Core Commands

```bash
pt              # Interactive mode - scan, select, kill
pt scan         # Scan only - show candidates without killing
pt history      # Show past decisions
pt clear        # Clear decision memory
pt help         # Show help
```

### Key Concepts

| Concept | Description |
|---------|-------------|
| **Score** | 0-100+ rating of how suspicious a process is |
| **KILL** | Score >= 50, pre-selected for killing |
| **REVIEW** | Score 20-49, worth checking |
| **SPARE** | Score < 20, probably safe |
| **Pattern** | Normalized command used for learning |

### Process Detection

Detected as suspicious:
- Test runners (`bun test`, `jest`, `pytest`) running > 1 hour
- Dev servers (`next dev`, `vite`, `--hot`) running > 2 days
- Agent shells (`claude`, `codex`) running > 1 day
- Any orphaned process (PPID = 1)
- High memory + long age

Protected (never flagged):
- `systemd`, `dbus`, `sshd`, `cron`, `docker`

### Configuration

| File | Purpose |
|------|---------|
| `~/.config/process_triage/decisions.json` | Learned decisions |
| `~/.config/process_triage/triage.log` | Operation log |

### Safety Notes

- Always tries SIGTERM before SIGKILL
- Requires confirmation before any kill
- Logs all operations
- Never kills system services

---

## Troubleshooting

### Common Issues

**gum not found**: Run `pt` and it will auto-install, or:
```bash
sudo apt install gum  # Debian/Ubuntu
brew install gum      # macOS
```

**No candidates found**: System may be clean, or minimum age (1 hour) not reached.

**Decision memory not working**: Install jq:
```bash
sudo apt install jq
```

### Debug Mode

Set `PT_DEBUG=1` to see verbose output:
```bash
PT_DEBUG=1 pt scan
```

---

## Contributing to This Codebase

When modifying `pt`:

1. **Test changes**: Run `bats test/` before committing
2. **Check with shellcheck**: `shellcheck bin/pt lib/*.sh`
3. **Update docs**: If changing user-facing behavior, update README
4. **Preserve safety**: Never remove confirmation prompts or protected patterns

---

## MCP Agent Mail — Multi-Agent Coordination

A mail-like layer that lets coding agents coordinate asynchronously via MCP tools and resources. Provides identities, inbox/outbox, searchable threads, and advisory file reservations with human-auditable artifacts in Git.

### Why It's Useful

- **Prevents conflicts:** Explicit file reservations (leases) for files/globs
- **Token-efficient:** Messages stored in per-project archive, not in context
- **Quick reads:** `resource://inbox/...`, `resource://thread/...`

### Same Repository Workflow

1. **Register identity:**
   ```
   ensure_project(project_key=<abs-path>)
   register_agent(project_key, program, model)
   ```

2. **Reserve files before editing:**
   ```
   file_reservation_paths(project_key, agent_name, ["pt", "lib/**"], ttl_seconds=3600, exclusive=true)
   ```

3. **Communicate with threads:**
   ```
   send_message(..., thread_id="FEAT-123")
   fetch_inbox(project_key, agent_name)
   acknowledge_message(project_key, agent_name, message_id)
   ```

### Macros vs Granular Tools

- **Prefer macros for speed:** `macro_start_session`, `macro_prepare_thread`, `macro_file_reservation_cycle`, `macro_contact_handshake`
- **Use granular tools for control:** `register_agent`, `file_reservation_paths`, `send_message`, `fetch_inbox`, `acknowledge_message`

### Common Pitfalls

- `"from_agent not registered"`: Always `register_agent` in the correct `project_key` first
- `"FILE_RESERVATION_CONFLICT"`: Adjust patterns, wait for expiry, or use non-exclusive reservation

---

## Beads (br) — Dependency-Aware Issue Tracking

Beads provides a lightweight, dependency-aware issue database and CLI (`br`) for selecting "ready work," setting priorities, and tracking status.

**Note:** `br` (beads_rust) is non-invasive and never executes git commands directly. You must manually run git operations after `br sync --flush-only`.

### Typical Agent Flow

1. **Pick ready work (Beads):**
   ```bash
   br ready --json  # Choose highest priority, no blockers
   ```

2. **Reserve edit surface (Mail):**
   ```
   file_reservation_paths(project_key, agent_name, ["pt"], ttl_seconds=3600, exclusive=true, reason="br-123")
   ```

3. **Announce start (Mail):**
   ```
   send_message(..., thread_id="br-123", subject="[br-123] Start: <title>", ack_required=true)
   ```

4. **Work and update:** Reply in-thread with progress

5. **Complete and release:**
   ```bash
   br close br-123 --reason "Completed"
   ```
   ```
   release_file_reservations(project_key, agent_name, paths=["pt"])
   ```

### Mapping Cheat Sheet

| Concept | Value |
|---------|-------|
| Mail `thread_id` | `br-###` |
| Mail subject | `[br-###] ...` |
| File reservation `reason` | `br-###` |
| Commit messages | Include `br-###` for traceability |

---

## bv — Graph-Aware Triage Engine

bv is a graph-aware triage engine for Beads projects (`.beads/beads.jsonl`). It computes PageRank, betweenness, critical path, cycles, HITS, eigenvector, and k-core metrics deterministically.

**CRITICAL: Use ONLY `--robot-*` flags. Bare `bv` launches an interactive TUI that blocks your session.**

### The Workflow: Start With Triage

**`bv --robot-triage` is your single entry point.** It returns:
- `quick_ref`: at-a-glance counts + top 3 picks
- `recommendations`: ranked actionable items with scores, reasons, unblock info
- `quick_wins`: low-effort high-impact items
- `blockers_to_clear`: items that unblock the most downstream work
- `project_health`: status/type/priority distributions, graph metrics
- `commands`: copy-paste shell commands for next steps

```bash
bv --robot-triage        # THE MEGA-COMMAND: start here
bv --robot-next          # Minimal: just the single top pick + claim command
```

### Command Reference

**Planning:**
| Command | Returns |
|---------|---------|
| `--robot-plan` | Parallel execution tracks with `unblocks` lists |
| `--robot-priority` | Priority misalignment detection with confidence |

**Graph Analysis:**
| Command | Returns |
|---------|---------|
| `--robot-insights` | Full metrics: PageRank, betweenness, HITS, eigenvector, critical path, cycles |
| `--robot-diff --diff-since <ref>` | Changes since ref: new/closed/modified issues, cycles |

### jq Quick Reference

```bash
bv --robot-triage | jq '.quick_ref'                        # At-a-glance summary
bv --robot-triage | jq '.recommendations[0]'               # Top recommendation
bv --robot-plan | jq '.plan.summary.highest_impact'        # Best unblock target
bv --robot-insights | jq '.Cycles'                         # Circular deps (must fix!)
```

---

## UBS — Ultimate Bug Scanner

**Golden Rule:** `ubs <changed-files>` before every commit. Exit 0 = safe. Exit >0 = fix & re-run.

### Commands

```bash
ubs pt                                  # Main script (< 1s)
ubs $(git diff --name-only --cached)    # Staged files — before commit
ubs --only=bash .                       # Bash files only
ubs .                                   # Whole project
```

### Output Format

```
⚠️  Category (N errors)
    pt:42:5 – Issue description
    💡 Suggested fix
Exit code: 1
```

Parse: `file:line:col` → location | 💡 → how to fix | Exit 0/1 → pass/fail

### Fix Workflow

1. Read finding → category + fix suggestion
2. Navigate `file:line:col` → view context
3. Verify real issue (not false positive)
4. Fix root cause (not symptom)
5. Re-run `ubs <file>` → exit 0
6. Commit

---

## ast-grep vs ripgrep

**Use `ast-grep` when structure matters.** It parses code and matches AST nodes, ignoring comments/strings, and can **safely rewrite** code.

**Use `ripgrep` when text is enough.** Fastest way to grep literals/regex.

### Rule of Thumb

- Need correctness or **applying changes** → `ast-grep`
- Need raw speed or **hunting text** → `rg`
- Often combine: `rg` to shortlist files, then `ast-grep` to match/modify

### Bash Examples

```bash
# Find all function definitions
rg -n '^[a-z_]+\s*\(\)' pt

# Find all gum usage
rg -n 'gum ' -t bash

# Quick textual hunt
rg -n 'TODO\|FIXME' .
```

---

## Morph Warp Grep — AI-Powered Code Search

**Use `mcp__morph-mcp__warp_grep` for exploratory "how does X work?" questions.** An AI agent expands your query, greps the codebase, reads relevant files, and returns precise line ranges with full context.

**Use `ripgrep` for targeted searches.** When you know exactly what you're looking for.

### When to Use What

| Scenario | Tool | Why |
|----------|------|-----|
| "How is process scoring implemented?" | `warp_grep` | Exploratory; don't know where to start |
| "Where is the kill confirmation logic?" | `warp_grep` | Need to understand architecture |
| "Find all uses of `gum confirm`" | `ripgrep` | Targeted literal search |

### warp_grep Usage

```
mcp__morph-mcp__warp_grep(
  repoPath: "/path/to/process_triage",
  query: "How does the learning memory work?"
)
```

Returns structured results with file paths, line ranges, and extracted code snippets.

### Anti-Patterns

- **Don't** use `warp_grep` to find a specific function name → use `ripgrep`
- **Don't** use `ripgrep` to understand "how does X work" → wastes time with manual reads

---

## cass — Cross-Agent Session Search

`cass` indexes prior agent conversations (Claude Code, Codex, Cursor, Gemini, ChatGPT, Aider, etc.) into a unified, searchable index so you can reuse solved problems.

**NEVER run bare `cass`** — it launches an interactive TUI. Always use `--robot` or `--json`.

### Quick Start

```bash
# Check if index is healthy (exit 0=ok, 1=run index first)
cass health

# Search across all agent histories
cass search "bash process killing" --robot --limit 5

# View a specific result (from search output)
cass view /path/to/session.jsonl -n 42 --json

# Expand context around a line
cass expand /path/to/session.jsonl -n 42 -C 3 --json

# Learn the full API
cass capabilities --json      # Feature discovery
cass robot-docs guide         # LLM-optimized docs
```

### Key Flags

| Flag | Purpose |
|------|---------|
| `--robot` / `--json` | Machine-readable JSON output (required!) |
| `--fields minimal` | Reduce payload: `source_path`, `line_number`, `agent` only |
| `--limit N` | Cap result count |
| `--agent NAME` | Filter to specific agent (claude, codex, cursor, etc.) |
| `--days N` | Limit to recent N days |

**stdout = data only, stderr = diagnostics. Exit 0 = success.**

### Pre-Flight Health Check

```bash
cass health --json
```

Returns in <50ms:
- **Exit 0:** Healthy—proceed with queries
- **Exit 1:** Unhealthy—run `cass index --full` first

### Exit Codes

| Code | Meaning | Retryable |
|------|---------|-----------|
| 0 | Success | N/A |
| 1 | Health check failed | Yes—run `cass index --full` |
| 2 | Usage/parsing error | No—fix syntax |
| 3 | Index/DB missing | Yes—run `cass index --full` |

Treat cass as a way to avoid re-solving problems other agents already handled.

<!-- bv-agent-instructions-v1 -->

---

## Beads Workflow Integration

This project uses [beads_rust](https://github.com/Dicklesworthstone/beads_rust) for issue tracking. Issues are stored in `.beads/` and tracked in git.

**Note:** `br` (beads_rust) is non-invasive and never executes git commands directly. You must manually run git operations after `br sync --flush-only`.

### Essential Commands

```bash
# CLI commands for agents
br ready              # Show issues ready to work (no blockers)
br list --status=open # All open issues
br show <id>          # Full issue details with dependencies
br create --title="..." --type=task --priority=2
br update <id> --status=in_progress
br close <id> --reason="Completed"
br close <id1> <id2>  # Close multiple issues at once
br sync --flush-only  # Export to JSONL (does NOT run git commands)
```

### Workflow Pattern

1. **Start**: Run `br ready` to find actionable work
2. **Claim**: Use `br update <id> --status=in_progress`
3. **Work**: Implement the task
4. **Complete**: Use `br close <id>`
5. **Sync**: Run `br sync --flush-only` then manually: `git add .beads/ && git commit -m "Update beads" && git push`

### Key Concepts

- **Dependencies**: Issues can block other issues. `br ready` shows only unblocked work.
- **Priority**: P0=critical, P1=high, P2=medium, P3=low, P4=backlog (use numbers, not words)
- **Types**: task, bug, feature, epic, question, docs
- **Blocking**: `br dep add <issue> <depends-on>` to add dependencies

<!-- end-bv-agent-instructions -->

## Landing the Plane (Session Completion)

**When ending a work session**, you MUST complete ALL steps below. Work is NOT complete until `git push` succeeds.

**MANDATORY WORKFLOW:**

1. **File issues for remaining work** - Create issues for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **PUSH TO REMOTE** - This is MANDATORY:
   ```bash
   git pull --rebase
   br sync --flush-only  # Export to JSONL (does NOT run git commands)
   git add .beads/ && git commit -m "Update beads" && git push
   git status  # MUST show "up to date with origin"
   ```
5. **Clean up** - Clear stashes, prune remote branches
6. **Verify** - All changes committed AND pushed
7. **Hand off** - Provide context for next session

**CRITICAL RULES:**
- Work is NOT complete until `git push` succeeds
- NEVER stop before pushing - that leaves work stranded locally
- NEVER say "ready to push when you are" - YOU must push
- If push fails, resolve and retry until it succeeds


---

Note for Codex/GPT-5.2:

You constantly bother me and stop working with concerned questions that look similar to this:

```
Unexpected changes (need guidance)

- Working tree still shows edits I did not make in Cargo.toml, Cargo.lock, src/cli/commands/upgrade.rs, src/storage/sqlite.rs, tests/conformance.rs, tests/storage_deps.rs. Please advise whether to keep/commit/revert these before any further work. I did not touch them.

Next steps (pick one)

1. Decide how to handle the unrelated modified files above so we can resume cleanly.
2. Triage beads_rust-orko (clippy/cargo warnings) and beads_rust-ydqr (rustfmt failures).
3. If you want a full suite run later, fix conformance/clippy blockers and re‑run cargo test --all.
```

NEVER EVER DO THAT AGAIN. The answer is literally ALWAYS the same: those are changes created by the potentially dozen of other agents working on the project at the same time. This is not only a common occurence, it happens multiple times PER MINUTE. The way to deal with it is simple: you NEVER, under ANY CIRCUMSTANCE, stash, revert, overwrite, or otherwise disturb in ANY way the work of other agents. Just treat those changes identically to changes that you yourself made. Just fool yourself into think YOU made the changes and simply don't recall it for some reason.
