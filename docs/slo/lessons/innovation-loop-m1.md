# Lessons Learned — innovation-loop Milestone 1

## What changed
- Shipped the spine of the Innovation Sandbox loop: the `experiment-book-template_v_1.md` (Creative Experiment Contract — §0–§11, Definition of Learned), the `/slo-experiment` umbrella skill, the `innovation_loop_m1_spine.rs` structural-contract test (9 assertions), and registry/overlay/`.gitignore` wiring. Catalog 41→42.

## Design decisions and why
- **Template is the binding artifact, skill is thin.** `/slo-experiment` only opens/resumes + seeds §0–§2; the frozen vocabularies, moods, Definition-of-Learned blocks, and seed tables live in the template (single source of truth). Keeps drift impossible across the 7 phase skills that fill §3–§10.
- **Slug validation lives in the skill (runtime), not the test (static).** Critique S1's point: a structural test can only check the template's static paths; the actual traversal risk is the runtime `<slug>`. So `/slo-experiment` mandates `^[a-z0-9][a-z0-9-]*$` and the test asserts the skill *states the rule*.
- **Fence assertion counts `~~~text` openers (≥2).** Cheap, robust proxy for "user strings are fenced" without parsing the template structurally.

## Assumptions verified
- `discover_skills()` needs NO change — `cargo run -p sldo-install -- --dry-run` lists `slo-experiment` purely on the presence of `SKILL.md`. (Confirmed the architect/code-map assumption.)
- The `/slo-critique` SHA-256 baseline test is undisturbed (M1 touches no skill it pins).

## Assumptions still unresolved
- Whether the structural test's sentinel-string checks for `/slo-play` divergence (M2) will feel too brittle in practice — deferred to M2 + the M5 dogfood (critique E1 acknowledged this is not a tonal guarantee).

## Mistakes made
- **Critique E2 undercounted the count-pinning tests.** I searched with `head` truncation and found only `e2e_cloud_threat_model_m1.rs`; a second test (`e2e_slo_nettacker.rs`) pinned the same string and broke mid-execution. Surfaced via the allow-list rule, got approval, fixed both. → DW-002.
- **`.gitignore` `experiments/` was unanchored.** It would have ignored the tracked Books under `docs/slo/experiments/`. Caught by the `.gitignore` compatibility check before commit. → DW-003.

## Root causes
- E2 undercount: trusted a truncated grep during critique instead of a complete `grep -rn` over the test dirs. **Rule for next milestone: when enumerating "all sites that assert X", run the search WITHOUT `head` and count the matches explicitly.**
- `.gitignore`: forgot gitignore patterns match at every depth unless anchored with a leading slash. **Rule: anchor repo-root-only ignore patterns with a leading `/`.**

## What was harder than expected
- Getting the test's path tokenizer right: splitting on `<`/`>` fragmented the `<slug>` placeholder path into a false-positive absolute `/EXPERIMENT.md`. Fixed by keeping `<>` out of the split set.

## Invariants/assertions added or strengthened
- Frozen §0–§11 order; exactly-8 exit states; 5 status values; 5 phase modes; §2A Judgment Timing Rule presence (+ "judge safety only" sentinel); Safety Rails + Safety Check + Phase Contract presence; 3 Definition-of-Learned blocks; 5 promotion-seed headers; output-path allow-list + traversal rejection; S1 slug regex stated; S2 `~~~text` fence count ≥2; PII/secret scan over `docs/slo/experiments/`.

## Resource bounds established or verified
- Experiment Book bounded to §0–§11 (12 sections); exit vocabulary bounded to 8; both test-asserted (template tamper → red).

## Debugging / inspection notes
- Confirmed the 3 clippy errors are pre-existing by `git status --porcelain` (the 3 files are untouched by M1) and `cargo clippy -p sast-verify -- -D warnings` showing the same 3 on lib/bin only. My test file is clippy-clean.

## Naming conventions established
- Test file: `xtasks/sast-verify/tests/innovation_loop_m<N>_<topic>.rs`.
- Book path `docs/slo/experiments/<slug>/EXPERIMENT.md`; scratch root `/experiments/<slug>/<spike-id>/`.

## Test patterns that worked well
- Mirroring `sap_imp_m5_agents.rs`: `workspace_root()` + `extract_frontmatter()` + `serde_yaml_ng` + `Component::ParentDir` traversal check. Reuse for M2–M5.

## Missing tests that should exist now
- (M2+) sentinel checks for `/slo-play` divergence; (M5) the example-Book end-to-end exit-state check.

## Rules for the next milestone
1. Enumerate "all sites" with a complete `grep -rn` (no `head`), count matches.
2. Anchor repo-root `.gitignore` patterns with a leading `/`.
3. Both count-pinning tests (`e2e_cloud_threat_model_m1.rs`, `e2e_slo_nettacker.rs`) re-point each milestone as the catalog count rises.
4. Phase skills fill ONE section per invocation; the template's frozen text is copied verbatim, never rewritten.

## Template improvements suggested
- Consider a CI grep that fails if any test pins `"Shipped skills at HEAD: N"` with a stale N (would have caught DW-002 automatically).
