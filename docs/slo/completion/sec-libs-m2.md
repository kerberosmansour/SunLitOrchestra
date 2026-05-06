# Completion Summary - sec-libs Milestone 2

## Goal completed

`/slo-sec-libs` now has an M2 capability matcher contract. It reads target runbook proactive-control rows, matches them against M1 catalog entries, emits `matched`, `unmatched`, and `diagnostics`, and refuses recommendations that cannot cite a catalog `bom_ref`.

## Files changed

- `skills/slo-sec-libs/SKILL.md` (extended)
- `skills/slo-sec-libs/references/methodology-m2-matcher.md` (new)
- `crates/sldo-install/tests/e2e_sec_libs_m2.rs` (new)
- `docs/skill-pack-catalog.md` (modified)
- `docs/slo/future/RUNBOOK-SLO-SEC-LIBS.md` (tracker/evidence update)
- `docs/slo/lessons/sec-libs-m2.md` (new)
- `docs/slo/completion/sec-libs-m2.md` (new)

## Tests added

- `crates/sldo-install/tests/e2e_sec_libs_m2.rs` - 11 structural-contract tests for M2 methodology, SKILL.md dispatch, specificity tiebreakers, tie disposition, conservative fallback, output shape, fabricated-ID refusal, empty states, M1 compatibility, and deny-list compatibility.

## Validation performed

- `cargo test -p sldo-install --test e2e_sec_libs_m2`
- `cargo test -p sldo-install --test e2e_sec_libs_m1`
- Fixture matcher smoke: preferred the higher-specificity catalog component for a C5 strict-schema control.
- Fixture matcher smoke: emitted one unmatched record for a C9 audit control with no catalog candidate.
- Fabricated catalog ID guard fixture refused a made-up `bom_ref`.

## Known limitations

- M2 is host-driven and read-only. There is no new standalone matcher executable.
- The matcher contract is structural; M5 dogfood should validate it against multiple real runbooks.
- Filing starts in M3 and remains intentionally out of scope here.
