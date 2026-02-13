#!/usr/bin/env bats

setup() {
    TEST_DIR="$( cd "$( dirname "$BATS_TEST_FILENAME" )" && pwd )"
    PROJECT_ROOT="$(dirname "$TEST_DIR")"
    export PROJECT_ROOT
}

@test "pt --version succeeds" {
    run "$PROJECT_ROOT/pt" --version
    [ "$status" -eq 0 ]
    [[ "$output" == *"pt "* ]]
}

@test "pt --help succeeds" {
    run "$PROJECT_ROOT/pt" --help
    [ "$status" -eq 0 ]
    [[ "$output" == *"Usage:"* ]]
}

@test "pt scan command is exposed" {
    run "$PROJECT_ROOT/pt" --help
    [ "$status" -eq 0 ]
    [[ "$output" == *"scan"* ]]
}
