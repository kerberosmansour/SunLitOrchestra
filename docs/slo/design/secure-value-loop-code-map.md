# Code Map — secure-value-loop

Brownfield: the target is the SLO skill pack itself. The "code" being changed is
Markdown skill contracts + the v4 template + LOOPS docs, plus Rust structural
tests. This map names the seams an executing agent must inspect before editing.

## Four-object summary

1. **The v4 runbook template (dual copy)** — `docs/slo/templates/runbook-template_v_4_template.md`
   (mirror) and `skills/slo-plan/references/runbook-template_v_4_template.md`
   (primary). Kept **byte-identical**; the contract surface every runbook inherits.
   The new "Secure Value & Security Contract" section is inserted here, between
   §5A (Measurement Contract) and §6 (Global Execution Rules), without renumbering.
2. **The sprint-loop skills** — `skills/slo-plan`, `skills/slo-execute`,
   `skills/slo-verify`, `skills/slo-retro`, `skills/slo-ship`, `skills/slo-resume`.
   These are Markdown `SKILL.md` files with `references/` sub-docs. They are the
   behavioural contract; editing them is the bulk of the work.
3. **The structural-contract test crate** — `xtasks/sast-verify/tests/*.rs`. One
   test file per milestone-family (e.g. `mloop_m3_plan.rs`). They read repo files
   and assert section/row presence, dual-copy byte-identity, SKILL.md SHA
   baselines, and "no renumbering of existing sections". This is the only gate.
4. **The canonical loop docs** — `docs/LOOPS-ENGINEERING.md` (primary catalog) +
   `docs/LOOPS-BUSINESS.md` (cross-ref). New: `docs/SECURE-VALUE-LOOP.md`.

## Exemplar code to copy

- **The measurement-loop change is the gold-standard exemplar** — it made the
  *exact same kind* of change (additive optional v4-template section + Contract
  Block row + skill wiring + LOOPS doc + structural test, all backward
  compatible). Copy its shape:
  - Overview/design: `docs/slo/design/measurement-loop-slo-improvements-*.md`
  - Template insertion pattern: §5A Measurement Contract (lines 314–335 of the v4
    template) — an optional section with explicit "legacy runbooks remain valid"
    framing and an `N/A — <reason>` escape.
  - Structural test pattern: `xtasks/sast-verify/tests/mloop_m3_plan.rs` —
    `TEMPLATE_PRIMARY`/`TEMPLATE_MIRROR` byte-identity assert + required-fields
    array + "existing sections not renumbered/removed" assert.
- **Carry-forward §10** (template lines 545–573) — exemplar for the additive,
  optional, "consumers fall back gracefully" posture the status-enum change needs.
- **Threat-model `.slo.json` provenance idiom** —
  `docs/slo/design/measurement-loop-slo-improvements-threat-model.slo.json`
  (producer SHA + input SHAs + frozen abuse IDs).
- **`/slo-retro` issue-filing discipline** —
  `skills/slo-retro/references/issue-filing-discipline.md` — the lane vocabulary +
  dedupe + per-session cap the ledger must reuse, not duplicate.

## Anti-exemplar code not to copy

- **Do NOT clobber repo-root `SECURITY.md`** or invent a root `ARCHITECTURE.md` —
  the measurement-loop precedent deliberately kept the diagram in the overview
  and left root security files alone.
- **Do NOT introduce a new `sldo-*` crate** for ledger/readiness validation — the
  `slo-security-embedding` design explicitly rejected forcing LLM-native
  judgement into Rust; this work follows that.
- **Do NOT add a third disposition taxonomy.** The five ledger dispositions must
  resolve to existing `/slo-retro` lanes; a parallel vocabulary is the
  anti-pattern.
- **Do NOT cite OWASP controls by bare number** (`C5`) — that is the exact drift
  bug this work fixes (2018 vs 2024 renumber).

## Dangerous seams (inspect before editing)

- **The dual-template byte-identity.** Any edit to one template copy MUST be
  mirrored to the other in the same change, or the structural test fails (and, if
  the test is stale, two different contracts ship). Verify with
  `diff docs/slo/templates/runbook-template_v_4_template.md skills/slo-plan/references/runbook-template_v_4_template.md`.
- **Section renumbering.** Inserting the new section must not shift §6/§10/§17
  numbers that tests and skills reference. The measurement-loop §5A insertion is
  the safe precedent (it used a letter suffix, not a renumber).
- **The status-enum parser fallback.** `/slo-resume` (lines ~20–35 of its
  SKILL.md) and `/slo-execute` Step 1.5 branch on Status values. Adding values
  requires the "unknown → `blocked`" rule to be explicit in *both*, or a legacy
  parser silently mis-reads a new value.
- **SKILL.md SHA baselines.** Several `xtasks/sast-verify` tests pin SKILL.md
  SHA-256 baselines (e.g. `sap_imp_m5_agents.rs` pins `/slo-critique`). Editing a
  SKILL.md that has a pinned baseline requires updating the baseline in the same
  PR — search the tests for the skill name before editing.
- **`/slo-retro` per-session issue cap (40/hr).** The ledger→retro bridge must not
  bypass this cap; reuse the existing flow, don't re-implement filing.
