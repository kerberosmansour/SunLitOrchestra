# Lessons Learned — kani Milestone 2

## What changed
- Four method-dispatch references (`harness-generation.md`, `run-and-triage.md`, `fallback-strategies.md`, `verified-scope-writeup.md`), seven eval cases, SKILL.md dispatch links activated (removed *(M2)* annotations), and seven new structural assertions in `kani_m1_skill_contract.rs`.

## Design decisions and why
- Embedded the honesty/scope gates as **detailed methodology in the references**, not just one-liners in SKILL.md — so the structural test asserts the *methodology* exists, not just a slogan.
- ENG-2 (fail-closed) and SEC-1 (write-path by construction) live in `run-and-triage.md` and `harness-generation.md` respectively, mirroring where the runtime behavior would be implemented.
- Eval cases mirror the `/slo-tla` seven-category set (happy/adversarial/ambiguous/missing-context/tool-failure/high-risk/outdated) for consistency.

## Mistakes made
- The sound-stub rule was written "**S**ound over-approximating stubs only" (capital S) but the assertion checked the lowercase substring — the exact-substring gotcha from kani-m1, recurring.

## Root causes
- Even with the kani-m1 rule in hand, a sentence-initial capital broke a case-sensitive `contains()`. The durable fix is to lowercase the haystack in the assertion, not to fight English capitalization in prose.

## What was harder than expected
- Nothing structural. The hardest part was choosing contiguous gate phrases that read naturally AND survive a substring check.

## Naming conventions established
- Gate-phrase assertions lowercase the file text before `contains()` (see `slo_kani_sound_stub_rule_documented`, `slo_kani_write_path_validation_documented`). Apply this pattern for all future prose-gate assertions.

## Test patterns that worked well
- One assertion per gate phrase, each naming the threat-model row or finding id (ENG-2 / SEC-1 / abuse-N) in the failure message — makes a red immediately traceable to its requirement.

## Missing tests that should exist now
- None for M2. M3 adds integration-seam assertions (architect key, §5 header, execute/verify/retro hook prose) in a *new* sibling test file `kani_m3_integration.rs`.

## Rules for the next milestone
- **Lowercase the haystack for all prose-gate substring checks** — do not rely on matching capitalization.
- M3 edits SIX existing skill/template files. Per kani-m1: keep each edit additive and behavior-preserving; assert the new hook prose in a NEW sibling test file; never edit an existing baseline test (esp. `sap_imp_m5_agents.rs` SHA baseline).
- Continue scoping clippy to new code; pre-existing `sast-verify` warnings stay waived.

## Template improvements suggested
- None.

## filed_issues
- none — M2 lessons are forward-rules captured here and read by `/slo-execute M3`. No cross-cutting tracked-issue-worthy item.
