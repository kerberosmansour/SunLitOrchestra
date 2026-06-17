# Reversibility — outcome-first

Hard-to-change decisions for the Outcome First Engineering change, why each is
hard to reverse, the reversibility tactic that keeps the cost down, the
rollback/migration path, and the proof required. Mirrors
[measurement-loop-slo-improvements-reversibility.md](measurement-loop-slo-improvements-reversibility.md).

| # | Decision | Why hard to change | Reversibility tactic | Rollback / migration path | Proof required |
|---|---|---|---|---|---|
| D1 | **Elevate Outcome Validation inside `/slo-verify` (Pass 0), not a new `/slo-outcome` skill** | Once runbooks + muscle memory treat "Verify Pass 0" as the outcome gate, splitting it back into a separate stage means re-teaching the loop and re-pointing every reference | The gate is *named* "Outcome Validation" in docs independent of its host skill, so promoting it to a standalone `/slo-outcome` later is a lift-out, not a redesign; the `oc-`/`cuj-` IDs and §5C contract are skill-agnostic | To split out: create `skills/slo-outcome/`, move Pass 0 prose there, add the handoff `Execute → Outcome → Verify`, register in catalog/loops + `discover_skills` (automatic). No artifact data migrates. | Decision recorded here + in [overview](outcome-first-overview.md); founder confirmed elevate-in-place. |
| D2 | **New v4 template sections (§5C + §17 sub-sections + §11 layer)** | Template is the output contract of `/slo-plan`; many in-flight runbooks reference its section numbers | **Additive + optional + insertion-only** (the §5A/§5B/§10 precedent): legacy runbooks without the sections stay valid; no section is renumbered or removed | To revert: mark the new sections deprecated/optional (they already are); legacy runbooks are unaffected because they never required them | Structural test asserts the new sections exist in the template; `/slo-plan` flags-not-fails on absence for value-bearing milestones. |
| D3 | **Authority inversion: "Outcome outranks unit" (§6 Global Execution Rule)** | This is the load-bearing *semantic* change — it redefines Definition of Done across every future milestone | The rule is a single named §6 entry + a `/slo-retro` refusal gate; it gates *value-bearing* milestones only, so non-value-bearing work is unaffected | To soften: downgrade the §6 rule from "blocks" to "warns" and relax the `/slo-retro` refusal to a flag; no data migration | `/slo-retro` refuses a value-bearing milestone with an unproven outcome row; `/slo-verify` Pass 0 is highest authority in the SKILL.md. |
| D4 | **Frozen ID schemes `oc-<slug>-N` and `cuj-<slug>-N`** | Once Outcome Scenarios / Critical User Journeys are cited by Evidence Logs, Pass 0 rows, and Definition of Done, renumbering breaks the citations | **Frozen, contiguous-from-1, supersede-don't-renumber** — same discipline as `tm-<slug>-abuse-N` | A scheme change is a parallel-scheme addition with a mapping table, never an in-place renumber | Interfaces doc declares both schemes `stable`; structural test can assert the regex shape if needed. |
| D5 | **`/slo-verify` Outcome Validation as a non-renumbering "Pass 0"** | Other docs + the structural test reference pass numbers; a *renumber* would break repo-wide `Pass 4/5/6` citations | Insert Outcome Validation as a leading **Pass 0** and leave Passes 1–6 numbered + worded as-is (the DW-001 decision) — the renumber seam is avoided entirely, not just managed | To revert: drop the Pass 0 block; Passes 1–6 already unchanged, so nothing migrates | `/slo-verify` SKILL.md SHA baseline updated; structural test asserts Pass 0 == "Outcome Validation" AND Pass 4/5/6 anchors still present. |
| D6 | **No `<slug>-outcome.slo.json` schema companion in v1** | Freezing a schema later is easy; *un*-freezing a premature one is the hard direction | Defer until real outcome-test fixtures exist (the measurement `.slo.json` deferral precedent) | Additive when promoted; nothing to roll back | Recorded as a deferred interface in [interfaces](outcome-first-interfaces.md). |

## Net reversibility posture

The change is **deliberately low-cost to reverse** because the founder chose
elevate-in-place (D1) and the template additions are optional/additive (D2). The
only genuinely sticky decision is the **semantic authority inversion (D3)** — but
that is the *point* of the change, and even it degrades cleanly from "blocks" to
"warns" without touching any prior artifact. No schema is frozen prematurely (D6),
and every ID/pass decision is documented rather than silently renumbered (D4, D5).
