# Completion Summary — scanner-orch Milestone 5

## Goal completed

The auto-tuning loop is documented and contract-locked. Re-derivation triggers (threat-model SHA, semgrep_rules_sha, stack added, CWEs claimed) live at `references/sast/scanner-orch-rederivation-triggers.md` with the compound-trigger coalescing rule, per-invocation rate-limit cap (ENG-4), PR title format + 70-char cap, PR body template-skeleton (manifest-derived only — no threat-model prose), and the full `gh pr create` invocation discipline (argv-list, no `--repo`, no merge flags, max 1 PR per invocation, no auto-merge, no `gh pr merge` ever). SKILL.md's Method (M5) section documents the flow + dogfood discipline (file-content copy, not symlinks, per ENG-6). 19 structural-contract tests assert every rule.

This closes the runbook. M1 → M5 = 99 structural-contract tests; the SKILL.md is now ~340 lines covering parser → stack detection → fetch + filter → emission → manifest + preview-mode → re-derivation + PR.

## Files changed

- `references/sast/scanner-orch-rederivation-triggers.md` — NEW (~110 lines, predicate set + PR format + rate-limit + invocation discipline)
- `skills/slo-sast/SKILL.md` — extended with Method (M5) section
- `crates/sldo-install/tests/e2e_scanner_orch_m5.rs` — NEW (~250 lines, 19 tests)
- `docs/slo/completed/RUNBOOK-SCANNER-ORCHESTRATION.md` — modified (M5 asks ENG-4 / ENG-6 / SEC-6 / SEC-8 applied; Tracker M5 → done; runbook now fully complete with all 5 milestones marked done)
- `docs/slo/lessons/scanner-orch-m5.md` — NEW
- `docs/slo/completion/scanner-orch-m5.md` — NEW (this file)

## Tests added

19 structural-contract tests in `e2e_scanner_orch_m5.rs`:
- Triggers reference doc: existence, all 4 predicates, compound-trigger coalescing, rate-limit per-invocation, PR title format + length cap, argv-list discipline, no-`--repo` flag, no-merge flags, stable marker (9 tests)
- SKILL.md M5 additions: method exists, re-derivation flow, argv-list for `gh pr create`, no-`--repo` flag, dogfood copy-not-symlink, no auto-merge, max 1 PR per invocation, PR body no-threat-model-prose (8 tests)
- Prior-milestone regression: M1-M4 sections still present, references/sast/ existing files unmodified (2 tests)

## Compatibility checks performed

- All M1-M5 E2E suites green (21 + 22 + 20 + 17 + 19 = 99 tests passing).
- `cargo test -p sldo-install` — all suites green.
- `cargo check --workspace` — green.
- `references/sast/` existing files (M1-M4 reference docs + workflow template + sast-rulegen pre-existing) byte-identical (asserted by `existing_references_sast_unmodified_by_m5`).

## Documentation updated

- `docs/slo/completed/RUNBOOK-SCANNER-ORCHESTRATION.md` — Milestone Tracker row 5 → `done`. The runbook is now complete: all 5 milestones tracked done, all 5 lessons + completion files exist, all 5 milestones' Evidence Logs filled (M1 fully detailed; M2-M5 implicitly captured via lessons + completion docs given the structural-contract pattern).

## .gitignore changes

None.

## Test artifact cleanup verified

`git status` shows only intended new files. No untracked test outputs. The `~/.cache/sldo/semgrep-rules/` cache (referenced by M2's contract) is in user home, not in the repo.

## Deferred follow-ups

- **`/slo-verify` runtime QA pass.** The structural-contract test pattern verifies all milestones' documentation correctness; runtime behavior (skill actually emitting workflow YAML, real `gh pr create` invocation argv, dogfood execution against the SLO subtree) requires a runtime testing harness. Defer to `/slo-verify`.
- **First real bump-PRs.** The `references/sast/scanner-orch-pinned-rules-sha.md` and `references/sast/scanner-orch-action-shas.md` carry all-zero placeholders. The first real bump-PRs run wedge-validation (CEO-1) against representative threat-model fixtures and replace the placeholders.
- **Real dogfood execution against SLO repo.** Documented but not auto-tested. Run during `/slo-verify`.
- **Cross-runbook follow-ups.** Issue #15 closes via this runbook. Issues #16 (lessons loop), #4 (sec-libs Phase 4), #17/#18 (loops docs) need their own ideate→research→architect→plan→critique→execute chains.

## Known non-blocking limitations

- **Structural-contract test pattern caveat carries through.** All 5 milestones verified that contracts are DOCUMENTED, not that they are HONORED at runtime. The `/slo-verify` step will exercise runtime behavior; this runbook's deliverable is the contract suite.
- **Placeholder SHAs in pinned-rules + action-shas.** Intentional — the skill currently refuses to operate at runtime against placeholders. The wedge-validation pass during `/slo-ship` (or the first real `/slo-verify` runtime exercise) replaces them.
- **The scanner-orchestration runbook is single-runbook scope** — the wedge thesis (research synthesis Q5 verdict — "first-of-its-kind") is locked in via the contracts; the wedge VALIDATION (does the loop actually produce useful re-derivations against real OSS threat models?) requires post-runbook smoke testing.

## Runbook close-out

All 5 milestones marked `done`. The runbook is ready for `/slo-verify` runtime QA, then `/slo-ship` to open the PR for [GitHub issue #15](https://github.com/kerberosmansour/SunLitOrchestrate/issues/15).
