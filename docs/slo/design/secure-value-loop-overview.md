---
name: secure-value-loop
tla_required: false
tla_reason: "No concurrent actors, shared mutable state, distributed protocol, leases/locks, or failure-recovery ordering. This is Markdown skill-contract + v4-template + loop-doc work plus additive enum/section/row changes and structural-test baseline updates. Nothing to model-check."
kani_required: false
kani_reason: "No Rust kernels introduced. The only Rust touched is xtasks/sast-verify structural tests (SKILL.md SHA baselines, frontmatter/section assertions, dual-copy byte-identity) — no unsafe, arithmetic, parser, or representation-invariant kernel worth a bounded proof."
security_libs_required: false
security_libs_reason: "This work adds runbook-contract discipline; it does not implement a security boundary in code. The SecureLibraries adoption rule it documents points downstream runbooks at SunLitSecurityLibraries/Hulumi — it does not itself consume them."
ai_component: false
ai_reason: "Execution discipline expressed as Markdown contracts. The skills are agent-executed but this work introduces no new LLM/agent invocation surface or model call."
compliance: [soc2, asvs]
status: planned
source_proposal: "~/Downloads/sunlit_orchestra_secure_value_loop(1).md"
---

# Overview — Secure Value Loop

## Goal

Wrap SunLit Orchestra's existing sprint loop (Ideate → Research → Architect →
Critique → Plan → Execute → Verify → Ship → Retro) in a **Secure Value
Envelope** so every value-bearing runbook carries three *typed, unavoidable*
disciplines that the pack does not enforce today, while reusing — not rebuilding
— the ~80% of security machinery already shipped.

The operating rule the envelope encodes:

> Do not optimize for the smallest wedge. Optimize for the smallest **valuable,
> secure, testable, unblocked, reviewable** outcome.

## What already ships (explicit non-goals — do NOT rebuild)

The grounding sweep (see [research synthesis](../research/secure-value-loop/synthesis.md))
confirmed these proposal items are **already shipped**; this work cites them, it
does not re-implement them:

- Threat model as a first-class `/slo-architect` Step 3.5 artifact (`SECURITY.md`
  + `*-threat-model.md` + `*-threat-model.slo.json`, STRIDE, frozen
  `tm-<slug>-abuse-N` IDs, compliance mapping, `tla/kani/security_libs/ai`
  flags). → proposal Stage 4.
- `/slo-critique` class-elimination + variant-analysis security persona,
  concrete-scenario-only gate. → proposal Stage 5.
- `/slo-verify` Pass 4 (SAST/SCA/secrets/IaC/DAST-via-`/slo-dast-tuner`/privacy-PII)
  + Pass 5 (LLM, gated on `ai_component`). → proposal Stage 8 + most of Bundle E.
- Nine security skills (`slo-sast`, `slo-rulegen`, `slo-ruleverify`,
  `slo-dast-tuner`, `slo-nettacker`, `slo-cloud-threat-model`, `slo-sec-libs`,
  `slo-kani`, `slo-tla`) + the threat-model producer runbook. → proposal's scanner
  lanes + Hulumi lane + SecureLibraries adoption.
- `/slo-retro` lane-classified issue filing (`product | upstream-OSS |
  slo-process`) with dedupe + per-session cap. → proposal Stage 10 (partial).
- v4 Contract Block rows: `Data classification`, `Proactive controls in play`,
  `Abuse acceptance scenarios`, `AI tolerance contract`, `Measurement
  deliverables`. → proposal Stage 6 (partial).

## Target shape (three disciplines added + the doc/template surface, enrichment-only)

1. **Operator Readiness Gate** — a first-class, *pre-execution* milestone state.
   The runbook declares each milestone's operator prerequisites (cloud account,
   OAuth app, API key, test device, DNS, approval) with owner + needed-by +
   validation; `/slo-execute`'s Global Entry refuses to start a milestone whose
   readiness is `blocked` with `safe_to_continue_without_blockers: false`.
2. **Detected Work Ledger** — a runbook artifact + `/slo-execute` discipline.
   Every finding discovered mid-execution gets exactly one **disposition** and
   may never end as merely "observed". The five dispositions **reuse, not
   replace**, `/slo-retro`'s existing taxonomy (see the reconciliation in
   [interfaces](secure-value-loop-interfaces.md)).
3. **Honest exit states** — an *additive* milestone-status vocabulary
   (`human_review_required | blocked_by_operator | blocked_by_upstream |
   issue_filed | accepted_risk`) extending — never replacing — the existing
   `not_started | in_progress | blocked | done`, so `/slo-resume`, `/slo-execute`
   Step 1.5, and the Tracker parser keep working unchanged.

Plus the surface that makes the three usable:

4. **Canonical `docs/SECURE-VALUE-LOOP.md`** — the one place the envelope is
   defined, citing NIST SSDF (PO/PS/PW/RV outcome groups) + OWASP Proactive
   Controls **2024 edition by name** as the external authority.
5. **v4-template "Secure Value & Security Contract" section** — optional/additive
   (same posture as §5A Measurement Contract and §10 Carry-forward): Value Wedge,
   Security Definition of Ready (Operator Readiness), Detected Work Ledger,
   Security Test Plan referencing **Bundles A–F** (resolved by `/slo-verify`
   surface detection, not a new test runner). Both template copies stay
   byte-identical.
6. **LOOPS docs** — `docs/LOOPS-ENGINEERING.md` (primary) names each stage's
   security output; `docs/LOOPS-BUSINESS.md` gets the "security-visible proof of
   safety" cross-reference.

## The core architectural decision (settled here)

**Inline-first, additive-only, no NEW crate, no new taxonomy.** (One additive
edit to the EXISTING `sldo-common` crate in M3 — F-ENG-1, surfaced by
`/slo-critique`: make the published `MilestoneStatus` parser total over the
additive status set. Additive + semver bump, not a new crate.)

- The three disciplines live as **inline Markdown sections/rows** in the v4
  runbook, exactly mirroring the Measurement-Contract precedent. No machine
  schema, no `.slo.json` for the ledger in v1.
- The milestone-status enum is **extended additively**; the documented status
  comment in the template lists the new values as optional, and the structural
  test asserts the *old four still parse*.
- The ledger dispositions **map onto** existing `/slo-retro` lanes rather than
  shipping a third vocabulary (the single most important anti-duplication
  decision — see [stack-decision](secure-value-loop-stack-decision.md)).
- OWASP proactive controls are cited **by name + edition year (2024)**, never bare
  number, because OWASP renumbered C1–C10 between 2018 and 2024
  ([dossier](../research/secure-value-loop/dossier.md)). The existing v4 Contract
  Block row ("e.g., C1, C5, C9") is tightened in the same milestone.
- SBOM/provenance in the Ship checklist is **conditional** — resolves to
  `not_applicable` for the common markdown-skill milestone; only release-artifact
  milestones (crates.io publish, release-zip) trigger it.
- The primary deterministic gate is the **structural-contract test** under
  `xtasks/sast-verify/tests/`; M3 adds **round-trip unit tests** in the existing
  `sldo-common` crate for the extended status enum. No new crate, no runtime.

## Why no root ARCHITECTURE.md / SECURITY.md regeneration

Following the measurement-loop precedent (a peer meta-change to SLO): the
"architecture" of a skill-pack overlay is the loop diagram below, captured in
this overview. The repo-root `SECURITY.md` already exists and governs the SLO
project itself; this work does not change the project's own security posture
(`security_libs_required: false`, contained threat surface), so it is **not
regenerated** — clobbering it would violate the idempotency rule for no benefit.

## Architecture diagram (envelope overlay across existing skills)

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│                    SunLit Orchestra — Secure Value Envelope overlay                       │
│                                                                                          │
│  IDEATE/RESEARCH    ARCHITECT       CRITIQUE        PLAN            EXECUTE / VERIFY       SHIP / RETRO │
│  ┌──────────┐      ┌──────────┐    ┌──────────┐   ┌──────────┐    ┌──────────────┐      ┌──────────┐  │
│  │ value +  │      │ threat   │    │ security │   │/slo-plan │    │/slo-execute  │      │/slo-ship │  │
│  │ risk     │─────▶│ model    │───▶│ assess.  │──▶│ + v4     │───▶│  · Operator  │─────▶│ secure   │  │
│  │ hints    │ SHIP │ (Step3.5)│    │ (persona)│   │ Secure   │    │    Readiness │ test │ release  │  │
│  │ (exists) │ PED  │ (exists) │    │ (exists) │   │ Value &  │    │    Gate  NEW │ evid.│ checklist│  │
│  └──────────┘      └──────────┘    └──────────┘   │ Security │    │  · Detected  │      │  NEW     │  │
│                                                    │ Contract │    │    Work      │      └────┬─────┘  │
│                                                    │   NEW    │    │    Ledger NEW│           │        │
│                                                    └────┬─────┘    │/slo-verify   │           ▼        │
│                                                         │ cites    │ Pass4/5 evid │      ┌──────────┐  │
│                                                         │          │  (exists)    │      │/slo-retro│  │
│                          ┌──────────────────────────────┘          └──────┬───────┘      │ dispose  │  │
│                          ▼                                                 │ findings     │ ledger   │  │
│                    ┌──────────────────────────┐                           │              │ rows via │  │
│                    │ docs/SECURE-VALUE-LOOP.md │   additive status enum:   │              │ EXISTING │  │
│                    │  (canonical envelope def, │   human_review_required / │              │ lanes NEW│  │
│                    │   SSDF + OWASP PC 2024)   │   blocked_by_operator /   │              └────┬─────┘  │
│                    │            NEW            │   blocked_by_upstream /   │                   │        │
│                    └──────────────────────────┘   issue_filed/accepted_risk────────────────────┘ feeds  │
│                                                                                              next runbook │
│  Legend:  ─── existing skill surface   - - - reference   ▶ artifact/data flow   NEW = additive surface  │
│           Bundles A–F = security-test selection inputs to /slo-verify Pass 4 (NOT a new runner)         │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

## Component summary

| Component | Responsibility | Existing/New/Changed | Milestone (provisional) | Key interfaces |
|---|---|---|---|---|
| `docs/SECURE-VALUE-LOOP.md` | Canonical envelope definition; cites SSDF + OWASP PC 2024 by name; the agent one-page prompt | NEW | M1 | Project doc; referenced by LOOPS + v4 template |
| `docs/slo/templates/runbook-template_v_4_template.md` + `skills/slo-plan/references/...` (dual copy) | Add optional **Secure Value & Security Contract** section + tighten proactive-controls row; both copies byte-identical | changed | M1 | v4 runbook contract |
| `skills/slo-plan/SKILL.md` | Require the contract for value-bearing/security-relevant milestones; populate Value Wedge + Operator Readiness + Security Test Plan | changed | M2 | Runbook authoring contract |
| v4 template Milestone Tracker status comment + `skills/slo-execute/SKILL.md` + `skills/slo-resume/SKILL.md` | Additive status enum; `/slo-execute` Global Entry Operator-Readiness check; `/slo-resume` recognises new states read-only | changed | M3 | Milestone-status vocabulary (additive) |
| `crates/sldo-common/src/runbook.rs` (+ `Cargo.toml` version) | Extend published `MilestoneStatus` parser to be total over the additive status set; unknown→`Blocked`; fix `all_done` silent-completion (F-ENG-1); crates.io 0.1.2→0.1.3 | changed | M3 | `sldo-common::runbook` public API |
| `skills/slo-execute/SKILL.md` + v4 template ledger block | Open/update Detected Work Ledger; every finding gets a disposition; refuse `done` while a row is undisposed | changed | M4 | Ledger row contract + disposition vocab |
| `skills/slo-retro/SKILL.md` (+ `references/issue-filing-discipline.md`) | Dispose every ledger row through existing lanes; reconcile 5 dispositions ↔ existing taxonomy | changed | M4 | Issue-filing/lane vocabulary |
| `skills/slo-verify/SKILL.md` | Record Bundle A–F security tests as first-class evidence rows (`pass`/`not_applicable`/`waived_with_reason`) | changed | M4 | Verify pass list / evidence rows |
| `skills/slo-ship/SKILL.md` | Secure-release checklist (SBOM/provenance *when applicable*, canary, rollback, residual-risk owner, `ship_state`) | changed | M5 | Ship checklist / `ship_state` |
| `docs/LOOPS-ENGINEERING.md` / `docs/LOOPS-BUSINESS.md` | Name each stage's security output; cross-ref | changed | M5 | Loop catalog |
| `xtasks/sast-verify/tests/*` | New structural tests: section/row presence, dual-copy byte-identity, additive-enum (old four still present), SKILL SHA baselines | NEW + changed | each M | Structural-contract gate |

## Data flow summary

| Flow | From | To | Mechanism | Bounded? | Failure mode | Milestone |
|---|---|---|---|---|---|---|
| Operator prerequisites | `/slo-plan` (Secure Value Contract) | `/slo-execute` Global Entry | runbook section + Contract Block row | yes — per milestone | `blocked` + `safe_to_continue_without_blockers:false` → milestone must not start | M2→M3 |
| Detected work | `/slo-execute` (during run) | Ledger → `/slo-retro` | runbook ledger table; disposition per row | yes — per finding | undisposed row → milestone not `done` | M4 |
| Milestone status | `/slo-execute`/`/slo-retro` | Tracker, `/slo-resume` | Tracker Status cell (extended enum) | yes — closed enum | unknown value → parser falls back to `blocked` semantics | M3 |
| Bundle selection | Secure Value Contract Security Test Plan | `/slo-verify` Pass 4/5 | Bundle table reference resolved by surface detection | yes — A–F | wrong bundle → over/under-testing; waiver requires reason | M4 |
| Secure-release evidence | `/slo-verify` + Ship checklist | `/slo-ship` `ship_state` | checklist + `ship_state` field | yes — closed enum | critical untriaged finding → not `shipped` | M5 |
| Envelope definition | this design | `docs/SECURE-VALUE-LOOP.md`, LOOPS | Markdown | n/a | n/a | M1/M5 |

## User-visible outcome

- An agent/operator knows **before a milestone starts** exactly what human action
  is required — no more "operator assistance needed" surprises mid-run.
- Every finding discovered during execution ends with a named disposition and
  owner; nothing rots as "observed".
- A milestone can end honestly as `human_review_required` / `blocked_by_operator`
  / `blocked_by_upstream` / `issue_filed` / `accepted_risk` instead of being
  forced into `done` or a bare `blocked`.
- Every value-bearing runbook carries a Secure Value & Security Contract; legacy
  runbooks keep working untouched.

## Hand-off

`tla_required: false`, `kani_required: false` → proceed to **`/slo-plan
secure-value-loop`**. Then **`/slo-critique`** before any `/slo-execute`.
