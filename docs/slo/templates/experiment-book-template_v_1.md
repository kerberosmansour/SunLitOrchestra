# EXPERIMENT-<slug> — <title> (Creative Experiment Contract v1)

> **Purpose**: explore a fuzzy technical/product hunch safely before turning it into delivery work.
> **Creative posture**: play first, judge later. Surprise is a valid signal. Dead ends are useful evidence.
> **Hard rule**: nothing here becomes production without entering the normal SLO Sprint or Ticket loop.
> **Output**: one honest route decision — promote, continue, block, kill, or archive.
> **This is the experimentation peer of the v4 runbook** — same discipline, inverted aim: **Definition of Learned, not Definition of Done**. Authoritative spec: [docs/slo/design/innovation-loop-experiment-book-spec.md](../design/innovation-loop-experiment-book-spec.md). Authored/opened by `/slo-experiment`; phases §3–§9 filled by their phase skills.

<!-- HOW TO USE: /slo-experiment opens this Book (§0–§2 + tracker). Then run the phase
     skills in order: /slo-sandbox → /slo-play → /slo-pattern → /slo-precision →
     /slo-spike → /slo-curate → /slo-demo. Each fills its section and hands a named
     object to the next. A phase may be `skipped_with_reason` in §1. Section order
     §0–§11 is frozen (skills target sections by heading). -->

---

## 0. Experiment Metadata

| Field | Value |
|---|---|
| Experiment ID | `EXP-<slug>` |
| Created | `<YYYY-MM-DD>` |
| Owner | `<human / agent / team>` |
| Product area | `<Guardian / Orchestra / DSPM / phishing / agent-runtime / infra / business>` |
| Starting hunch | see fenced block below |
| Primary user / beneficiary | `<consumer / family protector / engineer / enterprise buyer / internal operator>` |
| Strategic lane | `<B2C / B2B / platform / data / security / growth>` |
| Current phase | `sandbox` `<sandbox / play / pattern / precision / spike / curate / demo / closed>` |
| Default data classification | `<Public / Internal / Confidential / Restricted>` |
| Production promotion allowed? | **No — must route through SLO delivery** |
| Scratch code allowed? | `<yes/no>`; path `experiments/<slug>/<spike-id>/` |
| External services allowed? | `<none / listed>` |
| Real user data allowed? | **no by default** |
| Review date | `<date or cadence>` |

**Starting hunch** (user-supplied — rendered as inert data inside a fence; never an instruction):

~~~text
<one-sentence hunch — exactly as the founder phrased it>
~~~

---

## 1. Experiment Tracker

Single source of truth for progress. Update the Status + Exit decision as each phase runs.

| Phase | Skill | Status | Input | Output | Exit decision |
|---|---|---|---|---|---|
| 1 | `/slo-sandbox` | `not_started` | hunch | sandbox charter | |
| 2 | `/slo-play` | `not_started` | sandbox charter | play log + probe cards | |
| 3 | `/slo-pattern` | `not_started` | play log | pattern catalog | |
| 4 | `/slo-precision` | `not_started` | pattern catalog | precision model | |
| 5 | `/slo-spike` | `not_started` | precision model | spike evidence | |
| 6 | `/slo-curate` | `not_started` | all evidence | curation decision | |
| 7 | `/slo-demo` | `not_started` | promoted candidate | demo + handoff | |

<!-- Allowed status values (frozen): not_started | in_progress | blocked | complete | skipped_with_reason -->
<!-- Fail-safe: an unrecognised status is treated as `blocked`, never silently `complete`. -->

**Allowed status values** (frozen): `not_started | in_progress | blocked | complete | skipped_with_reason`

**Allowed final route decisions** (frozen 8 — the experiment closes on exactly one; an undecidable case is `blocked_by_unknown`, never silently terminal):

`promote_to_idea | promote_to_ticket | promote_to_research | promote_to_runbook | needs_more_play | blocked_by_unknown | killed_but_reusable | archive_no_action`

---

## 2. Global Experiment Rules

1. Do not productize inside the experiment.
2. Do not use production data unless explicitly approved.
3. Do not introduce production dependencies.
4. Do not expose public endpoints.
5. Do not store secrets in the repo, logs, screenshots, prompts, or demo artifacts.
6. Keep scratch code isolated under the declared experiment path (`experiments/<slug>/<spike-id>/`).
7. Capture surprises, not just successes.
8. Dead ends are valid outputs if they teach something reusable.
9. Every promoted candidate must include a handoff contract (§10).
10. Every experiment closes with one honest route decision.

### Experiment Safety Rails (defaults)

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

### Safety Check (every phase appends one)

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

### §2A Judgment Timing Rule (phase mood — protects the joy)

Critique is **phase-dependent**. Read your phase's mode before you start; it tells you whether to diverge or converge. During `/slo-play`, critique is **banned except for safety**. During `/slo-curate`, critique is **mandatory**.

| Phase | Mode | Agent behaviour | What may be judged |
|---|---|---|---|
| `/slo-sandbox` | framing | choose the playground, not the product | boundaries, not ideas |
| `/slo-play` | divergent | generate probes, defer judgment | **judge safety only**; defer quality judgment |
| `/slo-pattern` | convergent | name reusable tricks | reusability |
| `/slo-precision` | measurement | turn vague claims into handles | measurability |
| `/slo-spike` | evidence | test the smallest claim | evidence |
| `/slo-curate` | convergent | kill, continue, or promote | value, risk, promotion |
| `/slo-demo` | communication | make it communicable | clarity |

> Phase `Mode` is a frozen 5-value set: `divergent | convergent | measurement | evidence | communication` (`framing` is the sandbox-setup posture; `communication` covers the demo narrative).

### Experiment Phase Contract (every §3–§9 phase opens with this)

Lighter than the v4 Contract Block — enough structure to repeat the loop without killing exploration:

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

### Definition of Learned (replaces Definition of Done)

**General phase — Definition of Learned.** A phase is complete only when: the prior phase's output was read; this phase's output was written into the Book; any safety-rail violation is recorded; any dead end is captured with what it taught; the next phase has a concrete handoff; the §1 tracker is updated; no scratch work was silently promoted to production.

**Spike — Definition of Learned.** A spike is complete only when: the learning question was answered or explicitly blocked; evidence is attached/summarized; accept/kill thresholds were evaluated; resource + safety bounds were checked; the scratch path is declared; no production files were changed; the spike has a decision hint.

**Curation — Definition of Learned.** Curation is complete only when: every candidate has exactly one disposition; every promoted candidate has a next SLO route; every killed candidate has a reusable lesson or archive reason; no vague "maybe" remains unowned.

---

## 3. Sandbox Charter

> Filled by `/slo-sandbox`. Mode: **framing**. Choose the material, not the feature.

### Phase Contract

| Field | Value |
|---|---|
| Phase goal | choose a rich material + bound the playground |
| Mode | framing (setup) |
| Inputs consumed | §0 hunch |
| Primary output | `SandboxCharter` + `ProbeSeedList` |
| Creative permission | wide — name strange materials |
| Boundaries | no feature commitment yet |
| Safety rails | inherit §2 defaults |
| Scratch space | none (no code in this phase) |
| Resource budget | time-boxed framing |
| Evidence required | ≥3 concrete probe seeds + explicit safety rails |
| Kill criteria | the material has no untried surface worth playing in |
| Handoff requirement | material, boundaries, weirdness budget, probe seeds → `/slo-play` |

**Material** (user-supplied — fenced, inert):

~~~text
<what are we playing with? e.g. local embeddings, device attestation, DSPM findings>
~~~

**Why this sandbox is rich**: <why this material might contain surprising ideas>

**Not a Feature Yet**: <what we are explicitly NOT deciding>

**Boundaries**:

| Boundary | Rule |
|---|---|
| Product | no user-facing commitment |
| Code | scratch only |
| Data | synthetic or redacted only |
| Network | no uncontrolled external calls |
| Cost | `<budget>` |
| Time | `<budget>` |

**Creative constraints**: <useful limits that sharpen invention>

**Weirdness budget**: <low = obvious extensions | medium = unusual combinations | high = category-shifting — which is allowed?>

**Probe Seed List** (≥3):

| ID | Probe seed | Why try it? | Risk |
|---|---|---|---|
| P1 | | | |
| P2 | | | |
| P3 | | | |

**Kill criteria**: <what would tell us this sandbox is not worth continuing>

---

## 4. Play Log

> Filled by `/slo-play`. Mode: **divergent**. Generate probes; **judge safety only; defer quality judgment**. Do NOT rank, pick a winner, or turn a probe into a product plan yet — that is `/slo-pattern`'s job.

### Phase Contract

| Field | Value |
|---|---|
| Phase goal | map possibilities; surface surprises and dead ends |
| Mode | divergent |
| Inputs consumed | §3 SandboxCharter + ProbeSeedList |
| Primary output | `ProbeLedger` + `DeadEndList` + `StrangeButInterestingList` |
| Creative permission | maximum — weird combinations encouraged |
| Boundaries | no convergence, no ranking, no product plan |
| Safety rails | inherit §2; the ONLY judgment allowed here is safety |
| Scratch space | none (throwaway probes; real code is `/slo-spike`) |
| Resource budget | enough probes to reveal a vocabulary |
| Evidence required | observations + surprises + dead ends |
| Kill criteria | the material produces no surprise after honest play |
| Handoff requirement | probe board, observations, strange-but-interesting, dead ends → `/slo-pattern` |

**Probe Board**:

| ID | Probe | Type | Setup | Observation | Surprise | Reusable? | Safety note |
|---|---|---|---|---|---|---|---|

> Probe types (frozen): `mechanism_probe | interaction_probe | failure_probe | security_probe | data_probe | latency_probe | magic_probe | composition_probe`.

**Raw observations**: <bullets>

**Strange but interesting**: <not useful yet, but might matter>

**Dead ends**:

| Dead end | What failed | What it taught | Reusable fragment |
|---|---|---|---|

**Candidate patterns** (for `/slo-pattern`): <bullets>

---

## 5. Pattern Catalog

> Filled by `/slo-pattern`. Mode: **convergent**. Name reusable tricks. Cite probe IDs for every pattern. ≤5 serious candidates.

### Phase Contract

| Field | Value |
|---|---|
| Phase goal | turn raw play into named reusable mechanisms |
| Mode | convergent |
| Inputs consumed | §4 ProbeLedger |
| Primary output | `PatternCatalog` (+ NextCurve / ProductPull / ArchitecturePull) |
| Creative permission | naming + framing |
| Boundaries | do not promote everything; ≤5 candidates |
| Safety rails | inherit §2 |
| Scratch space | none |
| Resource budget | ≤5 serious candidates |
| Evidence required | cite probe IDs for every pattern |
| Kill criteria | no pattern survives the cite-evidence test |
| Handoff requirement | 1–5 candidates + evidence + claims to measure → `/slo-precision` |

**Pattern candidates**:

| Pattern | Mechanism | Probe evidence | Why surprising | Reuse cases | Risks |
|---|---|---|---|---|---|

**Next-Curve check** (10% improvement vs. category change):

| Pattern | Current curve | Possible next curve | Why |
|---|---|---|---|

**DICEE check** (Deep / Intelligent / Complete / Empowering / Elegant):

| Pattern | Deep | Intelligent | Complete | Empowering | Elegant | Notes |
|---|---|---|---|---|---|---|

**Sunlit strategic fit**:

| Pattern | B2C | B2B | Secure-data | Cybersecurity | Notes |
|---|---|---|---|---|---|

**Product pull**: <which patterns suggest a user-facing wedge>

**Architecture pull**: <which patterns suggest a reusable platform capability>

---

## 6. Precision Model

> Filled by `/slo-precision`. Mode: **measurement**. Make claims falsifiable. No "feels better" without a handle. Every candidate needs an accept AND a kill threshold.

### Phase Contract

| Field | Value |
|---|---|
| Phase goal | convert promising patterns into measurable claims |
| Mode | measurement |
| Inputs consumed | §5 PatternCatalog |
| Primary output | `PrecisionModel` (handles, thresholds, bounds, invariants) |
| Creative permission | choose the instruments |
| Boundaries | no unmeasured claims proceed to spike |
| Safety rails | inherit §2 + name security invariants |
| Scratch space | none (planning only) |
| Resource budget | declare expected resource bounds per claim |
| Evidence required | a metric / observable / falsifiable threshold per claim |
| Kill criteria | a candidate has no falsifiable claim |
| Handoff requirement | learning questions + instrumentation + accept/kill thresholds + invariants → `/slo-spike` |

**Claims that need handles**:

| Claim | Measurement handle | Instrumentation | Accept threshold | Kill threshold |
|---|---|---|---|---|

**Invisible variables**:

| Variable | Unit | Expected range | Hard bound | How measured |
|---|---:|---:|---:|---|

**Reliability / compounding risk**:

| Chain | Per-step risk | Combined risk | Mitigation |
|---|---|---|---|

**False positive / false negative plan** (required for any classification/detection/retrieval/ML claim):

| Error type | How tested | Accept threshold | Must-never case |
|---|---|---|---|

**Resource budget**:

| Resource | Expected bound | Hard limit | Behavior at limit |
|---|---:|---:|---|

**Security invariants** (what must never happen): <bullets — e.g. no raw secret leaves device; no unredacted PII enters a demo artifact>

---

## 7. Spike Cards and Evidence

> Filled by `/slo-spike`. Mode: **evidence**. The ONLY phase that may run code — scratch-only under `experiments/<slug>/<spike-id>/`. A spike is done when the learning question is answered, NOT when the prototype is polished. Every spike ends with a delete-or-promote decision. **No production files. No production promotion.**

### Spike Card — `<spike-id>`

**Phase Contract**:

| Field | Value |
|---|---|
| Phase goal / learning question | `<the one question this spike answers>` |
| Mode | evidence |
| Inputs consumed | §6 PrecisionModel (the falsifiable claim) |
| Primary output | `SpikeCard` + `EvidenceLog` |
| Scratch path | `experiments/<slug>/<spike-id>/` (git-ignored) |
| Production files allowed | none by default |
| Data allowed | synthetic / redacted / generated |
| External calls allowed | `<none / listed>` |
| Dependency policy | no new dependency unless declared (scratch only, with a note) |
| Resource budget | `<CPU / memory / time / network>` + behavior-at-limit |
| Cleanup rule | no untracked junk outside the scratch path |

**Setup**: <what was built or simulated>

**Method**: <how the claim was tested>

**Commands / Evidence**:

| Step | Command / action | Expected | Actual | Notes |
|---|---|---|---|---|

**Results**: <what happened — declared budget vs. actual recorded>

**Surprise**: <what we did not expect>

**Safety Result**:

| Invariant | Result | Evidence |
|---|---|---|

**Decision hint**: `promote_to_idea | promote_to_ticket | promote_to_research | needs_more_play | killed_but_reusable | archive_no_action`

---

## 8. Curation Decision

> Filled by `/slo-curate`. Mode: **convergent**. Kill / continue / promote. Exactly one disposition per candidate, each citing a probe/spike. No vague maybes survive. Dead ends route to §11 compost.

### Phase Contract

| Field | Value |
|---|---|
| Phase goal | decide what to promote, continue, kill, or archive |
| Mode | convergent |
| Inputs consumed | §3–§7 (all evidence) |
| Primary output | `CurationDecision` + `CompostEntries` |
| Creative permission | none — this is the honesty gate |
| Boundaries | no candidate left undisposed |
| Safety rails | inherit §2 |
| Scratch space | none |
| Resource budget | exactly one disposition per candidate |
| Evidence required | every decision cites probes/spikes |
| Kill criteria | (n/a — this phase decides) |
| Handoff requirement | promoted candidates → `/slo-demo` |

**Candidate board**:

| Candidate | Evidence | Surprise | Value | Risk | Decision |
|---|---|---|---|---|---|

**Decision rubric**:

| Dimension | Question | Score / Notes |
|---|---|---|
| Meaning | Does this matter to Sunlit's mission? | |
| User value | Would a user behave differently? | |
| Surprise | Does it create a "wait, that's possible?" moment? | |
| Reliability | Can this become dependable? | |
| Security | Can this be made safe without ruining it? | |
| Strategic fit | B2C / B2B / secure-data / cybersecurity? | |
| Reuse | A reusable platform capability? | |
| Evidence quality | Actually tested, or only speculated? | |
| Elegance | Simple from the user's point of view? | |

**Final disposition** (exactly one of the frozen 8 per candidate):

| Candidate | Decision | Why | Next artifact |
|---|---|---|---|
| | `promote_to_idea` | | `docs/slo/idea/<slug>.md` |
| | `promote_to_ticket` | | `docs/slo/tickets/ticket-<issue>-<slug>.md` |
| | `promote_to_research` | | `docs/slo/research/<slug>/` |
| | `promote_to_runbook` | | `docs/RUNBOOK-<feature>.md` |
| | `killed_but_reusable` | | §11 compost entry |
| | `archive_no_action` | | §11 archive note |

---

## 9. Demo Pack

> Filled by `/slo-demo`. Mode: **communication**. Make the discovery communicable. Promotion is a **suggestion** the human accepts — never an auto-invocation of the next skill.

### Phase Contract

| Field | Value |
|---|---|
| Phase goal | make the discovery handable to the next SLO skill without chat memory |
| Mode | communication |
| Inputs consumed | §8 CurationDecision (promoted candidate) |
| Primary output | `PromotionPacket` (§10) + Demo Pack |
| Creative permission | narrative framing |
| Boundaries | no auto-invoke of a downstream skill |
| Safety rails | inherit §2 + the Security Posture table below |
| Scratch space | none |
| Resource budget | one demo per promoted candidate |
| Evidence required | reproducible demo path + evidence table |
| Kill criteria | (n/a) |
| Handoff requirement | a filled §10 seed table matching the disposition |

**One-sentence magic**: <the memorable moment>

**Before**: <what was hard / invisible / scary / slow / confusing>

**After**: <what becomes obvious / safe / fast / empowering>

**Demo path**: <step-by-step reproduction>

**Evidence**:

| Evidence | Location | What it proves |
|---|---|---|

**Security posture**:

| Concern | Status | Notes |
|---|---|---|
| Data exposure | | |
| Secret handling | | |
| Network calls | | |
| Abuse scenario | | |
| Resource use | | |

**Productization route** (choose exactly one): `/slo-ideate` · `/slo-ticket-plan` · `/slo-research` · `/slo-plan` · archive.

---

## 10. Handoff Contract

> Filled by `/slo-demo` (or `/slo-curate`). Fill the ONE seed table that matches the chosen disposition. Promotion is a suggestion; the human runs the next skill.

### Idea Seed → `/slo-ideate`

| Field | Value |
|---|---|
| Working title | |
| Discovered pattern | |
| User who might care | |
| Pain hypothesis | |
| Smallest complete value slice candidate | |
| One-sentence magic | |
| Worst-day starter risks | |
| Success thesis draft | |
| Open questions | |
| Evidence from experiment | |

### Ticket Seed → `/slo-ticket-plan`

| Field | Value |
|---|---|
| Proposed ticket title | |
| Exact change | |
| Why now | |
| Files likely touched | |
| Out of scope | |
| Acceptance scenario | |
| Test expectation | |
| Security concern | |
| Evidence from experiment | |

### Research Seed → `/slo-research`

| Field | Value |
|---|---|
| Research question | |
| Decision it will unblock | |
| Sources needed | |
| Competing approaches | |
| Claims to verify | |
| Evidence already collected | |

### Runbook Seed → `/slo-plan` (rare — only when architecture clarity already exists)

| Field | Value |
|---|---|
| Proposed runbook title | |
| Target architecture sketch | |
| Milestone candidates | |
| Interfaces likely touched | |
| Data classification | |
| Threat-model starter rows | |
| Measurement-contract starter | |
| Evidence from experiment | |

### Compost Entry → archive / lessons

| Field | Value |
|---|---|
| What we tried | |
| Why it failed | |
| What it taught | |
| Reusable fragment | |
| Future trigger to revisit | |

---

## 11. Compost / Lessons

> Always filled — even a fully-promoted experiment records what it learned; a killed one records the reusable fragment.

- **What should future experiments or runbooks remember?** <bullets>
- **Reusable fragments**: <bullets>
- **Final experiment-level exit state**: `<one of the frozen 8>`
