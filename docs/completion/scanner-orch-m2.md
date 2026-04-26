# Completion Summary — scanner-orch Milestone 2

## Goal completed

Stack detection contract, `semgrep-rules` cache fetch contract (with 40-char SHA-only enforcement and the placeholder all-zero pending first real pin), and CWE × technology rule filter logic are documented in `references/sast/stack-detection-contract.md`, `references/sast/scanner-orch-pinned-rules-sha.md`, and `skills/slo-sast/SKILL.md`'s extended Method section. M2 output (the JSON envelope with `cwes_extracted`, `detected_stack`, `selected_rules`, `selection_strategy`) is documented as the new contract; M1's bare-list output is superseded but its parser-scope-rule assertions remain green (no migration needed at the structural-contract test layer).

## Files changed

- `references/sast/scanner-orch-pinned-rules-sha.md` — NEW (~80 lines)
- `references/sast/stack-detection-contract.md` — NEW (~70 lines)
- `skills/slo-sast/SKILL.md` — extended (M2 sections added; M1 sections unchanged)
- `crates/sldo-install/tests/e2e_scanner_orch_m2.rs` — NEW (~280 lines, 22 tests)
- `docs/RUNBOOK-SCANNER-ORCHESTRATION.md` — modified (M2 asks ENG-2/ENG-5/ENG-8/CEO-1/SEC-2/SEC-6 applied; Milestone Tracker M2 → done)
- `docs/lessons/scanner-orch-m2.md` — NEW
- `docs/completion/scanner-orch-m2.md` — NEW (this file)

## Tests added

22 structural-contract tests in `e2e_scanner_orch_m2.rs`:
- Pinned-SHA doc: existence + 40-char enforcement + abuse-case citation + bump procedure + placeholder OR real-SHA shape + placeholder-refusal language (6 tests)
- Stack-detection contract: existence + manifest priority order (all 8) + polyglot behavior + default-fallback + stable marker (5 tests)
- SKILL.md: cites both new reference docs + JSON envelope keys + default-fallback strategy + argv-list discipline + YAML parser safety + cache layout + cache-hit behavior + rule filter logic + M1 parser scope still enforced (10 tests)
- references/sast/ existing files unmodified by M2 (1 test)

## Runtime validations added

- N/A at the auto-running-test layer (per the structural-contract pattern from M1).
- Smoke-test wedge validation (CEO-1) deferred to `/slo-verify` or first real bump-PR.

## Compatibility checks performed

- M1 E2E tests still green (21 tests pass) — confirms M2 SKILL.md additions did NOT break M1 contracts.
- All sldo-install test suites green (incremental sub-second).
- `cargo check --workspace` green.
- `references/sast/` existing files (M1's parser-contract + sast-rulegen pre-existing files) byte-identical (asserted by `existing_references_sast_unmodified_by_m2`).
- No `Cargo.toml` workspace deps changed; no per-crate `[dev-dependencies]` additions needed.

## Documentation updated

- `docs/RUNBOOK-SCANNER-ORCHESTRATION.md` — Milestone Tracker row 2 → `done`. M2 Evidence Log to be filled by automation or the user; manual entries follow the M1 pattern.

## .gitignore changes

None.

## Test artifact cleanup verified

`git status` shows only intended new files. No untracked test outputs. The `~/.cache/sldo/semgrep-rules/` cache directory is in user home (not in the repo), so no `.gitignore` entry is needed.

## Deferred follow-ups

- **First real pinned-SHA bump.** The all-zero placeholder is in place; the first real bump-PR happens at M5 dogfood-prep when wedge validation runs against a representative threat-model fixture set.
- **Runtime YAML-parser-safety verification** — `serde_yaml_ng` default-safe behavior is documented but not runtime-tested at this layer. Defer to `/slo-verify`.
- **Real `git clone` exercise of cache-fetch logic** — not exercised at the auto-running-test layer (would require either a stubbed `git` binary or network access to the upstream repo). Defer to smoke testing.

## Known non-blocking limitations

- **Placeholder pinned-SHA is intentionally invalid.** Until a wedge-validation pass runs against a real `semgrep-rules` SHA, the skill cannot actually fetch + filter at runtime. This is acceptable for the M2 milestone (which lands the contracts and tests); M5 dogfood-prep is when the placeholder gets replaced.
- **The structural-contract test pattern verifies that M2 contracts are DOCUMENTED, not that they are HONORED at runtime.** Same caveat as M1; defended by SLO's broader skill-pack invariant (Claude Code reads SKILL.md and follows it) and by smoke + `/slo-verify` runtime testing.
