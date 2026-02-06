#!/usr/bin/env bats

load "./test_helper/common.bash"

# Extract normalize_pattern without executing pt main.
normalize_cmd() {
    local input="$1"
    bash -c 'source <(sed -n "/^normalize_pattern()/,/^}/p" pt); normalize_pattern "$1"' _ "$input"
}

setup() {
    test_start "pattern normalization tests" "validate normalize_pattern behavior"
    skip "legacy bash normalize_pattern removed; normalization now lives in crates/pt-redact (canonicalize.rs) with Rust unit tests"
}

teardown() {
    test_end "pattern normalization tests" "pass"
}

@test "normalize_pattern removes 5+ digit PIDs" {
    run normalize_cmd "node 12345 server.js"
    [ "$status" -eq 0 ]
    [[ "$output" != *"12345"* ]]
}

@test "normalize_pattern removes 6 digit PIDs" {
    run normalize_cmd "process 123456 running"
    [ "$status" -eq 0 ]
    [[ "$output" != *"123456"* ]]
}

@test "normalize_pattern keeps 4 digit numbers unless port normalized" {
    run normalize_cmd "server on port 3000"
    [ "$status" -eq 0 ]
    [[ "$output" == *"3000"* ]] || [[ "$output" == *"PORT"* ]]
}

@test "normalize_pattern --port=3000 becomes --port=PORT" {
    run normalize_cmd "next dev --port=3000"
    [ "$status" -eq 0 ]
    [[ "$output" == *"--port=PORT"* ]]
}

@test "normalize_pattern --port 8080 becomes --port=PORT" {
    run normalize_cmd "server --port 8080"
    [ "$status" -eq 0 ]
    [[ "$output" == *"--port=PORT"* ]]
}

@test "normalize_pattern :8080 in URL becomes :PORT" {
    run normalize_cmd "http://localhost:8080/api"
    [ "$status" -eq 0 ]
    [[ "$output" == *":PORT"* ]]
}

@test "normalize_pattern standard UUID becomes UUID" {
    run normalize_cmd "process-550e8400-e29b-41d4-a716-446655440000-worker"
    [ "$status" -eq 0 ]
    [[ "$output" == *"UUID"* ]]
    [[ "$output" != *"550e8400"* ]]
}

@test "normalize_pattern temp path becomes /tmp/TMP" {
    run normalize_cmd "node /tmp/abc123xyz/server.js"
    [ "$status" -eq 0 ]
    [[ "$output" == *"/tmp/TMP"* ]]
}

@test "normalize_pattern collapses multiple spaces" {
    run normalize_cmd "command    with   many    spaces"
    [ "$status" -eq 0 ]
    [[ "$output" != *"  "* ]]
}

@test "normalize_pattern truncates to 150 chars" {
    local long_cmd
    long_cmd=$(printf 'a%.0s' {1..200})
    run normalize_cmd "$long_cmd"
    [ "$status" -eq 0 ]
    [ "${#output}" -le 150 ]
}

@test "normalize_pattern complex command fully normalized" {
    local cmd
    cmd="bun test --port=3000 /tmp/test123/spec 12345678 550e8400-e29b-41d4-a716-446655440000"
    run normalize_cmd "$cmd"
    [ "$status" -eq 0 ]
    [[ "$output" == *"--port=PORT"* ]]
    [[ "$output" == *"/tmp/TMP"* ]]
    [[ "$output" != *"12345678"* ]]
    [[ "$output" == *"UUID"* ]]
}

@test "normalize_pattern identical processes with different PIDs match" {
    local pattern1 pattern2
    pattern1=$(normalize_cmd "bun test --watch pid:12345")
    pattern2=$(normalize_cmd "bun test --watch pid:67890")
    [ "$pattern1" = "$pattern2" ]
}

@test "normalize_pattern identical processes with different ports match" {
    local pattern1 pattern2
    pattern1=$(normalize_cmd "next dev --port=3000")
    pattern2=$(normalize_cmd "next dev --port=8080")
    [ "$pattern1" = "$pattern2" ]
}
