---
name: outcome-first
created: 2026-06-17
status: design-locked
tla_required: false
tla_reason: "Offline, single-process, interactive Markdown authoring + a deterministic Rust structural test. No concurrent shared state, consensus, leases, or cross-process ordering to model-check."
kani_required: false
kani_reason: "No Rust kernel is produced. Deliverables are Markdown SKILL.md edits + v4 template sections; the only Rust touched is a structural-contract test in xtasks/sast-verify (a test, not a verifiable bounded kernel)."
security_libs_required: false
ai_component: false
ai_reason: "The shipped artifacts are static docs + skill prose + a deterministic structural test — no LLM call is embedded in the deliverable at runtime. The relevant AI risk (an agent generating outcome tests that are theatre) is addressed by anti-theatre gate rules in this design, not by an AI tolerance contract on a nondeterministic runtime surface."
compliance: [soc2, asvs]
---

# Design overview — Outcome First Engineering (Outcome Validation gate)

Single source of truth for downstream skills (`/slo-tla` reads `tla_required`,
`/slo-kani` reads `kani_required`, `/slo-plan` reads scope, `/slo-critique`
reads `compliance` and `ai_component`, `/slo-verify` cites the threat-model row
IDs).

## What this is

A **structural change to the SLO engineering methodology itself** that elevates
**user outcomes to first-class, testable artifacts**. It implements the
founder's "Outcome First Engineering" proposal (11 changes) so that **a
milestone is no longer complete because the code works — it is complete only
when the promised user outcome exists AND the existing important outcomes still
exist.**

The change is delivered as **edits to existing skills + new v4 template
sections + loop/doc updates + one structural-contract test** — **no new
installable skill.** Per the decision recorded in
[reversibility](outcome-first-reversibility.md), the "Outcome Validation" gate
(proposal Change #11) is realised by **elevating Outcome Validation to the
highest-authority, leading pass inside `/slo-verify`**, not by adding a separate
`/slo-outcome` stage. This honours SLO's anti-process-theatre rule (no new gate
that duplicates an outcome `/slo-verify` already produces — `/slo-verify` is
already the runtime-BDD/Playwright authority) and minimises reversibility cost.

### The reframe in one sentence

> BDD/E2E moves from *"acceptance tests / nice verification"* to *"the primary
> Definition of Done — the proof that the milestone outcome exists and adjacent
> outcomes are preserved."*

### Authority inversion (the new pyramid)

```
            ┌───────────────────────────────┐
            │   OUTCOME  (smallest layer,    │   ← HIGHEST AUTHORITY
            │            highest authority)  │     /slo-verify Pass 0
            ├───────────────────────────────┤
            │            E2E                 │
            ├───────────────────────────────┤
            │        Integration            │
            ├───────────────────────────────┤
            │            Unit               │   ← largest layer, base authority
            └───────────────────────────────┘

  Rule: if 1000 unit tests pass but an Outcome Validation row fails,
        the milestone FAILS. Code completion alone is insufficient.
```

## Frontmatter rationale

- `tla_required: false` — no concurrency, distributed state, leases, or
  cross-process ordering. Pure interactive Markdown authoring + a deterministic
  test. Nothing to model-check at the design level.
- `kani_required: false` — no Rust kernel is shipped. The only Rust is a
  structural-contract test asserting the new template sections + edited SKILL.md
  baselines (a test, not a bounded verifiable kernel).
- `security_libs_required: false` — no service surface, no auth, no crypto. Local
  skill pack + Markdown artifacts only.
- `ai_component: false` — the deliverable embeds no runtime LLM call. The genuine
  AI-driven risk here is **outcome-test theatre** (an agent emitting a green-but-
  vacuous outcome test). That is mitigated by the anti-theatre gate rules and
  `/slo-critique` review (see [threat model](outcome-first-threat-model.md)
  `tm-outcome-first-abuse-2`), not by an AI tolerance contract on a
  nondeterministic surface. (Contrast `innovation-loop`, which set `true` because
  its phase skills generate probes/evidence at runtime.)
- `compliance: [soc2, asvs]` — defaults. The Security-BDD addition (Change #7)
  exercises authn/authz outcomes, which ASVS covers; the audit-event assertions
  map to SOC 2 logging controls.

## Why elevate-in-place, not a new `/slo-outcome` skill

The proposal's mental model orders the loop `Execute → Outcome Validation →
Verify (unit/integration/static/security) → Learn`. **SLO's reality is already
arranged differently:** unit/integration/static analysis run inside
`/slo-execute`'s Global Exit (template §8) and `/slo-verify` Pass 4/5/6, while
**runtime BDD + Playwright already run inside `/slo-verify` Passes 1–3.** So
`/slo-verify` is *already* the runtime-outcome authority. Inserting a new
`/slo-outcome` stage would either duplicate that runtime-BDD ownership (process
theatre) or force a high-churn migration of `/slo-verify`'s Passes 1–3 into a new
binary. Elevating Outcome Validation to a **non-renumbering leading "Pass 0" of
`/slo-verify`** delivers the same authority inversion with the lowest
reversibility cost (Passes 1–6 keep their numbers, so repo-wide `Pass 4/5/6`
citations are untouched). The named gate still exists in the loop docs — as the
leading, highest-authority tier of Verify.

## Where each of the 11 proposed changes lands

> Phasing across ≤5 milestones is `/slo-plan`'s job; this is the *landing map*,
> not the milestone cut. Interface IDs are locked in
> [interfaces](outcome-first-interfaces.md).

| # | Proposal change | Lands in | Mechanism |
|---|---|---|---|
| — | **New principle: Outcome First Engineering** | `docs/LOOPS-ENGINEERING.md`, `references/agent/operating-contract.md`, `docs/skill-pack-catalog.md` | Named principle + the authority-inversion rule ("code completion alone is insufficient") added to the host-neutral contract and the catalog. |
| 1 | **Make BDD the source of truth** (outcome-shaped, multi-`And` scenarios) | v4 template §11.3 Scenario Format + §17 BDD Acceptance Scenarios | Strengthen the scenario format to require an *observable user outcome* + follow-on `And` assertions (severity shown, remediation shown, appears in history, survives restart). BDD reframed from "acceptance tests" to "primary DoD". |
| 2 | **Outcome Scenarios** (mandatory, primary DoD) | v4 template **new §17 sub-section** "Outcome Scenarios" + Definition of Done | New required (for value-bearing milestones) sub-section; all rows must be automated; added to the milestone Definition of Done. |
| 3 | **Front-to-End / Outcome test layer** | v4 template §11.4 Test File Naming + §11 layer list | Add the `Outcome` layer above E2E: input → backend → storage → processing → UI → user outcome. New naming row + location convention. |
| 4 | **Critical User Journeys** (declared per milestone) | v4 template **new Contract Block + §17 sub-section** "Critical User Journeys" | Each value-bearing milestone declares its journeys (seed → scan → finding → risk visible → remediate); they become mandatory automated tests with frozen IDs. |
| 5 | **Regression Journey Matrix** (Core Capability Regression Matrix) | v4 template **new §17 sub-section** "Core Capability Regression Matrix" | Capability × must-still-pass table; each row resolves to `pass \| not_applicable \| waived_with_reason` (never blank) — mirrors the §5B Bundle discipline. |
| 6 | **Cross-Capability Verification** ("did it break anything important?") | `/slo-verify` Pass 0 + `/slo-retro` refusal gate | Pass 0 runs the Regression Matrix journeys; any failure blocks milestone completion. |
| 7 | **Security BDD** (authz/abuse as outcome tests) | v4 template §17 Outcome Scenarios (security rows) + ties to threat-model abuse IDs | Security scenarios (e.g. "User B cannot read User A findings; access denied; audit event created") cite `tm-<slug>-abuse-N`; verified in Pass 0, distinct from Pass 4 *scans*. |
| 8 | **Reliability BDD** (degraded/outage as outcome tests) | v4 template §17 Outcome Scenarios (reliability rows) + §4.4/§4.8 linkage | Reliability scenarios (e.g. "NVD API down → inventory completes, local findings appear, outage is visible") extend the existing bounded-resource / no-silent-failure rules into runtime outcome proof. |
| 9 | **New test pyramid** (Outcome at the apex, highest authority) | v4 template §11 + `docs/LOOPS-ENGINEERING.md` | Document the inverted-authority pyramid; encode "outcome failure outranks unit pass" as a §6 Global Execution Rule. |
| 10 | **Outcome Validation Contract** (in the technical contract) | v4 template **new §5C** "Outcome Validation Contract" (after §5B) + Contract Block row | Per-milestone: Outcome statement, Success Criteria, Front-to-End Validation steps, Regression Requirements. `/slo-plan` requires it for value-bearing milestones (peer to §5A/§5B). |
| 11 | **Gate 7: Outcome Validation** (between Execute and Verify) | `/slo-verify` **Pass 0 = Outcome Validation** (leading, highest authority, non-renumbering) + `/slo-retro` gate + loop docs | Realised as the elevate-in-place gate; existing Passes 1–6 keep their numbers; `/slo-retro` refuses to close while any outcome/journey/regression row is unproven. |

## Inputs

- **Founder-authored "Outcome First Engineering" proposal** (the 11 changes +
  the Outcome-First principle + Guardian/DSPM worked examples) — provided
  in-session, not repo-tracked. Stands in for `/slo-ideate` + `/slo-research`
  for this methodology change (the founder explicitly entered at
  `/slo-architect`).
- This repo's HEAD state — `docs/slo/templates/runbook-template_v_4_template.md`
  (the artifact most of the 11 changes edit), `skills/slo-plan/`,
  `skills/slo-execute/`, `skills/slo-verify/`, `skills/slo-retro/`,
  `skills/slo-critique/` (the loop skills), `docs/LOOPS-ENGINEERING.md`,
  `docs/skill-pack-catalog.md`, `references/agent/operating-contract.md`,
  `xtasks/sast-verify/tests/sap_imp_m5_agents.rs` (structural-contract
  exemplar), `crates/sldo-install/src/install.rs` (`discover_skills`),
  `SECURITY.md`, `docs/ARCHITECTURE.md`.

## Outputs (this skill, this run)

- [docs/slo/design/outcome-first-overview.md](outcome-first-overview.md) — this file.
- [docs/slo/design/outcome-first-stack-decision.md](outcome-first-stack-decision.md)
- [docs/slo/design/outcome-first-interfaces.md](outcome-first-interfaces.md)
- [docs/slo/design/outcome-first-threat-model.md](outcome-first-threat-model.md)
- [docs/slo/design/outcome-first-threat-model.slo.json](outcome-first-threat-model.slo.json)
- [docs/slo/design/outcome-first-reversibility.md](outcome-first-reversibility.md)
- [docs/slo/design/outcome-first-code-map.md](outcome-first-code-map.md)

`ARCHITECTURE.md` and `SECURITY.md` are **not** rewritten in this design run:
`ARCHITECTURE.md` is reality-first (updated when the change ships, per repo
convention), and the existing `SECURITY.md` is preserved — Security-BDD is an
additive discipline, not a change to project-wide security defaults (idempotency
rule: no clobber).

## Compatibility posture (load-bearing)

**Every template addition is optional and additive, required only for
value-bearing milestones** — the same backward-compat posture as the v4
`§5A Measurement Contract`, `§5B Secure Value Contract`, and `§10 Carry-forward`
sections. Legacy runbooks without an Outcome Validation Contract / Outcome
Scenarios / Critical User Journeys / Regression Matrix **remain valid**;
`/slo-plan` *flags* the gap for a value-bearing feature, it does not
retroactively invalidate prior artifacts. No v4 section is renumbered or removed
— additions are insertions (§5C inserts after §5B; §17 gains sub-sections).

## Handoff

Next: `/slo-plan outcome-first` (no TLA — `tla_required: false`; no Kani —
`kani_required: false`). `/slo-plan` cuts the ≤5 milestones; the recommended
shape is in [interfaces](outcome-first-interfaces.md) §Milestone-shape hint.
