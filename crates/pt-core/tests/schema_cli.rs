//! CLI tests for `pt-core schema` command behavior.

use assert_cmd::cargo::cargo_bin_cmd;
use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;
use std::time::Duration;

fn pt_core() -> Command {
    let mut cmd = cargo_bin_cmd!("pt-core");
    cmd.timeout(Duration::from_secs(60));
    cmd
}

#[test]
fn schema_list_json_returns_type_array() {
    let output = pt_core()
        .args(["--format", "json", "schema", "--list"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: Value = serde_json::from_slice(&output).expect("schema --list should return JSON");
    let types = json
        .as_array()
        .expect("schema --list --format json should return an array");

    assert!(!types.is_empty(), "schema type list should not be empty");
    assert!(
        types.iter().any(|entry| entry["name"] == "Plan"),
        "schema list should include Plan type"
    );
    assert!(
        types.iter().any(|entry| entry["name"] == "DecisionOutcome"),
        "schema list should include DecisionOutcome type"
    );
}

#[test]
fn schema_list_jsonl_is_line_delimited() {
    let output = pt_core()
        .args(["--format", "jsonl", "schema", "--list"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).expect("stdout should be utf-8");
    let mut line_count = 0usize;
    let mut saw_plan = false;

    for line in stdout.lines().filter(|line| !line.trim().is_empty()) {
        line_count += 1;
        let entry: Value =
            serde_json::from_str(line).expect("each jsonl line should be valid JSON");
        assert!(
            entry.get("name").is_some(),
            "jsonl list entry should contain name field"
        );
        assert!(
            entry.get("description").is_some(),
            "jsonl list entry should contain description field"
        );
        if entry["name"] == "Plan" {
            saw_plan = true;
        }
    }

    assert!(
        line_count > 10,
        "expected many schema entries, got {}",
        line_count
    );
    assert!(saw_plan, "jsonl list should contain Plan entry");
}

#[test]
fn schema_all_json_returns_schema_map() {
    let output = pt_core()
        .args(["--format", "json", "schema", "--all"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: Value = serde_json::from_slice(&output).expect("schema --all should return JSON");
    let map = json
        .as_object()
        .expect("schema --all --format json should return object map");

    assert!(map.contains_key("Plan"), "schema map should include Plan");
    assert!(
        map.contains_key("DecisionOutcome"),
        "schema map should include DecisionOutcome"
    );
    assert!(
        map["Plan"].get("$schema").is_some() || map["Plan"].get("type").is_some(),
        "Plan schema should contain JSON Schema top-level keys"
    );
}

#[test]
fn schema_all_jsonl_returns_one_object_per_line() {
    let output = pt_core()
        .args(["--format", "jsonl", "schema", "--all"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).expect("stdout should be utf-8");
    let mut line_count = 0usize;
    let mut saw_plan = false;

    for line in stdout.lines().filter(|line| !line.trim().is_empty()) {
        line_count += 1;
        let entry: Value =
            serde_json::from_str(line).expect("each jsonl line should be valid JSON");
        assert!(
            entry.get("type").is_some(),
            "jsonl schema entry should contain type field"
        );
        assert!(
            entry.get("schema").is_some(),
            "jsonl schema entry should contain schema field"
        );
        if entry["type"] == "Plan" {
            saw_plan = true;
        }
    }

    assert!(
        line_count > 10,
        "expected many schema entries, got {}",
        line_count
    );
    assert!(saw_plan, "jsonl schema entries should include Plan");
}

#[test]
fn schema_single_type_jsonl_is_single_line_json_object() {
    let output = pt_core()
        .args(["--format", "jsonl", "schema", "Plan"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).expect("stdout should be utf-8");
    let lines: Vec<&str> = stdout
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    assert_eq!(lines.len(), 1, "jsonl single schema should be one line");

    let schema: Value =
        serde_json::from_str(lines[0]).expect("single schema line should be valid JSON");
    assert!(
        schema.get("$schema").is_some() || schema.get("type").is_some(),
        "single schema output should look like JSON Schema"
    );
}

#[test]
fn schema_unknown_type_returns_error_with_hint() {
    pt_core()
        .args(["schema", "DefinitelyUnknownType"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Unknown type: DefinitelyUnknownType",
        ))
        .stderr(predicate::str::contains("pt schema --list"));
}
