#!/usr/bin/env bats

load "./test_helper/common.bash"

@test "test helper creates env and mock command" {
    test_start "test helper creates env and mock command" "ensure setup and mocks work"
    setup_test_env

    [ -d "$CONFIG_DIR" ]
    [ -d "$MOCK_BIN" ]
    [ -f "$CONFIG_DIR/decisions.json" ]

    create_mock_command "hello" "world" 0
    [ -x "$MOCK_BIN/hello" ]
    run "$MOCK_BIN/hello"
    [ "$status" -eq 0 ]
    [ "$output" = "world" ]

    test_end "test helper creates env and mock command" "pass"
}
