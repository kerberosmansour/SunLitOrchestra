# Engineering loops — SunLit Orchestra

> **Purpose**: name the cyclic feedback structures that move work through the engineering side of the skill pack, so a newcomer (human or freshly-loaded Claude instance) can answer "which loop am I in, and what do I run next?" in 90 seconds.
>
> **Companion doc**: business loops live at [docs/LOOPS-BUSINESS.md](LOOPS-BUSINESS.md). Static structure is in [docs/ARCHITECTURE.md](ARCHITECTURE.md). This doc is the cyclic complement.

---

## Start here

Pick the row that matches the question you have right now. The "First skill" column is what to run; the "Loop" column is the section below that explains why.

| Your question | First skill | Loop | Expected artifact |
|---|---|---|---|
| "I have a weird technical hunch — can we explore it before making it a feature?" | `/slo-experiment` | [Innovation Sandbox loop](#innovation-sandbox-loop) | `docs/slo/experiments/<slug>/EXPERIMENT.md` |
| "I have an idea — is it worth building?" | `/slo-ideate` | [Sprint loop](#sprint-loop) | `docs/slo/idea/<slug>.md` |
| "I'm starting a new feature, what do I do?" | `/slo-ideate` then `/slo-research` | [Sprint loop](#sprint-loop) | `docs/RUNBOOK-<feature>.md` once `/slo-plan` completes |
| "I have a GitHub issue — can an agent take it?" | `/slo-ticket-pick #<issue>` | [Ticket loop](#ticket-loop) | `docs/slo/tickets/ticket-<issue>-<slug>.md` |
| "I have a repeated regression — where do I start?" | `/slo-resume` (orient) then check prior `docs/slo/lessons/` | [Lessons loop](#lessons-loop) | A scope candidate at the next milestone's pre-flight |
| "Findings keep coming back from SAST — how do I tune?" | `/slo-rulegen --extend` | [Security-tuning loop](#security-tuning-loop) | A new rule pack rev under `.semgrep/<lang>/` |
| "A milestone needs secure code or secure cloud resources — what should the agent use?" | `/slo-execute` | [Secure-construction loop](#secure-construction-loop) | A surface map with matched secure libraries, tests, gaps, or residual risks |
| "Did the feature we shipped actually create value?" | `/slo-ideate` (success thesis) then `/slo-plan` (§5A) | [Feature-performance loop](#feature-performance-loop) | A success thesis, a §5A Measurement Contract, Pass 6 telemetry evidence, and a results-vs-thesis readout |
| "An upstream tool has a gap — what now?" | `/slo-sec-libs` (when shipped) | [Library-feedback loop](#library-feedback-loop) | An issue in the upstream repo |
| "I stepped away — where was I?" | `/slo-resume` | (any) | A one-screen orientation message |

Each loop below documents **user-visible outcome**, **trigger**, **steps**, **exit condition**, **artifacts**, **skills involved**, and a **diagram**.

---

## Sprint loop

> **User-visible outcome**: a runbook closes with all milestones `done`, a PR is open, and the lessons file teaches the next sprint.

**Trigger**: a new feature or non-trivial change is about to start.

**Steps**:

1. `/slo-ideate` — interrogate the idea, produce `docs/slo/idea/<slug>.md`.
2. `/slo-research` — sourced dossier under `docs/slo/research/<slug>/`.
3. `/slo-architect` — `ARCHITECTURE.md` updates plus stack lock-in, sets `tla_required` / `kani_required`.
4. `/slo-tla` — only when `tla_required: true`; verify the design (protocol level).
4b. `/slo-kani` — only when `kani_required: true`; verify small bounded Rust kernels (code level). Pairs with `/slo-tla` by refinement (action → fn → harness); Kani never claims concurrency.
5. `/slo-plan` — author `docs/RUNBOOK-<feature>.md` interactively, one milestone at a time.
6. `/slo-critique` — adversarial four-pass review BEFORE any milestone executes.
7. Per milestone: `/slo-execute M<N>` → `/slo-verify M<N>` → `/slo-retro M<N>`.
8. `/slo-ship` — open the PR with a runbook-aware description.

**Exit condition**: every milestone tracker row is `done`, every Evidence Log row has an Actual Result, the PR is open, and a completion summary plus lessons file is written.

**Artifacts**: `docs/slo/idea/<slug>.md`, `docs/slo/research/<slug>/`, `docs/RUNBOOK-<feature>.md`, `docs/slo/lessons/<prefix>-m<N>.md`, `docs/slo/completion/<prefix>-m<N>.md`, the PR.

**Skills involved**: `/slo-ideate`, `/slo-research`, `/slo-architect`, `/slo-tla`, `/slo-kani`, `/slo-plan`, `/slo-critique`, `/slo-execute`, `/slo-verify`, `/slo-retro`, `/slo-ship`.

```
   /slo-ideate ──► /slo-research ──► /slo-architect ──► /slo-plan
        ▲                                               │
        │                                               ▼
        │                                          /slo-critique
        │                                               │
        │                                               ▼
   /slo-retro ◄── /slo-verify ◄── /slo-execute ─────────┘
        │                                               ▲
        │                                               │
        └───────────► next milestone ──────────────────┘
                              │
                              ▼
                          /slo-ship
```

### Secure Value Loop overlay (every stage carries a security output)

The Sprint loop runs inside a **Secure Value Envelope**: each stage produces a cybersecurity artefact, so security is unavoidable rather than bolted on. Canonical definition: [docs/SECURE-VALUE-LOOP.md](SECURE-VALUE-LOOP.md). The rule: *optimize for the smallest valuable, secure, testable, unblocked, reviewable outcome.*

| Stage | Security output | Where |
|---|---|---|
| Idea | high-level risks, data-classification guess, trust-boundary hints | `/slo-ideate` |
| Research | security source pack (standards, scopes, threat intel, prereqs) | `/slo-research` |
| Architect | threat model (STRIDE, abuse cases, mitigations) | `/slo-architect` Step 3.5 |
| Critique | security assessment (class-elimination, variant analysis) | `/slo-critique` |
| Plan | §5B Secure Value & Security Contract (Value Wedge, Operator Readiness, Security Test Plan, Detected Work Ledger) | `/slo-plan` |
| Execute | proactive controls, Operator Readiness Gate (fail closed), Detected Work Ledger (every finding disposed) | `/slo-execute` |
| Verify | Bundle A–F security tests as first-class evidence (Pass 4/5) | `/slo-verify` |
| Ship | secure-release checklist, `ship_state`, SBOM/provenance when applicable, residual-risk sign-off | `/slo-ship` |
| Retro | dispose every ledger row through existing lanes; upstream feedback; reusable rules | `/slo-retro` |

Honest exit states (additive milestone status): `human_review_required | blocked_by_operator | blocked_by_upstream | issue_filed | accepted_risk` — an unknown status is treated as `blocked`, never silently `done`. **Review cadence**: re-check the Bundle table and cited standard editions (OWASP Proactive Controls / ASVS / MASVS / API / LLM Top 10, by name + year) when an upstream edition changes — that is the anti-drift control.

### Outcome-First overlay (the user outcome is the primary Definition of Done)

Distinct from the Secure Value overlay (security envelope) above: the Outcome-First overlay makes **user outcomes first-class, testable artifacts**. *Code completion alone is insufficient — a value-bearing milestone is done only when the promised user outcome exists AND existing important outcomes still exist.* The test pyramid is **authority-inverted**:

```
        OUTCOME      ← smallest layer, HIGHEST authority  (/slo-verify Pass 0)
          E2E
      Integration
         Unit        ← largest layer, base authority
```

| Stage | Outcome-First output | Where |
|---|---|---|
| Plan | §5C Outcome Validation Contract + §17 Outcome Scenarios (`oc-N`) / Critical User Journeys (`cuj-N`) / Core Capability Regression Matrix | `/slo-plan` (required for value-bearing milestones) |
| Execute | Outcome Scenario + Critical Journey tests written first, front-to-end (never mock-only) | `/slo-execute` |
| Verify | **Pass 0 Outcome Validation** — runs outcomes/journeys/regression front-to-end; highest authority (fails the milestone even if Passes 1–6 are green) | `/slo-verify` |
| Critique | flags outcome-test theatre (vacuous / single-`And` / mock-only) as `ask` | `/slo-critique` |
| Retro | refuses to close on an unproven outcome row; records `## Outcome vs promise` | `/slo-retro` |

The rule: **if 1000 unit tests pass but one Outcome Scenario, Critical User Journey, or required Regression-Matrix row fails, the milestone fails.** Enforced by `/slo-verify` Pass 0 + `/slo-retro`'s refusal gate; the principle is in [`references/agent/operating-contract.md`](../references/agent/operating-contract.md).

---

## Innovation Sandbox loop

> **User-visible outcome**: a fuzzy technical hunch turns into either a promotable candidate (idea / ticket / research / runbook) or a documented dead-end with reusable lessons — never dying in chat. Every experiment closes with exactly one honest exit state.

**Trigger**: the user has a material, theme, technology, surprising failure, or "what if?" but **not yet a crisp feature** — the pre-idea phase `/slo-ideate` is too early for.

**Steps** (shipping across the innovation-loop runbook; M1 ships `/slo-experiment` + the template, M2–M5 the phase skills):

1. `/slo-experiment <slug>` — open/resume `docs/slo/experiments/<slug>/EXPERIMENT.md` from `experiment-book-template_v_1.md`; seed §0–§2 + tracker; validate the slug; fence user strings.
2. `/slo-sandbox` — §3 choose the material (not the feature) + safety rails + probe seeds. *(M2)*
3. `/slo-play` — §4 raw probes, dead-ends, surprises; **divergent, judge safety only**. *(M2)*
4. `/slo-pattern` — §5 name reusable tricks + next-curve + DICEE. *(M3)*
5. `/slo-precision` — §6 make invisible variables measurable (accept/kill thresholds). *(M3)*
6. `/slo-spike` — §7 bounded proof artifacts, the only code phase (scratch under `experiments/<slug>/`). *(M4)*
7. `/slo-curate` — §8 one disposition per candidate. *(M5)*
8. `/slo-demo` — §9 demo + §10 PromotionPacket → `/slo-ideate` | `/slo-ticket-plan` | `/slo-research` | `/slo-plan`, or §11 compost. *(M5)*

**Exit condition**: every experiment closes with exactly one of the frozen 8 states (`promote_to_idea | promote_to_ticket | promote_to_research | promote_to_runbook | needs_more_play | blocked_by_unknown | killed_but_reusable | archive_no_action`); promoted candidates carry a filled §10 handoff seed; nothing reaches production without re-entering the Sprint or Ticket loop.

**Artifacts**: `docs/slo/experiments/<slug>/EXPERIMENT.md` (+ optional `probes/`, `spikes/`, `evidence/`); scratch under `experiments/<slug>/<spike-id>/` (git-ignored).

**Skills involved**: `/slo-experiment`, `/slo-sandbox`, `/slo-play`, `/slo-pattern`, `/slo-precision`, `/slo-spike`, `/slo-curate`, `/slo-demo`.

```
   fuzzy "what if?" ──► /slo-experiment ──► EXPERIMENT.md (§0–§11)
                              │
   /slo-sandbox ─► /slo-play ─► /slo-pattern ─► /slo-precision ─► /slo-spike ─► /slo-curate ─► /slo-demo
   (framing)     (DIVERGENT)   (converge)      (measure)         (evidence)    (decide)        (handoff)
                              │
                              └──► promote_to_idea/ticket/research/runbook → Sprint/Ticket loop
                                   killed_but_reusable / archive_no_action → §11 compost
```

> **Promotion is a typed handoff, never an in-loop merge** — the hard rule is that nothing becomes production without the normal SLO plan → critique → execute → verify gates. *(All 8 skills shipped across innovation-loop M1–M5; the loop is closed end-to-end.)*

---

## Innovation Sandbox loop

> **User-visible outcome**: a fuzzy technical hunch turns into either a promotable candidate (idea / ticket / research / runbook) or a documented dead-end with reusable lessons — never dying in chat. Every experiment closes with exactly one honest exit state.

**Trigger**: the user has a material, theme, technology, surprising failure, or "what if?" but **not yet a crisp feature** — the pre-idea phase `/slo-ideate` is too early for.

**Steps** (shipping across the innovation-loop runbook; M1 ships `/slo-experiment` + the template, M2–M5 the phase skills):

1. `/slo-experiment <slug>` — open/resume `docs/slo/experiments/<slug>/EXPERIMENT.md` from `experiment-book-template_v_1.md`; seed §0–§2 + tracker; validate the slug; fence user strings.
2. `/slo-sandbox` — §3 choose the material (not the feature) + safety rails + probe seeds. *(M2)*
3. `/slo-play` — §4 raw probes, dead-ends, surprises; **divergent, judge safety only**. *(M2)*
4. `/slo-pattern` — §5 name reusable tricks + next-curve + DICEE. *(M3)*
5. `/slo-precision` — §6 make invisible variables measurable (accept/kill thresholds). *(M3)*
6. `/slo-spike` — §7 bounded proof artifacts, the only code phase (scratch under `experiments/<slug>/`). *(M4)*
7. `/slo-curate` — §8 one disposition per candidate. *(M5)*
8. `/slo-demo` — §9 demo + §10 PromotionPacket → `/slo-ideate` | `/slo-ticket-plan` | `/slo-research` | `/slo-plan`, or §11 compost. *(M5)*

**Exit condition**: every experiment closes with exactly one of the frozen 8 states (`promote_to_idea | promote_to_ticket | promote_to_research | promote_to_runbook | needs_more_play | blocked_by_unknown | killed_but_reusable | archive_no_action`); promoted candidates carry a filled §10 handoff seed; nothing reaches production without re-entering the Sprint or Ticket loop.

**Artifacts**: `docs/slo/experiments/<slug>/EXPERIMENT.md` (+ optional `probes/`, `spikes/`, `evidence/`); scratch under `experiments/<slug>/<spike-id>/` (git-ignored).

**Skills involved**: `/slo-experiment`, `/slo-sandbox`, `/slo-play`, `/slo-pattern`, `/slo-precision`, `/slo-spike`, `/slo-curate`, `/slo-demo`.

```
   fuzzy "what if?" ──► /slo-experiment ──► EXPERIMENT.md (§0–§11)
                              │
   /slo-sandbox ─► /slo-play ─► /slo-pattern ─► /slo-precision ─► /slo-spike ─► /slo-curate ─► /slo-demo
   (framing)     (DIVERGENT)   (converge)      (measure)         (evidence)    (decide)        (handoff)
                              │
                              └──► promote_to_idea/ticket/research/runbook → Sprint/Ticket loop
                                   killed_but_reusable / archive_no_action → §11 compost
```

> **Promotion is a typed handoff, never an in-loop merge** — the hard rule is that nothing becomes production without the normal SLO plan → critique → execute → verify gates. *(All 8 skills shipped across innovation-loop M1–M5; the loop is closed end-to-end.)*

---

## Secure-construction loop

> **User-visible outcome**: `/slo-execute` starts implementation with secure defaults already selected, not discovered after the fact.

**Trigger**: a milestone touches request boundaries, auth, secrets, persistence,
subprocesses, SQL, UI/DOM, GitHub Actions, Pulumi/Hulumi cloud resources, or
another security-relevant surface.

**Steps**:

1. `/slo-execute` reads the contract block, security context, and threat model.
2. It builds a surface map before BDD tests are written.
3. Rust surfaces are matched through `/slo-sec-libs` against SunLitSecurityLibraries declarations.
4. Pulumi TypeScript cloud surfaces use the secure-IaC lane; Hulumi is preferred when it is explicit or detected.
5. If a capability is missing, `/slo-sec-libs` records or files the gap before local code hand-rolls the control.
6. `/slo-verify` selects security tests from the surface map and threat model.

**Exit condition**: every touched surface has one of `matched secure capability`,
`control-first fallback`, `capability gap filed`, or `residual risk accepted`
with evidence.

**Artifacts**: milestone Evidence Log rows, optional
`docs/slo/verify/<prefix>-dogfood.md`, upstream issues, and capability
declaration updates.

**Skills involved**: `/slo-execute`, `/slo-sec-libs`, `/slo-plan`, `/slo-verify`,
`/slo-dast-tuner`, `/slo-cloud-threat-model`.

```
   /slo-execute pre-flight
          │
          ▼
   surface map ──► /slo-sec-libs match
          │              │
          │              ├── matched capability ──► BDD + implementation
          │              │
          │              └── gap ──► upstream issue/fix or residual risk
          ▼
   /slo-verify security-test selector
          │
          ▼
   runtime/static/IaC evidence, or N/A with reason
```

---

## Ticket loop

> **User-visible outcome**: one GitHub issue turns into a compact SLO ticket contract, a bounded branch, a reviewable PR, and an issue workpad with validation evidence.

**Trigger**: a small GitHub issue or tracker ticket should be taken on without creating a full multi-milestone runbook.

**Steps**:

1. `/slo-ticket-pick` — select or claim one GitHub issue, apply the bite-sized gate, and create/update the issue workpad.
2. `/slo-ticket-plan` — write `docs/slo/tickets/ticket-<issue>-<slug>.md` from `docs/slo/templates/ticket-contract-template_v_1.md`.
3. `/slo-ticket-execute` — implement BDD-first inside the ticket contract's file allow-list.
4. `/slo-ticket-verify` — run runtime checks, static/security gates, compatibility checks, and regression-test-first bug handling.
5. `/slo-ticket-close` — fill closure summary, open/update the PR, and move the issue to review without auto-merge.

**Exit condition**: every ticket Validation Plan row is pass or N/A-with-reason, the issue workpad is current, the PR is open, and follow-ups are surfaced with a lane (`micro | milestone | fresh-runbook`).

**Artifacts**: `docs/slo/tickets/ticket-<issue>-<slug>.md`, issue workpad comment marked `slo-ticket-workpad:v1`, optional `docs/slo/verify/ticket-<issue>-<slug>.md`, the PR.

**Skills involved**: `/slo-ticket-pick`, `/slo-ticket-plan`, `/slo-ticket-execute`, `/slo-ticket-verify`, `/slo-ticket-close`.

```
   GitHub issue
        │
        ▼
   /slo-ticket-pick ──► issue workpad
        │
        ▼
   /slo-ticket-plan ──► docs/slo/tickets/ticket-<issue>-<slug>.md
        │
        ▼
   /slo-ticket-execute ──► code + tests + evidence rows
        │
        ▼
   /slo-ticket-verify ──► runtime/static/security evidence
        │
        ▼
   /slo-ticket-close ──► PR + issue review state
```

**Escalation rule**: if the ticket fails the sizing gate, stop and route to the [Sprint loop](#sprint-loop) with `/slo-plan` and the full v4 runbook template.

---

## Security-tuning loop

> **User-visible outcome**: SAST signal stays sharp — false positives drop, real findings keep landing, and every fix produces a regression rule that catches the next variant.

**Trigger**: a SAST finding (true positive or false positive) lands, OR a new threat-model row introduces a CWE the current rule pack does not cover, OR `/slo-architect` sets `security_libs_required: true` and the existing pack does not yet have rules for the named capability.

**Steps**:

1. `/slo-architect` — confirm or update the threat model row (CWE references, abuse cases) that the rule should defend.
2. `/slo-sast` — emit or refresh the workflow plus baselined `.semgrep.yml` so the new rule lands in CI.
3. `/slo-rulegen --extend` — generate 3-5 variation rules from the agent-found bug summary plus fix diff. New rules are appended ONLY after `cargo xtask sast-verify gate` passes for every rule.
4. `/slo-ruleverify` — re-run the deterministic gate (`validate + test + check-coverage + check-clean`) to confirm the pack still passes end-to-end.
5. `/slo-verify` — runtime QA against the BDD scenarios that introduced the finding.
6. `/slo-critique` — security pass surfaces residual risk and abuse-case coverage gaps.

**Exit condition**: `cargo xtask sast-verify gate` is green, the new rule(s) detect the original variant plus 2-3 reasonable evasions, and the threat model row that motivated the rule references it.

**Artifacts**: `.semgrep/<lang>/<rule>.yml`, paired test corpora, `.semgrep/manifest.json`, the threat-model row update.

**Skills involved**: `/slo-architect`, `/slo-sast`, `/slo-rulegen`, `/slo-ruleverify`, `/slo-verify`, `/slo-critique`.

```
   threat-model row (CWE)
            │
            ▼
       /slo-sast ─────► .github/workflows/sast.yml
            │
            ▼
   /slo-rulegen --extend ──► .semgrep/<lang>/*.yml
            │
            ▼
   /slo-ruleverify ─► cargo xtask sast-verify gate
            │
            ▼          (gate green = rule lands)
       /slo-verify ──► /slo-critique (security pass)
            │
            └──► next finding feeds back into the threat model row
```

---

## Lessons loop

> **User-visible outcome**: a lesson learned at milestone M<N> is visible at milestone M<N+k>'s pre-flight, NOT just buried in a markdown file. The same regression does not get re-introduced two milestones later.

**Trigger**: `/slo-retro` runs at the close of any milestone.

**Steps**:

1. `/slo-retro` writes `docs/slo/lessons/<prefix>-m<N>.md` (always — discipline rule, even if `gh` is unavailable).
2. `/slo-retro` classifies each lesson as `product`, `upstream-OSS`, or `slo-process`.
3. `/slo-retro` dedupes via `gh search issues` (three-strike: literal + NFKC-normalized + ASCII-collapsed).
4. `/slo-retro` files each lesson as a tracked issue with **explicit user confirmation** — never auto-files.
5. Fallback: when `gh` is unavailable, the lesson is appended to `LESSONS-BACKLOG.md` with a 12-field audit row (date, classification, prefix, agent_version, originating_milestone, dedupe_search_result, filed_to, issue_url_or_local_ref, disposition, body_sha256, retry_count, status).
6. At the next milestone, `/slo-execute` pre-flight queries open prior-retro issues filtered by the runbook's prefix and surfaces them as scope candidates with a suggested lane (`micro | milestone | fresh-runbook`).
7. `/slo-resume` reads the runbook tracker plus the optional "Carry-forward from prior retros" section to emit one next action and lane.

**Exit condition**: every lesson worth filing is either filed (with confirmation) or recorded in `LESSONS-BACKLOG.md`; the next milestone's pre-flight surfaces the open ones.

**Artifacts**: `docs/slo/lessons/<prefix>-m<N>.md`, `docs/slo/completion/<prefix>-m<N>.md`, GitHub issues with `retro-derived` marker (locked in `skills/slo-retro/references/issue-filing-discipline.md`), optional `LESSONS-BACKLOG.md` rows, the runbook's "Carry-forward from prior retros" section.

**Skills involved**: `/slo-retro`, `/slo-execute`, `/slo-resume`.

```
   /slo-retro M<N>
        │
        ├── writes docs/slo/lessons/<prefix>-m<N>.md (always)
        │
        ├── classify each lesson ─► product / upstream-OSS / slo-process
        │
        ├── dedupe via gh search (three-strike)
        │
        ├── confirm with user ──► gh issue create (argv-list, no --repo)
        │                          OR LESSONS-BACKLOG.md (gh unavailable)
        ▼
   /slo-execute M<N+k> pre-flight
        │
        ├── gh issue list --label retro-derived --search prefix
        │
        └── surface as scope candidates
                │
                ▼
        /slo-resume reads tracker + carry-forward
                │
                └── emits one next action + lane (micro | milestone | fresh-runbook)
```

---

## Feature-performance loop

> **User-visible outcome**: the team can answer "did the feature we shipped actually create value, and what should we change next?" — not just "does it technically work?". Every value-bearing feature leaves planning with a written success thesis + measurement contract, ships with verified telemetry, and gets an honest results-vs-thesis readout.

**Trigger**: a value-bearing feature enters the sprint loop — `/slo-ideate` names a success thesis, and `/slo-plan` requires a §5A Measurement Contract.

**Steps**:

1. `/slo-ideate` — Q3 asks for the smallest *complete* value slice and the idea doc's `## Success thesis` names the leading metric, lagging metric, guardrails, and review window.
2. `/slo-product metrics` — the feature gets a measurement spec (north-star link, leading/lagging metric, guardrails, telemetry requirements) and the artifact sets `feature_measurement_spec: true`.
3. `/slo-plan` — the runbook's §5A Measurement Contract is filled (value hypothesis, review windows, metrics, guardrails, telemetry deliverables, rollout, diagnosis, experiment, privacy controls); each value-bearing milestone names its slice in the Contract Block **Measurement deliverables** row.
4. `/slo-execute` → `/slo-verify` Pass 6 — milestones emit the named telemetry; Pass 6 checks events fire, are masked/pseudonymised, emit on failure paths, the `feature_measurement_spec` flag isn't gamed, and author strings are fenced.
5. `/slo-retro` — the lessons file's `## Results vs thesis` records whether the leading + lagging metrics moved and what that implies (iterate / hold / cut / ship next).
6. Post-ship — the founder reads the financial half via `/slo-metrics consumer|b2b` cohort tracking against the thesis window (see [LOOPS-BUSINESS.md](LOOPS-BUSINESS.md)); PM-side adoption is refreshed via `/slo-product metrics`.

**Exit condition**: the success thesis is either affirmed (telemetry verified, leading/lagging on-target) or disconfirmed (a review-window threshold breached) — and the next runbook's scope reflects the decision.

**Artifacts**: idea-doc `## Success thesis`; `docs/biz-public/product/metrics.md` feature spec (`feature_measurement_spec: true`); runbook §5A Measurement Contract + per-milestone Measurement-deliverables rows; `/slo-verify` Pass 6 evidence; lessons-file `## Results vs thesis`.

**Skills involved**: `/slo-ideate`, `/slo-product`, `/slo-plan`, `/slo-execute`, `/slo-verify`, `/slo-retro`, `/slo-metrics`.

```
   /slo-ideate (## Success thesis)
        │
        ▼
   /slo-product metrics (feature spec; feature_measurement_spec: true)
        │
        ▼
   /slo-plan §5A Measurement Contract ──► per-milestone Measurement deliverables
        │
        ▼
   /slo-execute ──► /slo-verify Pass 6 (events fire? masked? failure-path? flag? fenced?)
        │
        ▼
   /slo-retro ## Results vs thesis ──► affirmed / disconfirmed
        │
        ├── post-ship: /slo-metrics consumer|b2b (cohort vs thesis window)
        │
        └── decision feeds the next runbook's scope
```

---

## Library-feedback loop

> **User-visible outcome**: when SLO discovers a capability gap or bug in an upstream tool (Semgrep, Playwright, `cargo audit`, etc.), the lesson does not die in a local markdown file — it gets filed against the upstream repo and re-checked when the upstream improves.

> **Status**: the dedicated upstream-filing surface ships in **Runbook 4** (`/slo-sec-libs`). Until then, upstream-OSS classified lessons go through the [Lessons loop](#lessons-loop) and rely on `/slo-retro`'s issue-filing flow with the upstream-OSS classification (resolved via `.sldo/upstream-mapping.toml`). This section is here so the loop is documented up-front; the dedicated skill is the next iteration, not a removed feature.

**Trigger**: `/slo-execute` (or another skill) discovers a bug or capability gap in an upstream tool while pursuing the current milestone's contract.

**Steps** (target shape, ships with R4):

1. Capture the gap as a lesson during `/slo-retro` with classification `upstream-OSS`.
2. Resolve the upstream repo via `.sldo/upstream-mapping.toml` (with crates.io / npm fallback resolution).
3. `/slo-sec-libs` files an issue against the resolved upstream repo via `gh issue create` (argv-list discipline, NO `--repo` flag, rate-limit cap of 40 issues/hour per session, body wrapped in `~~~text` fence per `/slo-architect` template).
4. The local milestone proceeds with whatever workaround the runbook's allow-list permits.
5. When the upstream issue is closed and a release ships, the next sprint loop iteration re-checks against the new upstream version; if the gap is fixed, the workaround is removed and a regression test pinned to the new upstream version is added.

**Exit condition** (target): every `upstream-OSS` lesson is either filed against an upstream repo or recorded in `LESSONS-BACKLOG.md` with the `filed_to: <upstream>` audit row, and the next sprint that touches that subsystem re-checks for upstream resolution.

**Artifacts**: filed upstream issues; `.sldo/upstream-mapping.toml`; `LESSONS-BACKLOG.md` rows for unfileable items.

**Skills involved**: `/slo-sec-libs` (Runbook 4), `/slo-retro`, `/slo-execute`.

```
   /slo-execute M<N>
        │
        │ (discovers upstream gap)
        ▼
   /slo-retro classifies as `upstream-OSS`
        │
        ▼
   resolve upstream via .sldo/upstream-mapping.toml
        │
        ▼
   /slo-sec-libs ──► gh issue create (argv-list, no --repo)
                       │
                       ▼
              upstream repo: <owner>/<project>
                       │
                       ▼
              upstream fix lands in a release
                       │
                       ▼
   next sprint re-checks; remove workaround; add pinned regression test
```

---

## Anti-process-theatre check

Every loop here exists because it produces a user-visible outcome the static architecture doc cannot make visible. Loop diagrams are kept short on purpose: the artifact tells you the answer; the loop only names the cycle.

If a future addition to this doc cannot point at a concrete user-visible outcome that an existing loop already produces, that addition belongs in a skill's reference file, not in this doc.

---

## See also

- [docs/ARCHITECTURE.md](ARCHITECTURE.md) — static structure of the skill pack at HEAD.
- [docs/LOOPS-BUSINESS.md](LOOPS-BUSINESS.md) — business-side loops (user-interview, GTM, pricing, founder-check).
- [docs/slo/templates/runbook-template_v_4_template.md](templates/runbook-template_v_4_template.md) — the canonical planning artifact whose "Carry-forward from prior retros" section is the lessons loop's read-back. (The earlier [v3 template](templates/runbook-template_v_3_template.md) remains in place for runbooks already authored against it.)
- [docs/slo/templates/ticket-contract-template_v_1.md](slo/templates/ticket-contract-template_v_1.md) — compact v4-derived contract for the ticket loop.
- [docs/slo/design/ticket-sized-slo-workflow.md](slo/design/ticket-sized-slo-workflow.md) — proposed GitHub Issues-first workflow inspired by Symphony.
- [skills/slo-retro/SKILL.md](../skills/slo-retro/SKILL.md) — the writer end of the lessons loop.
- [skills/slo-execute/SKILL.md](../skills/slo-execute/SKILL.md) — the reader end of the lessons loop (pre-flight carry-forward).
- [skills/slo-resume/SKILL.md](../skills/slo-resume/SKILL.md) — one-screen orientation across loops.
- [docs/RUNBOOK-measurement-loop-slo-improvements.md](RUNBOOK-measurement-loop-slo-improvements.md) — the runbook that introduced the Feature-performance loop (success thesis → measurement contract → Pass 6 → results-vs-thesis).
