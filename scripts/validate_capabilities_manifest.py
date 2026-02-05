#!/usr/bin/env python3
"""Validate capabilities manifest fixtures (schema + basic sanity)."""
from __future__ import annotations

import argparse
import json
import re
import sys
from datetime import datetime
from pathlib import Path
from typing import Any, Dict, List

SCHEMA_RELATIVE = Path("specs/schemas/capabilities-manifest.schema.json")
VERSION_RE = re.compile(r"^(\d+)\.(\d+)\.(\d+)$")


def load_json(path: Path) -> Dict[str, Any]:
    try:
        with path.open("r", encoding="utf-8") as handle:
            return json.load(handle)
    except FileNotFoundError:
        raise SystemExit(f"manifest not found: {path}")
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON in {path}: {exc}")


def validate_schema_with_jsonschema(
    manifest: Dict[str, Any], schema_path: Path, errors: List[str]
) -> None:
    try:
        import jsonschema  # type: ignore
    except Exception:
        return

    try:
        schema = load_json(schema_path)
        jsonschema.Draft202012Validator(schema).validate(manifest)
    except jsonschema.ValidationError as exc:
        errors.append(f"schema validation failed: {exc.message}")
    except Exception as exc:  # pragma: no cover
        errors.append(f"schema validation error: {exc}")


def parse_rfc3339(value: str) -> bool:
    try:
        if value.endswith("Z"):
            value = value[:-1] + "+00:00"
        datetime.fromisoformat(value)
        return True
    except ValueError:
        return False


def validate_version(version: str, errors: List[str]) -> None:
    match = VERSION_RE.match(version)
    if not match:
        errors.append(f"schema_version not semver: {version}")
        return
    major = int(match.group(1))
    if major != 1:
        errors.append(f"unsupported schema_version major: {version}")


def validate_manifest(manifest: Dict[str, Any], schema_path: Path) -> List[str]:
    errors: List[str] = []

    validate_schema_with_jsonschema(manifest, schema_path, errors)

    for field in ["schema_version", "os", "tools", "user", "paths", "discovered_at"]:
        if field not in manifest:
            errors.append(f"missing required field: {field}")

    schema_version = manifest.get("schema_version")
    if isinstance(schema_version, str):
        validate_version(schema_version, errors)
    else:
        errors.append("schema_version must be string")

    os_info = manifest.get("os")
    if not isinstance(os_info, dict):
        errors.append("os must be object")
    else:
        if not isinstance(os_info.get("family"), str):
            errors.append("os.family must be string")

    tools = manifest.get("tools")
    if not isinstance(tools, dict):
        errors.append("tools must be object")

    user = manifest.get("user")
    if not isinstance(user, dict):
        errors.append("user must be object")
    else:
        if not isinstance(user.get("uid"), int):
            errors.append("user.uid must be integer")
        if not isinstance(user.get("username"), str):
            errors.append("user.username must be string")
        if not isinstance(user.get("home"), str):
            errors.append("user.home must be string")

    paths = manifest.get("paths")
    if not isinstance(paths, dict):
        errors.append("paths must be object")
    else:
        if not isinstance(paths.get("config_dir"), str):
            errors.append("paths.config_dir must be string")
        if not isinstance(paths.get("data_dir"), str):
            errors.append("paths.data_dir must be string")

    discovered_at = manifest.get("discovered_at")
    if isinstance(discovered_at, str):
        if not parse_rfc3339(discovered_at):
            errors.append("discovered_at must be RFC3339 timestamp")
    else:
        errors.append("discovered_at must be string")

    return errors


def main() -> int:
    parser = argparse.ArgumentParser(description="Validate capabilities manifest JSON.")
    parser.add_argument("manifest", type=Path, help="Path to capabilities manifest JSON")
    args = parser.parse_args()

    manifest_path = args.manifest
    root = Path(__file__).resolve().parents[1]
    schema_path = root / SCHEMA_RELATIVE

    manifest = load_json(manifest_path)
    errors = validate_manifest(manifest, schema_path)

    if errors:
        for error in errors:
            print(error, file=sys.stderr)
        return 1

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
