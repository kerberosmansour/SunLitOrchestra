#!/usr/bin/env python3
# Copyright 2026 Sherif Mansour and SunLit Orchestra contributors.
# SPDX-License-Identifier: Apache-2.0 OR MIT
#
# scenario_catalog.py — offline lister / validator for the
# /slo-cloud-threat-model scenario catalog.
#
# This is the deterministic helper behind a pure-Markdown skill: it does NOT
# author the threat model (the agent does that per references/methodology.md).
# It only (a) enumerates scenario IDs in a stable declared order, (b) validates
# the bundled scenario fixtures against the schema, the IDs-only control shape,
# the known-framework map, and a verbatim-prose license-boundary deny-list, and
# (c) prints one resolved scenario object.
#
# Discipline:
#   - Invoke it argv-list: never splice user text into a shell string. It runs
#     no child process and builds no shell command of any kind.
#   - Offline only: standard-library imports, no network client at all.
#   - Bounded: each scenario file is capped at 1 MiB before it is read.
#   - Symlink-safe: every path component of `--scenarios-dir` is checked for a
#     symlink BEFORE the path is resolved.
#
# Usage (argv-list):
#   python3 scenario_catalog.py list   [--scenarios-dir DIR]
#   python3 scenario_catalog.py validate [--scenarios-dir DIR]
#   python3 scenario_catalog.py show <scenario-id> [--scenarios-dir DIR]

import argparse
import json
import re
import sys
from pathlib import Path

# Per-file size cap. A scenario fixture is small; anything larger is rejected
# before the file is read so a hostile or corrupt catalog cannot exhaust memory.
MAX_SCENARIO_BYTES = 1 * 1024 * 1024  # 1 MiB

# Stable declared scenario order. This is the stability contract (mirrors
# Hulumi's list-scenarios.mjs): the BDD/structural tests assert this exact
# sequence and that it equals the on-disk set. New scenarios append at the END.
SCENARIO_ORDER = [
    # AWS (ported + modernized from hulumi-threat-model)
    "aws-multi-account-baseline",
    "s3-public-bucket-hardening",
    "iam-least-privilege",
    "rds-encryption-at-rest",
    "lambda-secrets-access",
    # GitHub (ported + modernized)
    "github-oidc-trust-cloud-account",
    "github-actions-supply-chain",
    "github-app-token-exposure",
    "github-self-hosted-runner",
    # Cloudflare / edge (new — Hulumi v1.3.2 Edge Platform)
    "cloudflare-zone-and-dns-foundation",
    "cloudflare-edge-waf-and-bot-protection",
    "cloudflare-origin-bypass-prevention",
    "cloudflare-protected-admin-access",
    # Cross-provider (new)
    "github-aws-oidc-deployment-identity",
]

PROVIDERS = {"aws", "github", "cloudflare", "cross-provider"}
STRIDE_LETTERS = {"S", "T", "R", "I", "D", "E"}

# Framework key -> upstream URL. MUST stay in sync with
# references/citation-and-licensing.md. IDs-only: no control prose lives here.
FRAMEWORK_URLS = {
    "CCM": "https://cloudsecurityalliance.org/artifacts/cloud-controls-matrix-v4-1",
    "CIS-AWS-v5.0.0": "https://www.cisecurity.org/benchmark/amazon_web_services",
    "CIS-GitHub-v1.2.0": "https://www.cisecurity.org/benchmark/github_foundations",
    "NIST-800-53-r5": "https://csrc.nist.gov/pubs/sp/800/53/r5/upd1/final",
    "NIST-800-218": "https://csrc.nist.gov/projects/ssdf",
    "NIST-800-218A": "https://csrc.nist.gov/projects/ssdf",
    "NIST-SSDF-v1.1": "https://csrc.nist.gov/projects/ssdf",
    "ATLAS": "https://atlas.mitre.org/",
    "MITRE-ATTCK": "https://attack.mitre.org/",
    "OpenSSF-Scorecard": "https://scorecard.dev/",
    "GitHub-Well-Architected": "https://wellarchitected.github.com/library/scenarios/nist-ssdf-implementation/",
    "OWASP-ASVS-v5.0": "https://owasp.org/www-project-application-security-verification-standard/",
    "ISO-27001-2022": "https://www.iso.org/standard/27001",
    "SOC2-TSC-2017": "https://www.aicpa-cima.com/resources/landing/system-and-organization-controls-soc-suite-of-services",
    "Hulumi-Policy": "https://github.com/kerberosmansour/hulumi/blob/main/docs/components/cloudflare-policy-packs.md",
}

# Bare Hulumi policy rule IDs (no colon) -> framework Hulumi-Policy.
HULUMI_POLICY_RULES = {
    "CF_DNS_1_NO_DNS_ONLY_PUBLIC_APP_RECORD",
    "CF_DNSSEC_1_REQUIRE_PUBLIC_ZONE_DNSSEC",
    "CF_ORIGIN_1_REQUIRE_SECURE_ORIGIN_MODE",
    "X_ORIGIN_1_NO_PUBLIC_AWS_ORIGIN_BYPASS",
    "G_OIDC_1",
}

# A control token is an identifier, never a sentence. Either FRAMEWORK:SUFFIX
# or a bare Hulumi rule ID. Spaces / quotes / sentence punctuation are rejected,
# which is the enforceable core of the IDs-only rule.
CONTROL_ID_RE = re.compile(r"^[A-Za-z0-9][A-Za-z0-9._-]*(?::[A-Za-z0-9][A-Za-z0-9._/+-]*)?$")

# Defense-in-depth license-boundary deny-list: short, distinctive trigger
# phrasings that should never appear in a fixture because they signal pasted
# verbatim benchmark/control prose rather than an SLO-authored paraphrase.
LICENSE_DENYLIST = (
    "verbatim from the benchmark",
    "copyright center for internet security",
    "cis controls v8",
    "reproduced with permission of the cloud security alliance",
    "this recommendation is from the cis",
)


def assert_no_symlink_path(p: Path) -> None:
    """Fail if any component of `p` (as given, before resolution) is a symlink."""
    probe = Path(p.anchor) if p.anchor else Path(".")
    parts = p.parts[1:] if p.anchor else p.parts
    if p.is_symlink():
        raise SystemExit(f"refusing symlinked path component: {p}")
    for part in parts:
        probe = probe / part
        if probe.is_symlink():
            raise SystemExit(f"refusing symlinked path component: {probe}")


def resolve_scenarios_dir(arg: str | None) -> Path:
    """Resolve --scenarios-dir; symlink check runs BEFORE resolution."""
    if arg:
        scenarios_dir_arg = Path(arg)
    else:
        scenarios_dir_arg = Path(__file__).resolve().parent.parent / "scenarios"
    assert_no_symlink_path(scenarios_dir_arg)
    scenarios_dir = scenarios_dir_arg.resolve(strict=True)
    if not scenarios_dir.is_dir():
        raise SystemExit(f"--scenarios-dir is not a directory: {scenarios_dir}")
    return scenarios_dir


def load_scenario(scenarios_dir: Path, scenario_id: str) -> dict:
    path = scenarios_dir / f"{scenario_id}.json"
    if not path.is_file():
        raise SystemExit(f"scenario fixture not found: {path}")
    size = path.stat().st_size
    if size > MAX_SCENARIO_BYTES:
        raise SystemExit(
            f"scenario fixture exceeds {MAX_SCENARIO_BYTES} bytes (1 MiB cap): {path} ({size})"
        )
    with path.open("r", encoding="utf-8") as fh:
        data = json.load(fh)
    if not isinstance(data, dict):
        raise SystemExit(f"scenario fixture is not a JSON object: {path}")
    return data


def framework_of(control_id: str) -> str | None:
    if ":" in control_id:
        return control_id.split(":", 1)[0]
    if control_id in HULUMI_POLICY_RULES:
        return "Hulumi-Policy"
    return None


def validate_scenario(scenario_id: str, obj: dict) -> list[str]:
    """Return a list of human-readable errors (empty == valid)."""
    errs: list[str] = []

    def need(key: str, typ) -> object | None:
        if key not in obj:
            errs.append(f"{scenario_id}: missing required key `{key}`")
            return None
        if not isinstance(obj[key], typ):
            errs.append(f"{scenario_id}: key `{key}` has wrong type")
            return None
        return obj[key]

    if obj.get("id") != scenario_id:
        errs.append(f"{scenario_id}: `id` does not match filename ({obj.get('id')!r})")
    need("title", str)
    provider = need("provider", str)
    if provider is not None and provider not in PROVIDERS:
        errs.append(f"{scenario_id}: provider `{provider}` not in {sorted(PROVIDERS)}")
    need("description", str)

    req_fw = need("requiredFrameworks", list) or []
    for fw in req_fw:
        if fw not in FRAMEWORK_URLS:
            errs.append(f"{scenario_id}: requiredFrameworks has unknown framework `{fw}`")

    for key in ("actors", "assets"):
        val = need(key, list) or []
        if not val:
            errs.append(f"{scenario_id}: `{key}` must be non-empty")

    stride = need("stride", list) or []
    if len(stride) < 3:
        errs.append(f"{scenario_id}: stride must have >= 3 rows (has {len(stride)})")
    for i, row in enumerate(stride):
        if not isinstance(row, dict):
            errs.append(f"{scenario_id}: stride[{i}] is not an object")
            continue
        if row.get("type") not in STRIDE_LETTERS:
            errs.append(f"{scenario_id}: stride[{i}].type `{row.get('type')}` invalid")
        if not isinstance(row.get("name"), str) or not row.get("name"):
            errs.append(f"{scenario_id}: stride[{i}].name missing/empty")
        if not isinstance(row.get("description"), str) or not row.get("description"):
            errs.append(f"{scenario_id}: stride[{i}].description missing/empty")
        controls = row.get("controls")
        if not isinstance(controls, list):
            errs.append(f"{scenario_id}: stride[{i}].controls must be a list")
            continue
        for ctrl in controls:
            if not isinstance(ctrl, str) or not CONTROL_ID_RE.match(ctrl):
                errs.append(
                    f"{scenario_id}: stride[{i}] control `{ctrl}` is not an IDs-only token"
                )
                continue
            fw = framework_of(ctrl)
            if fw is None or fw not in FRAMEWORK_URLS:
                errs.append(
                    f"{scenario_id}: stride[{i}] control `{ctrl}` has unknown framework"
                )

    comps = need("recommendedComponents", list) or []
    if len(comps) < 2:
        errs.append(f"{scenario_id}: recommendedComponents must have >= 2 entries")
    for j, comp in enumerate(comps):
        if not isinstance(comp, dict):
            errs.append(f"{scenario_id}: recommendedComponents[{j}] is not an object")
            continue
        for k in ("name", "availability", "rationale"):
            if not isinstance(comp.get(k), str) or not comp.get(k):
                errs.append(f"{scenario_id}: recommendedComponents[{j}].{k} missing/empty")

    blob = json.dumps(obj, ensure_ascii=False).lower()
    for needle in LICENSE_DENYLIST:
        if needle in blob:
            errs.append(
                f"{scenario_id}: license-boundary hit — possible verbatim prose `{needle}`"
            )
    return errs


def on_disk_ids(scenarios_dir: Path) -> list[str]:
    return sorted(p.stem for p in scenarios_dir.glob("*.json"))


def cmd_list(scenarios_dir: Path) -> int:
    declared = set(SCENARIO_ORDER)
    disk = set(on_disk_ids(scenarios_dir))
    if declared != disk:
        missing = sorted(declared - disk)
        extra = sorted(disk - declared)
        sys.stderr.write(
            "scenario catalog drift: "
            f"missing on disk={missing} extra on disk={extra}\n"
        )
        return 1
    for sid in SCENARIO_ORDER:
        print(sid)
    return 0


def cmd_validate(scenarios_dir: Path) -> int:
    declared = set(SCENARIO_ORDER)
    disk = set(on_disk_ids(scenarios_dir))
    errors: list[str] = []
    if declared != disk:
        errors.append(
            f"catalog drift: missing={sorted(declared - disk)} extra={sorted(disk - declared)}"
        )
    for sid in SCENARIO_ORDER:
        if sid not in disk:
            continue
        try:
            obj = load_scenario(scenarios_dir, sid)
        except SystemExit as exc:  # surface load failure as a validation error
            errors.append(str(exc))
            continue
        errors.extend(validate_scenario(sid, obj))
    if errors:
        sys.stderr.write("INVALID\n")
        for e in errors:
            sys.stderr.write(f"  - {e}\n")
        return 1
    print(f"OK: {len(SCENARIO_ORDER)} scenarios valid")
    return 0


def cmd_show(scenarios_dir: Path, scenario_id: str) -> int:
    if scenario_id not in SCENARIO_ORDER:
        sys.stderr.write(
            f"unknown scenario `{scenario_id}`; run `list` for the valid set\n"
        )
        return 1
    obj = load_scenario(scenarios_dir, scenario_id)
    json.dump(obj, sys.stdout, indent=2, ensure_ascii=False)
    sys.stdout.write("\n")
    return 0


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(
        prog="scenario_catalog.py",
        description="Offline lister/validator for the /slo-cloud-threat-model scenario catalog.",
    )
    parser.add_argument(
        "--scenarios-dir",
        default=None,
        help="catalog directory (defaults to the bundled scenarios/ dir)",
    )
    sub = parser.add_subparsers(dest="command", required=True)
    sub.add_parser("list", help="print scenario IDs in declared order")
    sub.add_parser("validate", help="validate every bundled scenario fixture")
    show = sub.add_parser("show", help="print one resolved scenario object")
    show.add_argument("scenario_id")

    args = parser.parse_args(argv)
    scenarios_dir = resolve_scenarios_dir(args.scenarios_dir)

    if args.command == "list":
        return cmd_list(scenarios_dir)
    if args.command == "validate":
        return cmd_validate(scenarios_dir)
    if args.command == "show":
        return cmd_show(scenarios_dir, args.scenario_id)
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
