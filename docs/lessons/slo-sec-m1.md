# Lessons Learned — slo-sec Milestone 1

## What changed

- `skills/slo-ideate/SKILL.md` gained a 7th forcing question (worst-day / breach / compliance fine / prolonged outage) and a `## Top risks` block in the idea-doc shape; stop-rule updated; one anti-pattern added (generic Top risks are rejected).
- `skills/slo-architect/SKILL.md` gained a Step 3.5 that (a) does a STRIDE sweep on the diagram, (b) generates `SECURITY.md` at the target repo root via `references/SECURITY-md-template.md`, (c) generates `docs/design/<slug>-threat-model.md` via `references/threat-model-template.md`, (d) sets three new frontmatter keys (`security_libs_required`, `ai_component`, `compliance`) with documented types, defaults, and the allowed vocabulary for `compliance`, and (e) documents idempotency on re-run (detect existing, diff, prompt for overwrite/merge/skip — never silent clobber).
- New `skills/slo-architect/references/SECURITY-md-template.md` (~200 lines) — the canonical SECURITY.md template with placeholder tokens and the `~~~text` fence rule for user-provided strings.
- New `skills/slo-architect/references/threat-model-template.md` (~150 lines) — the canonical threat-model template with STRIDE table, abuse cases, compliance mapping (SOC 2 + ASVS default; GDPR gets a section when opted in), AI triad conditional on `ai_component: true`, and residual-risks table.
- New `crates/sldo-install/tests/e2e_slo_sec_m1.rs` — 23 structural-contract tests: 12 back-compat / invariant / helpers pass at baseline; 11 feature tests fail at baseline and pass after implementation.
- Dogfood artifacts committed: `SECURITY.md` at SLO repo root and `docs/design/slo-security-embedding-threat-model.md` generated manually by executing the updated `/slo-architect` Step 3.5 contract against SLO itself.
- `docs/design/slo-security-embedding-overview.md` frontmatter now includes `ai_component: true` and `compliance: [soc2, asvs]` — this update was outside M1's stated file-allow-list and is called out as a scope-boundary judgment in the Evidence Log (natural continuation of the architect re-run authorized by the dogfood smoke).

## Design decisions and why

- **Generator-first, not prompt-first.** The architect emits `SECURITY.md` and the threat model with placeholders filled; the user reviews. Rationale: Google PSC's 80/20 burden framing — if the security team (architect skill) doesn't generate 80% of the artifact, developer time is spent on the wrong things. Prompting the user to author STRIDE rows from scratch defeats the point.
- **`~~~text` fence rule is load-bearing.** User-provided strings from idea docs are always rendered inside a `~~~text` fenced code block in the generated templates. Rationale: template-placeholder injection is the real attack surface (f10 from critique) — a hostile or unwary idea doc containing triple-backticks, `}}`, or HTML could otherwise smuggle prompt content into the project's security defaults.
- **Reference templates as separate files, not inline in `SKILL.md`.** The SECURITY-md and threat-model templates live at `skills/slo-architect/references/*.md` rather than inside `SKILL.md` prose. Rationale: templates have their own maintenance cadence (track OWASP ASVS version bumps, SunLitSecureLibraries API changes, MITRE ATLAS updates); inlining them would make every template edit look like a skill-behavior change.
- **Conditional AI triad section.** The MITRE ATLAS + OWASP LLM Top 10 + NIST AI RMF triad is gated on `ai_component: true` in the overview frontmatter. Rationale: firing the triad unconditionally dilutes its signal — most runbooks don't embed LLMs. Gating ensures it appears where it matters (and it does matter for SLO itself, where the dogfood fires the triad).
- **Four allowed data classifications, fixed enum.** `Public`, `Internal`, `Confidential`, `Restricted`. Rationale: free-form data-classification vocabularies drift across teams; a fixed enum lets `/slo-critique` and `/slo-verify` reason about classification consistently. Pinned in the overview frontmatter `compliance:` key vocabulary.
- **Backward compatibility via absent-key defaults.** Existing overview files (e.g., `tla-sha-autopop-overview.md`) do not have the three new keys. The type checker treats absent keys as defaults (`security_libs_required: false`, `ai_component: false`, `compliance: [soc2, asvs]`) so old overviews still pass. Rationale: changing the schema without migration would break every existing feature's pipeline; defaults are a clean migration strategy.
- **Dogfood SLO itself.** The manual smoke test for `/slo-architect` runs against SLO's own repo, producing a real `SECURITY.md` and threat model that are committed. Rationale: SLO dogfooding is the strongest signal the skill works end-to-end on a real target. A fixture repo under `tests/fixtures/` would be an artificial stand-in. Trade-off: updating the overview frontmatter crossed the milestone's stated allow-list; flagged in Evidence Log rather than silently bent.

## Mistakes made

- Initially wrote the E2E backend command as `cargo test --test e2e_slo_sec_m<N>` (workspace-level path); critique persona `eng` flagged the convention divergence from prior `slo-sp` runbooks, which use `crates/sldo-install/tests/e2e_slo_sp_m<N>.rs`. Corrected during critique before execution started; no rework required.
- Almost shipped the runbook with content-hash fixture capture "at milestone start" as the invariant mechanism; critique flagged that the fixture capture step was missing from the Step-by-Step and the timing was ambiguous. Swapped to inline const SHA-256 (for M2+) and inline expected-string (for M3/M4) patterns before execution began — cleaner and does not depend on execution timing.
- Accidentally embedded `Register in Cargo.toml` language in multiple places in the runbook during initial authoring; auto-discovery under `crates/sldo-install/tests/` makes registration unnecessary. Found by grep; fixed before M1 coded.

## Root causes

- The convention drift (workspace `tests/` vs. per-crate `crates/sldo-install/tests/`) arose because prior research / plan / run runbooks use workspace-level tests (they don't validate skill-pack content; their tests target the `sldo-*` binaries themselves), while `slo-sp-m*` runbooks are the only prior skill-pack work and naturally live in the install crate. Without reading the `slo-sp-m2` lessons file before writing the runbook, the convention mismatch was easy to miss. **Rule: read `docs/lessons/slo-sp-*.md` before writing any runbook that edits SKILL.md files** — those lessons are the canonical skill-pack prior art.
- The fixture-capture timing ambiguity came from copying the "golden file" pattern from other test frameworks where the fixture is recorded at test-authoring time; in Rust integration tests the distinction between "authoring time" and "run time" collapses to the `cargo test` invocation, so the test needs its expected value baked in as a `const` rather than compared against a filesystem fixture.

## What was harder than expected

- Deciding what level of detail belonged in the `SECURITY.md` template vs. the threat-model template. The SECURITY.md is project-wide defaults (Crypto policy, Auth model, Input handling); the threat model is per-feature STRIDE + abuse cases + compliance mapping. Kept them distinct by keeping SECURITY.md stack-aware (library vocabulary changes per stack) and the threat model surface-specific (abuse cases per new endpoint / handler / subprocess).
- Writing a regex-based frontmatter type checker in Rust without pulling `serde_yaml` as a dep. Landed on `strip_prefix` + manual list parsing for `compliance: [soc2, asvs]` syntax. Imperfect (doesn't handle multi-line YAML lists) but sufficient for M1's contract — the overview files always use flow-style lists.
- Balancing the dogfood smoke against the allow-list. The smoke authorizes producing `SECURITY.md` + threat-model.md at SLO's root, but completing the threat model with an AI triad section required updating the overview frontmatter — a file not explicitly in the M1 allow-list. Surfaced rather than silently resolved; user can course-correct.

## Naming conventions established

- Skill-pack validation tests live at `crates/sldo-install/tests/e2e_slo_sec_m<N>.rs` (integration tests of the `sldo-install` crate). No root `Cargo.toml` `[[test]]` entry needed — cargo auto-discovers.
- Reference templates live at `skills/<skill>/references/*.md` (new convention; `slo-architect` is the first to use it). Previously only `slo-ideate/examples/` existed.
- Dogfood artifacts committed to SLO's tree when they are intentional outputs of a runbook milestone. `SECURITY.md` at repo root is the canonical location; threat models go in `docs/design/`.
- Overview frontmatter keys for security: `security_libs_required`, `ai_component`, `compliance`. All three documented in `docs/design/slo-security-embedding-interfaces.md` as the canonical contract.

## Test patterns that worked well

- **Regex-free Markdown section checks.** `body.contains("## Top risks")` is less fragile than a YAML or Markdown parser and reads clearly when the assertion fires. Matches the prior `slo-sp` test style.
- **Inline content-hash invariants via `assert_eq!` on `include_str!`-style baked strings.** Avoids fixture-capture-timing bugs. Deferred the actual use to M2/M3/M4 tests, but the pattern is documented in the runbook so the agent coding those milestones doesn't re-invent.
- **Minimal YAML-ish frontmatter type checker** (`check_overview_frontmatter`). Handles the three new keys with `strip_prefix` + list-split. Tested against both good and three distinct bad cases (string bool, int bool, scalar list, unknown list value). Passes on the existing `tla-sha-autopop-overview.md` which has none of the new keys — proving backward compat.
- **Dogfood as the smoke test.** Running `/slo-architect` against SLO itself (by executing the contract manually in this session) produced `SECURITY.md` + threat-model.md with real content, not filler — the same test as would apply to any downstream user.

## Missing tests that should exist now

- A runtime test that actually invokes a future `/slo-architect` agent session and diffs its output against the template placeholders. Today, the test asserts template shape; it does not assert agent behavior. Deferred because invoking a Claude Code session non-interactively from a Rust test is not yet wired.
- A test that `/slo-architect` on a re-run truly detects existing `SECURITY.md` and prompts. Today only the SKILL.md prose is asserted to document the rule. Enforcement requires an agent harness.
- A test that the `~~~text` fence rule is actually applied at runtime when user content contains triple-backticks. Same blocker — requires runtime invocation.
- A generative test that pseudo-randomly constructs malformed overview frontmatter and asserts the type checker rejects it. Property-based testing via `proptest` would be cheap and high-signal; deferred to a future hardening pass.

## Rules for the next milestone (M2 — `/slo-plan` Contract Block expansion)

- **Read `docs/lessons/slo-sp-m2.md` AND this file before writing M2 tests.** The slo-sp convention is the test-location canon; this file captures the inline-const invariant pattern that M2's `template_not_modified` relies on.
- **Do not modify `docs/runbook-template_v_3_template.md`.** It is load-bearing for backward compat with every existing runbook. M2's contract is to edit `/slo-plan`'s SKILL.md so it emits richer Contract Blocks for NEW runbooks, not to change the template. Use `include_str!` to pull the current template content into the test, compute SHA-256, and compare against a `const` captured before M2 edits.
- **The three new Contract Block rows are required-when-new-surface; N/A-with-reason is acceptable for refactor-only or doc-only milestones.** Silent row omission is the anti-pattern. Document this in the SKILL.md edit and enforce it in at least one BDD scenario.
- **The proactive-controls vocabulary file should mirror SunLitSecureLibraries crate names directly (`secure_boundary`, `secure_data`, etc.) for Rust-axum targets.** Don't re-coin names. For Pulumi/AWS, use Hulumi component names. For other stacks, fall back to OWASP Proactive Controls v3 category names.
- **Check the absence of shell metacharacters in any vocabulary table cell** at lint time via the structural-contract test. Vocabulary values are substituted into runbook tables; injection through vocab is a real surface.
- **Compute the SHA-256 of `docs/runbook-template_v_3_template.md` with `sha256sum < docs/runbook-template_v_3_template.md` (macOS: `shasum -a 256`) BEFORE editing anything in M2, paste it into the test as a `const`, and commit.** This is Step 0 of M2's Step-by-Step.

## Template improvements suggested

- Runbook v3 template's "Required Test Coverage Categories" list does not include "abuse case" today. M2 adds it by editing the SKILL.md, but the runbook template itself lags. Consider adding a M5/future milestone to align the template — or keep skill-driven emission as the only source of truth. Deferred.
- The Evidence Log template has a fixed shape; adding a "Scope-boundary note" row (like M1's final row) requires agents to know to add it. Consider making this an optional but documented row in the template's Evidence Log template. Deferred.
