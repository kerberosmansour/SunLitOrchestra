# Completion Summary — slo-threat-model-producer Milestone 2

## Goal completed

The wedge-retro coverage gap is closed: a committed dogfood fixture
(`docs/slo/design/slo-threat-model-producer-demo-threat-model.slo.json`)
carries a live `status: superseded` row with non-empty `superseded_by` +
`supersede_reason`, and a Rust structural-contract test strict-parses it
with its own `deny_unknown_fields` structs (ENG-2), enforces the
superseded-row invariants, and guards the M1 algorithm-prose against
weakening. Together with M1, the producer runbook delivers the full CEO-1
slice: `/slo-architect` Step 3.5 now documents emitting the `.slo.json`
companion + the supersede-don't-renumber re-run algorithm, and a real
superseded-row example proves the algorithm has teeth.

## Files changed

- `docs/slo/design/slo-threat-model-producer-demo-threat-model.slo.json` (new) — demo fixture with a live superseded row.
- `xtasks/sast-verify/tests/slo_tmp_m2_rerun.rs` (new; 4 tests) — owns the demo-fixture strict-parse; algorithm-prose regression-guard.
- `docs/RUNBOOK-SLO-THREAT-MODEL-PRODUCER.md` — M2 evidence, DoD, compatibility, tracker.

All on the M2 allow-list. **`slo-architect/SKILL.md` and
`threat-model-template.md` deliberately not modified in M2** — M1's item 8
already shipped the algorithm wording (verified at execute time); honest
scope contraction. No new crate, no `skills/` directory.

## Tests added

- `xtasks/sast-verify/tests/slo_tmp_m2_rerun.rs` — 4 tests:
  `rerun_algorithm_documented_in_step35` (regression-guard for M1's
  wording), `demo_fixture_strict_parses_with_own_structs` (ENG-2
  own-the-parse), `demo_fixture_has_live_superseded_row` (the wedge-retro
  coverage gap closed), `demo_fixture_ids_match_demo_slug_and_are_contiguous`
  (frozen-ID + supersede-don't-renumber for the demo).

## Runtime validations added

- `docs/slo/verify/slo-threat-model-producer-m2.md` — every BDD scenario
  exercised at runtime; five forced-failure mutations (G1 renumber,
  G2 silent drop, G3 supersede_reason missing, G4 no superseded row,
  G5 algorithm-prose weakening) proven to bite at **execute time** per the
  M1 lesson rule; verify-side independent re-confirm on G1/G3/G5; restore
  integrity exact; Pass 4 supply-chain clean; Pass 5 AI-tolerance pass with
  every must-never proven to bite.

## Compatibility checks performed

- `slo_tmp_m1_producer` 6, `slo_tm_m1_schema` 6, `slo_tm_m2_consumers` 5 still green.
- Four `e2e_*` guards (`e2e_slo_sec_m1` 11, `e2e_cloud_threat_model_m1` 7, `e2e_fowler_ai_arch_m1` 6, `e2e_fowler_ai_arch_m3` 23) unchanged.
- `git status`: M2 footprint = exactly the M2 allow-list (no SKILL.md/template edits in M2).

## Documentation updated

- `docs/RUNBOOK-SLO-THREAT-MODEL-PRODUCER.md` only (M2 tracker + evidence).
- `docs/ARCHITECTURE.md` / `docs/skill-pack-catalog.md` deliberately NOT
  updated — reality-first; the producer extends an internal contract, no new
  public skill surface (same precedent as M1 + wedge).

## .gitignore changes

- None.

## Test artifact cleanup verified

- `git status` shows only intended files. Verify-side mutation backups lived
  in `/tmp/v2_*` (outside the repo). Demo fixture + `slo-architect/SKILL.md`
  restored byte-identically (SHA-256 round-trip on each). No residue.

## Deferred follow-ups

- **CEO-1** — this runbook delivered it. The parked
  `LESSONS-BACKLOG.md` Row 1 candidate issue is now functionally satisfied
  by what this runbook ships; the PR body can note that.
- **SEC-2** (public-repo redaction enforcement) — wedge-retro parked,
  unchanged by this retro; the producer runbook did not absorb SEC-2 by
  design (out-of-scope per the runbook §6).
- Multi-supersession chain demo fixture — coverage refinement, below the
  bar; future producer-extension work.

## Known non-blocking limitations

- No deterministic local way to run a live `/slo-architect` agent and
  observe an end-to-end real re-emission cycle. Verified structurally + by
  the synthetic demo fixture and the mutate-force-restore guards. Same
  accepted property as the merged wedge + M1 producer.
- `cargo fmt --all -- --check` remains `blocked-unrelated` (pre-existing
  ~30 files outside the allow-list); both M2 new files are rustfmt-clean
  (Markdown not applicable, JSON not formatted by rustfmt; the Rust test is
  rustfmt-clean exit 0).
