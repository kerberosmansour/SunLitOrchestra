---
name: fowler-ai-architecture-slo-improvements
tla_required: false
tla_reason: "No concurrent actors or distributed state; this is Markdown skill-contract and Rust structural-test work."
security_libs_required: false
ai_component: true
compliance: [soc2, asvs]
status: planned
---

# Design Overview — Fowler AI Architecture SLO Improvements

## Goal

Add Fowler-informed architecture and AI-era engineering disciplines to the SLO skill pack without changing host install semantics or widening the existing sprint/ticket loops.

## Target Shape

The feature adds five planning/verification disciplines:

1. **Reversibility** — `/slo-architect` emits hard-to-change decisions plus reversibility tactics and rollback/migration proof.
2. **Brownfield code map** — `/slo-architect` emits a concise map of existing components, exemplar code, anti-exemplar code, and dangerous seams before planning modifications.
3. **Exemplar code** — `/slo-plan` and ticket contracts name code to copy and code not to copy.
4. **True refactoring** — refactor budgets require behavior-preserving microsteps, tests, and evidence.
5. **AI tolerance** — AI/LLM components declare acceptable variance, deterministic boundaries, golden/scenario evals, "must never" outcomes, and verification commands.

## Non-Goals

- No new runtime host automation.
- No new executable skill runner.
- No rename of existing skills.
- No change to `sldo-install` install roots or manifest format.
- No edit to current Nettacker skill work in this branch beyond reading around it.

## Why TLA+ Is Not Required

The runbook edits Markdown skill contracts, runbook/ticket templates, and Rust structural tests. There are no concurrent actors sharing state, no ordering guarantees, no leases, and no failure-recovery protocol. The correctness risks are contract drift, missing rows, prompt-injection through source notes, and brittle template updates; structural tests and critique are the right gates.

## Architecture Diagram

```text
┌────────────────────────────────────────────────────────────────────┐
│                 SLO Fowler-Informed Skill Updates                  │
│                                                                    │
│  External sources + user notes                                     │
│   Fowler / Thoughtworks / InfoQ                                    │
│          │                                                         │
│          ▼                                                         │
│  docs/slo/research/fowler-ai-architecture-slo-improvements/        │
│          │                                                         │
│          ▼                                                         │
│  docs/slo/design/*                                                 │
│   - overview                                                       │
│   - stack decision                                                 │
│   - interfaces                                                     │
│   - threat model                                                   │
│   - reversibility matrix                                           │
│   - brownfield code map                                            │
│          │                                                         │
│          ▼                                                         │
│  docs/slo/future/RUNBOOK-FOWLER-AI-ARCHITECTURE-SLO-IMPROVEMENTS.md│
│          │                                                         │
│          ├──────────────┬────────────────┬─────────────────────┐  │
│          ▼              ▼                ▼                     ▼  │
│  /slo-architect   /slo-plan + v4   /slo-verify          /slo-critique│
│  SKILL.md         template refs    AI tolerance pass    eng persona │
│          │              │                │                     │  │
│          └──────────────┴────────────────┴─────────────────────┘  │
│                         │                                          │
│                         ▼                                          │
│                  ticket-flow parity                                │
│                                                                    │
│  Legend: solid = existing artifact family; all outputs are Markdown │
└────────────────────────────────────────────────────────────────────┘
```

## Component Summary

| Component | Responsibility | Existing/New/Changed | Milestone | Key interfaces |
|---|---|---|---|---|
| `/slo-architect` | Produces design, security, and lock-in artifacts | Changed | M1, M3 | `skills/slo-architect/SKILL.md`, design output file names |
| Reversibility matrix | Lists hard-to-change decisions and how to reverse/test them | New | M1 | `docs/slo/design/<slug>-reversibility.md` |
| Brownfield code map | Names four-object summary, exemplars, anti-exemplars, seams | New | M1 | `docs/slo/design/<slug>-code-map.md` |
| `/slo-plan` | Authors v4 runbooks one milestone at a time | Changed | M2, M3 | Contract Block rows, methodology references |
| v4 runbook template | Human-browsable mirror of SLO milestone contracts | Changed | M2, M3 | `docs/slo/templates/runbook-template_v_4_template.md` |
| `/slo-verify` | Runtime QA and security Pass 4 | Changed | M3 | AI tolerance pass |
| `/slo-critique` | Adversarial runbook review | Changed | M4 | Engineering persona architecture-coherence pass |
| Ticket flow | Issue-sized contract path | Changed | M5 | Ticket template + plan/execute/verify skills |
| Rust structural tests | Keep Markdown contracts from silently drifting | New/changed | M1-M5 | `crates/sldo-install/tests/e2e_fowler_ai_arch_m<N>.rs` |

## Data Flow Summary

| Flow | From | To | Protocol/Mechanism | Bounded? | Failure mode | Milestone |
|---|---|---|---|---|---|---|
| Source claims | Research artifacts | Runbook/design docs | Markdown citations | yes | Unsourced claim drifts into skill prose | M1-M5 |
| Reversibility decisions | `/slo-architect` | `/slo-plan` Contract Blocks | Markdown file path citation | yes | A hard-to-change decision is locked without rollback proof | M1-M2 |
| Exemplar code | Code map | Plan/ticket Contract Blocks | File path rows | yes | Agent copies messy legacy pattern | M1-M2, M5 |
| AI tolerance | AI component design | `/slo-verify` | Scenario eval rows and command evidence | yes | Nondeterministic behavior marked green without tolerance proof | M3 |
| Critique feedback | `/slo-critique` | Runbook revision | Findings table | yes | Coherence gap left as theory instead of a concrete fix | M4 |

## User-Visible Outcome

A future SLO user can ask for a feature runbook and receive contracts that explicitly say:

- which architectural decisions are hard to reverse;
- which code to copy;
- which legacy areas not to copy;
- what counts as true refactoring;
- what variance is acceptable for AI behavior;
- how critique and verification will catch drift.
