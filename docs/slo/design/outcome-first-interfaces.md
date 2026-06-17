# Interfaces — outcome-first

Stability levels: `stable` (frozen — rename/reshape requires migration work),
`evolving` (may change with migration), `internal` (fair game). Mirrors the
convention in [measurement-loop-slo-improvements-interfaces.md](measurement-loop-slo-improvements-interfaces.md)
and [biz-skill-pack-interfaces.md](biz-skill-pack-interfaces.md).

## Stable interfaces this work MUST NOT break

| Interface | Stability | Description |
|---|---|---|
| Existing v4 template sections (§1–§20, incl. §5A, §5B) | `stable` | `docs/slo/templates/runbook-template_v_4_template.md`. Not renumbered or removed; §5C is an insertion after §5B, §17 sub-sections and §11 layer rows are insertions. |
| v3 template | `stable` | `runbook-template_v_3_template.md` untouched; historical artifact. |
| `/slo-verify` Pass 4/5/6 contract (security, AI tolerance, measurement) | `stable` | Re-indexed downward by the new leading pass, but their *content + resolution vocabulary* (`pass \| not_applicable \| waived_with_reason`) is unchanged. |
| `/slo-verify` "regression-test-first on every bug found" flow | `stable` | The STOP → regression test → hand back to `/slo-execute` → re-verify loop is preserved and reused by Pass 0 findings. |
| `/slo-retro` existing refusal gates (blank Evidence-Log actuals, pending BDD, untracked test artifacts, blank Kani row) | `stable` | The new outcome/journey/regression refusal is *additive* to this list. |
| `/slo-plan` §5A / §5B requirement logic for value-bearing / security-relevant milestones | `stable` | The new §5C requirement is a peer addition, same trigger shape. |
| `/slo-critique` four-persona pass structure + finding format | `stable` | Outcome-scenario review folds into the existing eng-lead + security passes; no new persona. |
| Threat-model frozen ID scheme `tm-<slug>-abuse-N` | `stable` | Security-BDD rows *cite* these IDs; they do not introduce a parallel scheme. |
| `xtasks/sast-verify` structural-contract gate | `stable` | Every edited SKILL.md keeps the gate green; SHA baselines updated in the same milestone. |
| `crates/sldo-install` `discover_skills()` | `stable` | Untouched — no new skill directory is added (elevate-in-place decision). |

## New additive interfaces introduced by this work

> All are **optional on legacy runbooks, required by `/slo-plan` for
> value-bearing milestones** (the §5A/§5B precedent). "Value-bearing" =
> introduces or changes user-facing capability; pure refactor / docs / tooling
> is exempt with `N/A — <reason>`.

| Interface | Stability | Description |
|---|---|---|
| v4 **§5C Outcome Validation Contract** section | `evolving` | Per-milestone table: `Outcome` (the promised user value, one sentence), `Success Criteria` (bulleted, observable), `Front-to-End Validation` (ordered steps: seed test data → run → verify backend → verify persisted record → verify API/IPC response → verify UI display), `Regression Requirements` (which core capabilities must still work). Inserted after §5B. |
| v4 §17 **Outcome Scenarios** sub-section | `evolving` | The *primary Definition of Done*. Outcome-shaped Gherkin: one observable user outcome + follow-on `And` assertions (severity shown, remediation shown, appears in history, survives restart). All rows MUST be automated. Includes **security** rows (cite `tm-<slug>-abuse-N`) and **reliability** rows (degraded/outage outcomes). Distinct from the existing per-scenario BDD Acceptance table (which stays, as lower-authority detail). |
| v4 §17 **Critical User Journeys** sub-section + Contract Block row | `evolving` | Declared per value-bearing milestone; each journey is an ordered front-to-end path with a frozen ID `cuj-<slug>-N`. Each becomes a mandatory automated test. |
| v4 §17 **Core Capability Regression Matrix** sub-section | `evolving` | `Capability \| Must still pass \| Evidence path \| Resolution` table. Resolution ∈ `pass \| not_applicable \| waived_with_reason` — **never blank** (mirrors §5B Bundle discipline). Cross-capability "did it break anything important?" proof. |
| v4 §11 **Outcome test layer** | `evolving` | New layer above E2E in §11.4 Test File Naming + the §11 layer narrative: `Outcome` tests are user-centric, cross-system, cross-service, cross-device — input → backend → storage → processing → UI → user outcome. Naming: `tests/outcome/<prefix>_outcome_<journey>.<ext>` (backend) / `outcome/<journey>.outcome.test.tsx` (frontend, Playwright-driven for UI). |
| v4 §6 Global Execution Rule **"Outcome outranks unit"** | `stable` (semantic) | New rule encoding the authority inversion: a failing Outcome Validation / Critical Journey / Regression Matrix row blocks milestone completion regardless of unit/integration pass counts. |
| Frozen ID scheme **`cuj-<slug>-N`** (Critical User Journey) | `stable` | Contiguous from 1, never renumbered (same discipline as `tm-<slug>-abuse-N`). Cited by Outcome Scenarios and Pass 0 evidence rows. |
| Frozen ID scheme **`oc-<slug>-N`** (Outcome Scenario) | `stable` | Contiguous from 1, never renumbered. The Definition of Done references these IDs. |
| `/slo-verify` **Pass 0 = Outcome Validation** (leading, highest authority, non-renumbering) | `evolving` | Runs Outcome Scenarios + Critical User Journeys (front-to-end, Playwright for UI) + Core Capability Regression Matrix + the security/reliability outcome rows, at runtime. **Highest authority**: a Pass 0 failure fails the milestone even if all other passes are green. Inserted ahead of Pass 1; existing Passes 1–6 keep their numbers + content (so repo-wide `Pass 4/5/6` citations are untouched). |
| `/slo-retro` **outcome refusal gate** | `stable` | Refuses to close a value-bearing milestone while any Outcome Scenario, Critical User Journey, or Regression Matrix row is unproven / blank / `waived_with_reason`-without-reason. Additive to the existing refusal list. |
| `/slo-retro` lessons **`## Outcome vs promise`** section | `evolving` | Did the promised user outcome actually materialise at runtime? Which adjacent outcomes were at risk and how were they preserved? Complements the existing `## Results vs thesis` (measurement loop). |
| `docs/LOOPS-ENGINEERING.md` Sprint-loop **Outcome-First overlay** + pyramid note | `evolving` | Documents the authority inversion and the "Verify Pass 0 = Outcome Validation gate" in the standard loop format; cross-referenced from the Secure Value overlay. |
| Outcome First Engineering **principle** | `stable` (semantic) | Added to `references/agent/operating-contract.md` + `docs/skill-pack-catalog.md`: "Code completion alone is insufficient. A milestone is done only when the promised user outcome exists AND existing important outcomes still exist." |

## Explicitly deferred interfaces (NOT created in v1)

| Deferred interface | Why deferred | Promotion trigger |
|---|---|---|
| `<slug>-outcome.slo.json` machine-readable outcome/journey schema companion | No outcome-test fixtures exist yet; freezing a schema now is stable-interface debt with no consumer (same reasoning that deferred the measurement `.slo.json`). | A future `/slo-architect` pass once a dogfooded runbook emits real outcome-journey fixtures. |
| New `/slo-outcome` installable skill (separate Gate 6 binary) | Founder chose elevate-in-place to avoid duplicate runtime-BDD ownership. | A future decision that the gate needs standalone invocation independent of full verify. |
| Cross-device / cross-service outcome harness tooling | The proposal names cross-device outcome tests; SLO ships the *contract*, not a device farm. The target product supplies the runner. | A target product with a real multi-device surface and a dogfood need. |

## Compatibility commitments

- **Every addition is optional and additive.** Legacy v4 runbooks without §5C /
  Outcome Scenarios / Critical User Journeys / Regression Matrix remain valid;
  `/slo-plan` *flags* the gap for a value-bearing feature, it does not
  invalidate prior artifacts.
- **No section renumber, no command-verb change, no output-path change, no pass
  renumber.** §5C inserts after §5B; §17 gains sub-sections; `/slo-verify` keeps
  its name and argument shape (`/slo-verify M<N>`); Pass 4/5/6 keep their numbers
  AND content.
- **Pass 0 is a non-renumbering insertion.** Outcome Validation is inserted as a
  leading "Pass 0"; the `/slo-verify` SKILL.md and the structural test record
  that Pass 0 is Outcome Validation and that Passes 1–6 are unchanged (this is
  the DW-001 decision that avoids breaking repo-wide `Pass 4/5/6` citations).
- **Structural-test gate stays green.** Edited SKILL.md SHA baselines and the new
  template-section assertions in `xtasks/sast-verify` are updated in the same
  milestone as the edit, never waived.

## Milestone-shape hint (advisory — `/slo-plan` owns the cut)

A natural ≤5-milestone decomposition, **core-load-bearing first** (the founder
chose "all 11, phased"):

1. **M1 — Template contract.** v4 §5C Outcome Validation Contract + §17 Outcome
   Scenarios / Critical User Journeys / Core Capability Regression Matrix
   sub-sections + §11 Outcome layer + §6 "Outcome outranks unit" rule + the
   `oc-`/`cuj-` ID schemes. Structural test asserts the new sections exist.
2. **M2 — `/slo-plan` enforcement.** Require §5C + Outcome Scenarios + Critical
   User Journeys + Regression Matrix for value-bearing milestones; BDD-specificity
   gate extended to outcome-shape. SHA baseline updated.
3. **M3 — `/slo-verify` Pass 0 = Outcome Validation.** Insert the leading
   highest-authority pass WITHOUT renumbering (Passes 1–6 unchanged); wire the
   regression-test-first flow to Pass 0 findings. SHA baseline updated.
4. **M4 — `/slo-retro` gate + `/slo-execute` + `/slo-critique`.** Retro outcome
   refusal gate + `## Outcome vs promise`; execute writes outcome/journey tests
   first; critique reviews outcome scenarios for theatre. SHA baselines updated.
5. **M5 — Principle + loop docs.** Outcome First Engineering principle in
   operating-contract + catalog; LOOPS-ENGINEERING Outcome-First overlay +
   inverted pyramid; cross-references.

Security BDD (Change #7) and Reliability BDD (Change #8) are **not their own
milestones** — they are row-types inside the §17 Outcome Scenarios sub-section
delivered in M1 and enforced in M2–M4, so the "all 11" scope lands without
exceeding the 5-milestone cap.
