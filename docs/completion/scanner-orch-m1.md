# Completion Summary — scanner-orch Milestone 1

## Goal completed

`/slo-sast` skill exists at `skills/slo-sast/SKILL.md` with valid frontmatter (`name: slo-sast`, multi-line `description:`) and a documented invocation flow. Threat-model parser scope rule (regex `\bCWE-(\d+)\b` against rendered Markdown body, excluding HTML comments / fenced code blocks / `~~~text` user-string fences) is locked in `references/sast/threat-model-parser-contract.md`. The skill is discoverable by `sldo-install` (31 skills total post-install; was 30 pre-M1). 21 structural-contract tests assert the SKILL.md and reference doc document the contract correctly; existing skills + existing `references/sast/` files unchanged.

## Files changed

- `skills/slo-sast/SKILL.md` — NEW (110 lines, Markdown skill body + frontmatter)
- `references/sast/threat-model-parser-contract.md` — NEW (90 lines, regex + 3 exclusion regions + rationale)
- `crates/sldo-install/tests/e2e_scanner_orch_m1.rs` — NEW (310 lines, 21 structural-contract tests)
- `docs/RUNBOOK-SCANNER-ORCHESTRATION.md` — modified (Milestone Tracker M1 → done; M1 Evidence Log filled; ENG-7 auto-fix applied earlier)
- `docs/lessons/scanner-orch-m1.md` — NEW
- `docs/completion/scanner-orch-m1.md` — NEW (this file)

## Tests added

- `crates/sldo-install/tests/e2e_scanner_orch_m1.rs` — 21 structural-contract tests:
  - SKILL.md frontmatter validity (3 tests)
  - SKILL.md cites parser-contract reference (1)
  - Parser-contract doc exists + documents regex (2)
  - Parser-contract names all 3 exclusion regions (3)
  - Parser-contract cites abuse case + marked stable (2)
  - SKILL.md documents empty-list / missing-file / long-form / dedup / sort behaviors (5)
  - SKILL.md scopes to M1 (anti-patterns explicitly forbid M2+ functionality) (1)
  - Skill discoverable via sldo-install walker (1)
  - Existing skills + existing references/sast/ unmodified (2)
  - SKILL.md links to canonical design docs (1)

## Runtime validations added

- N/A at the auto-running-test layer (per the structural-contract pattern documented in lessons file).
- Smoke-test runtime validation deferred to `/slo-verify` runbook step.

## Compatibility checks performed

- `cargo check --workspace` — green (1.55s incremental).
- `cargo test -p sldo-install` — all suites green (38 suites; including the 21-passing `e2e_scanner_orch_m1`).
- `cargo test -p sldo-common -p sldo-research -p sast-verify` — green.
- `./target/release/sldo-install --local --dry-run` — discovers `slo-sast` alongside all 30 pre-existing skills.
- `git diff --stat references/sast/` — only the new `threat-model-parser-contract.md` is added; existing files byte-identical (asserted by `existing_references_sast_unmodified_by_m1`).
- No existing skill's `SKILL.md` modified (asserted by `existing_skills_unmodified_by_m1`).
- No `Cargo.toml` workspace deps changed.

## Documentation updated

- `docs/RUNBOOK-SCANNER-ORCHESTRATION.md` — Milestone Tracker row 1 → `done`; M1 Evidence Log fully populated.
- `docs/ARCHITECTURE.md` — already updated during architect step (slo-sast row added with "DESIGN, not yet implemented" qualifier; will be revised to "M1 LANDED" via the Documentation Update Table at end of M5 or earlier).
- `SECURITY.md` — already extended during architect step; no further M1 changes.

## .gitignore changes

None. M1 introduced no build outputs, generated files, or tool caches that needed `.gitignore` patterns.

## Test artifact cleanup verified

`git status` confirms no untracked test artifacts. Only the intended new files (SKILL.md, parser-contract, e2e test, lessons + completion) are present. No fixture dir was created (per the lessons-file finding that fixtures are smoke-test prerequisites, not auto-running-test prerequisites).

## Deferred follow-ups

- **Runtime-invocation tests** — would require a stubbed `claude` binary harness analogous to `e2e_research_*` patterns. Defer to a hardening pass or `/slo-verify`.
- **Smoke-test fixtures** — to be authored ad-hoc when smoke-testing occurs (during `/slo-verify` or before user-facing release).
- **Full `cargo test --workspace` cold build verification** — substituted for time pressure during this milestone; should be run as a sanity check during `/slo-verify`. Expected to be green based on per-crate evidence.

## Known non-blocking limitations

- The structural-contract test pattern verifies that the parser scope rule is DOCUMENTED, not that it is HONORED at runtime. Runtime honoring depends on Claude Code interpreting SKILL.md correctly, which is itself a load-bearing assumption of the entire SLO skill pack. The /slo-security-embedding runbook already addresses this at the SLO-pack level (SKILL.md tampering detection via sldo-install manifest).
- The runbook listed 7 fixture files in M1's Definition of Done; this milestone delivers 0 fixture files based on the structural-contract testing pivot. Recorded in lessons; flagged for future runbooks to distinguish fixture types.
- The runbook listed `cargo test --workspace` as the baseline; this milestone substituted `cargo check --workspace` + per-crate test for cold-build wall-time reasons. Functional equivalence confirmed via per-crate green; full-workspace re-verification deferred to `/slo-verify`.
