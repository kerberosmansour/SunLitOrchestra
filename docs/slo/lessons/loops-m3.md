# Lessons Learned — loops Milestone 3

## What changed
- Extended `skills/slo-retro/SKILL.md` with a new "Issue filing" section that classifies each lesson (`product` / `upstream-OSS` / `slo-process`), dedupes via three-strike `gh search`, files with user confirmation, and falls back to `LESSONS-BACKLOG.md` when `gh` is unavailable.
- Authored `skills/slo-retro/references/issue-filing-discipline.md` locking: marker choice (label `retro-derived`), three-strike dedupe with NFKC + ASCII collapse, body SHA-256 cross-session dedupe, argv-list rule, NO `--repo` rule, 40-issues/hour rate-limit cap with adaptive backoff, full 12-field audit-row schema for `LESSONS-BACKLOG.md`, body content rules (`~~~text` fence, 65,536-char truncation, reference-cycle detection).
- Added structural-contract tests at `crates/sldo-install/tests/e2e_loops_m3.rs` (10 tests asserting the SKILL.md extension + every locked rule in the reference file).
- Updated `ARCHITECTURE.md` Feedback-loops paragraph to cite the new reference file.

## Design decisions and why
- **Marker choice locked: GitHub label `retro-derived`** (with `[retro]` title prefix as a secondary human-scanning aid). Rationale: labels survive title edits, `gh search issues --label X` is more reliable than title-substring matching at scale, and labels are queryable by `/slo-execute` M4 carry-forward without parsing titles. The runbook's M3 spike step asked us to pick between title-prefix / label / body-sentinel; label wins on dedupe reliability.
- **Issue filing is strictly additive.** The lessons file is the always-on artifact. The "## Outputs" section now spells this out so a hurried agent reading only the Outputs block sees the ordering rule.
- **Three-strike dedupe + body SHA-256 cross-session dedupe** instead of a single `gh search`. The runbook's paradigm-driven enhancement section explicitly calls for this multi-layer defense; the marginal cost is zero for an LLM but catches homoglyph / zero-width / RTL evasions plus cross-session duplicates that single-session `gh search` cannot see.
- **NO `--repo` flag** is the M3 enforcement of `/slo-sast` M5's confused-deputy defense. Rather than re-prove the threat model, the reference file inherits explicitly with a one-line citation.
- The full audit-row schema lives in the reference file, not in SKILL.md. SKILL.md is the agent-facing prose; the reference file is the locked structural contract. The structural-contract test grep targets the reference file.

## Mistakes made
- The first SKILL.md edit attempt collided with a stale read because I'd already appended the back-link footer (M1) and the next-step content needed reading the new file shape. The Edit tool blocked correctly. Re-read fixed it on the second try.

## Root causes
- Long-running runbooks that touch the same SKILL.md across multiple milestones need a re-read between edits. The Edit tool's mtime check is the right defense; just a workflow note.

## What was harder than expected
- Picking the marker between three options (label / title prefix / body sentinel) without a real spike on a populated test repo. Decided to lock `label` based on documented reliability of `gh search --label` over title-substring; the runbook authorized a spike but auto-mode let me proceed on a defensible default. Future M3-of-other-runbooks may revisit if the label scheme proves brittle, in which case the reference file is the single point of update and the structural-contract test re-runs.

## Naming conventions established
- Marker label: `retro-derived` (kebab-case, matches GitHub's label naming convention).
- Audit-row body_sha256: 12-hex-char prefix of SHA-256(NFKC + whitespace-collapse(body)). Defined exactly so cross-session dedupe is reproducible.
- Reference file location: `skills/<skill>/references/<topic>.md` (matches existing `skills/slo-architect/references/...` and `skills/slo-sast/references/...` conventions). The new file does NOT mint a separate skill — `sldo-install` only walks `skills/<name>/SKILL.md`.

## Test patterns that worked well
- Structural-contract tests that grep for **exact discipline phrases** in the reference file (e.g., `argv-list`, `NO --repo`, `40 issues`, `body_sha256`, `NFKC`, `LESSONS-BACKLOG.md`, `three-strike`). Each rule has a one-line presence test; if a future edit removes the rule from the file, the test trips immediately.
- The `slo_retro_install_unchanged` test asserts the new reference file lives under `skills/slo-retro/references/`, NOT as a sibling SKILL.md. This guards against accidentally minting a second skill.

## Missing tests that should exist now
- A runtime test that synthesizes a candidate filing with a homoglyph title and asserts the three-strike dedupe catches it. The structural-contract test only verifies the discipline is documented; the runtime behavior of the agent following that discipline is harder to assert without an integration harness.
- A test that asserts every existing `docs/slo/lessons/*.md` file still parses cleanly after M3's filing flow lands (backward-compat sentinel).
- A test that asserts no SKILL.md other than `slo-retro` mentions `LESSONS-BACKLOG.md` (would catch accidental copy-paste of the filing flow into another skill).

## Rules for the next milestone
- M4 introduces `/slo-execute` pre-flight carry-forward + the runbook template's "Carry-forward from prior retros" section. The marker query MUST be `gh issue list --label retro-derived --search "<runbook-prefix>"` — locked here in M3, must not drift.
- M4's `/slo-execute` extension MUST be additive after the existing pre-flight Step 1 (read previous milestone's lessons), not a replacement.
- The `micro | milestone | fresh-runbook` lane vocabulary is introduced in M4 — keep the wording exact across SKILL.md, the runbook template, and the runbook's own dogfood "Carry-forward from prior retros" section.

## Template improvements suggested
- The runbook template's per-milestone "Files Allowed To Change" should explicitly support the pattern "skill SKILL.md + new sibling reference file under skills/<name>/references/" — M3 followed this pattern but had to read the install-discovery rule (`sldo-install` walks only SKILL.md) to confirm the reference file would not be installed.
- The runbook template should provide a default `Forbidden shortcuts` row with `--repo` flag pre-populated, because every `gh`-using milestone re-asserts it.
