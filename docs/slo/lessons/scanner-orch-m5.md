# Lessons Learned — scanner-orch Milestone 5

## What changed

- New reference `references/sast/scanner-orch-rederivation-triggers.md` — four trigger predicates (threat-model SHA, semgrep_rules_sha, stack added, CWEs claimed), compound-trigger coalescing rule, per-invocation rate-limit cap (ENG-4), PR title format + 70-char cap, PR body template (manifest-derived only), full `gh pr create` invocation discipline.
- Extended `skills/slo-sast/SKILL.md` — Method (M5) section with re-derivation flow + PR creation + dogfood discipline + M5-specific anti-patterns (no auto-merge, no cross-repo, no `--repo` flag, no merge flags, copy-not-symlink for dogfood, max 1 PR per invocation).
- New test `crates/sldo-install/tests/e2e_scanner_orch_m5.rs` — 19 structural-contract tests asserting triggers doc + SKILL.md M5 additions + framing discipline + prior-milestone regression.
- Asks applied: ENG-4 (rate limit = max 1 PR per invocation; cross-invocation external), ENG-6 (copy-not-symlink for dogfood), SEC-6 (argv-list for `gh`), SEC-8 (no `--repo` flag).

## Design decisions and why

- **Per-invocation cap, not persistent counter.** ENG-4 surfaced the choice between (a) persisted counter at `~/.cache/sldo/rate-limit-state.json` with file lock, OR (b) max-1-PR-per-invocation with cross-invocation rate as the user's responsibility. Picked (b) — simpler, matches the skill's single-invocation discipline, no new persistence surface, no file-locking bugs. The defense narrative is "the skill does at most one PR per call; runaway loops require external mitigation."
- **`gh` invoked without `--repo` flag.** SEC-8's confused-deputy defense: rely on `gh`'s default origin-based resolution from `git remote get-url origin`. If the user is in a tampered tarball with malicious `.git/config`, `gh` will file against the malicious origin — but at least an explicit `--repo` value cannot make this worse. (A complete defense would also verify `git remote -v` looks expected, but that's deferred.)
- **PR body template-skeleton, no threat-model prose.** Same defense as M3's workflow YAML: only manifest-derived values flow into the PR body. The structural-contract test `skill_md_m5_pr_body_no_threat_model_prose` asserts this is documented.
- **Section-bounded assertions for M5-specific properties.** Like M4, M5's test file uses `let m5_start = skill.find("Method (M5")` to scope assertions to the M5 section, preventing M2's stale references from satisfying M5-specific tests by accident.

## Mistakes made

- The `triggers_doc_forbids_repo_flag` test initially failed because my doc said `NO --repo flag` (no backticks around `--repo`). Adjusted test to accept multiple phrasings:
  - `NO ` `\``--repo`\`` ` flag`
  - `no ` `\``--repo`\``
  - `no \`--repo\` flag` (case-insensitive)
- Took one iteration. Lesson: case-insensitive contains-checks for keyword phrases that don't have a single canonical form.

## Root causes

- Same as M3+M4: structural-contract testing of phrasing variations is fragile to formatting choices. Mitigated by allowing multiple variants in the assertion.

## What was harder than expected

- **`gh` invocation discipline articulation.** Three intersecting rules: argv-list (SEC-6), no-`--repo` (SEC-8), no-merge-flags (M5 contract block). All three needed to be documented in BOTH the triggers reference AND the SKILL.md, with the structural-contract tests asserting each rule independently. ~12 lines of test asserts to cover ~6 lines of doc requirement, but each test pinpoints a specific failure mode.

## Naming conventions established

- M5-specific tests use `triggers_doc_<property>` for triggers-doc assertions, `skill_md_documents_<property>` for SKILL.md M5 assertions.
- The dogfood-fixture path convention: `crates/sldo-install/tests/fixtures/scanner-orch/m5/dogfood-slo-subtree/` (file-content copy at fixture-authoring time, not at test runtime).

## Test patterns that worked well

- **Section-bounded scope** (`let m5_section = &skill[m5_start..]`) — already established in M4, used heavily in M5. Makes M5-specific assertions immune to M2's stale citations.
- **Multi-variant phrasing checks** — `assert!(s.contains(a) || s.contains(b) || s.to_lowercase().contains(c))` — flexible while still asserting the intent.
- **Discipline-rule arrays** — for each forbidden flag (`--auto`, `--squash`, etc.), check the doc enumerates it. Declarative, easy to extend.

## Missing tests that should exist now

- **Runtime `gh pr create` argv inspection.** Documented in M5's BDD scenarios but not exercised at the auto-running-test layer. Defer to `/slo-verify`.
- **Real dogfood test against the live SLO repo subtree.** The runbook's M5 BDD describes this; the structural-contract tests verify the discipline is documented (file-content copy, not symlinks). Defer the actual dogfood execution to `/slo-verify` runtime QA.
- **Trigger-evaluation-correctness test.** No test verifies the four trigger predicates produce the expected output for given target-repo states. Defer to `/slo-verify`.

## Rules for the next milestone

- **There is no next milestone in this runbook.** M5 closes the runbook. Next steps are `/slo-verify` (runtime QA) and `/slo-ship` (PR open).
- **For future scanner-orchestration v2 work** (DAST, two-workflow split, audit-coverage skill, real `/slo-rulegen` integration): each is a fresh runbook. The M5 contract (re-derivation triggers + PR format + invocation discipline) is `stable` — v2 runbooks must preserve it.

## Template improvements suggested

- The runbook v3 template's "Out of Scope / Must Not Do" section grows to be substantive in security-sensitive milestones. Consider hoisting recurring forbids ("no auto-merge", "no `--repo` flag", "no shell-string subprocess") into the runbook's "Global Red Lines" section so each milestone doesn't re-enumerate them.
- The Self-Review Gate could include: "Did I check that all forbidden subprocess flags / merge flags are enumerated in BOTH the reference doc AND the SKILL.md anti-patterns? (Drift between the two is a common failure mode.)"
- For runbooks with a "dogfood test" pattern, the v3 template should include a "Dogfood Isolation" section explicitly mandating file-content copy + tempdir isolation. ENG-6 surfaced this as a real concern; codifying it would prevent future runbooks from re-encountering it.
