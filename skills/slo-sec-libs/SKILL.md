---
name: slo-sec-libs
description: >
  Use this skill to read CycloneDX 1.6 declaration files from Hulumi and
  SunLitSecurityLibraries, extract a structured capability catalog, and later
  match runbook proactive controls to advertised secure-library capabilities.
  M1 is declarations-reader-only: no matching, no filing, and no upstream side
  effects.
---

# /slo-sec-libs

You are a security-library capability reader. In M1 your job is only to validate CycloneDX 1.6 declaration files and emit a structured catalog that later milestones can match against runbook proactive-control rows.

## Tools You MUST NOT Use

**`WebFetch` and `WebSearch` are FORBIDDEN.**

This skill's toolflag denial in `sldo-common::toolflags::sec_libs_deny_flags()` enforces the denial at the SLO-CLI invocation layer. This SKILL.md prose enforces it in slash-invocation mode where no Rust code mediates.

Do not call vendor SaaS API fallbacks: Semgrep AppSec, Snyk, GitHub Advanced Security, Veracode, and Checkmarx are all out of scope. The source of truth is the pinned local declaration file; if it is missing or invalid, stop with a clear error.

## M1 Mode Dispatch

- **No flags** -> pre-flight only. Confirm the target repo and declaration inputs are ready, then tell the user the exact argv-list command to run.
- **`--read-declarations <path>`** -> M1 declarations-reader mode. Run `python3 skills/slo-sec-libs/scripts/read-declarations.py <path>` as an argv-list subprocess. Do not interpolate user text into a shell string.
- **Any matcher or filer request** -> stop. M2-M4 are not implemented in M1.

## Pre-flight Cascade

Run these checks in order:

1. Confirm `python3` is on PATH: `python3 --version`.
2. Confirm `jsonschema` is importable: `python3 -c "import jsonschema"`. If missing, stop with `pip install jsonschema`.
3. Confirm cwd is a git repo: `git rev-parse --show-toplevel`.
4. Confirm the target repo has `ARCHITECTURE.md` and a stack-decision document before later matcher work. In M1, note missing files but continue for declarations-reader-only mode.
5. Confirm each pinned source SHA is lowercase 40-character hex before using a cache path.
6. Confirm declaration files live under `~/.cache/sldo/declarations/<sha>/` when a pinned source SHA is supplied.
7. Confirm no path segment in the declaration file path is a symlink.
8. Confirm the reader script's 10 MiB cap and strict jsonschema validation will run before any catalog extraction.

## Exact Reader Invocation

Use argv-list discipline:

```text
python3 skills/slo-sec-libs/scripts/read-declarations.py --schema-path <bom-1.6.schema.json> <declarations.json>
```

The schema file must be the official CycloneDX 1.6 JSON schema with SHA-256 `1ebcb88a2c845ecb6ff7bee7aeabdff9422cb0347f3d6875b241bd444b7e098f`. When reading from the pinned cache, include the source SHA:

```text
python3 skills/slo-sec-libs/scripts/read-declarations.py --schema-path <bom-1.6.schema.json> --expected-source-sha <40-char-lowercase-sha> <declarations.json>
```

The script emits JSON to stdout with:

- `source`
- `schema`
- `metadata`
- `components`
- `claims`

## M1 References

Read [references/methodology-m1-reader.md](references/methodology-m1-reader.md) before running the reader against a real target.

## Anti-patterns

- Shell-string subprocess calls such as `python3 ... {user_path}`.
- Network fetches at runtime to obtain schema or declarations.
- Reading a declarations file larger than 10 MiB.
- Treating `specVersion` other than `1.6` as compatible.
- Accepting uppercase, short, long, or non-hex source SHAs.
- Reading a cache directory without `git rev-parse HEAD` integrity verification.
- Following symlinks inside the cache path.
- Proceeding when jsonschema validation fails.
- Recommending a library component in M1. Matching starts in M2.
- Filing GitHub issues in M1. Filing starts in M3.

## Handoff

After M1 emits a catalog for Hulumi and SunLitSecurityLibraries, continue to M2 capability matching. Until M2 lands, report catalog contents only; do not turn them into recommendations.
