# Lessons — biz-b1 Milestone 1 (`/slo-talk-to-users` + Pass 4 PII scan)

## What changed

- NEW `skills/slo-talk-to-users/SKILL.md` (generator, mode arg `pre-interview` | `post-interview`)
- EDITED `skills/slo-verify/SKILL.md` — added Pass 4 sub-step "Biz-pack PII-pattern scan over `docs/biz-public/`" with regex set + override mechanism
- EDITED `references/biz/artifact-schema.md` — added `archetype` (advisor | generator), `mode_arg`, `pii_scan_override`, `tier_override_reason` keys
- NEW `crates/sldo-install/tests/e2e_biz_b1_m1.rs` — 8 structural-contract tests, all green
- NEW `docs/slo/verify/biz-b1-m1-smoke.md` — 5 manual fixtures
- ARCHITECTURE.md — `/slo-talk-to-users` row added

## Design decisions

- **Generator archetype is enforced negatively**: the structural test asserts `slo-talk-to-users/SKILL.md` mentions each predicate ID at most ONCE. Advisor cluster citation contract was substring-presence; generator non-citation is substring-bounded-count. Cleaner than a "must-not-contain" rule because a single forwarding pointer to `/slo-legal` is legitimate.
- **PII regex set is conservative-with-override**: false-positive tolerance HIGH on the scan; founder can mark anonymised content with `pii_scan_override: true` + reason. Better to flag-and-override than to miss. Documented in `/slo-verify` SKILL.md.
- **Pass 4 PII-scan scope = `docs/biz-public/` only**: `docs/biz/` is gitignored by founder convention; scanning it would hit real PII intentionally. The two-tier convention is the load-bearing rule; scan is the runtime enforcement on the public side.

## Course corrections

None. Pattern from Runbook A transferred cleanly.

## Recommendations for B1 M2 (`/slo-gtm`)

- M2 is a pure generator with no mode arg — the simplest possible skill in the pack. Tests will be similarly compact.
- `/slo-gtm` cites `references/biz/artifact-schema.md` (already extended in M1) but does NOT need new shared references.
- The PII scan is now in place; M2's outputs land in `docs/biz-public/gtm/strategy.md` (public tier — strategy docs don't typically contain PII). Test should confirm scan does not flag.
