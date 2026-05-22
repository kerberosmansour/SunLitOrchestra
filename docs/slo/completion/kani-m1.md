# Completion Summary — kani Milestone 1

## Goal completed
- A discoverable, installable `/slo-kani` skill skeleton now exists: pinned Kani toolchain, prereq cascade, suitability gate, candidate-scoring rubric, honesty/scope gates, and an output-path allow-list — all asserted by a structural-contract test. `sldo-install` auto-discovers it with no installer change.

## Files changed
- `skills/slo-kani/SKILL.md` (NEW)
- `skills/slo-kani/tools.toml` (NEW — pinned `kani-verifier@0.56.0`)
- `skills/slo-kani/references/candidate-scoring.md` (NEW)
- `xtasks/sast-verify/tests/kani_m1_skill_contract.rs` (NEW)
- `docs/skill-pack-catalog.md` (added `/slo-kani` row; noted `kani_required` flag)
- `.gitignore` (Kani/CBMC artifact patterns)

## Tests added
- `xtasks/sast-verify/tests/kani_m1_skill_contract.rs` — 5 structural assertions: frontmatter `name`, honesty + concurrency-refusal gates, pinned toolchain, output-path allow-list, candidate-scoring reference presence.

## Runtime validations added
- Skill discovery exercised via `sldo-install --dry-run`; structural test run as the E2E gate. Report: `docs/slo/verify/kani-m1.md`.

## Compatibility checks performed
- Full `cargo test -p sast-verify` green (13 test-files); `sap_imp_m5_agents` SHA baseline untouched; no existing baseline test edited; `sldo-install` discovery unaffected.

## Documentation updated
- `docs/skill-pack-catalog.md` sprint-flow table — new "Verify code" row.
- ARCHITECTURE.md planned-work note for `/slo-kani` was added during the architect pass (folds into the HEAD table in M5).

## .gitignore changes
- Added `**/*.kani-metadata.json` and `kani_concrete_playback_*` (`target/` already covered Kani scratch).

## Test artifact cleanup verified
- `git status` shows only intended new/modified files; no untracked test output.

## Deferred follow-ups
- M2 extends `kani_m1_skill_contract.rs` with the four honesty/scope gate sentences + ENG-2 fail-closed + SEC-1 write-path assertions.
- The harness-generation / run-and-triage / fallback / verified-scope references referenced by the SKILL.md dispatch table are M2 deliverables (marked *(M2)* in the table).

## Known non-blocking limitations
- Pre-existing clippy warnings in the `sast-verify` bin and `sap_imp_m3_standards` test remain (out of scope; waived with rationale in the Evidence Log).
- The prereq cascade's live behavior (refusing a missing/mismatched Kani toolchain) is documented and asserted-to-exist but exercised against a real toolchain only in M4 (ENG-1 scenario).
