#!/usr/bin/env python3
"""Validate E2E artifact manifests (schema, files, checksums)."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from copy import deepcopy
from datetime import datetime
from pathlib import Path
from typing import Any, Dict, List

SCHEMA_RELATIVE = Path("specs/schemas/e2e-artifact-manifest.schema.json")
VERSION_RE = re.compile(r"^(\d+)\.(\d+)\.(\d+)$")
SHA256_RE = re.compile(r"^[a-f0-9]{64}$")

LOG_KINDS = {"jsonl", "text", "tap", "stdout", "stderr"}
ARTIFACT_KINDS = {
    "snapshot",
    "plan",
    "telemetry",
    "bundle",
    "report",
    "daemon",
    "install",
    "tui",
    "manifest",
    "other",
}

REDACTION_REQUIRED_KINDS = {
    "snapshot",
    "plan",
    "telemetry",
    "bundle",
    "report",
}


def load_json(path: Path) -> Dict[str, Any]:
    try:
        with path.open("r", encoding="utf-8") as handle:
            return json.load(handle)
    except FileNotFoundError:
        raise SystemExit(f"manifest not found: {path}")
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON in {path}: {exc}")


def canonical_manifest_bytes(manifest: Dict[str, Any]) -> bytes:
    clone = deepcopy(manifest)
    clone.pop("manifest_sha256", None)
    canonical = json.dumps(clone, sort_keys=True, separators=(",", ":"), ensure_ascii=False)
    return canonical.encode("utf-8")


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def validate_schema_with_jsonschema(manifest: Dict[str, Any], schema_path: Path, errors: List[str]) -> None:
    try:
        import jsonschema  # type: ignore
    except Exception:
        return

    try:
        schema = load_json(schema_path)
        jsonschema.Draft202012Validator(schema).validate(manifest)
    except jsonschema.ValidationError as exc:
        errors.append(f"schema validation failed: {exc.message}")
    except Exception as exc:  # pragma: no cover - unexpected
        errors.append(f"schema validation error: {exc}")


def parse_rfc3339(value: str) -> bool:
    try:
        if value.endswith("Z"):
            value = value[:-1] + "+00:00"
        datetime.fromisoformat(value)
        return True
    except ValueError:
        return False


def require_field(manifest: Dict[str, Any], field: str, errors: List[str]) -> None:
    if field not in manifest:
        errors.append(f"missing required field: {field}")


def validate_version(version: str, errors: List[str]) -> None:
    match = VERSION_RE.match(version)
    if not match:
        errors.append(f"schema_version not semver: {version}")
        return
    major = int(match.group(1))
    if major != 1:
        errors.append(f"unsupported schema_version major: {version}")


def validate_file_entry(entry: Dict[str, Any], base_dir: Path, label: str, errors: List[str]) -> None:
    path_value = entry.get("path")
    if not isinstance(path_value, str) or not path_value:
        errors.append(f"{label} entry missing path")
        return
    path = Path(path_value)
    if not path.is_absolute():
        path = base_dir / path

    if not path.exists():
        errors.append(f"{label} file missing: {path}")
        return

    expected_hash = entry.get("sha256")
    expected_bytes = entry.get("bytes")

    actual_bytes = path.stat().st_size
    actual_hash = sha256_file(path)

    if not isinstance(expected_bytes, int):
        errors.append(f"{label} bytes missing for {path_value}")
    elif expected_bytes != actual_bytes:
        errors.append(
            f"{label} bytes mismatch for {path_value}: expected {expected_bytes}, got {actual_bytes}"
        )

    if not isinstance(expected_hash, str) or not SHA256_RE.match(expected_hash):
        errors.append(f"{label} sha256 invalid for {path_value}")
    elif expected_hash != actual_hash:
        errors.append(
            f"{label} sha256 mismatch for {path_value}: expected {expected_hash}, got {actual_hash}"
        )


def validate_jsonl_run_id(entry: Dict[str, Any], base_dir: Path, run_id: str, errors: List[str]) -> None:
    path_value = entry.get("path")
    if not isinstance(path_value, str) or not path_value:
        return
    path = Path(path_value)
    if not path.is_absolute():
        path = base_dir / path
    if not path.exists():
        return

    with path.open("r", encoding="utf-8") as handle:
        for idx, line in enumerate(handle, start=1):
            line = line.strip()
            if not line:
                continue
            try:
                payload = json.loads(line)
            except json.JSONDecodeError:
                errors.append(f"jsonl parse error in {path_value} line {idx}")
                continue
            if payload.get("run_id") != run_id:
                errors.append(f"jsonl run_id mismatch in {path_value} line {idx}")


def validate_manifest(manifest: Dict[str, Any], manifest_path: Path, schema_path: Path) -> List[str]:
    errors: List[str] = []

    validate_schema_with_jsonschema(manifest, schema_path, errors)

    for field in [
        "schema_version",
        "run_id",
        "suite",
        "test_id",
        "timestamp",
        "env",
        "commands",
        "logs",
        "artifacts",
        "metrics",
        "manifest_sha256",
    ]:
        require_field(manifest, field, errors)

    schema_version = manifest.get("schema_version")
    if isinstance(schema_version, str):
        validate_version(schema_version, errors)
    else:
        errors.append("schema_version must be string")

    if not isinstance(manifest.get("run_id"), str):
        errors.append("run_id must be string")
    if not isinstance(manifest.get("suite"), str):
        errors.append("suite must be string")
    if not isinstance(manifest.get("test_id"), str):
        errors.append("test_id must be string")

    if isinstance(manifest.get("timestamp"), str):
        if not parse_rfc3339(manifest["timestamp"]):
            errors.append("timestamp not RFC3339")
    else:
        errors.append("timestamp must be string")

    env = manifest.get("env")
    if isinstance(env, dict):
        for field in ("os", "arch", "kernel", "ci_provider"):
            if not isinstance(env.get(field), str):
                errors.append(f"env.{field} must be string")
    else:
        errors.append("env must be object")

    manifest_sha256 = manifest.get("manifest_sha256")
    if isinstance(manifest_sha256, str):
        if not SHA256_RE.match(manifest_sha256):
            errors.append("manifest_sha256 not sha256 hex")
        computed_hash = sha256_bytes(canonical_manifest_bytes(manifest))
        if manifest_sha256 != computed_hash:
            errors.append(
                f"manifest_sha256 mismatch: expected {manifest_sha256}, got {computed_hash}"
            )
    else:
        errors.append("manifest_sha256 must be string")

    run_id = manifest.get("run_id") if isinstance(manifest.get("run_id"), str) else ""
    base_dir = manifest_path.parent

    commands = manifest.get("commands")
    if isinstance(commands, list) and commands:
        for idx, entry in enumerate(commands):
            if not isinstance(entry, dict):
                errors.append(f"commands[{idx}] must be object")
                continue
            argv = entry.get("argv")
            if not isinstance(argv, list) or not argv or not all(isinstance(v, str) for v in argv):
                errors.append(f"commands[{idx}].argv must be non-empty string array")
            if not isinstance(entry.get("exit_code"), int):
                errors.append(f"commands[{idx}].exit_code must be integer")
            if not isinstance(entry.get("duration_ms"), int):
                errors.append(f"commands[{idx}].duration_ms must be integer")
    else:
        errors.append("commands must be non-empty array")

    logs = manifest.get("logs")
    if isinstance(logs, list):
        for entry in logs:
            if not isinstance(entry, dict):
                errors.append("log entry must be object")
                continue
            kind = entry.get("kind")
            if kind not in LOG_KINDS:
                errors.append(f"log kind invalid: {kind}")
            validate_file_entry(entry, base_dir, "log", errors)
            if entry.get("kind") == "jsonl" and run_id:
                validate_jsonl_run_id(entry, base_dir, run_id, errors)
    else:
        errors.append("logs must be array")

    artifacts = manifest.get("artifacts")
    if isinstance(artifacts, list):
        for entry in artifacts:
            if not isinstance(entry, dict):
                errors.append("artifact entry must be object")
                continue
            kind = entry.get("kind")
            if kind not in ARTIFACT_KINDS:
                errors.append(f"artifact kind invalid: {kind}")
            validate_file_entry(entry, base_dir, "artifact", errors)
            if kind in REDACTION_REQUIRED_KINDS and not entry.get("redaction_profile"):
                errors.append(f"artifact {entry.get('path')} missing redaction_profile")
    else:
        errors.append("artifacts must be array")

    return errors


def main() -> int:
    parser = argparse.ArgumentParser(description="Validate E2E artifact manifest")
    parser.add_argument("manifest", help="Path to manifest.json")
    parser.add_argument(
        "--schema",
        default=None,
        help="Path to schema (default: specs/schemas/e2e-artifact-manifest.schema.json)",
    )

    args = parser.parse_args()
    manifest_path = Path(args.manifest).resolve()

    repo_root = Path(__file__).resolve().parents[1]
    schema_path = Path(args.schema).resolve() if args.schema else (repo_root / SCHEMA_RELATIVE)

    manifest = load_json(manifest_path)
    errors = validate_manifest(manifest, manifest_path, schema_path)

    if errors:
        for error in errors:
            print(f"ERROR: {error}", file=sys.stderr)
        return 1

    print(f"OK: {manifest_path}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
