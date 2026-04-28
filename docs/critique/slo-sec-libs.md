---
name: slo-sec-libs
runbook: docs/RUNBOOK-SLO-SEC-LIBS.md
critiqued: 2026-04-28
personas_run: [ceo, eng, security]
design_persona_skipped: yes — no UI surface
---

# Critique — slo-sec-libs

| id | persona | category | runbook section | finding | concrete scenario | recommendation |
|---|---|---|---|---|---|---|
| C-1 | ceo | reduce-scope | M5 dogfood | M5 picks ONE target milestone for dogfood. If the target has no `security_libs_required: true`, dogfood is inconclusive — the BDD already accepts this as empty-state. | Author picks the wrong target; dogfood produces "0 matched, 0 unmatched"; M5 closes inconclusive; the runbook ships without proving end-to-end value. | Pre-flight step: shortlist 2-3 candidate target milestones; pick the one with the richest proactive-controls surface. **auto-fix** applied to M5 pre-flight. |
| C-2 | ceo | hold-scope | Pre-requisites out-of-band | Three one-time setups (intake repo, declarations on Hulumi + SLSL, gh scopes) are listed as pre-requisites BEFORE M1. Treating them as out-of-band means they could slip. | Runbook starts; M1 fails on declarations file missing in Hulumi; the runbook stalls until the upstream repo work catches up. | Acceptable framing — pre-requisites are real prereqs. Reinforce the gating: M1 BDD MUST refuse to proceed without all three pre-requisites. **hold-scope** — already in pre-flight; reinforce in BDD. |
| C-3 | ceo | ask | M1 + M5 between Phase 4 + parallel R1/R2/R3 | This runbook is parallel-track per the bucket mapping. R1's `gh issue create` rate-limit pattern (40/hr) — if R1 ships first, R4 inherits. If R4 ships first, R1 inherits R4's pattern. | Either runbook ships first; the second creates a redundant rate-limit pattern; cleanup needed. | Add explicit "if R1 ships first, R4 cites R1's `references/issue-filing-discipline.md`. If R4 ships first, R1 cites R4's `upstream-filing-discipline.md`." **ask** user — pick the canonical home. |
| C-4 | eng | auto-fix | M1 cache layout `~/.cache/sldo/declarations/<sha>/` | Cache-eviction policy undefined. Same gap as `/slo-sast`. | Cache grows unbounded over months of use; user disk fills; runs slow. | Specify size cap (1 GiB) + age cap (90 days). Eviction is LRU. Document in `references/methodology-m1-reader.md`. **auto-fix** applied to M1 contract. |
| C-5 | eng | hold-scope | M1 Python script provenance | Script is stdlib-only; structural-contract test asserts no non-stdlib imports. License declaration absent (same as R3 B-5). | Future contributor adds `import jsonschema` (which IS the only allowed non-stdlib dep, per pre-flight check); the structural-contract test allow-lists it. Adds `import requests` weeks later under same allow-list pretext; gets through. | Test should allow-list ONLY `jsonschema` as non-stdlib import; everything else FAILS. **hold-scope** — current test is allow-listed correctly; reinforce in test code review. |
| C-6 | eng | ask | M2 capability matcher | Tiebreaker rule "more parametric claims = more specific = preferred" — but if Hulumi advertises Argon2id with iterations≥3 and SLSL with iterations≥4, both are valid; SLSL is "more conservative" but is it "more specific"? | Subtle judgment: M2 picks one as winner; founder follows; later realizes the other was a better fit. | Tie-disposition framing: when both are valid AND specificity is comparable, surface BOTH with `disposition: tie` (already covered in BDD). When specificity is comparable but parametric values differ, prefer the more conservative + flag the choice in stderr. **ask** user — confirm the conservative-by-default rule. |
| C-7 | eng | auto-fix | M3 capability-gap record schema | Schema is regex-validated. But what about gap records that span multiple capabilities (e.g., a runbook needs both Argon2id AND a secure JSON parser; both unmatched)? | Single gap record bundles two unmatched controls; reads ambiguously; downstream consumer (slo-security-intake maintainer) doesn't know which to address first. | Specify: one gap record per unmatched control. The matcher emits N records when N controls are unmatched. **auto-fix** applied to M3 contract. |
| S-1 | security | auto-fix | M3 capability-gap record schema | Regex-validated, but schema doesn't enforce no Unicode tricks (homoglyph, RTL override, zero-width spaces). | Adversary crafts a target repo's runbook proactive-controls row with Unicode tricks; gap record body inherits the prose; downstream issue body confuses the maintainer. | Schema validation includes NFKC normalization + zero-width / RTL-override rejection. **auto-fix** applied to M3 BDD adversarial scenario. |
| S-2 | security | hold-scope | M1 declarations file SHA pinning | SHA pinning + `git rev-parse HEAD` cache integrity check is solid. But `git clone` on cache miss happens against the full upstream repo (Hulumi, SLSL); could be slow + bandwidth-heavy. | First-run user with a slow connection waits 30+ seconds for `git clone`; UX feels broken. | Acceptable cost; `--depth=1` clone helps but doesn't eliminate. Document expected first-run latency in `references/methodology-m1-reader.md`. **hold-scope** — performance is documentation issue, not security. |
| S-3 | security | ask | M4 third-party filing rate-limit cap | 40 issues/hr defensive cap. Phase 1 research Q5 says undocumented; the choice is empirical. | Real-world telemetry from M5 dogfood + early adoption may show 40/hr is too low or too high. | Document refresh-rule: "after 3 months of real use, review the cap based on observed point-cost from runs that brushed the cap." **ask** user — accept review-after-3-months follow-up? |
| S-4 | security | auto-fix | M1 — vendor SaaS API fallbacks | Phase 1 stack-decision rejected vendor SaaS API fallbacks. M1 doesn't enumerate what those would be. | Future contributor "for convenience" wraps Snyk API (or equivalent) into the matcher; explicit ban undocumented in M1. | Enumerate forbidden fallbacks in M1 anti-patterns: Semgrep AppSec, Snyk, GitHub Advanced Security, Veracode, Checkmarx. **auto-fix** applied to M1 contract. |
| S-5 | security | hold-scope | Argv-list discipline | Standard discipline. Same pattern as `/slo-sast` M5 + `/slo-rulegen`. | Solid. | **hold-scope** — confirmed. |

## Auto-fix corrections applied

- C-1: M5 pre-flight enumerates 2-3 candidate target milestones.
- C-4: M1 contract specifies cache size cap (1 GiB) + age cap (90 days) + LRU eviction.
- C-7: M3 contract — one gap record per unmatched control.
- S-1: M3 BDD adversarial scenario includes Unicode-trick rejection.
- S-4: M1 anti-patterns enumerate forbidden vendor SaaS fallbacks.

## Asks for project-owner decision

- **C-3**: where does the canonical `gh issue create` rate-limit discipline live — R1 or R4?
- **C-6**: confirm conservative-by-default tiebreaker in matcher?
- **S-3**: accept review-after-3-months follow-up for the 40/hr cap?

## Final disposition

**Accept with minor edits + asks**. Pre-requisites (C-2) are the largest external dependency. No critical findings.
