# Lessons Learned — scanner-orch Milestone 2

## What changed

- New reference `references/sast/scanner-orch-pinned-rules-sha.md` — pinned-SHA value (currently the all-zero placeholder pending first wedge-validation pin), 40-char SHA-only enforcement, bump procedure.
- New reference `references/sast/stack-detection-contract.md` — manifest priority order (8 manifest types), tag derivation rules, polyglot behavior, default-fallback path.
- Extended `skills/slo-sast/SKILL.md` — added Outputs (M2) section, Stack Detection / Registry Fetch / Rule Filter sub-sections under "Method (M2 — stack detection + registry fetch + rule filter)", and M2-specific anti-patterns (no shell-string subprocess, no autofix, no parsed-data caching, no SaaS fallback).
- New test `crates/sldo-install/tests/e2e_scanner_orch_m2.rs` — 22 structural-contract tests asserting both reference docs + the SKILL.md M2 additions.
- Asks applied to runbook before implementation: ENG-2 (BDD `cache_hit_skips_clone` allows `git rev-parse`), ENG-5 (synthetic fixtures), ENG-8 (no-op — M1 tests already structural-contract style), CEO-1 (wedge-validation step in smoke tests), SEC-2 (billion-laughs YAML BDD + step), SEC-6 (argv-list discipline in forbidden shortcuts).

## Design decisions and why

- **ENG-8 migration was a no-op.** My M1 tests assert documented properties of SKILL.md content, not stdout shape. They survive M2 additions without modification. The "M1 tests will fail when M2 lands" failure mode that ENG-8 worried about doesn't apply to structural-contract testing. Documented in this lessons file so future runbook reviewers know to evaluate M1→M2 contract evolution risk against the actual test pattern.
- **Placeholder pinned-SHA.** The runbook documents a 40-char SHA constraint, but landing M2 in a session without doing wedge-validation against a real `semgrep-rules` SHA would be premature. The all-zero placeholder is intentionally invalid; the skill MUST refuse to operate against it; the first real bump-PR (M5 dogfood prep) replaces it. This decouples the milestone landing from the wedge-validation step (CEO-1).
- **Synthetic fixtures over copied upstream rules.** Per ENG-5, the M2 test fixtures are handcrafted YAML matching the Semgrep registry schema, NOT byte-copied from `semgrep-rules` upstream. Eliminates the Semgrep Rules License attribution question. Worth noting in lessons because it diverges from the runbook's original "fixture rule YAMLs" framing.
- **`regex` is not a transitive dev-dependency of sldo-install.** First test draft used `regex::Regex::new`; compile failed with "unresolved module `regex`". Refactored to use `chars().all(...)` instead. Lesson: even if a crate is in `[workspace.dependencies]`, individual crates must declare it in their own `[dev-dependencies]` to use it. The runbook's "no new dependencies" promise held at the workspace level — no Cargo.toml edits needed.

## Mistakes made

- **Initial `regex` usage** — assumed transitive availability from workspace deps; required iteration.
- **First pinned-SHA doc draft used "tag reference (develop, main, ...)" loose phrasing** — didn't include the literal word "branch", so the structural-contract test `pinned_sha_doc_documents_sha_only_enforcement` failed. One-line fix to add explicit branch refusal language; took one iteration.

## Root causes

- **Workspace dep declaration vs per-crate dev-dep declaration.** Easy to confuse. Documented as a rule for the next milestone.
- **Test-asserts-on-keyword-presence is fragile to phrasing variations.** My tests use `doc.contains("branch")` etc. which requires exact word presence. More robust would be regex-based or sentinel-based checks, but those add complexity. The keyword-presence pattern is acceptable for v1; if a future milestone has many such fragility issues, refactor to sentinels.

## What was harder than expected

- **Cold-build avoidance.** Continuing the M1 pivot to per-crate `cargo test -p sldo-install --test <test>` for fast iteration. The M2 work needed only sldo-install rebuilds (~0.3s incremental) for each iteration.

## Naming conventions established

- Reference docs that are skill-specific: `references/sast/scanner-orch-<topic>.md` (e.g., `scanner-orch-pinned-rules-sha.md`).
- Reference docs that are cross-skill (general): no `scanner-orch-` prefix (e.g., `threat-model-parser-contract.md`).
- Test naming: `<purpose>_<assertion>` (e.g., `pinned_sha_doc_documents_bump_procedure`, `stack_contract_documents_polyglot_behavior`).

## Test patterns that worked well

- **Helper functions per artifact** (`pinned_sha_doc()`, `stack_contract()`, `skill_md()`) — reads the file once per test invocation, keeps assertions terse.
- **Existence + content + stability triple** — for each new reference doc: assert it exists (size check), assert its required content is present (multiple keyword checks), assert it's marked `stable` (downstream lock-in claim). Three angles cover most documentation-quality failures.
- **Regression sentinel** — `existing_references_sast_unmodified_by_m2` picks one durable phrase per pre-existing file, asserts they're all still present. Catches accidental edits.

## Missing tests that should exist now

- **Runtime YAML-parser-safety test.** Currently `skill_md_documents_yaml_parser_safety` asserts the SKILL.md mentions billion-laughs / entity expansion. It does NOT assert that runtime YAML parsing actually rejects a billion-laughs payload. Defer to `/slo-verify` runtime smoke (which can call `serde_yaml_ng::from_str` directly against a fixture).
- **Cache-isolation runtime test.** `XDG_CACHE_HOME` override is documented but not exercised at the auto-running-test layer. Defer to `/slo-verify`.
- **Argv-list discipline runtime check.** Documented as anti-pattern but a tampered SKILL.md could still describe shell-string interpolation; the test asserts the description is correct, not that runtime invocation honors it. Defer.

## Rules for the next milestone

- **M3 must extend SKILL.md with Emission section.** Use the same incremental-extend pattern (add new sections; don't rewrite existing M1+M2 sections). The Emission section cites M3-specific reference docs.
- **M3's structural-contract test will be the most demanding.** It asserts the workflow YAML safety contract (no `pull_request_target`, `permissions: {}` scope, SHA-pinned actions, `fetch-depth: 0`, `SEMGREP_RULES` env var, no `secrets.*`). The test must parse the EMITTED YAML (in addition to asserting documented properties) — but at the structural-contract layer, "emitted YAML" doesn't exist (the skill emits at runtime, not test time). Solve by: assert the workflow TEMPLATE at `references/sast/scanner-orch-workflow-template.yml` already satisfies all safety properties (because the skill emits it verbatim with only SHA substitution). Template-as-frozen-fixture is the cleanest approach.
- **Beware Cargo dev-dep needs.** Adding any new crate to a test file requires `[dev-dependencies]` in the consuming crate, not workspace deps alone.
- **`cargo test -p sldo-install` continues to be the milestone-close test command.** Fast (sub-second incremental), full coverage of new tests + regressions.

## Template improvements suggested

- The runbook v3 Contract Block "New dependencies allowed" row should distinguish workspace-deps (already present) from per-crate dev-deps (need explicit Cargo.toml addition). Currently the row says "none — uses workspace deps" but that's not enough information to know whether a `regex` import will compile.
- The Contract Block's "Forbidden shortcuts" row keeps growing — by M5 it will be a 15-item list. Consider hoisting load-bearing forbids into a "Global Red Lines for this runbook" section earlier in the doc, with the Contract Block citing it.
