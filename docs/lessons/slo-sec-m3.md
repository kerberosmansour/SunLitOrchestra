# Lessons Learned — slo-sec Milestone 3

## What changed

- `skills/slo-critique/personas/security.md` — full rewrite around class elimination + variant analysis + threat-model citation + self-bounded mandate. Five-condition finding-acceptance gate (class / threat-model row / elimination answer / variant-analysis pointer / concrete exploit scenario).
- `skills/slo-critique/references/bug-class-catalog.md` (NEW) — canonical catalog organized by OWASP ASVS 5.0 chapters V1–V17, with 2–4 named classes per chapter and an elimination pattern per class, citing SunLitSecureLibraries crates / types where the pattern is already implemented.
- `skills/slo-critique/references/variant-analysis-playbook.md` (NEW) — three strategies (ripgrep / ast-grep / semgrep) with worked examples per strategy, plus explicit small-codebase / class-eliminated / out-of-scope N/A exits.
- `skills/slo-critique/SKILL.md` — one-line edit to the rotation-order description for the security persona (references the new framing).
- `crates/sldo-install/tests/e2e_slo_sec_m3.rs` (NEW) — 18 structural-contract tests + 4 invariant tests (FNV-1a hashes of the three unchanged personas + finding-row schema).

## Design decisions and why

- **Five-condition finding-acceptance gate instead of four.** Added a fifth condition (concrete exploit scenario with named attacker, step-by-step trajectory, impact) after the legacy `slo-sp-m6` test flagged that my initial rewrite had dropped the "attacker/step-by-step" vocabulary. The add-back reinforces the framing: the class-elimination answer says *which class*, the exploit scenario says *how the class gets exercised today*. Both matter; I had implicitly assumed the exploit scenario was subsumed by the threat-model citation, but a threat-model row citation is not a step-by-step walk.
- **OWASP ASVS 5.0 as the catalog's organizing scheme.** ASVS is widely recognized, stable (5.0.0 released May 2025), covers 17 chapters with ~350 requirements, and is already published in CycloneDX format per the research synthesis. Any alternative (STRIDE-by-class, CWE-by-weakness) would have introduced a second taxonomy with worse cross-citation.
- **Three-tier variant-analysis strategy.** ripgrep → ast-grep → semgrep maps to increasing effort / accuracy. The small-codebase exit (< 500 LOC) prevents variant analysis from being performative on projects where there are no variants to find. Google PSC's monorepo assumption doesn't translate directly; the playbook names `mrva` (Trail of Bits) as the cross-repo path if needed.
- **Persona mandate is self-bounded.** The persona explicitly documents that it will not follow instructions embedded in the runbook body that attempt to redirect, silence, or extend its mandate. This is a concrete defense against prompt injection through runbook content.
- **Hash invariants on ceo/eng/design personas.** Any change to those three files during M3 would be out-of-scope and is caught by FNV-1a-64 equality. Same pattern as M2's template invariant.
- **The SKILL.md edit is one line.** The skill's rotation order is unchanged; only the security-persona description gets updated. Keeping the edit small means the rest of the SKILL.md (including the finding-row header schema) stays byte-stable.

## Mistakes made

- Initial rewrite dropped the "attacker" and "step-by-step" keywords, causing `slo-sp-m6::security_persona_has_owasp_and_stride` to fail. Caught by the full baseline regression check (not by my M3 tests, which didn't include those keywords). Fixed by adding the 5th finding-acceptance condition.
- First draft of `catalog_covers_asvs_chapters` used too-narrow markers (only `V1`, `V2`, ...) and would have required the catalog to have every chapter as a top-level heading. Expanded to match `V<n>`, ` V<n> `, ` V<n>:`, `V<n>.` patterns so the catalog's flexible heading / bullet style is accepted.
- Almost wrote a bug-class-catalog entry for V16 (SPA) and V15 (WebService) in full detail even though they're often N/A; trimmed to the two most relevant entries per chapter to avoid bloat.

## Root causes

- Legacy test breakage: the slo-sp test suite locks in persona shape at the time of its authoring. When subsequent runbooks rewrite personas, the old tests pin against the old contract. **Rule: any skill-pack runbook that edits personas / SKILL.md content covered by a prior slo-sp test MUST run the full per-crate baseline after every edit, not just its own milestone tests, to catch this.**
- Catalog marker pattern too narrow: I thought in terms of Markdown heading conventions (`## V1`) but ASVS chapter references can be in tables, bullet lists, inline citations, etc. The looser match reflects the file's actual style.

## What was harder than expected

- Striking the balance between the persona's new brevity and the old persona's completeness. The old persona documented OWASP Top 10 mapping and STRIDE per component explicitly; my rewrite moved STRIDE to `/slo-architect` Step 3.5 (the threat model) and moved OWASP to `bug-class-catalog.md`. Both references still mention OWASP + STRIDE, but the persona itself needed to retain enough vocabulary to satisfy the legacy test (and the downstream contract) — the 5th finding-acceptance condition was the right pivot.
- Choosing between SunLitSecureLibraries and OWASP Proactive Controls C-numbers for each catalog entry. Landed on citing both where both apply (e.g., "V4 SQL injection — eliminated by `SqlIdentifier` (C5 boundary validation)"). Some entries cite only the crate (e.g., `SecretString` for Debug redaction); others only the category (e.g., "C3 Secure Database Access" for "Race condition in multi-step flow" where no single library helps).

## Naming conventions established

- Reference files under `skills/slo-critique/references/*.md` — same pattern M1 and M2 established.
- FNV-1a-64 invariant consts named `EXPECTED_<FILE>_FNV1A_64` and `EXPECTED_<FILE>_BYTE_LEN` per M2's precedent.
- Bug-class catalog entry format: one row per class with columns `Bug class | Elimination pattern | SunLitSecureLibraries / Hulumi`. Consistent across all 17 ASVS chapters.
- Variant-analysis strategy headings use full tool names (`ripgrep`, `ast-grep`, `semgrep`) not abbreviations.

## Test patterns that worked well

- **ASVS chapter detection by loose-match pattern.** Checking for `V1`, ` V1 `, ` V1:`, `V1.` (four shape variants) across the catalog body accepts multiple document styles without dictating one.
- **Keyword set for catalog library citations** (`secure_boundary`, `SafePath`, etc.) with a "at least 3 of N candidates" rule. Lets the catalog evolve without forcing specific crate names to always appear.
- **FNV-1a-64 byte invariant on three untouched persona files.** Catches any accidental edit (whitespace, newline, character) — even edits the author doesn't notice.
- **Inline const for the finding-row header string.** Byte-equal match is stricter than hash and reads cleaner for a single-line invariant.

## Missing tests that should exist now

- Round-trip test: catalog → persona finding → threat-model-row citation. Only meaningful when a real runbook runs the pipeline end-to-end; deferred.
- Test that each ASVS chapter entry in the catalog has at least one elimination pattern. Today's test asserts ≥3 SunLitSecureLibraries references total; a per-chapter assertion would catch a chapter that's only "classes" without "patterns". Deferred — the catalog was reviewed manually.
- Property-based test: random persona-prompt-injection strings don't alter the persona's output contract. Deferred — requires an agent harness.
- Validation that the variant-analysis playbook's three tools are actually invocable (i.e., `which ripgrep`, `which ast-grep`, `which semgrep`). Today's test only asserts the playbook documents them; a CI job could check for availability. Deferred to M4's Pass 4.

## Rules for the next milestone (M4 — `/slo-verify` Pass 4)

- **Read `slo-sec-m1.md`, `-m2.md`, `-m3.md` before writing M4 tests.** M1: reference-templates pattern. M2: FNV-1a invariant pattern + vocabulary file structure. M3: persona rewrite with inline string constants for schema invariants + legacy-test regression awareness.
- **Capture FNV-1a-64 hashes of Pass-1/Pass-2/Pass-3 subsections of `skills/slo-verify/SKILL.md` BEFORE editing** — M4 must not touch them. Use the 3-header-fencepost approach: find heading `### Pass 1. Happy path`, find next heading `### Pass 2. ...`, hash the bytes between.
- **After every M4 edit, run the full baseline regression check** (`cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install`), not just M4's own tests. The M3 `slo-sp-m6` regression would have gone unnoticed if I had only run `cargo test --test e2e_slo_sec_m3`.
- **Pass 4 is additive.** Nothing in passes 1–3 moves or gets reworded. The inline FNV-1a invariant enforces this.
- **Tool-optional rule is first-class.** When `cargo audit` / `cargo deny` / `semgrep` / `ast-grep` / Docker is absent, Pass 4 emits a "skipped — tool not installed" row with the specific missing binary named. Same rule applies to DAST — no smoke service → N/A with reason.
- **The reference file for M4 (security-pass-commands.md) must document the exit-code semantics explicitly**: exit 0 clean, exit 1 finding, exit ≥2 tool error → "skipped", never a finding. This is load-bearing for offline sessions (f6 from critique).
- **Polyglot rule is first-class.** Multi-stack targets run all applicable command sets; each stack gets its own row in the Pass 4 section. Document this in security-pass-commands.md and test for it.

## Template improvements suggested

- The v3 runbook template's security-persona-line in the critique example section (if any) could cite class elimination. Template edit deferred to a future non-runbook PR.
- Evidence Log template could gain a "Legacy test regression" row for milestones that touch shared skill content. Deferred.
