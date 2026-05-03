# Lessons Learned — biz-imp Milestone 1

## What changed

- Added `crates/sldo-install/tests/e2e_biz_imp_m1.rs`, a structural-contract test for source-verified UK regulator/statute authority files.
- Source-verified `references/biz/uk-regulator-enumeration.md` with a 25-row audit register, retrieval date, annual refresh date, official URLs, and confidence values.
- Added three authority files: `uk-employment-statute-anchors.md`, `uk-consumer-statute-anchors.md`, and `uk-marketing-statute-anchors.md`.
- Refreshed `references/biz/hmrc-vcm-index.md` with short `quoted_text:` anchors for VCM34080, VCM3000, VCM31000, and the Abingdon Health preferential-rights marker.
- Refreshed `references/biz/ico-duaa-index.md` with DUAA commencement and PECR penalty anchors.
- Fixed unrelated red baseline behavior in `tests/e2e_research_m1.rs` through `tests/e2e_research_m7.rs`: the tests now build `sldo-research` if missing instead of assuming the binary exists.

## Design decisions and why

- The source verification register lives before the existing enum tables. The M1 test resolves the first row for each `regulator_id`, so putting the audit register first prevents the older human-readable enum rows from shadowing the verified rows.
- The statute anchor files use short `quoted_text:` fields instead of long copied passages. That gives downstream SKILL.md prose a stable citation handle without turning the reference files into statute mirrors.
- ASA remains `medium` confidence in the register. The CAP Code is the right official advertising-code anchor, but ASA is self-regulatory rather than a direct `legislation.gov.uk` statutory body row.
- The Abingdon Health marker stays in HMRC VCM references, but the primary source is HMRC's VCM33020 preferential-rights text. I did not find a stable official tribunal/GOV.UK endpoint for the Abingdon judgment during M1, so the file avoids relying on third-party commentary.

## Course corrections taken in flight

- `cargo test --workspace` was red before M1 authority work because root research integration tests assumed `target/debug/sldo-research` already existed. I fixed that baseline first by adding a `OnceLock`-backed helper that builds the binary when needed.
- Once `sldo-research` existed, two M1 research prompt tests could accidentally reach the real `claude` CLI. I isolated `PATH` for those tests so they exercise prompt acceptance without live provider calls.
- The refreshed ICO file initially dropped legacy compatibility strings (`2026-02-05`, `ico.org.uk`, `£17.5M`). Package smoke caught that, and the refreshed source-verified file now keeps those compatibility anchors.
- The refreshed HMRC file initially dropped the "6 weeks" Advance Assurance practical floor. Package smoke caught that as well, and the floor is restored as an explicit practical anchor.

## What was harder than expected

- Official UK legislation pages are easier to audit through `/data.xml` endpoints than through browser-rendered pages. The HTML route can be script-heavy; the XML endpoints are stable enough for quote/source extraction.
- DMCC and DUAA are broad Acts. Section-level anchoring is useful for specific advisor behavior, but contents-level and schedule-level anchors are still needed where the runbook cares about policy surfaces rather than one short operative clause.

## What was easier than expected

- Existing biz-pack compatibility tests did their job. They caught date/phrase anchors that were not part of the new M1 test but still matter to downstream skills.
- Keeping the four hard-block predicate IDs immutable required no changes to `triage-gate.md`; the M1 structural test gives us a simple guard for later milestones.

## Recommendations for M2

- Start M2 with a failing structural test that proves every advisor SKILL.md cites the new authority files and intake contracts.
- Decide deliberately whether to update `jurisdiction-uk.md` with a cross-reference to `uk-regulator-enumeration.md`; M1 left it untouched to respect the allow-list.
- Keep citation updates in SKILL.md prose as links to reference sections, not restated statute text.

## Tests run

- `cargo test --workspace` after baseline fix: passed.
- `cargo test -p sldo-install --test e2e_biz_imp_m1`: passed, 6/6.
- `cargo test -p sldo-install`: passed.

## Changes to runbook tracker

- M1 status `in_progress` to `done`. Started 2026-05-03, completed 2026-05-03.
