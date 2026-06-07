# EXPERIMENT-example-context-validator — On-device embeddings for DSPM validation (Creative Experiment Contract v1)

> **Synthetic, non-normative gallery example.** No real data, secrets, or persons.
> Shows what a *closed* Experiment Book looks like after a full Innovation Sandbox
> loop walk (sandbox → play → pattern → precision → spike → curate → demo) that
> reached a terminal exit state. Read it to calibrate before running the loop.

---

## 0. Experiment Metadata

| Field | Value |
|---|---|
| Experiment ID | `EXP-example-context-validator` |
| Created | `2026-06-07` |
| Owner | `Sherif (synthetic)` |
| Product area | `DSPM` |
| Starting hunch | see fenced block below |
| Primary user / beneficiary | `engineer triaging DSPM findings` |
| Strategic lane | `security` |
| Current phase | `closed` |
| Default data classification | `Internal` |
| Production promotion allowed? | **No — routed through SLO delivery (promote_to_idea)** |
| Scratch code allowed? | `yes`; path `experiments/example-context-validator/spike-001/` |
| External services allowed? | `none` |
| Real user data allowed? | `no` (synthetic seeded corpus only) |
| Review date | `2026-07-07` |

**Starting hunch** (inert, fenced):

~~~text
Can on-device embeddings make DSPM findings feel less noisy and more trustworthy — by validating context after deterministic detection, rather than using ML to extract secrets?
~~~

---

## 1. Experiment Tracker

| Phase | Skill | Status | Input | Output | Exit decision |
|---|---|---|---|---|---|
| 1 | `/slo-sandbox` | `complete` | hunch | sandbox charter | proceed |
| 2 | `/slo-play` | `complete` | sandbox charter | play log + probe cards | proceed |
| 3 | `/slo-pattern` | `complete` | play log | 4 patterns | proceed |
| 4 | `/slo-precision` | `complete` | pattern catalog | precision model | proceed |
| 5 | `/slo-spike` | `complete` | precision model | spike-001 evidence | promote |
| 6 | `/slo-curate` | `complete` | all evidence | 1 disposition | promote_to_idea |
| 7 | `/slo-demo` | `complete` | promoted candidate | demo + idea seed | promote_to_idea |

**Allowed status values** (frozen): `not_started | in_progress | blocked | complete | skipped_with_reason`

**Allowed final route decisions** (frozen 8): `promote_to_idea | promote_to_ticket | promote_to_research | promote_to_runbook | needs_more_play | blocked_by_unknown | killed_but_reusable | archive_no_action`

---

## 2. Global Experiment Rules

(Inherited verbatim from the template — synthetic example does not restate them.)
No production data; no real secrets; scratch isolated under the experiment path;
surprises captured; dead ends valid; one honest route decision at close.

### Safety Check (example)
- Data classification: Internal
- Raw secrets present? no
- PII present? no
- External service called? no
- Scratch path: `experiments/example-context-validator/spike-001/`
- Cleanup required: yes (done)
- Abuse sketch: a crafted fixture tries to look like a real key — handled by deterministic detection first, ML only validates context

---

## 3. Sandbox Charter

**Material**: local embeddings + deterministic DSPM findings + false-positive context + on-device privacy.

**Why this sandbox is rich**: secret detection is noisy; the surprising move is to use ML to *validate context* (is this finding real?) rather than to *extract* secrets.

**Not a Feature Yet**: we are NOT deciding a UI, a product tier, or an architecture — only exploring whether the material yields a trustworthy validator.

**Boundaries**: product = no commitment; code = scratch only; data = synthetic seeded corpus; network = none; cost/time = one afternoon.

**Creative constraints**: must work local-first; ML validates context, does not extract secrets; must reduce reviewer fear/confusion.

**Weirdness budget**: medium (unusual combinations allowed).

**Probe Seed List**:

| ID | Probe seed | Why try it? | Risk |
|---|---|---|---|
| P1 | compare true secret vs fixture vs docs example | does context separate them? | low |
| P2 | cluster surrounding 256 tokens | does locality cluster false positives? | low |
| P3 | generate an explanation before the alert | does explanation change trust? | low |
| P4 | adversarial fake AWS-looking IDs | does it resist spoofing? | low |

**Kill criteria**: if synthetic fixtures and real-shaped secrets are indistinguishable by local embeddings, stop.

---

## 4. Play Log

(Phase Contract Mode: divergent — judge safety only; defer quality judgment.)

**Probe Board**:

| ID | Probe | Type | Setup | Observation | Surprise | Reusable? | Safety note |
|---|---|---|---|---|---|---|---|
| P1 | true vs fixture vs docs | data_probe | seeded corpus | docs examples cluster separately from real config | yes | yes | synthetic only |
| P2 | cluster context | mechanism_probe | 256-token window | locality clusters likely false positives | mild | yes | synthetic only |
| P3 | explanation-before-alert | interaction_probe | text gen | explanation changes trust perception | yes | yes | none |
| P4 | adversarial fake keys | security_probe | crafted fixtures | fake AWS-looking IDs remain hard to pass as real | yes | yes | synthetic only |

**Raw observations**: raw cosine score is not human-comprehensible; explanation text matters.

**Strange but interesting**: a "finding confidence narrative" may matter more than a binary suppress/allow.

**Dead ends**:

| Dead end | What failed | What it taught | Reusable fragment |
|---|---|---|---|
| raw cosine score in UI | users can't read it | scores need narrative wrapping | confidence-as-narrative |

**Candidate patterns**: ML-as-validator-not-extractor; finding-confidence-narrative; local-exemplar-memory; false-positive-compost-set.

---

## 5. Pattern Catalog

(Mode: convergent — cite probe IDs; ≤5 candidates.)

| Pattern | Mechanism | Probe evidence | Why surprising | Reuse cases | Risks |
|---|---|---|---|---|---|
| ML-as-validator-not-extractor | ML validates context after deterministic detection | P1, P2, P4 | inverts the usual "ML finds secrets" framing | any noisy detector | model error |
| finding-confidence-narrative | explanation before alert | P3 | narrative > score for trust | alerts, triage UX | over-trust |

**Next-Curve check**: ML-as-validator-not-extractor is a category change (from "shows findings" to "explains trust"), not a 10% improvement.

**DICEE check**: ML-as-validator-not-extractor — Deep ✓ Intelligent ✓ Complete ~ Empowering ✓ Elegant ✓.

**Product pull**: finding-confidence-narrative suggests a user-facing wedge.
**Architecture pull**: ML-as-validator-not-extractor suggests a reusable validator lane.

---

## 6. Precision Model

(Mode: measurement — accept + kill thresholds.)

| Claim | Measurement handle | Instrumentation | Accept threshold | Kill threshold |
|---|---|---|---|---|
| context validator reduces false positives | precision / recall on seeded corpus | local benchmark | FP reduction ≥ 40% with FN increase ≤ 5% | FN increase > 15% |
| validation is fast enough to feel alive | latency per finding | timer | < 150 ms median | > 500 ms median |

**Security invariants**: no raw secret leaves device; no finding text enters a remote prompt; no scan exceeds the resource budget.

---

## 7. Spike Cards and Evidence

### Spike Card — `spike-001`

**Learning question**: can local embeddings separate fixtures/docs from likely real secrets on a synthetic seeded corpus?
**Scratch path**: `experiments/example-context-validator/spike-001/` · **Production files**: none · **Data**: synthetic seeded corpus · **External calls**: none · **Resource budget**: 1 CPU, 512 MB, 2 min, no network; stop at limit.

**Commands / Evidence**:

| Step | Command / action | Expected | Actual | Notes |
|---|---|---|---|---|
| run benchmark | `python spike-001/bench.py --corpus synthetic` | FP reduction reported | FP reduction 46%, FN +4% | within accept threshold |
| latency | timer over 500 findings | < 150 ms median | 112 ms median | pass |

**Results**: partial-but-promising — accept thresholds met on synthetic data; declared budget not exceeded.
**Surprise**: the explanation text may be more valuable than the binary suppression.
**Safety Result**: no raw secret left the device; no remote call. (synthetic only.)
**Decision hint**: `promote_to_idea` — product/user value needs interrogation before architecture. **Delete-or-promote**: scratch deleted (evidence distilled into this card).

---

## 8. Curation Decision

(Mode: convergent — exactly one disposition per candidate, each citing evidence.)

| Candidate | Evidence | Surprise | Value | Risk | Decision |
|---|---|---|---|---|---|
| ML-as-validator-not-extractor | P1/P2/P4 + spike-001 | yes | high | model error | `promote_to_idea` |
| finding-confidence-narrative | P3 | yes | medium | over-trust | `needs_more_play` |

**Final disposition**: `ML-as-validator-not-extractor` → **`promote_to_idea`** (cites spike-001) → next artifact `docs/slo/idea/embedding-context-validator.md`. `finding-confidence-narrative` → `needs_more_play` (one more round on trust UX).

**Compost**: raw-cosine-score-in-UI killed; reusable fragment = confidence-as-narrative.

---

## 9. Demo Pack

**One-sentence magic**: Sunlit does not just shout "secret found" — it explains, locally and privately, why a finding looks real or probably harmless.

**Before**: a wall of noisy, low-trust secret alerts. **After**: each finding carries a calm, local explanation of why it is (or isn't) likely real.

**Demo path**: run the synthetic benchmark; show FP reduction 46% / FN +4% / 112 ms median; show an example explanation.

**Evidence**:

| Evidence | Location | What it proves |
|---|---|---|
| benchmark output | §7 spike-001 | accept thresholds met on synthetic data |

**Security posture**: data exposure — none (synthetic, on-device); secret handling — no raw secret leaves device; network — none; abuse — adversarial fake keys resisted; resource — within budget.

**Productization route**: `/slo-ideate` (product/feature wedge).

---

## 10. Handoff Contract

### Idea Seed → `/slo-ideate`

| Field | Value |
|---|---|
| Working title | embedding-context-validator |
| Discovered pattern | ML-as-validator-not-extractor |
| User who might care | an engineer drowning in DSPM false positives |
| Pain hypothesis | secret-scan noise erodes trust until alerts are ignored |
| Smallest complete value slice candidate | local context validator over deterministic findings, with an explanation |
| One-sentence magic | Sunlit explains why a finding looks real or harmless, locally |
| Worst-day starter risks | model error suppresses a real secret; over-trust of the narrative |
| Success thesis draft | FP reduction ≥ 40% with FN increase ≤ 5%; reviewers act on explanations |
| Open questions | does the narrative hold on real corpora? what FN ceiling is acceptable? |
| Evidence from experiment | §7 spike-001 (synthetic): 46% FP reduction, +4% FN, 112 ms median |

---

## 11. Compost / Lessons

- **What future experiments/runbooks should remember**: inverting ML from "extractor" to "validator" turned a noisy detector into a trust surface; the explanation mattered more than the score.
- **Reusable fragments**: confidence-as-narrative; false-positive-compost-set; local-exemplar-memory.
- **Final experiment-level exit state**: `promote_to_idea`.
