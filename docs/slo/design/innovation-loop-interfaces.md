# Interfaces — innovation-loop

Every interface downstream milestones must not rename or reshape without
explicit migration work. Stability levels: `stable` (frozen), `evolving` (may
change with migration), `internal` (fair game).

## 1. Skill surface (commands)

| Skill | Invocation | Stability |
|---|---|---|
| `/slo-experiment <slug>` | open or resume `docs/slo/experiments/<slug>/EXPERIMENT.md` | `stable` |
| `/slo-sandbox <slug>` | fill §3 Sandbox Charter | `stable` |
| `/slo-play <slug>` | fill §4 Play Log | `stable` |
| `/slo-pattern <slug>` | fill §5 Pattern Catalog | `stable` |
| `/slo-precision <slug>` | fill §6 Precision Model | `stable` |
| `/slo-spike <slug> [spike-id]` | fill §7 Spike Cards + evidence | `stable` |
| `/slo-curate <slug>` | fill §8 Curation Decision | `stable` |
| `/slo-demo <slug>` | fill §9 Demo + §10 Handoff | `stable` |

Each skill is `skills/slo-<name>/SKILL.md`, frontmatter `name` + `description`
(folded), discovered by `discover_skills()`. `name` values frozen as above.

## 2. Persisted-artifact shapes

### 2.1 Experiment Book — `docs/slo/experiments/<slug>/EXPERIMENT.md` — `stable`

The single durable artifact. Section order is frozen (skills target sections by
heading); follows `docs/slo/templates/experiment-book-template_v_1.md`, whose
**authoritative spec is [innovation-loop-experiment-book-spec.md](innovation-loop-experiment-book-spec.md)**.
Beyond the section list below, the spec freezes these contract elements (all
`stable`): the **Definition of Learned** blocks (general / spike / curation) that
replace Definition of Done; the **§2A Judgment Timing Rule** (phase mood —
critique is phase-dependent; `/slo-play` = safety-only); the **Experiment Safety
Rails** defaults table + per-phase **Safety Check** block in §2; the per-phase
**primary output objects** (§4 below); and the five **§10 promotion-seed tables**
(Idea / Ticket / Research / Runbook / Compost seeds).

```
§0  Experiment Metadata        (table: id, created, owner, product area,
                                starting hunch, beneficiary, strategic lane,
                                current phase, data classification,
                                production-promotion-allowed = No,
                                scratch-code path, external-services,
                                real-user-data = no-by-default, review date)
§1  Experiment Tracker         (per-phase: skill, status, input, output, exit)
§2  Global Experiment Rules    (the 10 hard rails — frozen list)
§3  Sandbox Charter            (/slo-sandbox)
§4  Play Log                   (/slo-play)
§5  Pattern Catalog            (/slo-pattern)
§6  Precision Model            (/slo-precision; includes ProtocolFreeze + append-only amendments)
§7  Spike Cards and Evidence   (/slo-spike)
§8  Curation Decision          (/slo-curate)
§9  Demo Pack                  (/slo-demo)
§10 Handoff Contract           (/slo-demo or /slo-curate)
§11 Compost / Lessons          (what future experiments/runbooks remember)
```

Optional supporting files under `docs/slo/experiments/<slug>/` (created only
when a real spike produces them, to keep v1 lightweight):
`probes/probe-NNN.md`, `spikes/spike-NNN.md`, `evidence/{screenshots,metrics,notes}/`,
`demo.md`, `handoff.md`.

### 2.2 Scratch code — `experiments/<slug>/<spike-id>/` (repo root) — `stable` (path), `internal` (contents)

The **only** place `/slo-spike` may write executable code. Bounded by the
spike's declared file/data/network/dependency/resource budget. Carries a
delete-or-promote decision. NEVER promoted to a real package without the Sprint
or Ticket loop. (Reversibility doc covers the git-tracked-vs-ignored decision.)

### 2.3 Promotion handoff (`§10 Handoff Contract`) — `stable`

When `/slo-curate` promotes a candidate, `/slo-demo` writes a
`RecommendationPacket` naming the destination skill + the next artifact path.
The legacy `PromotionPacket` is a compatible subset; missing rigor fields
downgrade confidence and blocks engineering routes rather than being invented:

| Decision | Next skill | Next artifact |
|---|---|---|
| `promote_to_idea` | `/slo-ideate` | `docs/slo/idea/<slug>.md` |
| `promote_to_ticket` | `/slo-ticket-plan` | `docs/slo/tickets/ticket-<issue>-<slug>.md` |
| `promote_to_research` | `/slo-research` | `docs/slo/research/<slug>/` |
| `promote_to_runbook` | `/slo-plan` | `docs/RUNBOOK-<feature>.md` |

## 3. Frozen vocabularies

### 3.1 Exit states (Experiment-level + per-spike `Decision Hint`) — `stable`

```
promote_to_idea | promote_to_ticket | promote_to_research | promote_to_runbook
| needs_more_play | blocked_by_unknown | killed_but_reusable | archive_no_action
```

`unknown` ⇒ `blocked_by_unknown` (never silently terminal). Adding/removing a
value is a migration. Every experiment closes with exactly one experiment-level
state; every candidate in §8 gets exactly one disposition.

### 3.2 Phase status values (§1 tracker) — `stable`

```
not_started | in_progress | blocked | complete | skipped_with_reason
```

### 3.3 Phase modes (each Phase Contract's `Mode` row) — `stable`

```
divergent | convergent | measurement | evidence | communication
```

(`/slo-play` = divergent — judgment deferred; `/slo-pattern` / `/slo-curate` =
convergent; `/slo-precision` = measurement; `/slo-spike` = evidence;
`/slo-demo` = communication.)

### 3.4 Probe types (§4) — `evolving`

```
mechanism_probe | interaction_probe | failure_probe | security_probe
| data_probe | latency_probe | magic_probe | composition_probe
```

(`evolving`: a new probe type is additive and does not break consumers.)

## 4. Phase handoff objects (consume → produce)

The contract between adjacent skills. Each is a labelled block inside the
section the producing skill fills; the next skill reads it. Shapes are
`stable` at the field-name level; bodies are prose.

| Skill | Consumes | Produces (object) |
|---|---|---|
| `/slo-experiment` | fuzzy hunch | `ExperimentBook` (§0–§2 + tracker seeded) |
| `/slo-sandbox` | `ExperimentBook` | `SandboxCharter` + `ProbeSeedList` |
| `/slo-play` | `SandboxCharter`, `ProbeSeedList` | `ProbeLedger` (probe cards), `DeadEndList`, `StrangeButInterestingList` |
| `/slo-pattern` | `ProbeLedger` | `PatternCatalog` (+ `NextCurveCandidates`, `ProductPull`, `ArchitecturePull`) |
| `/slo-precision` | `PatternCatalog` | `PrecisionModel` + `ProtocolFreeze` + append-only `ProtocolAmendmentLog` |
| `/slo-spike` | `PrecisionModel` + active `ProtocolFreeze` | `DiscoveryRecord` and/or `ValidationRecord` + `EvidenceLog` (+ optional scratch) |
| `/slo-curate` | all prior | `CurationDecision` + evidence confidence + `AblationMatrix` + `FailureTaxonomy` + `CompostEntries` |
| `/slo-demo` | promoted candidate | `RecommendationPacket` (`PromotionPacket` compatible subset) + Demo Pack + §10 seed |

### `ProtocolFreeze` handoff fields — `stable`

**Protocol version · Frozen at · Hypothesis · Baseline · Candidate interventions ·
Benchmark arms · Split IDs · Primary metrics · Secondary metrics · Analysis plan ·
Scoring method · Repetition / stability rule · Accept rule · Kill rule · Resource
budget · Risk envelope**.

`ProtocolAmendment` is append-only and records amendment id, protocol version,
field, **Old value**, **New value**, **Reason**, **Impact**, author/date, and
validation status. An amendment makes earlier validation **stale** and requires a
**rerun**. A **legacy** v1 Book without these objects remains readable in
**degraded** mode; its evidence is exploratory and **not confirmed** by inference.

### `DiscoveryRecord` and `ValidationRecord` handoff fields — `stable`

Both records carry the shared bounded envelope: **record/spike ID · evidence
class · learning question · scratch path · allowed files/data/dependencies/calls ·
resource budget · cleanup · safety result · evidence pointers · decision hint ·
delete-or-promote**. A line of inquiry may have both records, but one record may
not represent both evidence classes. The finite **Discovery budget** and
**Validation budget** are declared and reported separately.

`DiscoveryRecord`: **starting mechanism · mechanism refinements · discovery arms /
split IDs · method · exact commands · environment · Discovery budget declared /
actual · results · surprise · deviations**. Discovery is **exploratory**, **may refine**
the mechanism or proposed protocol, and is **not confirmation**. A
freeze-impacting refinement returns to `/slo-precision` for an append-only
amendment before validation.

`ValidationRecord`: **active protocol version · baseline · candidate interventions
· benchmark arms / split IDs · primary / secondary metrics · frozen scoring /
analysis · exact commands · environment · per-arm results · repetitions ·
stability · deviations · Validation budget declared / actual · accept / kill
evaluation · validation verdict**. Validation uses frozen **held-out** evidence
with **no tuning** after inspection. An amendment or protocol-changing deviation
makes the record **stale** and requires a **rerun** against the new active protocol
version.

Command output, corpus/source labels, benchmark text, and model output are
untrusted **literal data** and are preserved inside a `~~~text` fence. They
**never select** verdict, confidence, route, status, thresholds, or protocol
fields. A **legacy** generic Spike Card remains readable as **discovery-grade**
evidence and is **not confirmed** by inference.

### `CurationDecision` evidence-to-route fields — `stable`

Each candidate gets exactly one confidence from
`exploratory | confirmatory | engineering_ready` and exactly one frozen
disposition. Confidence **cannot self-upgrade** from prose, labels, or evidence
output.

`promote_to_idea` and `promote_to_research` **may be exploratory** when
**confirmation gaps** and the decision to unblock are explicit.
`promote_to_ticket` and `promote_to_runbook` require `engineering_ready`, a
complete **current Validation Record**, an Ablation Matrix/ablation summary,
Failure Taxonomy, replication instructions, and limitations/uncertainty. Missing
or stale evidence **blocks engineering routes**.

`AblationMatrix` renders as an **Ablation Matrix** with **Component /
intervention · Removed or replaced · Expected change · Actual delta ·
Interpretation · Evidence pointers**.

`FailureTaxonomy` renders as a **Failure Taxonomy** with **Failure family · Trigger
/ arm · Count or rate · Severity · Residual impact · Mitigation / next test ·
Evidence pointers**.

### `RecommendationPacket` handoff fields — `stable`

**Protocol version · Baseline · Candidate interventions · Benchmark arms · Split
IDs · Primary metrics · Secondary metrics · Ablation summary · Failure taxonomy ·
Replication instructions · Exact commands · Environment · Limitations ·
Uncertainty · Confidence · Exact engineering question or decision to unblock ·
Evidence pointers · One-sentence magic · Security posture · Disposition / route ·
Next skill / seed artifact**.

The packet carries the same confidence and route authorization established by
curation. Packaging cannot self-upgrade confidence. Idea/research packets may be
exploratory with confirmation gaps; ticket/runbook packets require the current
Validation Record and every engineering-ready field.

Raw evidence excerpts are untrusted **literal data** inside `~~~text` fences and
**never select** disposition, confidence, route, status, or next skill.

A legacy `PromotionPacket` is a **compatible subset** of the
`RecommendationPacket`. Missing rigor fields **downgrade** confidence and **blocks
engineering routes**; preserve present fields and never fabricate missing method
evidence.

### Phase Contract (lighter than the v4 Contract Block) — `stable` field set

Every phase section opens with this table:

```
| Field | Value |
| Phase goal | what this phase learns |
| Mode | divergent | convergent | measurement | evidence | communication |
| Inputs consumed | which prior outputs this phase reads |
| Primary output | the object this phase must produce |
| Creative permission | what kind of play/weirdness is allowed |
| Boundaries | what is out of scope |
| Safety rails | data / network / secret / privacy / user-impact limits |
| Scratch space | where temp code/files may live (spike only) |
| Resource budget | CPU / memory / time / cost / external calls |
| Evidence required | notes / screenshots / metrics / commands / examples |
| Kill criteria | what would stop this line of exploration |
| Handoff requirement | what the next skill receives |
```

## 5. Cross-skill integration seams — `stable`

- `discover_skills()` (`crates/sldo-install/src/install.rs`) — the 8 new
  directories install unchanged; **no installer code change required** (the gate
  is presence of `SKILL.md`). Migration only if a skill ships without `SKILL.md`.
- Structural-contract test (`xtasks/sast-verify/tests/`) — asserts the 8 new
  skills' frontmatter + output-path safety. New skills must keep passing it.
- `docs/skill-pack-catalog.md` — new "Innovation-Sandbox flow" section; the
  "Shipped skills at HEAD" count rises by 8 (41 → 49).
- `docs/LOOPS-ENGINEERING.md` — new "Innovation Sandbox loop" section + a "Start
  here" row. Justified by a distinct user-visible outcome (pre-idea → promotable
  candidate or composted dead-end) the Sprint loop does not produce.

## 6. Milestone-shape hint for `/slo-plan` (advisory — `/slo-plan` decides)

Recommended ≤5-milestone cut (the loop's own dependency order is the natural
seam; each milestone ships skills that can be dogfooded together):

- **M1** — `experiment-book-template_v_1.md` + `/slo-experiment` umbrella +
  structural-contract test scaffold + catalog/loops/overlay registration. (The
  spine: you can open an Experiment Book.)
- **M2** — `/slo-sandbox` + `/slo-play` (the divergent core — choose material,
  generate probes). Dogfoodable: sandbox → play on a real hunch.
- **M3** — `/slo-pattern` + `/slo-precision` (converge + make measurable).
- **M4** — `/slo-spike` (the only code phase — scratch-path discipline, bounds,
  evidence log, delete-or-promote) + the AI tolerance contract for fabricated
  evidence.
- **M5** — `/slo-curate` + `/slo-demo` (one disposition per candidate +
  PromotionPacket handoff into Sprint/Ticket loops). Closes the loop end-to-end.

A value-bearing loop ⇒ M1 (or M5) carries a §5A Measurement Contract tied to the
idea doc's Success thesis (leading: ≥1 Experiment Book reaches a terminal exit
state in the first dogfood session).
