# Lessons Learned — slo-sec Milestone 2

## What changed

- `skills/slo-plan/SKILL.md` — Contract Block step documents three new required rows (Data classification, Proactive controls in play, Abuse acceptance scenarios). BDD categories list gains `abuse case`. Three new anti-patterns added (silent row omission, vague abuse cases, free-form vocabulary).
- `skills/slo-plan/references/proactive-controls-vocabulary.md` (NEW) — canonical vocabulary: fixed four-value Data classification enum; stack-aware Proactive controls (Rust-axum → SunLitSecureLibraries crate names + OWASP C-numbers; Pulumi/AWS → Hulumi component names; other stacks → OWASP Proactive Controls v3 category names).
- `skills/slo-plan/references/abuse-case-examples.md` (NEW) — six worked Given/When/Then rows covering six surface classes (HTTP / IPC / file write / subprocess / outbound request / persisted state), plus two AI-specific rows for when `ai_component: true`.
- `crates/sldo-install/tests/e2e_slo_sec_m2.rs` (NEW) — 16 structural-contract tests, including a non-cryptographic FNV-1a-64 invariant that catches any byte edit to `docs/runbook-template_v_3_template.md`.

## Design decisions and why

- **FNV-1a-64 template invariant, not SHA-256.** The critique finding (f3) asked for an inline golden hash to catch template drift. Using `sha2` required adding a dev-dependency; the milestone contract allows none. FNV-1a-64 is non-cryptographic but stable, implementable in ~8 lines of Rust, and sufficient for drift detection (threat model is "accidental edit during M2", not "adversarial modification"). Paired with a byte-length assertion — if anyone finds an FNV collision that preserves byte length on a 30KB template, we'd still catch it by reading the diff.
- **Vocabulary file covers three stack classes, not just one.** Rust-axum gets SunLitSecureLibraries; Pulumi/AWS gets Hulumi; other stacks fall back to OWASP Proactive Controls v3. Rationale: the vocabulary file is cited by `/slo-plan` on every milestone regardless of target stack; if it only covered Rust-axum, non-Rust projects would have no canonical source.
- **Abuse-case examples file includes AI-specific rows in a separate sub-section.** `ai_component: true` milestones get an additional prompt-injection + context-poisoning pair. Keeping them separate from the six surface-class examples means non-AI runbooks don't get diluted by AI-specific content.
- **Anti-pattern on silent row omission.** Added explicitly to the SKILL.md because the failure mode is predictable — an agent completes a doc-only milestone and leaves the three rows blank. The SKILL.md now documents the `N/A — no new surface introduced, see <reason>` acceptable fill and flags silent omission as the anti-pattern.

## Mistakes made

- Initially wrote the vocabulary file without the "values are Markdown-literal, never spliced into shell" safety note; M2 BDD #12 caught the omission. Added the note before implementation closed.
- First pass of `abuse_case_examples_cover_six_surface_classes` test used too-narrow keyword lists (e.g., only matched the literal phrase "HTTP endpoint"); broadened to include synonyms ("ssrf", "endpoint") so the test passes without dictating prose style.

## Root causes

- Vocabulary-file safety note omission: drafted the file focused on the positive content (what values exist) rather than the safety boundary (what values are NOT). The "defense-in-depth" framing needs to be first-class when writing any reference file that gets substituted into downstream tables.
- Narrow test keywords: writing the test before the reference file made the keywords reflect my anticipated wording; loosening them lets the prose evolve without breaking the test.

## What was harder than expected

- Deciding whether the FNV-1a hash should be in the test or a helper crate. Inlined in the test because it's 8 lines and reuse would be premature. Future M3/M4 may copy the helper — acceptable duplication until it reaches three call sites.
- Keeping the vocabulary file generic enough to cover three stack classes without becoming a generic document. Landed on explicit sub-headings per stack class + a shared "anti-patterns" section applying to all. Reads cleanly enough.

## Naming conventions established

- Reference files under `skills/slo-plan/references/*.md` follow the same convention M1 established under `skills/slo-architect/references/*.md`. Each file is single-topic and cited from the parent SKILL.md by path.
- FNV-1a-64 invariant constants in structural-contract tests use the naming pattern `EXPECTED_<FILE>_FNV1A_64` and `EXPECTED_<FILE>_BYTE_LEN`. The byte-length companion is a cheap collision check.
- Threat-model row ids follow `tm-<slug>-abuse-N` per the threat-model template from M1. Abuse-case BDD rows in future runbooks cite these ids.

## Test patterns that worked well

- **FNV-1a-64 hash + byte-length pair** for template drift detection. Deterministic; no deps; catches any edit.
- **Loose keyword matching for abuse-case surface classes.** Testing that a reference file *covers* a concept is weaker than testing *specific phrasing*. Broader keyword sets (e.g., `ssrf` OR `endpoint` OR `http`) let prose evolve without breaking the test while still failing if a surface class goes missing.
- **`| Given` case-insensitive count** as a lightweight BDD-row counter. Doesn't require a Markdown table parser; fits the existing `slo-sp` test style.

## Missing tests that should exist now

- A test that `/slo-plan` on a fresh runbook actually emits all three new rows. Today's tests verify the SKILL documents the behavior. Runtime verification requires a Claude Code harness (same blocker as M1).
- A test that rejects a new runbook where the three rows are silently omitted. Structural-contract enforcement post-emission, not pre-emission.
- Round-trip verification of the vocabulary → runbook → critique citation chain. Deferred until M3 rewrites the critique persona (which cites the vocabulary).

## Rules for the next milestone (M3 — `/slo-critique` security persona rewrite)

- **Read `docs/lessons/slo-sec-m1.md` and `slo-sec-m2.md` before writing M3 tests.** M1 established the reference-templates-under-`references/` convention; M2 established the FNV-1a invariant pattern. M3 uses both.
- **The security persona's new output format must cite a bug class from `skills/slo-critique/references/bug-class-catalog.md`.** The catalog should organize by OWASP ASVS 5.0 chapters (V1–V17), with 2–4 named classes per chapter and at least one elimination pattern per class — citing SunLitSecureLibraries crates where the elimination pattern is already implemented.
- **The variant-analysis playbook must document three strategies** (ripgrep / ast-grep / semgrep) with one worked example each. Include an explicit small-codebase exit (< 500 LOC → N/A, no variants possible).
- **Do NOT touch CEO / eng / design personas.** M3 touches only the security persona file; use inline expected-string constants to lock CEO / eng / design content byte-for-byte.
- **Do not invent a new bug-class taxonomy.** Use OWASP ASVS 5.0 chapter names as the top-level organization. Coining a parallel scheme dilutes the canonical citation chain.
- **The finding-row schema in `skills/slo-critique/SKILL.md` must remain unchanged** (id / persona / category / runbook section / finding / concrete scenario / recommendation). Use an inline expected-string const for this header row.
- **Compute the FNV-1a-64 hashes of `ceo.md`, `eng.md`, `design.md` BEFORE editing anything in M3**, paste into test constants, commit. Same Step 0 pattern as M2.

## Template improvements suggested

- `docs/runbook-template_v_3_template.md` could have a sample Contract Block that already includes the three new rows — but that would require a runbook-template version bump, which the contract forbids in this runbook. Deferred.
- The Evidence Log template could gain a "Scope-boundary note" row (like M1's and M2's final explicit Partial rows for smoke). Deferred until the template is revisited outside this runbook.
