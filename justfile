# Process Triage (pt) development tasks
# Install just: cargo install just

# Default recipe: show available commands
default:
    @just --list

# Build pt-core in release mode
build:
    cargo build --release -p pt-core

# Build with TUI support
build-ui:
    cargo build --release -p pt-core --features ui

# Build static musl binary (Linux only)
build-static:
    cargo build --release -p pt-core --target x86_64-unknown-linux-musl

# Run all workspace tests
test:
    cargo test --workspace

# Run pt-core tests only
test-core:
    cargo test -p pt-core

# Run BATS integration tests
test-bats:
    bats test/

# Run benchmarks
bench:
    cargo bench -p pt-core

# Check compilation (fast, no codegen)
check:
    cargo check --workspace

# Lint with clippy
clippy:
    cargo clippy --workspace -- -W clippy::all

# Format code
fmt:
    cargo fmt --all

# Format check (CI mode)
fmt-check:
    cargo fmt --all -- --check

# Generate shell completions
completions-bash:
    cargo run -p pt-core -- completions bash > completions/pt-core.bash

completions-zsh:
    cargo run -p pt-core -- completions zsh > completions/_pt-core

completions-fish:
    cargo run -p pt-core -- completions fish > completions/pt-core.fish

# Generate all shell completions
completions: completions-bash completions-zsh completions-fish
    @echo "Shell completions generated in completions/"

# Generate man pages
manpages:
    ./scripts/gen_manpages.sh

# Build Docker image
docker:
    docker build -t pt .

# Run Docker container scanning host processes
docker-scan:
    docker run --rm --pid=host pt scan

# Clean build artifacts
clean:
    cargo clean
