---
filed_issues: []
note: >
  No new issues filed by this retro. M2 found no bugs; the M1 retro's
  intra-runbook process lesson was applied literally (mutate-force-restore at
  execute time, per-file binding, ENG-2 own-the-parse) and proven effective —
  not a candidate for a new tracked issue. Wedge-retro carry-forwards
  (CEO-1 producer, SEC-2 redaction, #67 comment) remain parked in
  `LESSONS-BACKLOG.md` from 2026-05-19; unchanged by this retro.
---

# Lessons Learned — slo-threat-model-producer Milestone 2

## What changed

- New committed dogfood fixture
  `docs/slo/design/slo-threat-model-producer-demo-threat-model.slo.json` with
  a synthetic 4-abuse-case threat model (`tm-slo-threat-model-producer-demo-abuse-1..4`,
  contiguous; `abuse-2` `status: superseded`, replaced by `abuse-4` with a
  real `supersede_reason`). Closes the wedge-retro coverage gap that no live
  `status: superseded` row existed in any committed fixture.
- New `xtasks/sast-verify/tests/slo_tmp_m2_rerun.rs` (4 tests) — owns the
  demo-fixture strict-parse with its own `deny_unknown_fields` structs (ENG-2)
  plus the algorithm-prose regression-guard and the superseded-row /
  contiguity invariants.
- **No edits to `slo-architect/SKILL.md` or `threat-model-template.md` in
  M2** — M1's item 8 already contained the full re-run algorithm wording
  (`supersede`/`renumber`/`overwrite`/`merge`/`skip`/`no silent clobber`/`diff`
  + `-threat-model.slo.json` target). Honest scope contraction.

## Design decisions and why

- **Don't add an item 9 to Step 3.5 just to fill the M2 contract.** M1's
  item 8 already documents the algorithm. Re-stating it as a new item would
  be redundant skill prose and would violate "smallest safe change." The M2
  test stands as the regression-guard against future weakening of the M1
  wording — that IS M2's distinctive value alongside the demo fixture.
- **Demo fixture demonstrates a *realistic* supersession story** — abuse-2's
  original control (depth-only cap) missed a sibling-chain variant; abuse-4
  models depth + breadth together; abuse-2 is kept with `status: superseded`
  so prior `/slo-plan` citations of `…-abuse-2` still resolve. This isn't a
  trivial demo — it shows *why* supersede-don't-renumber matters.
- **`slo_tmp_m2_rerun` owns its own serde structs** (ENG-2 locked at
  critique). `slo_tm_m1_schema` only loads its hardcoded original fixture and
  was correctly left alone.

## Mistakes made

- None. The M1 lesson rule (design+exercise mutations at execute time, not
  verify) was applied literally: all five M2 invariants had their breaking
  mutations designed and executed at execute time (G1 renumber, G2 drop,
  G3 supersede_reason missing, G4 no superseded row, G5 algorithm-prose
  weakening) — all bit. Verify re-confirmed independently (G1/G3/G5) with
  zero new findings.

## Root causes

- N/A — no defects. The M1 lessons (mutate-force-restore at execute, per-file
  binding, ENG-2 own-the-parse) prevented the recurrence patterns they were
  written for.

## What was harder than expected

- The execute-time *discovery* that M1's item 8 already shipped the full
  algorithm prose meant the runbook's M2 scope was overspecified. Honest
  scope contraction at execute (no new SKILL.md edits) rather than
  performative work. Cost: a re-read of M1's prose to confirm the wording
  was sufficient — small.

## Naming conventions established

- Demo-fixture slug: `slo-threat-model-producer-demo` (the dogfood subject
  of the producer-runbook demo, distinct from any real feature slug).
- Demo-fixture path: `docs/slo/design/slo-threat-model-producer-demo-threat-model.slo.json`.
- Frozen abuse-id pattern for the demo: `^tm-slo-threat-model-producer-demo-abuse-\d+$`.

## Test patterns that worked well

- **Five distinct mutations across two file types** (JSON fixture: G1–G4;
  Markdown skill prose: G5) covered the full M2 surface. Reusable shape:
  any new schema/contract milestone should design one mutation per invariant
  AND mix file types when the contract spans both data and prose.
- **Verify independently re-runs a representative subset** (G1/G3/G5) rather
  than re-running all five — confirms execute's claim cheaply without
  duplicating execute's exhaustive sweep. Belt and braces, not belt and
  belt.

## Missing tests that should exist now

- A second demo fixture that exercises a *chain* of supersessions
  (abuse-X superseded by abuse-Y, then abuse-Y itself later superseded by
  abuse-Z) would harden the chain-traversal invariants. The current demo
  has a single supersession. Below the bar for this runbook; a candidate for
  the future producer-extension work (multi-supersession is rare in practice).

## Rules for the next milestone

- This is the **last milestone of the runbook**. The next step is `/slo-ship`,
  not `/slo-execute`. The single coherent feature branch
  `slo/slo-threat-model-producer` is ready to push; the PR will close CEO-1
  (the wedge-retro umbrella deferred follow-up).

## Template improvements suggested

- Plan should be more cautious about specifying re-stating-in-prose work
  for the next milestone when the prior milestone may already have shipped
  it. A `/slo-plan` heuristic: before specifying "M2 documents X", grep the
  M1 prose for X; if already present, plan M2 around the *guard* (test),
  not a redundant prose addition. Captured as a process refinement; below
  the standalone-issue bar.
