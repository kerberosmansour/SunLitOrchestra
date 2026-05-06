# Completion Summary - sec-libs Milestone 5

## Goal completed

`/slo-sec-libs` has now been dogfooded against a completed SLO milestone using real Hulumi and SunLitSecurityLibraries CycloneDX 1.6 declarations. The dogfood target was `slo-security-embedding` M3, selected from a three-candidate shortlist because it declares `security_libs_required: true` and has the richest security-library surface.

## Files changed

- `docs/sec-libs-dogfood-2026-05-06.md` (new)
- `crates/sldo-install/tests/e2e_sec_libs_m5.rs` (new)
- `docs/slo/future/RUNBOOK-SLO-SEC-LIBS.md` (tracker, M5 BDD clarification, evidence)
- `docs/slo/lessons/sec-libs-m5.md` (new)
- `docs/slo/completion/sec-libs-m5.md` (new)

## Tests added

- `crates/sldo-install/tests/e2e_sec_libs_m5.rs` - 10 structural-contract tests for report frontmatter, candidate shortlist, reader evidence, target M3 references, matched catalog refs, unmatched gaps, deferred filing status, canonical SunLitSecurityLibraries spelling, M1-M4 deliverables, and deny-list compatibility.

## Validation performed

- `rustfmt --check crates/sldo-install/tests/e2e_sec_libs_m5.rs`
- `cargo test -p sldo-install --test e2e_sec_libs_m5`
- `cargo test -p sldo-install --test e2e_sec_libs_m1 --test e2e_sec_libs_m2 --test e2e_sec_libs_m3 --test e2e_sec_libs_m4 --test e2e_sec_libs_m5`
- `cargo test -p sldo-install`
- `cargo test --workspace`

All validation passed. Existing warnings remain unrelated to M5:

- `crates/sldo-install/tests/e2e_biz_followup_m5.rs` has an unused `Path` import.
- `xtasks/sast-verify` has existing dead-code warnings for schema/tier fields.

## Known limitations

- No live issue was filed because M3/M4 require explicit per-issue confirmation. The dogfood report records two `deferred-pending-confirmation` filing candidates.
- M5 remains host-driven: it validates the real reader and documented matcher/filer contracts, not a new standalone matcher or filer executable.
- The current declarations do not advertise exact agent-prompt-boundary or variant-analysis-schema capabilities, so those remain capability gaps.
