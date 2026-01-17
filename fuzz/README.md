# Fuzz Testing for Process Triage

This directory contains fuzz targets for security-critical parsing code in process triage.

## Prerequisites

Install cargo-fuzz:

```bash
cargo install cargo-fuzz
```

You need a nightly Rust toolchain for libfuzzer:

```bash
rustup install nightly
```

## Available Targets

### Proc Parsers (`/proc/[pid]/*`)

| Target | File Parsed | Description |
|--------|------------|-------------|
| `fuzz_proc_stat` | `/proc/[pid]/stat` | Process status with comm field |
| `fuzz_proc_io` | `/proc/[pid]/io` | I/O statistics |
| `fuzz_proc_statm` | `/proc/[pid]/statm` | Memory statistics |
| `fuzz_proc_schedstat` | `/proc/[pid]/schedstat` | Scheduler statistics |
| `fuzz_proc_sched` | `/proc/[pid]/sched` | Scheduler info |
| `fuzz_proc_cgroup` | `/proc/[pid]/cgroup` | Cgroup membership |
| `fuzz_proc_environ` | `/proc/[pid]/environ` | Environment variables (binary) |

### Network Parsers (`/proc/net/*`)

| Target | File Parsed | Description |
|--------|------------|-------------|
| `fuzz_network_tcp` | `/proc/net/tcp[6]` | TCP connection table |
| `fuzz_network_udp` | `/proc/net/udp[6]` | UDP socket table |
| `fuzz_network_unix` | `/proc/net/unix` | Unix domain sockets |

### Configuration Loading

| Target | File Type | Description |
|--------|----------|-------------|
| `fuzz_config_priors` | `priors.json` | Bayesian prior configuration |
| `fuzz_config_policy` | `policy.json` | Safety policy configuration |

### Bundle Reader

| Target | File Type | Description |
|--------|----------|-------------|
| `fuzz_bundle_reader` | `.ptb` | Session bundle (ZIP format) |

## Running Fuzz Tests

### Quick Run (Discovery Mode)

Run a target for 60 seconds:

```bash
cargo +nightly fuzz run fuzz_proc_stat -- -max_total_time=60
```

### Using Corpus

Run with the pre-populated corpus:

```bash
cargo +nightly fuzz run fuzz_proc_stat corpus/proc_stat/
```

### List All Targets

```bash
cargo +nightly fuzz list
```

### Coverage Report

Generate coverage report:

```bash
cargo +nightly fuzz coverage fuzz_proc_stat
```

## CI Integration

For CI, run each target in time-limited mode:

```bash
#!/bin/bash
set -e

TARGETS=$(cargo +nightly fuzz list)
for target in $TARGETS; do
    echo "Fuzzing $target for 60 seconds..."
    cargo +nightly fuzz run "$target" -- -max_total_time=60 || {
        echo "CRASH FOUND in $target!"
        exit 1
    }
done
echo "All fuzz targets passed!"
```

## Corpus

The `corpus/` directory contains seed inputs for each target:

- `corpus/proc_stat/` - Sample /proc/[pid]/stat content
- `corpus/proc_io/` - Sample /proc/[pid]/io content
- `corpus/proc_statm/` - Sample /proc/[pid]/statm content
- `corpus/network_tcp/` - Sample /proc/net/tcp content
- `corpus/network_unix/` - Sample /proc/net/unix content
- `corpus/config_priors/` - Sample priors.json files
- `corpus/config_policy/` - Sample policy.json files

## Handling Crashes

If fuzzing finds a crash:

1. The crashing input is saved in `artifacts/<target>/crash-*`
2. Reproduce with: `cargo +nightly fuzz run <target> artifacts/<target>/crash-*`
3. Create a regression test from the crash
4. Fix the parsing code to handle the edge case

## Security Notes

These parsers handle untrusted input:

- `/proc/*` files: Content controlled by kernel, but format may vary
- Config files: User-provided JSON, must handle malformed input
- Bundle files: May come from untrusted sources, must validate completely

All parsers should:
- Never panic on malformed input
- Return `None` or `Err` for invalid data
- Have bounded memory usage (no unbounded allocations)
- Have bounded time (no infinite loops)
