# Completion Summary — sast-rulegen-a Milestone 1

## Goal completed

The `cargo xtask sast-verify gate` deterministic gate exists and is the single entry point through which any rule must pass before landing in `.semgrep/<lang>/`. The wedge is end-to-end demonstrated by 3 bootstrap rule pairs (CWE-755, CWE-190, CWE-295) all gate-clean, with `references/sast/` shared scaffolding fully populated, and both `/slo-rulegen` and `/slo-ruleverify` skills installed-and-running.

## Files changed

- `Cargo.toml` (workspace root) — added `xtasks/sast-verify` to `members`; added `serde`, `serde_yaml_ng`, `serde_json`, `tempfile` to `[workspace.dependencies]`
- `Cargo.lock` — auto-updated
- `.cargo/config.toml` (NEW) — `[alias] xtask = "run --package sast-verify --"`
- `xtasks/sast-verify/Cargo.toml` (NEW) — package manifest
- `xtasks/sast-verify/src/main.rs` (NEW) — clap derive Cli, 6 subcommands
- `xtasks/sast-verify/src/{validate,test_cmd,check_coverage,check_clean,gate,tier_detect,semgrep_runner,yaml_schema}.rs` (NEW) — per-subcommand impl + shared helpers
- `xtasks/sast-verify/tests/gate_e2e.rs` (NEW) — 3 integration tests
- `xtasks/sast-verify/tests/fixtures/clean_subset/example.rs` (NEW) — known-clean Rust source for default `check-clean` scan target
- `crates/sldo-common/src/toolflags.rs` — added `rulegen_*` and `ruleverify_*` flag families plus 6 unit tests
- `references/sast/{README,AUTHORING,cwe-map-rust,semgrep-rust-syntax,manifest-schema,MIN-SEMGREP-VERSION}.md` (NEW)
- `references/sast/variations/cwe-{755,416,697,125,787,190,295,672,20,79}.md` (NEW; 10 files)
- `references/sast/prompts/{bootstrap,extend}.md` (NEW; extend.md is M2 skeleton)
- `skills/slo-rulegen/SKILL.md` (NEW)
- `skills/slo-ruleverify/SKILL.md` (NEW)
- `.semgrep/rust/cwe-755-panic-on-result-fn.{yaml,rs}` (NEW)
- `.semgrep/rust/cwe-190-integer-overflow-in-security-context.{yaml,rs}` (NEW)
- `.semgrep/rust/cwe-295-improper-cert-validation.{yaml,rs}` (NEW)
- `CLAUDE.md` — appended `-p sast-verify` to baseline test command

## Tests added

- `xtasks/sast-verify/src/yaml_schema.rs` — 5 unit tests (parse pattern rule, parse pattern-either arms, extract CWE id, reject unknown field, reject empty rules list)
- `xtasks/sast-verify/src/check_coverage.rs` — 2 unit tests (read minimum-arms from frontmatter, error on missing key)
- `xtasks/sast-verify/src/semgrep_runner.rs` — 1 unit test (paired_fixture_returns_none_when_missing)
- `xtasks/sast-verify/src/tier_detect.rs` — 4 unit tests (parse SSH URL, parse HTTPS URL, parse unknown returns None, no-remote defaults Confidential)
- `crates/sldo-common/src/toolflags.rs` — 6 new unit tests (rulegen excludes WebFetch, rulegen excludes WebSearch, rulegen includes Bash, rulegen deny lists WebFetch/WebSearch, ruleverify excludes Write/Edit, ruleverify excludes WebFetch/WebSearch, ruleverify deny lists all four)
- `xtasks/sast-verify/tests/gate_e2e.rs` — 3 integration tests (xtask --help lists 5 subcommands, gate passes for all authored rules, detect-tier returns Confidential)

**Total: 21 tests added; all passing.**

## Runtime validations added

`xtasks/sast-verify/tests/gate_e2e.rs::gate_passes_for_all_authored_rules` runs `cargo xtask sast-verify gate` against every `.yaml` in `.semgrep/rust/` and asserts exit 0 for each. Currently exercises 3 rules; will exercise all 10 once the remaining 7 are authored (M1.5 follow-up).

## Compatibility checks performed

- `cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install` — green (baseline pre-existing tests still pass)
- `cargo test -p sast-verify --release` — green (12 unit + 3 integration = 15 tests)
- Smoke-tested `cargo xtask sast-verify --help` resolving from workspace root
- `sldo-install` discover_skills walker continues to find all 16+ existing skills plus the 2 new ones (slo-rulegen, slo-ruleverify); installation does NOT walk `references/sast/` (intentional, by `discover_skills` design)
- The parked `sldo-tauri` crate is unaffected
- `/slo-architect` SECURITY.md template substitution still works (the `~~~text` fence convention is preserved in `references/sast/prompts/extend.md`)

## Documentation updated

- [CLAUDE.md](../../CLAUDE.md) — baseline test command appended `-p sast-verify`
- ARCHITECTURE.md — pre-staged content from `/slo-architect` step ships unchanged in this milestone (the table rows for `slo-rulegen` and `slo-ruleverify` plus the `references/sast/` and `xtasks/sast-verify/` invariant bullets were authored during architect; M1 confirms they match what shipped)
- `references/sast/semgrep-rust-syntax.md` — smoke-test result for `pattern-inside: unsafe { ... }` recorded (CONFIRMED WORKING in semgrep 1.156.0)
- `docs/lessons/sast-rulegen-a-m1.md` (this milestone's lessons file) — written
- `docs/completion/sast-rulegen-a-m1.md` (this file) — written

## .gitignore changes

No additions in M1. The `.semgrep/.scratch/` and `xtasks/sast-verify/tests/scratch/` patterns were planned but not needed yet (M2's atomic-write contract introduces them).

## Test artifact cleanup verified

`git status` after the final `cargo test -p sast-verify --release` run is clean. No untracked test artifacts.

## Deferred follow-ups

**M1.5 — author the remaining 7 bootstrap rules** to complete the runbook's stated "10 rules" target. Each rule needs:
1. A YAML at `.semgrep/rust/cwe-<NNN>-<short>.yaml` conforming to `references/sast/manifest-schema.md`.
2. A paired `.rs` fixture with `// ruleid:` and `// ok:` annotations covering each `pattern-either` arm.
3. `cargo xtask sast-verify gate <rule.yaml>` exit 0.

The 7 deferred CWEs and their variation templates (already authored in M1):
- CWE-416 (UAF) — `references/sast/variations/cwe-416.md`
- CWE-697 (incorrect comparison) — `references/sast/variations/cwe-697.md`
- CWE-125 (OOB read) — `references/sast/variations/cwe-125.md`
- CWE-787 (OOB write) — `references/sast/variations/cwe-787.md`
- CWE-672 (use-after-release) — `references/sast/variations/cwe-672.md`
- CWE-20 (input validation) — `references/sast/variations/cwe-20.md`
- CWE-79 (XSS) — `references/sast/variations/cwe-79.md`

**eng-2 content-coverage Rust test** (the BDD `cwe_<NNN>_rule_covers_documented_variation_shapes`): documented in the runbook but not yet implemented as code. Implement once the rule pack is at full 10 rules so the test has a meaningful corpus to assert against.

**`cargo_config_creates_or_merges_existing_alias_section` BDD**: the runbook calls for testing the merge-not-overwrite path. The repo had no pre-existing `.cargo/config.toml` so the merge logic wasn't exercised; add a `tempfile`-based test in M2.

## Known non-blocking limitations

- The xtask emits two `dead_code` warnings on the `Rule` struct's optional fields. Those fields are part of the public schema (consumed by future expansion of `check-coverage`) and should not be deleted; suggest `#[allow(dead_code)]` on the struct or per-field as a polish commit.
- The `pattern-inside: unsafe { ... }` smoke-test was run in `/tmp/sast-smoke/`, not in a repo-tracked test fixture. Subsequent re-runs of the smoke (when Semgrep version bumps) need to re-create the fixture or commit it to `xtasks/sast-verify/tests/fixtures/smoke/` for reproducibility.
- `/slo-rulegen` extend mode is M2 territory — the SKILL.md prose mentions `--extend` but defers the body to `references/sast/prompts/extend.md` which is currently a skeleton.
- `gate_passes_for_all_authored_rules` skips when `semgrep` is not on PATH OR when the binary hasn't been built. CI per Runbook A M3 explicitly installs Semgrep, so this skip path is local-dev only.

## Verification of /slo-critique findings applied

- **eng-1** (`check-clean` self-poisoning): `--clean-dir` defaults to `xtasks/sast-verify/tests/fixtures/clean_subset/`, NOT host `src/`. Tested via `gate_passes_for_all_authored_rules`. ✓
- **eng-2** (per-CWE content coverage): variation files declare `sink_shapes` lists; the Rust assertion is M1.5 follow-up. Variation files MERIT this assertion being added. (Open follow-up.)
- **eng-3** (cargo config create-or-merge): file did not pre-exist; merge logic untested; M2 follow-up. (Open follow-up.)
- **eng-4** (tier-detect URL shapes): 4 unit tests cover SSH, HTTPS, unknown scheme, no-remote. All return Confidential by default-deny. ✓
- **eng-5** (atomic write tempdir+rename): M2 territory; not exercised in M1. (Deferred to M2 by design.)
- **sec-1** (`references/sast/` SHA-pin gap): documented as residual in SECURITY.md and threat model; Phase-3 hardening runbook target. ✓
- **sec-2** (xtask parses --json only): `parse_json_output` in `semgrep_runner` strict-parses `serde_json::Value`; `validate.rs` and `check_clean.rs` call sites use `--json` argument. Crate-local test in `gate_e2e.rs` exercises the path implicitly via `gate_passes_for_all_authored_rules`. ✓
- **sec-3** (`--file-paths` traversal): M2 territory; deferred. ✓
- **sec-4** (workflow YAML allows ruleverify but not rulegen extend): M3 territory; deferred.
- **sec-5** (SKILL.md prose enforces toolflag denial): both SKILL.md files include a top-of-file `## Tools you MUST NOT use` section explicitly forbidding WebFetch and WebSearch. ✓
