# Completion Summary — slo-threat-model-producer Milestone 1

## Goal completed

`/slo-architect` Step 3.5 now documents the `.slo.json` producer contract
end to end: emit `docs/slo/design/<slug>-threat-model.slo.json` conforming to
the merged `references/security/threat-model-schema.md` (frozen schema,
`slo_schema_version: 0.1.0`), with the established provenance idiom
(producing-skill SKILL.md git sha + input git blob shas), the
supersede-don't-renumber rule for frozen `tm-<slug>-abuse-N` ids, and the
SEC-1 producer-side neutralisation (structural serializer; user-controlled
idea-doc text never chooses `id`/`classification`/`accepted_residual`/
`status`). A structural-contract test binds the contract in skill prose and
proves the merged dogfood fixture still conforms with its ids unchanged. The
loop closes for live sprints once an agent run actually exercises the
producer (the prompt-based pack's accepted property, same as the wedge).

## Files changed

- `skills/slo-architect/SKILL.md` (append-only: new Step 3.5 item 8).
- `skills/slo-architect/references/threat-model-template.md` (append-only: new `## SLO JSON companion serialization mapping` section).
- `xtasks/sast-verify/tests/slo_tmp_m1_producer.rs` (new; 6 tests).
- `docs/RUNBOOK-SLO-THREAT-MODEL-PRODUCER.md` (M1 evidence, DoD, compatibility, tracker).
- `docs/slo/critique/slo-threat-model-producer.md` (critique disposition).

All on the M1 allow-list. No out-of-scope edits. The four `e2e_*` guards
(`e2e_slo_sec_m1` 11, `e2e_cloud_threat_model_m1` 7, `e2e_fowler_ai_arch_m1`
6, `e2e_fowler_ai_arch_m3` 23) and the merged wedge tests (`slo_tm_m1_schema`
6, `slo_tm_m2_consumers` 5) all still pass.

## Tests added

- `xtasks/sast-verify/tests/slo_tmp_m1_producer.rs` — 6 tests:
  `producer_contract_is_documented_in_step_3_5`,
  `append_only_preserved_existing_step35_anchors`,
  `template_mapping_maps_ids_1to1_and_forbids_renumber`,
  `merged_fixture_ids_unchanged_and_contiguous`,
  `producer_prose_mandates_sec1_neutralisation`,
  `sec1_clause_is_specifically_bound` (verify-time regression).

## Runtime validations added

- `docs/slo/verify/slo-threat-model-producer-m1.md` — every BDD scenario
  exercised at runtime; three negative-path guards (G1 schema citation, G2
  fixture-renumber, G3 SEC-1 weakening) forced to bite via controlled
  mutation then restored byte-identically; Pass 4 supply-chain clean; Pass 5
  AI-tolerance pass; one bug found (loose assertion) closed by an additive
  regression test in the same file.

## Compatibility checks performed

- The four `e2e_*` guards green (unchanged from baseline).
- Merged `slo_tm_m1_schema` 6 + `slo_tm_m2_consumers` 5 green.
- Append-only: Step 3.5 item 7 anchor + every cited substring preserved.
- `git status`: footprint = exactly the M1 allow-list.

## Documentation updated

- `skills/slo-architect/SKILL.md` Step 3.5 (item 8).
- `skills/slo-architect/references/threat-model-template.md` (SLO JSON
  companion mapping section).
- `docs/ARCHITECTURE.md` / `docs/skill-pack-catalog.md` deliberately NOT
  updated — reality-first; M1 hardens an existing internal contract (Step 3.5
  prose addition) and adds no new public skill surface. Same precedent as the
  merged wedge.

## .gitignore changes

- None.

## Test artifact cleanup verified

- `git status` shows only intended files. Mutation backups lived in `/tmp/p_*`
  (outside the repo); all three mutated source files restored byte-identically
  (SHA-256 round-trip on each). No residue.

## Deferred follow-ups

- The supersession algorithm + live `status: superseded` fixture is M2 (the
  remaining milestone of this runbook).
- CEO-1 (producer) is the goal of this whole runbook — its umbrella tracker
  is #67 and the dedicated retro-derived issue is parked in `LESSONS-BACKLOG.md`
  Row 1 from the wedge retro (harness-blocked filing; unchanged by this retro).
- SEC-2 (redaction enforcement) — wedge-retro parked, still pending.

## Known non-blocking limitations

- No deterministic local way to run a live `/slo-architect` agent emission
  end to end. Verified structurally + by mutate-force-restore that the
  contract binds the invariants. Same accepted property as the wedge.
- The "loose Markdown assertion" anti-pattern recurred (M1 lesson "guard
  must bind the invariant, not the prose"); caught by verify; carried into
  M2 rules as a design-time mutation-design discipline.
- `cargo fmt --all -- --check` remains `blocked-unrelated` (pre-existing ~30
  files outside the M1 allow-list); the new test file is rustfmt-clean.
