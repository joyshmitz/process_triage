# pt

<div align="center">

**Process Triage** — Bayesian-inspired zombie/abandoned process killer

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

</div>

`pt` is a CLI tool that identifies and kills abandoned processes on your system. It uses a Bayesian-inspired scoring engine with runtime evidence gathering, remembers your past decisions to improve future suggestions, and presents an interactive UI for reviewing and killing candidates.

<div align="center">
<h3>Quick Install</h3>

```bash
# Clone and add to PATH
git clone https://github.com/Dicklesworthstone/process_triage.git
ln -s $(pwd)/process_triage/pt ~/.local/bin/pt

# Or just run directly
./pt
```

<p><em>Works on Linux. Requires gum (auto-installs if missing).</em></p>
</div>

---

## TL;DR

**The Problem**: Long-running development machines accumulate zombie processes — stuck tests, abandoned dev servers, orphaned agent shells. These consume CPU and memory, and manually hunting them is tedious.

**The Solution**: `pt` estimates abandonment probability from multiple evidence sources, pre-selects the most suspicious ones for killing, and learns from your decisions to improve future suggestions.

### Why Use pt?

| Feature | What It Does |
|---------|--------------|
| **Smart Scoring** | Bayesian-inspired evidence engine for stuck tests, old dev servers, orphaned processes |
| **Learning Memory** | Remembers your kill/spare decisions for similar processes |
| **Pre-selection** | Most suspicious processes are pre-selected for quick review |
| **Interactive UI** | Gum-based interface with evidence and confidence indicators |
| **Safe by Default** | Confirms before killing, tries SIGTERM before SIGKILL |
| **System Aware** | Never flags system services (systemd, sshd, docker, etc.) |

### Quick Example

```bash
# Interactive mode (default) - scan, review, kill
pt

# Quick scan without killing
pt scan

# Deep scan with runtime evidence
pt deep

# View past decisions
pt history

# Clear learned decisions
pt clear
```

---

## How It Works

### Process Scoring

Each process receives a score based on multiple evidence sources:

| Factor | Score Impact | Rationale |
|--------|-------------|-----------|
| Process type prior | +10 to +25 | Test/dev/agent/build types start more suspicious |
| Age vs expected lifetime | +20 to +50 | Overdue relative to type lifetime |
| Absolute age | +15 to +40 | Old processes still get flagged |
| PPID = 1 (orphaned) | +35 | Parent died, likely abandoned |
| Stuck patterns | +30 to +35 | Known stuck tests or shells |
| High CPU or idle CPU | +20 to +30 | Suspected hang or spin |
| High memory + old | +15 to +20 | Resource hogs deserve attention |
| Past decisions | +30 kill / -40 spare | Bayesian update from history |
| System service | -200 | Never kill these |

### Recommendations

Based on score:

- **KILL** (score >= 60): Pre-selected for killing
- **REVIEW** (score 30-59): Worth checking
- **SPARE** (score < 30): Probably safe

### Decision Memory

When you kill or spare a process, `pt` remembers the pattern. For example, if you spare `gunicorn --workers 4`, similar gunicorn processes will be scored lower in the future.

Patterns are normalized (PIDs removed, ports generalized) so decisions apply across sessions.

---

## Commands

### `pt` or `pt run`

Interactive mode. Scans processes, presents candidates sorted by score, lets you select which to kill.

```bash
pt
pt run
```

### `pt deep`

Deep interactive scan. Uses runtime evidence (I/O, CPU progress, TTY state, children, network) for higher confidence.

```bash
pt deep
```

### `pt scan`

Scan-only mode. Shows candidates without killing. Useful for reviewing system state.

```bash
pt scan
```

### `pt scan deep`

Scan-only deep mode (evidence gathering, no killing).

```bash
pt scan deep
```

### `pt history`

Show past kill/spare decisions.

```bash
pt history
```

### `pt clear`

Clear all learned decisions (start fresh).

```bash
pt clear
```

### `pt help`

Show help message.

```bash
pt help
pt --help
pt -h
```

---

## Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PROCESS_TRIAGE_CONFIG` | `~/.config/process_triage` | Config directory |
| `XDG_CONFIG_HOME` | `~/.config` | XDG base (used if PROCESS_TRIAGE_CONFIG not set) |

### Files

| File | Purpose |
|------|---------|
| `~/.config/process_triage/decisions.json` | Learned kill/spare decisions |
| `~/.config/process_triage/priors.json` | Process type priors (reserved) |
| `~/.config/process_triage/triage.log` | Operation log |

---

## Process Detection Patterns

### Detected as Suspicious

- **Test runners**: `bun test`, `jest`, `vitest`, `pytest`, `cargo test`, `npm test`, `go test`, `rspec`, `mocha`
- **Dev servers**: `--hot`, `--watch`, `next dev`, `vite`, `webpack-dev`, `npm run dev`, `yarn dev`, `bun run dev`
- **Agent shells**: `claude`, `codex`, `gemini`, `copilot`, `cursor`, `aider`
- **Shell wrappers**: `shell-snapshot`, `/bin/sh -c`, `/bin/bash -c`, `zsh -c`
- **Builds**: `cargo build`, `npm run build`, `tsc`, `rustc`, `gcc`, `clang`, `make`
- **Orphaned processes**: PPID = 1

### Protected (Never Flagged)

- `systemd`, `dbus`, `pulseaudio`, `pipewire`
- `sshd`, `cron`, `docker`, `elasticsearch`, `postgres`, `redis`, `nginx`

---

## Dependencies

- **gum**: Charm's CLI component toolkit (auto-installs if missing)
- **jq**: JSON processor (optional, for decision memory)
- **bash**: Version 4.0+ (for arrays and mapfile)
- **standard utils**: `ps`, `kill`, `grep`, `awk`, `cut`, `sort`, `pgrep`, `who`, `ss`, `bc`

### Gum Installation

`pt` automatically installs gum if not found, supporting:

- apt (Debian/Ubuntu)
- brew (macOS/Linuxbrew)
- Direct binary download (fallback)

---

## Safety

### What pt Does

1. Uses SIGTERM first (graceful shutdown)
2. Only uses SIGKILL if SIGTERM fails
3. Requires confirmation before killing
4. Logs all operations
5. Saves decisions for future learning

### What pt Never Does

1. Kill system services (systemd, sshd, etc.)
2. Kill without confirmation
3. Modify any files outside its config directory

---

## Example Session

```
╔═══════════════════════════════════════════════╗
║                                               ║
║  Process Triage                               ║
║  Interactive zombie/abandoned process killer  ║
║                                               ║
╚═══════════════════════════════════════════════╝

  Load: 5.17 4.89 10.23 (64 cores) | Memory: 281Gi / 499Gi | Procs: 412

Found 7 candidate(s) for review:

KILL/REVIEW/SPARE reflect abandonment probability

[KILL]  85 PID:12345 11d 2048M TEST │ bun test --watch...
[KILL]  70 PID:23456 3d  512M  AGENT│ /bin/bash -c claude...
[REVIEW]35 PID:34567 26h 128M  DEV  │ next dev --port 3000...
[SPARE] 12 PID:45678 2h  64M   ???  │ gunicorn --workers 4...

> Select processes to KILL (space to toggle, enter to confirm)
```

---

## Troubleshooting

### "gum: command not found" after install

The auto-installer may need sudo. Run manually:

```bash
# Debian/Ubuntu
sudo apt update && sudo apt install gum

# Or download binary
curl -fsSL https://github.com/charmbracelet/gum/releases/download/v0.14.1/gum_0.14.1_linux_amd64.tar.gz | tar xz
sudo mv gum /usr/local/bin/
```

### No candidates found

If `pt scan` shows no candidates:

1. Check minimum age — by default, only processes > 1 hour are considered
2. Your system may be clean — congratulations!

### Decision memory not working

Ensure `jq` is installed:

```bash
sudo apt install jq  # or equivalent
```

---

## Origins & Authors

Created by **Jeffrey Emanuel** to tame the chaos of long-running development machines. Born from a session where 23 stuck `bun test` processes and a 31GB Hyprland instance brought a 64-core machine to its knees.

---

## License

MIT - see [LICENSE](LICENSE) for details.

---

Built with bash, gum, and hard-won frustration. `pt` is designed to keep your machine running smoothly without manual process hunting.
