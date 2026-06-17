# Threat Model — outcome-first

> Scope: the **Outcome First Engineering** methodology change (v4 template
> sections + `/slo-plan`/`/slo-execute`/`/slo-verify`/`/slo-retro`/`/slo-critique`
> edits + loop docs + one structural test). This change ships **no service, no
> network surface, no auth, no persisted runtime state** — so most classic
> STRIDE cells are `N/A`. The genuine risks are **content-injection through
> authored outcome/journey strings**, **outcome-test theatre**, **regression-
> matrix gaming**, and **gate bypass**. Compliance columns: `soc2`, `asvs`
> (per overview frontmatter).
>
> Companion machine-readable artifact:
> [outcome-first-threat-model.slo.json](outcome-first-threat-model.slo.json)
> (frozen `tm-outcome-first-abuse-N` IDs; `/slo-critique` + `/slo-verify` cite
> it, do not re-derive).

## Components in scope

| # | Component | What it is |
|---|---|---|
| C1 | v4 template §5C / §17 outcome sections | Author-filled Markdown contract sections (Outcome statement, Success Criteria, Critical User Journeys, Regression Matrix, Outcome Scenarios). |
| C2 | `/slo-verify` Pass 0 (Outcome Validation) | Highest-authority leading runtime pass (non-renumbering); runs outcome/journey/regression at runtime (Playwright for UI). |
| C3 | `/slo-retro` outcome refusal gate | Refuses milestone close while any outcome/journey/regression row is unproven. |
| C4 | `/slo-critique` outcome-scenario review | Adversarial review of outcome scenarios for theatre/vacuity. |
| C5 | `xtasks/sast-verify` structural test | Deterministic test pinning the new sections + edited SKILL.md SHA baselines. |

## STRIDE sweep (per component)

| Component | Spoofing | Tampering | Repudiation | Info disclosure | DoS | Elevation of privilege |
|---|---|---|---|---|---|---|
| C1 template sections | N/A — no principals | **mitigated** — author strings rendered as plain Markdown body / inside `~~~text` fences in any *generated* security artifact; never select control fields (id/status/classification) | N/A | **residual** — an author may paste a real customer name / secret into an outcome scenario or journey; routed to `/slo-verify` Pass 4 PII scan + data-classification row | N/A | **eliminated** — authored outcome text is descriptive only; it cannot choose a `tm-`/`oc-`/`cuj-` id, a resolution verb, or a gate outcome |
| C2 Pass 0 verify | N/A | **mitigated** — Pass 0 runs declared tests/journeys only; no eval of author-supplied shell from the contract prose | **mitigated** — Evidence Log records command/expected/actual per outcome row (who/what/when) | **mitigated** — read the same files the milestone touches; no new exfil path | **mitigated** — bounded to the milestone's declared journeys/regression rows | **eliminated** — a green Pass 0 is "proved at runtime for the declared rows", never "whole system proved" |
| C3 retro gate | N/A | N/A | **mitigated** — refusal + reason is recorded | N/A | N/A | **mitigated** — gate is fail-closed: unknown/blank resolution treated as not-passing, never silently `done` |
| C4 critique | N/A | N/A | N/A | N/A | N/A | **mitigated** — theatre findings are `ask`, surfaced to the user; critique never auto-edits scenario semantics |
| C5 structural test | N/A | **mitigated** — SHA baseline change is explicit + PR-reviewed, never waived | N/A | N/A | **mitigated** — bounded scan over a bounded file set | N/A |

## Abuse cases

> Frozen IDs `tm-outcome-first-abuse-N` 1:1 with the `.slo.json`. Security-BDD
> outcome rows in runbooks cite these IDs. (abuse-4 added 2026-06-17 from the
> `/slo-critique` SEC-1 finding.)

### `tm-outcome-first-abuse-1` — content-injection via authored outcome/journey strings

- **Surface**: authored §5C / §17 strings that may be interpolated into a
  *generated* SECURITY/threat-model artifact downstream.
- **Attacker**: unwary author / content-injector.
- **Attack step**: paste a crafted string (Markdown/YAML/HTML metacharacters, or
  agent-readable instructions) into an Outcome statement, journey step, or
  scenario `Then` clause.
- **Attacker outcome**: template-placeholder injection — break out of the section
  or smuggle instructions into a generated security default.
- **Control**: reuse the load-bearing `/slo-architect` fence rule — every
  user-provided string rendered into a generated security/threat artifact is
  wrapped in `~~~text`; outcome sections render author text as plain Markdown
  body only and **never** select control fields (`tm-`/`oc-`/`cuj-` ids,
  resolution verbs, gate outcomes are author-structural, not free text).

### `tm-outcome-first-abuse-2` — outcome-test theatre (the green-but-vacuous outcome)

- **Surface**: `/slo-execute` (or any agent) authoring Outcome Scenarios /
  Critical User Journey tests.
- **Attacker**: an agent (or hurried author) optimising for "green" over truth —
  the central failure mode the whole change exists to prevent.
- **Attack step**: emit an Outcome test that asserts something trivially true
  (`assert(true)`, asserts a 200 without asserting the *user outcome*, or
  asserts a mock's return instead of the real front-to-end path), then mark the
  Outcome Scenario passed.
- **Attacker outcome**: a milestone closes as "outcome proven" while the promised
  user value does not exist — the exact AI-assisted-development failure the
  founder named.
- **Control**: (a) §5C Front-to-End Validation requires **per-layer
  `applicable | not_applicable(reason)` steps** (seed → backend → persisted →
  API/IPC → UI) with **≥1 real cross-layer assertion** (e.g. backend → persisted)
  even when UI is N/A — a single-layer/mock assertion is non-conformant; (b)
  `/slo-critique` flags vacuous/single-`And`/mock-only scenarios as `ask`; (c)
  `/slo-verify` Pass 0 runs the journey **front-to-end** over the highest
  *applicable* layer chain (Playwright for UI), never a single mock; (d) Outcome
  Scenarios must carry ≥1 observable-outcome `Then` plus follow-on `And`s per
  §11.3. (Theme B — the per-layer rule closes the non-UI theatre window; SEC-2.)

### `tm-outcome-first-abuse-3` — regression-matrix / gate gaming

- **Surface**: `/slo-retro` outcome refusal gate + the Core Capability
  Regression Matrix resolution column.
- **Attacker**: author gaming the gate.
- **Attack step**: mark every Regression Matrix row `pass` (or
  `waived_with_reason` with an empty reason) without running the journeys, or set
  an Outcome Scenario to passed without an Evidence Log row, to satisfy the gate.
- **Attacker outcome**: false "nothing broke" / "outcome exists" signal; an
  adjacent capability silently regresses.
- **Control**: every matrix row resolves to exactly one of `pass |
  not_applicable | waived_with_reason` with a **non-empty evidence path /
  reason — never blank** (mirrors §5B Bundle discipline); `/slo-verify` Pass 0
  re-runs the matrix journeys and writes the actuals; `/slo-retro` refuses on any
  blank actual or reasonless waiver (extends the existing blank-Evidence-Log
  refusal). Mitigation, not elimination — see residual risk R1.

### `tm-outcome-first-abuse-4` — weakening the critique SHA-pin test (added from SEC-1)

- **Surface**: the two existing SHA-pin test files (`sap_imp_m5_agents.rs`,
  `slo_tm_m2_consumers.rs`) allow-listed in M4 to bump the `slo-critique` baseline.
- **Attacker**: an executor / contributor optimising for a green gate.
- **Attack step**: instead of updating only the SHA baseline *constant*, weaken or
  remove the F-ENG-6 SHA assertion (or the `## Rotation order` / persona-anchor
  assertions) in a pin test to make it pass.
- **Attacker outcome**: the canonical-critique-path tamper-evidence is silently
  removed; `slo-critique/SKILL.md` can thereafter be modified undetected.
- **Control**: M4 constrains edits in those two files to the baseline-constant
  **value only** (Forbidden shortcuts + guard); `outcome_first_m4_consumers.rs`
  cross-checks BOTH critique baselines == the live `slo-critique` SHA; PR review of
  the two pin files. (Class V17 — weakened security control.)

## Residual risks

| ID | Risk | Exploit path | Compensating control | Accepted? | Owner | Review by |
|---|---|---|---|---|---|---|
| R1 | An author can still hand-author a thin/dishonest outcome contract or fake an Evidence-Log actual | The framework forces shape + re-runs journeys, but cannot enforce author integrity; a determined author can fake a thin contract | `/slo-verify` Pass 0 runtime re-run + `/slo-critique` theatre review + `/slo-retro` refusal raise the cost of faking; mitigation not elimination (`tm-outcome-first-abuse-2/3`) | true | Sherif | 2026-09-17 |
| R2 | This change ships *discipline*, not the target product's outcome-test runner | SLO defines the §5C contract + Pass 0 expectation; a target product with no E2E/Playwright harness can mark UI outcome rows `not_applicable` | The contract makes the front-to-end path the path of least resistance; `not_applicable` requires a reason (theme B requires ≥1 real cross-layer assertion regardless); cross-device tooling is explicitly deferred (interfaces) | true | Sherif | 2026-09-17 |
| R3 | PII/secret pasted into an outcome scenario or journey could reach a git-tracked artifact | Author embeds a real customer name/credential in a §17 row that lands under a tracked path | Data-classification row + `/slo-verify` Pass 4 PII scan + `~~~text` fence in generated artifacts; `pii_scan_override` + reason required for intentional matches | true | Sherif | 2026-09-17 |

## Compliance mapping

| Control | soc2 | asvs |
|---|---|---|
| Outcome-test authority inversion = explicit, evidenced Definition of Done | CC8.1 (change management — evidence before "done") | N/A |
| Security-BDD outcome rows exercise authn/authz + audit-event assertions | CC6.1 (logical access), CC7.2 (audit logging) | V1 Architecture, V4 Access Control, V7 Logging |
| Authored-string fence + no-control-field-selection (anti-injection) | CC7.1 | V5 Validation / Encoding |
| Gate fail-closed (unknown/blank resolution ≠ done) | CC8.1 | N/A |
| Reliability-BDD outcome rows (degraded/outage visible, no silent failure) | A1.2 (availability), CC7.3 | N/A |
