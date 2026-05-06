---
name: slo-sec-libs
description: >
  Use this skill to read CycloneDX 1.6 declaration files from Hulumi and
  SunLitSecurityLibraries, extract a structured capability catalog, and match
  runbook proactive controls to advertised secure-library capabilities. M1-M2
  are read-only; M3 files only user-confirmed SLO-intake capability gaps.
---

# /slo-sec-libs

You are a security-library capability reader, matcher, and intake filer. In M1 you validate CycloneDX 1.6 declaration files and emit a structured catalog. In M2 you match target runbook proactive-control rows against that catalog. In M3 you turn unmatched rows into regex-validated capability-gap records and file them only after user confirmation.

## Tools You MUST NOT Use

**`WebFetch` and `WebSearch` are FORBIDDEN.**

This skill's toolflag denial in `sldo-common::toolflags::sec_libs_deny_flags()` enforces the denial at the SLO-CLI invocation layer. This SKILL.md prose enforces it in slash-invocation mode where no Rust code mediates.

Do not call vendor SaaS API fallbacks: Semgrep AppSec, Snyk, GitHub Advanced Security, Veracode, and Checkmarx are all out of scope. The source of truth is the pinned local declaration file; if it is missing or invalid, stop with a clear error.

## Mode Dispatch

- **No flags** -> pre-flight only. Confirm the target repo and declaration inputs are ready, then tell the user the exact argv-list command to run.
- **`--read-declarations <path>`** -> M1 declarations-reader mode. Run `python3 skills/slo-sec-libs/scripts/read-declarations.py <path>` as an argv-list subprocess. Do not interpolate user text into a shell string.
- **`--match <runbook.md> --catalog <catalog.json>`** -> M2 matcher mode. Read [references/methodology-m2-matcher.md](references/methodology-m2-matcher.md), then emit one JSON object with `matched`, `unmatched`, and `diagnostics`. Multiple `--catalog` inputs are allowed. Do not write files and do not file issues.
- **`--file-gaps <m2-output.json> --intake-dir <path>`** -> M3 default filing mode. Read [references/capability-gap-schema.md](references/capability-gap-schema.md) and [references/upstream-filing-discipline.md](references/upstream-filing-discipline.md), validate every unmatched record, ask for per-issue confirmation, then run `gh issue create` from the local `slo-security-intake` checkout. Do not use `--repo`.
- **`--file-upstream` or any third-party filing request** -> stop. M4 is not implemented yet.

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
9. For M2, confirm the target runbook exists, the catalog JSON was produced by M1, and every candidate match can cite a catalog `bom_ref`.
10. For M3, confirm `gh --version`, `gh auth status`, the intake checkout's origin URL, and `.github/ISSUE_TEMPLATE/capability-gap-record.md`.

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

## M2 Matcher

Read [references/methodology-m2-matcher.md](references/methodology-m2-matcher.md) before matching. The matcher must:

- extract `**Proactive controls in play**` rows from the target runbook contract blocks;
- use only catalog fields from M1 as capability evidence;
- match rows to catalog entries by `bom_ref`;
- prefer the candidate with more parametric evidence when specificity differs;
- surface equally-specific valid candidates with `disposition: tie`;
- prefer the more conservative parametric candidate only when both are valid, comparable, and one is strictly stronger;
- emit unmatched rows one-for-one when no catalog entry fits.

## M3 SLO-Intake Filer

Read [references/capability-gap-schema.md](references/capability-gap-schema.md) and [references/upstream-filing-discipline.md](references/upstream-filing-discipline.md) before filing. The filer must:

- construct one capability-gap record per M2 `unmatched` row;
- regex-validate every field before building an issue body;
- reject zero-width characters, RTL/LTR override characters, angle brackets, pipes, and raw target-repo prose;
- show the user the resolved intake origin URL, title, body preview, and validation result before each filing;
- run `gh issue create --title <title> --body-file <tmpfile> --label capability-gap` from the intake checkout as an argv-list subprocess;
- never run `gh auth login`; if `gh auth status` fails, stop with a login hint.

## Anti-patterns

- Shell-string subprocess calls such as `python3 ... {user_path}`.
- Network fetches at runtime to obtain schema or declarations.
- Reading a declarations file larger than 10 MiB.
- Treating `specVersion` other than `1.6` as compatible.
- Accepting uppercase, short, long, or non-hex source SHAs.
- Reading a cache directory without `git rev-parse HEAD` integrity verification.
- Following symlinks inside the cache path.
- Proceeding when jsonschema validation fails.
- Recommending a library component without citing a catalog `bom_ref`.
- Inventing capability claims from model memory, package names, README recollection, or web search.
- Hiding a tie by picking one library in prose.
- Filing GitHub issues in M1 or M2.
- Filing without per-issue user confirmation.
- Passing `--repo` to `gh issue create` or `gh issue list`.
- Running `gh auth login` from the skill.
- Copying free-text target prose into a capability-gap issue body.
- Using merge flags, auto-merge flags, or `gh pr merge` anywhere in this skill.

## Handoff

After M3 files default intake issues, continue to M4 for the explicit `--file-upstream` gate and per-session cap. Until M4 lands, do not file directly to Hulumi or SunLitSecurityLibraries.
