---
name: measurement-loop-slo-improvements
tla_required: false
tla_reason: "No concurrent actors, shared state, or distributed protocol. This is Markdown skill-contract + template + loop-doc work, plus a single additive frontmatter key and structural-test baseline updates. Nothing to model-check."
kani_required: false
kani_reason: "No Rust kernels introduced. The only Rust touched is xtasks/sast-verify structural tests (SKILL.md SHA baselines / frontmatter assertions) — no unsafe, arithmetic, parser, or invariant kernel worth a bounded proof."
security_libs_required: false
ai_component: false
ai_reason: "The measurement loop is execution discipline expressed as Markdown contracts. It does not add a new LLM/agent invocation surface; the skills are agent-executed but this work introduces no new model call."
compliance: [soc2, asvs, gdpr]
status: planned
source_report: "/Users/sherifmansour/Downloads/deep-research-report(5).md"
---

# Overview — Measurement-loop SLO improvements

## Goal

Add a first-class **feature-performance / value-realisation loop** across SunLit Orchestra's existing
skills, so that every non-trivial feature leaves planning with a written **success thesis** and a written
**measurement contract** — value hypothesis, leading + lagging metrics, guardrails, required telemetry,
diagnosis path, experiment plan, privacy controls, and a scheduled post-release readout. This closes the
structural gap the deep-research report identified: SLO has strong *build discipline* (ideate → plan →
execute → verify → retro) but no dedicated **ship → measure → diagnose → experiment → feed-back-into-plan**
loop.

## Target shape (five disciplines added, enrichment-only)

1. **Ideation philosophy shift** — `/slo-ideate` moves from "smallest wedge" to "smallest *complete* value
   slice" (complete enough for a user to experience the core value *and* for the team to learn why it worked
   or failed), and gains a `## Success thesis` section (leading metric, lagging metric, top guardrails).
2. **Feature measurement spec** — `/slo-product metrics` gains an optional feature-level measurement spec
   (north-star link, primary leading + lagging metric, guardrails, activation/completion funnel, adoption
   thresholds, diagnostic questions, segmentation, experiment backlog, telemetry requirements). The
   `/slo-product` (PM) vs `/slo-metrics` (financial) split is **preserved** — a deliberate prior critique
   decision.
3. **Measurement Contract in the runbook** — the v4 runbook template + `/slo-plan` gain a runbook-level
   **Measurement Contract** section and a per-milestone Contract Block row, so telemetry is a contracted
   deliverable, not a best-effort afterthought.
4. **Verification of telemetry** — `/slo-verify` gains a measurement pass: event/schema presence smoke
   check, telemetry PII/masking check, failure-path emission check, and replay-tagging check where enabled.
5. **Durable learning** — `/slo-retro` gains a **Results vs thesis** section; the new loop is documented in
   `docs/LOOPS-ENGINEERING.md` (primary) with a cross-reference from `docs/LOOPS-BUSINESS.md`.

## The core architectural decision (settled here)

**Inline-first, with one additive optional frontmatter key; no separate machine-readable telemetry schema
in v1.**

- The measurement contract is **embedded inline** as named Markdown sections in the idea doc, the v4 runbook,
  and the `/slo-product metrics` artifact. Runbooks aren't governed by `references/biz/artifact-schema.md`;
  their contract surface is the v4 template itself, so the Measurement Contract is a new (optional, additive)
  template section + Contract Block row.
- Exactly **one** new optional frontmatter key is authorized for the biz artifact schema:
  `feature_measurement_spec: bool` on the `/slo-product metrics` artifact (default-absent = `false`,
  backward compatible) so `/slo-verify` and future tooling can *detect* a feature spec without parsing prose.
- `/slo-verify`'s new measurement pass operates **by convention / heuristic** (presence + pattern checks,
  exactly like the existing Pass 4 PII regex scan) — it does **not** parse a machine schema.
- A full machine-readable telemetry schema + `.slo.json` companion (mirroring the threat-model companion) is
  **explicitly deferred to a future `/slo-architect` pass**, to be promoted only once real telemetry fixtures
  exist. See [stack-decision](measurement-loop-slo-improvements-stack-decision.md) for the rejected
  alternatives and rationale; see [reversibility](measurement-loop-slo-improvements-reversibility.md) for why
  this ordering is the low-regret path.

## Non-goals

- **Not** merging `/slo-product` and `/slo-metrics` (prior critique decision; PM-side vs financial split stays).
- **Not** changing the `/slo-product` / `/slo-metrics` command verbs or output paths (stable interfaces).
- **Not** prescribing application-specific metrics or telemetry. The framework stays SLO-generic so it can be
  dogfooded without encoding private product details (per the report's stated limitation).
- **Not** adding a vendor SDK dependency (PostHog / Clarity / OpenTelemetry are cited as *architecture
  anchors* the generated contract can name — not packages this repo installs).
- **Not** building the machine-readable telemetry schema in v1 (deferred, see above).

## Why TLA+ / Kani are not required

`tla_required: false` — no concurrent actors, shared mutable state, ordering guarantees across processes,
leases/locks, or failure-recovery protocol. `kani_required: false` — the only Rust touched is structural
tests in `xtasks/sast-verify`; no unsafe / arithmetic / parser / representation-invariant kernel.

## Architecture diagram (loop overlay across existing skills)

```
┌──────────────────────────────────────────────────────────────────────────────────────┐
│                  SunLit Orchestra — Feature-Performance Loop overlay                    │
│                                                                                          │
│   IDEATE                PLAN                 EXECUTE / VERIFY            RETRO            │
│  ┌─────────┐          ┌─────────┐           ┌──────────────┐          ┌─────────┐        │
│  │/slo-    │  success │/slo-plan│ measure-  │/slo-execute  │ telemetry│/slo-    │        │
│  │ ideate  │─ thesis ▶│ + v4    │─ ment    ▶│   ↓          │  evidence│ retro   │        │
│  │         │          │ template│ contract  │/slo-verify   │─────────▶│ results │        │
│  └────┬────┘          └────┬────┘           │ (meas. pass) │          │ vs      │        │
│       │                    │                └──────┬───────┘          │ thesis  │        │
│       │ feeds              │ cites                 │                  └────┬────┘         │
│       ▼                    ▼                       │                       │              │
│  ┌──────────────────────────────┐                 │ (post-ship)           │ next runbook │
│  │ /slo-product metrics          │◀────────────────┴───────────────────────┘  scope      │
│  │  · feature measurement spec   │                                                        │
│  │  · feature_measurement_spec:  │   - - - ▶  /slo-metrics (financial KPIs — UNCHANGED,   │
│  │    true (NEW optional key)    │           cohort tracking against the thesis window)   │
│  └──────────────────────────────┘                                                        │
│                                                                                          │
│  Legend:  ─── existing skill surface   - - - cross-skill reference   ▶ artifact/data flow │
│           NEW = additive surface introduced by this work                                 │
└──────────────────────────────────────────────────────────────────────────────────────┘
```

## Component summary

| Component | Responsibility | Existing/New/Changed | Milestone (provisional) | Key interfaces |
|---|---|---|---|---|
| `skills/slo-ideate/SKILL.md` | Reframe Q3 (wedge → complete value slice); add `## Success thesis` to idea-doc output | changed | M1 | Idea-doc section contract |
| `skills/slo-product/SKILL.md` | Add feature measurement spec to `mode_arg: metrics`; set `feature_measurement_spec: true` when present | changed | M2 | `mode_arg: metrics`, output path (unchanged) |
| `references/biz/artifact-schema.md` | Add one optional key `feature_measurement_spec: bool` | changed | M2 | Biz frontmatter contract |
| `docs/slo/templates/runbook-template_v_4_template.md` | New optional **Measurement Contract** section + Contract Block row | changed | M3 | v4 runbook contract |
| `skills/slo-plan/SKILL.md` | Require Measurement Contract for value-bearing features; populate the new section/row | changed | M3 | Runbook authoring contract |
| `skills/slo-verify/SKILL.md` | New measurement pass (event presence, telemetry PII/masking, failure-path emission, replay tagging) | changed | M4 | Verify pass list |
| `skills/slo-retro/SKILL.md` | Add **Results vs thesis** section to lessons template | changed | M4 | Lessons-file template |
| `docs/LOOPS-ENGINEERING.md` / `docs/LOOPS-BUSINESS.md` | Document the feature-performance loop + cross-ref | changed | M5 | Loop catalog |
| `xtasks/sast-verify/tests/*` | Update SKILL.md SHA baselines / frontmatter assertions for edited skills | changed | each M | Structural-contract gate |

## Data flow summary

| Flow | From | To | Mechanism | Bounded? | Failure mode | Milestone |
|---|---|---|---|---|---|---|
| Success thesis | `/slo-ideate` idea doc | `/slo-product metrics`, `/slo-plan` | Markdown section (read by author) | yes — 3 metrics + guardrails | thesis absent → `/slo-plan` flags missing contract | M1 |
| Feature measurement spec | `/slo-product metrics` artifact | `/slo-plan` Measurement Contract | Markdown section + `feature_measurement_spec: true` flag | yes — 1 spec per feature | flag absent → treated as no spec | M2 |
| Measurement Contract | runbook (v4) | `/slo-execute`, `/slo-verify` | runbook section + Contract Block row | yes — per milestone | missing for value-bearing feature → plan incomplete | M3 |
| Telemetry evidence | `/slo-execute` milestone | `/slo-verify` measurement pass | Evidence Log rows + emitted events | yes — per critical event | event not firing / PII unmasked → pass fails | M4 |
| Results vs thesis | `/slo-verify` + post-ship metrics | `/slo-retro` lessons | Markdown section | yes — per thesis metric | blank actuals → retro refuses (existing rule) | M4 |
| Loop documentation | this design | `docs/LOOPS-*` | Markdown loop entry | n/a | n/a | M5 |

## User-visible outcome

- A founder/agent can answer **"how do we know we did a good job, and what should we change next?"** for any
  shipped feature — not just "does it technically work?"
- Every value-bearing runbook carries a measurement contract before execution starts; `/slo-verify` proves
  the telemetry actually fires and respects PII/masking; `/slo-retro` records actual vs predicted movement.
- The new loop is discoverable in the loop catalog as a normal operating mode, not a hidden PM add-on.
- Privacy is stricter-by-default: pseudonymised event identifiers, masking, consent, and a DPIA trigger are
  part of the contract (GDPR/PECR-aware), per the threat model.

## Hand-off

`tla_required: false`, `kani_required: false` → proceed to **`/slo-plan measurement-loop-slo-improvements`**.
Then **`/slo-critique`** before any `/slo-execute`.
