#!/usr/bin/env bats

setup() {
    TEST_DIR="$( cd "$( dirname "$BATS_TEST_FILENAME" )" && pwd )"
    PROJECT_ROOT="$(dirname "$TEST_DIR")"
    export PROJECT_ROOT
}

@test "VERSION file matches pt script version" {
    run bash -c 'file_version=$(cat "$PROJECT_ROOT/VERSION"); script_version=$(grep "^readonly VERSION=\"" "$PROJECT_ROOT/pt" | cut -d"\"" -f2); [ "$file_version" = "$script_version" ]'
    [ "$status" -eq 0 ]
}
