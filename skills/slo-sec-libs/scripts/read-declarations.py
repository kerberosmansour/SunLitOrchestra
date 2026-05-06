#!/usr/bin/env python3
"""Read CycloneDX 1.6 declarations into an SLO capability catalog."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import unicodedata

try:
    import jsonschema
except ImportError:
    jsonschema = None


MAX_BYTES = 10 * 1024 * 1024
MAX_JSON_DEPTH = 200
SHA_RE = re.compile(r"^[0-9a-f]{40}$")
SCHEMA_URL = "https://cyclonedx.org/schema/bom-1.6.schema.json"
SCHEMA_SHA256 = "1ebcb88a2c845ecb6ff7bee7aeabdff9422cb0347f3d6875b241bd444b7e098f"

READER_SCHEMA = {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "type": "object",
    "required": ["bomFormat", "specVersion", "declarations"],
    "properties": {
        "$schema": {"type": "string"},
        "bomFormat": {"const": "CycloneDX"},
        "specVersion": {"const": "1.6"},
        "version": {"type": "integer", "minimum": 1},
        "metadata": {
            "type": "object",
            "properties": {
                "component": {
                    "type": "object",
                    "properties": {
                        "type": {"type": "string"},
                        "name": {"type": "string"},
                        "version": {"type": "string"},
                    },
                }
            },
        },
        "declarations": {
            "type": "object",
            "required": ["targets"],
            "additionalProperties": False,
            "properties": {
                "assessors": {"type": "array"},
                "attestations": {"type": "array"},
                "claims": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "additionalProperties": False,
                        "properties": {
                            "bom-ref": {"type": "string"},
                            "target": {"type": "string"},
                            "predicate": {"type": "string"},
                            "mitigationStrategies": {
                                "type": "array",
                                "items": {"type": "string"},
                            },
                            "reasoning": {"type": "string"},
                            "evidence": {"type": "array", "items": {"type": "string"}},
                            "counterEvidence": {
                                "type": "array",
                                "items": {"type": "string"},
                            },
                            "externalReferences": {"type": "array"},
                            "signature": {"type": "object"},
                        },
                    },
                },
                "evidence": {"type": "array"},
                "targets": {
                    "type": "object",
                    "required": ["components"],
                    "additionalProperties": False,
                    "properties": {
                        "organizations": {"type": "array"},
                        "components": {
                            "type": "array",
                            "minItems": 1,
                            "items": {
                                "type": "object",
                                "required": ["bom-ref", "type", "name"],
                                "properties": {
                                    "bom-ref": {"type": "string", "minLength": 1},
                                    "type": {"type": "string", "minLength": 1},
                                    "group": {"type": "string"},
                                    "name": {"type": "string", "minLength": 1},
                                    "version": {"type": "string"},
                                    "description": {"type": "string"},
                                    "properties": {
                                        "type": "array",
                                        "items": {
                                            "type": "object",
                                            "required": ["name", "value"],
                                            "additionalProperties": False,
                                            "properties": {
                                                "name": {"type": "string"},
                                                "value": {"type": "string"},
                                            },
                                        },
                                    },
                                },
                            },
                        },
                        "services": {"type": "array"},
                    },
                },
                "affirmation": {"type": "object"},
                "signature": {"type": "object"},
            },
        },
    },
}


class ReaderError(Exception):
    pass


def fail(message):
    print(f"read-declarations: {message}", file=sys.stderr)
    return 1


def parse_args(argv):
    parser = argparse.ArgumentParser(
        description="Validate CycloneDX 1.6 declarations and emit an SLO catalog."
    )
    parser.add_argument(
        "--expected-source-sha",
        help="lowercase 40-character git SHA expected at the declaration cache root",
    )
    parser.add_argument(
        "--schema-path",
        help="local official CycloneDX 1.6 schema; SHA-256 is verified before use",
    )
    parser.add_argument("declarations_json", help="path to declarations JSON")
    return parser.parse_args(argv)


def assert_jsonschema_available():
    if jsonschema is None:
        raise ReaderError("python package `jsonschema` is missing; install with `pip install jsonschema`")


def assert_no_symlink_path(path):
    current = Path(path.anchor) if path.is_absolute() else Path(".")
    parts = path.parts[1:] if path.is_absolute() else path.parts
    for part in parts:
        current = current / part
        if os.path.islink(current):
            raise ReaderError(f"path segment is a symlink: {current}")


def assert_size_cap(path):
    size = path.stat().st_size
    if size > MAX_BYTES:
        raise ReaderError(f"declarations file is {size} bytes; limit is 10 MiB")


def load_json(path):
    try:
        with path.open("r", encoding="utf-8") as fh:
            return json.load(fh)
    except json.JSONDecodeError as exc:
        raise ReaderError(f"malformed JSON at line {exc.lineno}, column {exc.colno}: {exc.msg}") from exc


def max_depth(value, depth=0):
    if depth > MAX_JSON_DEPTH:
        raise ReaderError(f"JSON nesting exceeds max depth {MAX_JSON_DEPTH}")
    if isinstance(value, dict):
        for child in value.values():
            max_depth(child, depth + 1)
    elif isinstance(value, list):
        for child in value:
            max_depth(child, depth + 1)


def read_json_file(path):
    try:
        with path.open("r", encoding="utf-8") as fh:
            return json.load(fh)
    except json.JSONDecodeError as exc:
        raise ReaderError(f"malformed JSON at line {exc.lineno}, column {exc.colno}: {exc.msg}") from exc


def validate_with(schema, data, label):
    jsonschema.Draft7Validator.check_schema(schema)
    validator = jsonschema.Draft7Validator(schema)
    errors = sorted(validator.iter_errors(data), key=lambda error: list(error.path))
    if errors:
        first = errors[0]
        location = ".".join(str(part) for part in first.path) or "<root>"
        raise ReaderError(f"{label} validation failed at {location}: {first.message}")


def validate_schema(data, schema_path):
    if schema_path is not None:
        assert_no_symlink_path(schema_path)
        observed = hashlib.sha256(schema_path.read_bytes()).hexdigest()
        if observed != SCHEMA_SHA256:
            raise ReaderError(f"schema SHA mismatch: expected {SCHEMA_SHA256}, observed {observed}")
        validate_with(read_json_file(schema_path), data, "official CycloneDX schema")

    validate_with(READER_SCHEMA, data, "strict jsonschema")


def assert_expected_sha(expected):
    if expected is not None and not SHA_RE.match(expected):
        raise ReaderError("expected source SHA must match ^[0-9a-f]{40}$")


def find_cache_root(path, expected):
    if expected is None:
        return None
    marker = Path.home() / ".cache" / "sldo" / "declarations" / expected
    try:
        path.relative_to(marker)
    except ValueError as exc:
        raise ReaderError(f"declarations file is not under cache root {marker}") from exc
    return marker


def verify_cache_head(cache_root, expected):
    if cache_root is None:
        return None
    result = subprocess.run(
        ["git", "-C", str(cache_root), "rev-parse", "HEAD"],
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    if result.returncode != 0:
        raise ReaderError(f"cannot verify cache git HEAD: {result.stderr.strip()}")
    observed = result.stdout.strip()
    if observed != expected:
        raise ReaderError(f"cache HEAD mismatch: expected {expected}, observed {observed}")
    return observed


def nfkc_stable(value, label):
    if value is None:
        return None
    normalized = unicodedata.normalize("NFKC", value)
    if normalized != value:
        raise ReaderError(f"{label} changes under NFKC normalization")
    return value


def split_controls(value):
    return [part.strip() for part in value.split(",") if part.strip()]


def component_properties(component):
    props = []
    for prop in component.get("properties", []):
        props.append({"name": prop["name"], "value": prop["value"]})
    return props


def extract_catalog(data, path, expected_sha, observed_sha):
    declarations = data["declarations"]
    claims = declarations.get("claims", [])
    claims_by_target = {}
    for claim in claims:
        target = claim.get("target")
        if target:
            claims_by_target.setdefault(target, []).append(claim)

    components = []
    for component in declarations["targets"]["components"]:
        bom_ref = nfkc_stable(component["bom-ref"], "component bom-ref")
        group = nfkc_stable(component.get("group"), "component group")
        name = nfkc_stable(component["name"], "component name")
        props = component_properties(component)
        controls = []
        capabilities = []
        for prop in props:
            if prop["name"] == "cdx:sunlit:controls":
                controls.extend(split_controls(prop["value"]))
            elif prop["name"] == "cdx:sunlit:capability":
                capabilities.append(prop["value"])
        components.append(
            {
                "bom_ref": bom_ref,
                "group": group,
                "name": name,
                "version": component.get("version"),
                "type": component.get("type"),
                "controls": controls,
                "capabilities": capabilities,
                "properties": props,
                "claims": claims_by_target.get(bom_ref, []),
            }
        )

    metadata_component = data.get("metadata", {}).get("component", {})
    return {
        "source": {
            "path": str(path),
            "expected_source_sha": expected_sha,
            "observed_source_sha": observed_sha,
        },
        "schema": {
            "url": SCHEMA_URL,
            "sha256": SCHEMA_SHA256,
            "validation": "strict-jsonschema",
        },
        "metadata": {
            "name": metadata_component.get("name"),
            "version": metadata_component.get("version"),
        },
        "components": components,
        "claims": claims,
    }


def run(argv):
    args = parse_args(argv)
    assert_jsonschema_available()
    assert_expected_sha(args.expected_source_sha)
    declarations_arg_path = Path(args.declarations_json).expanduser()
    assert_no_symlink_path(declarations_arg_path)
    path = declarations_arg_path.resolve(strict=True)
    if args.schema_path:
        schema_arg_path = Path(args.schema_path).expanduser()
        assert_no_symlink_path(schema_arg_path)
        schema_path = schema_arg_path.resolve(strict=True)
    else:
        schema_path = None
    cache_root = find_cache_root(path, args.expected_source_sha)
    observed_sha = verify_cache_head(cache_root, args.expected_source_sha)
    assert_size_cap(path)
    data = load_json(path)
    max_depth(data)
    validate_schema(data, schema_path)
    catalog = extract_catalog(data, path, args.expected_source_sha, observed_sha)
    json.dump(catalog, sys.stdout, indent=2, sort_keys=True)
    print()
    return 0


def main():
    try:
        return run(sys.argv[1:])
    except FileNotFoundError as exc:
        return fail(f"file not found: {exc}")
    except ReaderError as exc:
        return fail(str(exc))
    except RecursionError as exc:
        return fail(f"JSON nesting is too deep: {exc}")


if __name__ == "__main__":
    raise SystemExit(main())
