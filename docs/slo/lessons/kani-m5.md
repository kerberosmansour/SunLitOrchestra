# Lessons Learned — kani Milestone 5

## What changed
- TLA+↔Kani pairing doc (`docs/slo/design/kani-verification-kani-pairing.md`) with the refinement map + boundary invariant + a worked example using M4's `check_gcd_contract` / `check_zero_prefix`. Local deep-verification reference (`skills/slo-kani/references/local-deep-verification.md`) with quick + deep tiers, pinned toolchain, deep-before-release. Reciprocal `/slo-kani` note in `/slo-tla`; local-verify dispatch row in `/slo-kani`; `/slo-kani` added to the LOOPS-ENGINEERING sprint loop. Structural test `kani_m5_pairing.rs` (5 assertions incl. a no-Kani-CI guard).

## Design decisions and why
- **Replaced the original CI-split design with a local workflow** (user decision): no nightly/PR Kani CI in v1. The `no_kani_ci_workflow_added` assertion makes that decision *mechanically enforced* — a future Kani workflow added without revisiting the decision would fail the test.
- The boundary invariant ("Kani never claims what TLA+ owns") is the load-bearing sentence — asserted verbatim so the pairing can never be documented in a way that lets Kani overclaim concurrency.
- The worked example reuses M4's real harnesses rather than inventing fictional ones, so the pairing doc is grounded in something that actually verified.

## Mistakes made
- The reciprocal note added to `slo-tla/SKILL.md` was a 7-line paragraph that pushed the file from 150 → 158 lines, breaking `sldo-install`'s `e2e_eng_imp_m3::slo_tla_skill_md_at_or_under_150_lines_without_exception` (the thin-dispatcher ≤150-line budget). Caught at `/slo-ship` by the FULL baseline suite (not by the per-crate `sast-verify` runs during M5). Fixed by folding the `/slo-kani` reference into the existing 4-line handoff paragraph (net 0 lines added; file back to exactly 150).
- The M3 §5.8 edit to `docs/slo/templates/runbook-template_v_4_template.md` was NOT mirrored to `skills/slo-plan/references/runbook-template_v_4_template.md`; `e2e_v4_template::v4_skill_local_copy_matches_docs_mirror` requires the two to be byte-identical. Also surfaced only at `/slo-ship` by the full suite. Fixed by copying docs → skill reference.

## Root causes
- `slo-tla/SKILL.md` was already at its 150-line budget; ANY net line addition breaks the thin-dispatcher test. A "small additive note" to a budget-constrained file is not free. The per-crate test runs during a milestone can miss cross-crate assertions — only the full baseline catches them.

## What was harder than expected
- Nothing technical. The conceptual care was in stating the refinement relationship precisely enough to be both a useful template and a testable contract.

## Naming conventions established
- Pairing doc: `docs/slo/design/<slug>-kani-pairing.md`; refinement-map columns: TLA+ action → Rust fn → Kani harness → bound.
- Local-verify tiers: "quick" (small bounds, inner loop) and "deep" (larger bounds, before release).

## Test patterns that worked well
- The no-CI guard (`no_kani_ci_workflow_added`) turns a *decision to NOT do something* into an enforced invariant — a useful pattern for "we deliberately deferred X" choices.
- Extracting the pinned version from `tools.toml` in the test (rather than hardcoding) keeps the local-verify doc and the pin in sync automatically.

## Missing tests that should exist now
- None. The runbook is complete; future CI work (if revisited) would add its own milestone + tests.

## Rules for the next milestone
- N/A — final milestone. Next step is `/slo-ship`.

## Template improvements suggested
- The "enforce a deferral decision with a structural test" pattern (no_kani_ci_workflow_added) is worth reusing whenever a runbook deliberately scopes something out.

## filed_issues
- none — final milestone; lessons captured in-file. The only external follow-up (publish the demo repo) was completed by the user during M5.
