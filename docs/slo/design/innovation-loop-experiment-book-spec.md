# Experiment Book v1 — Creative Experiment Contract (authoritative template spec)

> This is the binding specification for `docs/slo/templates/experiment-book-template_v_1.md`,
> shipped by **M1** of [docs/RUNBOOK-innovation-loop.md](../../RUNBOOK-innovation-loop.md).
> The Experiment Book is to **experimentation** what the v4 runbook is to **delivery**:
> the same discipline (artifact-driven, gated, evidence-bearing, honest exit states),
> the inverted aim — **Definition of Learned, not Definition of Done**.
> The runbook is for building; the experiment book is for learning.
>
> Guiding rule that keeps it joyful: **early phases protect weirdness; later phases
> demand evidence; nothing skips the promotion gate.**

## Why a separate contract (not the v4 runbook)

A production runbook says "we know what we are building — now constrain, execute,
verify, ship." An experiment book says "we do not yet know the right idea — now
constrain the sandbox, play, observe, measure, and decide honestly." The Book must
protect three things at once:

| Need | Risk if missing | Experiment Book answer |
|---|---|---|
| Joy / creativity | the loop becomes a bureaucratic pre-runbook | divergent phases explicitly defer judgment (Judgment Timing Rule) |
| Safety / discipline | prototypes become production by accident | hard rails, data classification, scratch-only code, no-promotion gate |
| Handoff quality | cool ideas die in chat | typed promotion packet into `/slo-ideate`, `/slo-ticket-plan`, `/slo-research`, `/slo-plan` |

## The single durable artifact

`docs/slo/experiments/<slug>/EXPERIMENT.md` — one navigational artifact, like the
v4 runbook is the single source of truth for execution. Optional supporting files
are created ONLY when a real spike produces them (keep v1 lightweight):

```
docs/slo/experiments/<slug>/
├── EXPERIMENT.md            # the Book (everything lives here by default)
├── probes/probe-NNN.md      # only if a probe needs its own file
├── spikes/spike-NNN.md      # only if a spike needs its own file
├── evidence/{screenshots,metrics,notes}/
├── demo.md
└── handoff.md
```

Path is **experiments**, not "innovation" — "innovation" is an outcome; "experiment"
is the work. ("Innovation Sandbox loop" remains the loop name in prose;
"SLO Experimentation Loop" is an accepted alias.)

---

## Template skeleton (§0–§11)

```md
# EXPERIMENT-<slug> — <title> (Creative Experiment Contract v1)

> Purpose: explore a fuzzy technical/product hunch safely before turning it into delivery work.
> Creative posture: play first, judge later. Surprise is a valid signal. Dead ends are useful evidence.
> Hard rule: nothing here becomes production without entering the normal SLO Sprint or Ticket loop.
> Output: one honest route decision — promote, continue, block, kill, or archive.
```

### §0 Experiment Metadata

| Field | Value |
|---|---|
| Experiment ID | `EXP-<slug>` |
| Created | `<YYYY-MM-DD>` |
| Owner | `<human / agent / team>` |
| Product area | `<Guardian / Orchestra / DSPM / phishing / agent-runtime / infra / business>` |
| Starting hunch | `<one sentence>` |
| Primary user / beneficiary | `<consumer / family protector / engineer / enterprise buyer / internal operator>` |
| Strategic lane | `B2C / B2B / platform / data / security / growth` |
| Current phase | `sandbox / play / pattern / precision / spike / curate / demo / closed` |
| Default data classification | `Public / Internal / Confidential / Restricted` |
| Production promotion allowed? | `No — must route through SLO delivery` |
| Scratch code allowed? | `yes/no; path` |
| External services allowed? | `none / listed` |
| Real user data allowed? | `no by default` |
| Review date | `<date or cadence>` |

### §1 Experiment Tracker

| Phase | Skill | Status | Input | Output | Exit decision |
|---|---|---|---|---|---|
| 1 | `/slo-sandbox` | `not_started` | hunch | sandbox charter | |
| 2 | `/slo-play` | `not_started` | sandbox charter | play log + probe cards | |
| 3 | `/slo-pattern` | `not_started` | play log | pattern catalog | |
| 4 | `/slo-precision` | `not_started` | pattern catalog | precision model | |
| 5 | `/slo-spike` | `not_started` | precision model | spike evidence | |
| 6 | `/slo-curate` | `not_started` | all evidence | curation decision | |
| 7 | `/slo-demo` | `not_started` | promoted candidate | demo + handoff | |

- **Allowed status values** (frozen): `not_started | in_progress | blocked | complete | skipped_with_reason`
- **Allowed final decisions** (frozen 8): `promote_to_idea | promote_to_ticket | promote_to_research | promote_to_runbook | needs_more_play | blocked_by_unknown | killed_but_reusable | archive_no_action`

### §2 Global Experiment Rules + Safety Rails

The 10 global rules (frozen list):

1. Do not productize inside the experiment.
2. Do not use production data unless explicitly approved.
3. Do not introduce production dependencies.
4. Do not expose public endpoints.
5. Do not store secrets in the repo, logs, screenshots, prompts, or demo artifacts.
6. Keep scratch code isolated under the declared experiment path.
7. Capture surprises, not just successes.
8. Dead ends are valid outputs if they teach something reusable.
9. Every promoted candidate must include a handoff contract.
10. Every experiment closes with one honest route decision.

**Experiment Safety Rails (defaults table):**

| Concern | Default |
|---|---|
| Production data | forbidden |
| Real secrets | forbidden |
| Public endpoint | forbidden |
| Production file changes | forbidden unless promoted through SLO |
| New dependency | allowed only in scratch with a license/security note |
| Cloud infra | no `pulumi up`; mock/local/sandbox only |
| Security primitive | do not hand-roll if SunLitSecurityLibraries / Hulumi applies |
| AI / model calls | approved proxy or offline model; no raw sensitive prompt payloads |
| Persistence | scratch only; no schema migration |
| User impact | no live-user experiment without a separate runbook/legal/privacy gate |

**Per-phase Safety Check** (every phase appends this small block):

```md
## Safety Check
- Data classification:
- Raw secrets present? yes/no
- PII present? yes/no
- External service called? yes/no
- Scratch path:
- Cleanup required:
- Abuse sketch:
```

### §2A Judgment Timing Rule (phase mood — load-bearing for joy)

Critique is **phase-dependent**. This is how the loop keeps the joy of play without
becoming vague. Agents read their phase's mode to know whether to diverge or converge:

| Phase | Mode | Agent behaviour | What may be judged |
|---|---|---|---|
| `/slo-sandbox` | framing | choose the playground, not the product | judge **boundaries**, not ideas |
| `/slo-play` | divergent | generate probes, defer judgment | judge **safety only**; defer quality judgment |
| `/slo-pattern` | convergent (sense-making) | name reusable tricks | judge **reusability** |
| `/slo-precision` | measurement | turn vague claims into handles | judge **measurability** |
| `/slo-spike` | evidence | test the smallest claim | judge **evidence** |
| `/slo-curate` | convergent | kill, continue, or promote | judge **value, risk, promotion** |
| `/slo-demo` | communication (narrative) | make it communicable | judge **clarity** |

> During `/slo-play`, critique is **banned except for safety**. During `/slo-curate`,
> critique is **mandatory**. Mode is a frozen 5-value set in the Phase Contract:
> `divergent | convergent | measurement | evidence | communication` (`framing` maps to
> the sandbox-setup posture; `sense-making`/`narrative` are prose labels for
> `convergent`/`communication`).

### §3–§9 Phase sections

Each is filled by its phase skill and opens with the **Experiment Phase Contract**
(below), then the phase-specific body. §3 Sandbox Charter · §4 Play Log · §5 Pattern
Catalog · §6 Precision Model · §7 Spike Cards & Evidence · §8 Curation Decision ·
§9 Demo Pack. Bodies are specified in "Per-phase output objects + bodies" below.

### §10 Handoff Contract (promotion seeds)

Filled by `/slo-demo` (or `/slo-curate`). One of the five seed tables in
"Promotion gates" below, matching the chosen disposition.

### §11 Compost / Lessons

What should future experiments or runbooks remember? (Always filled — even a fully
promoted experiment records what it learned; a killed one records the reusable fragment.)

---

## The Experiment Phase Contract (lighter than the v4 Contract Block)

Every phase section (§3–§9) opens with this 12-field table. It is enough structure
to make the loop repeatable without killing exploration:

| Field | Value |
|---|---|
| Phase goal | what this phase is trying to learn |
| Mode | `divergent / convergent / measurement / evidence / communication` |
| Inputs consumed | which previous outputs this phase reads |
| Primary output | the object this phase must produce |
| Creative permission | what kind of weirdness/play is allowed |
| Boundaries | what is out of scope |
| Safety rails | data, network, secret, privacy, user-impact limits |
| Scratch space | where temporary code/files may live (spike only) |
| Resource budget | CPU, memory, time, cost, external calls |
| Evidence required | notes, screenshots, metrics, commands, examples |
| Kill criteria | what would stop this line of exploration |
| Handoff requirement | what the next skill receives |

---

## Definition of Learned (replaces Definition of Done)

The Book closes on learning, not shipped scope. Three variants:

**General phase — Definition of Learned.** A phase is complete only when:
- the input artifact from the previous phase was read;
- the phase output was written into `EXPERIMENT.md`;
- any safety-rail violation is explicitly recorded;
- any dead end is captured with what it taught;
- the next phase receives a concrete handoff;
- the tracker (§1) is updated;
- the phase did NOT silently promote scratch work into production.

**Spike — Definition of Learned.** A spike is complete only when:
- the learning question was answered or explicitly blocked;
- the evidence is attached or summarized;
- accept/kill thresholds were evaluated;
- resource and safety bounds were checked;
- the scratch path is declared;
- no production files were changed;
- the spike has a decision hint.

**Curation — Definition of Learned.** Curation is complete only when:
- every candidate has exactly one disposition;
- every promoted candidate has a next SLO route;
- every killed candidate has a reusable lesson or archive reason;
- no vague "maybe" remains unowned.

---

## Per-phase output objects + bodies

The named object each skill produces (the handoff currency), plus the section body.

### `/slo-experiment` → `ExperimentBook`
Fields: `slug, title, starting_hunch, strategic_lane, current_phase: sandbox, safety_defaults, tracker`.
Body (the opening of the Book): Starting Hunch · Why This Might Be Worth Exploring ·
**Why This Is Not Yet a Feature** (why `/slo-ideate` is premature) · Default Safety
Rails · Next Step (`run /slo-sandbox <slug>`). Gate: the hunch is framed as an
exploration, not a delivery commitment.

### `/slo-sandbox` → `SandboxCharter` (+ `ProbeSeedList`)
Fields: `material, why_rich, boundaries, safety_rails, creative_constraints, weirdness_budget, initial_probe_seeds, kill_criteria, next_phase_input`.
Body (§3): Material · Why This Sandbox Is Rich · **Not a Feature Yet** · Boundaries
table (product/code/data/network/cost/time) · Creative Constraints · **Weirdness
Budget** (low/medium/high allowed?) · Probe Seed List (≥3) · Kill Criteria · Handoff.
Gate: do not proceed without ≥3 concrete probe seeds + explicit safety rails.

### `/slo-play` → `ProbeLedger` (cards) + `DeadEndList` + `StrangeButInterestingList`
`ProbeCard` fields: `id, question, type, setup, observation, surprise, reusable_mechanism, failure, safety_notes, next_candidate`.
Probe types (frozen 8): `mechanism_probe | interaction_probe | failure_probe | security_probe | data_probe | latency_probe | magic_probe | composition_probe`.
Body (§4): Phase Contract (Mode = **divergent**; judgment rule = no ranking until raw
observations captured; forbidden = turning a probe into a product plan too early) ·
Probe Board · Raw Observations · Strange But Interesting · Dead Ends table · Candidate
Patterns · Handoff. Gate: output is "enough material to name patterns", NOT "good idea found".

### `/slo-pattern` → `PatternCatalog` (+ NextCurve / ProductPull / ArchitecturePull)
`PatternCandidate` fields: `name, mechanism, surprise, where_it_applies, why_it_might_matter, risk, evidence, product_pull, architecture_pull, next_curve_potential`.
Body (§5): Phase Contract (Mode = convergent; **cite probe IDs for every pattern**;
forbidden = promoting everything) · Pattern Candidates table · **Next-Curve Check**
(10% improvement vs category change) · **DICEE Check** (Deep/Intelligent/Complete/
Empowering/Elegant) · **Sunlit Strategic Fit** (B2C/B2B/secure-data/cybersecurity) ·
Product Pull · Architecture Pull · Handoff. Gate: ≤5 serious candidates.

### `/slo-precision` → `PrecisionModel` + `ProtocolFreeze` + `ProtocolAmendmentLog`
`PrecisionModel` fields: `claim, measurement_handle, instrumentation,
accept_threshold, kill_threshold, resource_bound, safety_invariant,
false_positive_plan, false_negative_plan`.

`ProtocolFreeze` fields (labels are stable): **Protocol version · Frozen at ·
Hypothesis · Baseline · Candidate interventions · Benchmark arms · Split IDs ·
Primary metrics · Secondary metrics · Analysis plan · Scoring method · Repetition / stability rule ·
Accept rule · Kill rule · Resource budget · Risk envelope**.

`ProtocolAmendment` fields: `amendment_id, protocol_version, field, old_value,
new_value, reason, impact, author_date, validation_status`. The amendment log is
**append-only**. A change makes the prior Validation Record **stale** and requires
a full **rerun** against the new protocol version; frozen rows are never rewritten.
The rendered stable labels are **Old value · New value · Reason · Impact**.

Body (§6): Phase Contract (Mode = measurement; forbidden = accepting "feels better"
without a handle) · Claims That Need Handles · Invisible Variables · Reliability /
Compounding Risk · **False Positive / False Negative Plan** (required for any
classification/detection/retrieval/ML claim) · Resource Budget · **Security
Invariants** (what must never happen) · Protocol Source Statements · Protocol Freeze
· Protocol Amendments · Handoff. Gate: every candidate that proceeds has ≥1
falsifiable claim with an accept AND a kill threshold; an incomplete freeze
**blocks validation**.

Raw user/source statements are **literal data** inside a `~~~text` fence and
**never select control fields** such as protocol version, ids, split assignment,
thresholds, status, confidence, verdict, or route. Arms, repetitions, and the
**sample budget** are finite; "**run until good**" is forbidden.

**Legacy compatibility**: a **legacy** v1 Book without `ProtocolFreeze` is readable
in **degraded** mode. Its evidence is exploratory and **not confirmed** by inference.

### `/slo-spike` → `DiscoveryRecord` and/or `ValidationRecord` + `EvidenceLog`

Both record types use a shared bounded spike envelope: `id, learning_question,
evidence_class, scratch_path, allowed_files, data_used, dependencies,
external_calls, resource_budget, cleanup_rule, safety_result, decision_hint,
delete_or_promote`. Body (§7, per spike): Phase Contract (Mode = evidence; scratch
path = `experiments/<slug>/<spike-id>/`; production files allowed = none by
default; data = synthetic/redacted/generated; cleanup rule) plus exactly one
evidence record. Discovery and validation budgets are finite and separately
reported as **Discovery budget** and **Validation budget**.

`DiscoveryRecord` fields: `id, evidence_class=exploratory, learning_question,
starting_mechanism, mechanism_refinements, discovery_arms_split_ids, method,
exact_commands, environment, discovery_budget_declared_actual, results, surprise,
deviations, evidence_pointers, safety_result, decision_hint, delete_or_promote`.
Discovery is **exploratory** and **not confirmation**. It **may refine** the
mechanism or proposed protocol. A change to a frozen field must return to
`/slo-precision` for an append-only amendment before validation.

`ValidationRecord` fields: `id, evidence_class=validation,
active_protocol_version, baseline, candidate_interventions, benchmark_arms_split_ids,
primary_secondary_metrics, frozen_scoring_analysis, exact_commands, environment,
per_arm_results, repetitions, stability, deviations,
validation_budget_declared_actual, evidence_pointers, safety_result,
accept_kill_evaluation, validation_verdict, decision_hint, delete_or_promote`.
Validation requires one complete **active protocol version**, frozen **held-out**
benchmark arms, and **no tuning** of mechanism, prompt, data selection, scoring,
thresholds, or analysis after validation evidence is inspected. It reports the
baseline and every candidate as **per-arm** results, all finite repetitions,
stability/dispersion, failed or missing runs, exact commands, environment, and
deviations rather than only the best headline.

An unresolved amendment blocks validation. A protocol-changing deviation is
recorded through `/slo-precision`, makes the Validation Record **stale**, and
requires a full **rerun** against the new active protocol version.

Command output, corpus/source labels, benchmark text, and model output are
untrusted **literal data** inside a `~~~text` fence. Evidence strings **never
select** verdict, confidence, route, status, thresholds, or protocol fields.

**Legacy compatibility**: a **legacy** generic `SpikeCard` remains readable as
**discovery-grade** evidence and is **not confirmed** by inference.

Evidence standard is tiered:
- scratch-only no-code/prototype: evidence log + safety rails + cleanup required;
- code that may be promoted: formatter/typecheck/tests required before the promotion packet;
- production code: **not allowed in this loop**.
Gate: discovery is complete when the learning question is answered and any
freeze-impacting refinement is amended; validation is complete only when the
active freeze, no-tuning rule, every per-arm result, repetition/stability summary,
exact rerun context, bounds, safety actuals, and non-stale status are recorded.
Neither is complete because the prototype is polished.

### `/slo-curate` → `CurationDecision` + evidence-strength analysis

`CurationDecision` fields: `candidate, confidence, decision, reason,
evidence_quality, strategic_fit, security_posture, confirmation_gaps,
next_artifact, owner`. Every candidate receives exactly one confidence value from
`exploratory | confirmatory | engineering_ready` and exactly one frozen
disposition. Confidence **cannot self-upgrade** from prose or model output.

`promote_to_idea` and `promote_to_research` **may be exploratory** when their
**confirmation gaps** and the exact decision to unblock are explicit.
`promote_to_ticket` and `promote_to_runbook` require `engineering_ready`, a
complete **current Validation Record**, an ablation summary, Failure Taxonomy,
replication instructions, and limitations/uncertainty. Missing or stale evidence
**blocks engineering routes**; a confidence label alone cannot satisfy the gate.

`AblationMatrix` fields (rendered as **Ablation Matrix**): `component_intervention,
removed_or_replaced, expected_change, actual_delta, interpretation,
evidence_pointers`. Stable rendered labels include **Removed or replaced** and
**Actual delta**.

`FailureTaxonomy` fields (rendered as **Failure Taxonomy**): `failure_family,
trigger_arm, count_or_rate, severity, residual_impact, mitigation_next_test,
evidence_pointers`. Stable rendered labels include **Failure family** and
**Residual impact**.

Body (§8): Phase Contract (Mode = convergent; forbidden = keeping vague maybes
alive; every decision cites probes/spikes) · Candidate Board · Decision Rubric ·
Evidence Confidence and Route Gate · Ablation Matrix · Failure Taxonomy ·
RecommendationPacket Readiness · Final Disposition · Compost · Handoff. Gate:
every candidate receives exactly one evidence-derived confidence and disposition;
the chosen route satisfies its evidence prerequisites.

### `/slo-demo` → `RecommendationPacket` (+ DemoPack + HandoffContract)

`RecommendationPacket` fields (stable rendered labels): **Protocol version ·
Baseline · Candidate interventions · Benchmark arms · Split IDs · Primary metrics
· Secondary metrics · Ablation summary · Failure taxonomy · Replication
instructions · Exact commands · Environment · Limitations · Uncertainty ·
Confidence · Exact engineering question or decision to unblock · Evidence pointers
· One-sentence magic · Security posture · Disposition / route · Next skill / seed
artifact**.

The packet preserves the curation confidence enum
`exploratory | confirmatory | engineering_ready`; packaging **cannot self-upgrade**
it. `promote_to_idea` and `promote_to_research` may be exploratory with explicit
confirmation gaps. `promote_to_ticket` and `promote_to_runbook` require the
complete current Validation Record, Ablation Matrix, Failure Taxonomy, replication
instructions, and limitations; otherwise the gap blocks engineering routes.

Raw evidence excerpts are untrusted **literal data** inside a `~~~text` fence and
**never select** disposition, confidence, route, status, or next skill.

The legacy `PromotionPacket` is a **compatible subset** of the
`RecommendationPacket`. Missing fields **downgrade** confidence and **blocks
engineering routes**; existing fields remain readable and may support an honest
idea/research handoff without fabricated method data.

Body (§9 + §10): One-Sentence Magic · Before · After · Demo Path · Evidence table ·
Security Posture · RecommendationPacket · Productization Route (choose exactly
one) · the matching §10 seed table. Gate: the route satisfies its evidence class,
the packet is complete for that class, and the next skill is suggested rather than
auto-invoked.

---

## Promotion gates (the §10 seed tables — most important part)

Promotion is a **typed handoff**, never an auto-invocation. The disposition picks
exactly one seed table; `/slo-demo` fills it so the receiving skill starts warm.

**→ `/slo-ideate` — Idea Seed**: working title · discovered pattern · user who might
care · pain hypothesis · smallest complete value slice candidate · one-sentence magic ·
worst-day starter risks · success thesis draft · open questions · evidence from experiment.

**→ `/slo-ticket-plan` — Ticket Seed**: proposed title · exact change · why now · files
likely touched · out of scope · acceptance scenario · test expectation · security
concern · evidence from experiment.

**→ `/slo-research` — Research Seed**: research question · decision it will unblock ·
sources needed · competing approaches · claims to verify · evidence already collected.

**→ `/slo-plan` — Runbook Seed** (use rarely — only when architecture clarity already
exists): proposed runbook title · target architecture sketch · milestone candidates ·
interfaces likely touched · data classification · threat-model starter rows ·
measurement-contract starter · evidence from experiment.

**→ archive/compost — Compost Entry**: what we tried · why it failed · what it taught ·
reusable fragment · future trigger to revisit.

---

## Worked example (non-normative, for the gallery example Book at M5)

`/slo-experiment on-device embeddings for DSPM validation` → sandbox (material:
local embeddings + deterministic DSPM findings + false-positive context; constraint:
"ML validates context, does not extract secrets") → play (P1: docs examples cluster
separately from real config; dead-end: raw cosine score is not user-comprehensible) →
pattern (`ML-as-validator-not-extractor`, `finding-confidence-narrative`) → precision
(precision/recall, false-positive reduction, latency-per-finding; invariant: no raw
secret leaves device) → spike (synthetic seeded corpus; result: partial; surprise:
explanation may matter more than binary suppression) → curate (`promote_to_idea`) →
demo (one-sentence magic: "Sunlit explains *why* a finding looks real or harmless,
locally and privately"; route: `/slo-ideate embedding-context-validator`).
