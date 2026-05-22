# Lessons Learned — mloop Milestone 2

## What changed
- `skills/slo-product/SKILL.md`: new `### 6. Feature measurement specification (optional, per-feature)` in the `mode_arg: metrics` body (north-star link, primary leading/lagging metric, guardrails, activation/completion funnel, adoption thresholds, diagnostic questions, segmentation, experiment backlog, telemetry requirements with pseudonymise/mask-by-default); `feature_measurement_spec: true` flag rule + frontmatter line.
- `references/biz/artifact-schema.md`: one optional `feature_measurement_spec: bool` key (default-absent = false).
- `xtasks/sast-verify/tests/mloop_m2_product.rs` (NEW): 6 structural-contract tests.

## Results vs. success thesis
- N/A — meta/tooling milestone (builds the PM-side feature-spec surface).

## Design decisions and why
- **§6 placed after the cross-reference** (not renumbering §5) — minimal diff; the split's CAC/LTV/NDR enumeration stays put and asserted.
- **Exactly one optional bool key** — honored the reversibility + interfaces lock; added a forbidden-key denylist test (`telemetry_schema`/`event_names`/`measurement_spec_json`) so a future cluster is caught.
- **Pseudonymise-by-default in the telemetry sub-block** — pushes the privacy posture (C8) into the artifact the loop consumes downstream, the earliest enforceable point on the PM side.

## Assumptions verified
- `tier` enum byte-unchanged (`confidential | public`); CAC/LTV/NDR cross-ref intact; `/slo-product` still discovered.

## Assumptions still unresolved
- None for M2.

## Mistakes made
- None material.

## Root causes
- N/A.

## What was harder than expected
- Nothing.

## Invariants/assertions added or strengthened
- Feature-spec section + 4 sub-field sentinels + flag rule; pseudonymisation mandate; key registered (bool/optional/default-false); split preserved (CAC/LTV/NDR + /slo-metrics); tier enum unchanged; single-key bound (denylist).

## Resource bounds established or verified
- One optional key; one feature-spec section; no key cluster (denylist-asserted).

## Debugging / inspection notes
- Pre-fix run confirmed the 4 new-content assertions failed (incl. single-key-bound, which needs the key present) before edits.

## Naming conventions established
- Same `mloop_m<N>_<area>.rs` / `slo_<skill>_<assertion>` conventions as M1.

## Test patterns that worked well
- Forbidden-key denylist as a positive single-key-bound proof (checkable, per ENG-3 critique fix).

## Missing tests that should exist now
- None for M2.

## Rules for the next milestone
- M3 edits the v4 template + `/slo-plan`; assert NO renumber of existing template sections, and that `kani_m3_integration.rs` stays green after inserting the new section near §5.

## Template improvements suggested
- None.

## Carry-forward
- Pre-existing clippy red still open (see mloop-m1 lessons) — unchanged by M2.
