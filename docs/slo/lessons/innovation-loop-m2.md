# Lessons Learned — innovation-loop Milestone 2

## What changed
- Shipped the divergent core: `skills/slo-sandbox/SKILL.md` (§3 framing — material-not-feature, boundaries, weirdness budget, ≥3 probe seeds, kill criteria) and `skills/slo-play/SKILL.md` (§4 divergent — raw probes, dead-ends, surprises, 8 frozen probe types). Test `innovation_loop_m2_divergent.rs` (5 assertions). Catalog 42→44.

## Design decisions and why
- **Frozen-sentinel framing (critique E1).** The test asserts `/slo-play` contains the verbatim "judge safety only" and does NOT contain ranking headings (`## Rank`/`## Pick the winner`/`## Best probe`) — presence/absence, not tonal analysis. Tonal convergence remains owned by the M5 dogfood + human read; the runbook DoD says so explicitly.
- **`/slo-play` "judge safety only" is the joy guard** — stated prominently in the skill so the agent knows critique is banned-except-safety here.

## Assumptions verified
- Both skills discovered by `discover_skills()` with no installer change (dry-run lists both).
- Both count-pinning tests (`e2e_cloud_threat_model_m1.rs`, `e2e_slo_nettacker.rs`) re-pointed 42→44 — the M1 lesson (enumerate all count sites) applied cleanly.

## Mistakes made
- A scripted catalog edit's row-add didn't match (stale target string); caught immediately by re-reading and adding the rows with an explicit Edit. No test impact.

## Rules for the next milestone
- M3 `/slo-pattern` ≤5 cap + cite-probe-IDs; `/slo-precision` accept+kill thresholds. Mirror the M2 test helpers.
- Keep re-pointing BOTH count tests (44→46).

## Invariants/assertions added
- `/slo-play` Mode divergent + safety-only sentinel + no ranking heading; 8 frozen probe types; `/slo-sandbox` Not-a-Feature gate + kill criteria; both target existing template sections (§3/§4); output-path safety on both.
