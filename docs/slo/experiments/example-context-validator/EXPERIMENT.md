# EXPERIMENT-example-context-validator — On-device embeddings for DSPM validation (Creative Experiment Contract v1)

> **Synthetic, non-normative gallery example.** No real data, secrets, or persons.
> Every number below is **synthetic fixture evidence** used to demonstrate the
> contract; it is **not real-world validation** and was **not independently replicated**.
> The Book shows a closed Innovation Sandbox walk from playful
> discovery through a frozen protocol, held-out validation, calibrated confidence,
> and a human-controlled handoff. Read it to calibrate the method, not the product.

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
| Review date | `2026-10-13` |

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
| 4 | `/slo-precision` | `complete` | pattern catalog | precision model + `PF-1` | proceed |
| 5 | `/slo-spike` | `complete` | `PF-1` | `DISC-001` + `VAL-001` | promote |
| 6 | `/slo-curate` | `complete` | all evidence | confidence + ablations/failures + 1 disposition | promote_to_idea |
| 7 | `/slo-demo` | `complete` | promoted candidate | RecommendationPacket + idea seed | promote_to_idea |

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

(Mode: measurement — accept + kill thresholds + Protocol Freeze.)

| Claim | Measurement handle | Instrumentation | Accept threshold | Kill threshold |
|---|---|---|---|---|
| context validator reduces false positives | precision / recall on seeded corpus | local benchmark | FP reduction ≥ 40% with FN increase ≤ 5% | FN increase > 15% |
| validation is fast enough to feel alive | latency per finding | timer | < 150 ms median | > 500 ms median |

**Security invariants**: no raw secret leaves device; no finding text enters a remote prompt; no scan exceeds the resource budget.

### Protocol Source Statements — literal data

~~~text
Can local context embeddings reduce false-positive secret findings without hiding true secrets?
~~~

### Protocol Freeze — `ProtocolFreeze`

| Field | Frozen value |
|---|---|
| Protocol version | `PF-1` |
| Frozen at | `2026-06-07T12:00:00Z` (before validation fixture evidence) |
| Hypothesis | local context validation reduces false positives by at least 40% while increasing the false-negative rate by no more than 5 percentage points |
| Baseline | deterministic secret detection without context validation |
| Candidate interventions | local embedding context validator after deterministic detection |
| Benchmark arms | discovery arm `DISC-SYN-1`; held-out validation arms `VAL-SYN-BENIGN-1` and `VAL-SYN-SECRET-1` |
| Split IDs | immutable synthetic fixture IDs above; 100 benign + 100 true-secret-shaped cases in validation |
| Primary metrics | false-positive reduction; false-negative rate delta |
| Secondary metrics | median latency per finding; invariant violations |
| Analysis plan | compare baseline/candidate per arm; report repetitions, stability, ablations, and failure families |
| Scoring method | exact confusion counts; FP reduction = `(baseline FP - candidate FP) / baseline FP`; missing run fails validation |
| Repetition / stability rule | 3 fixed-seed repetitions; all must meet thresholds; report range, failures, and missing results |
| Accept rule | FP reduction ≥40%; FN-rate delta ≤5 points; median latency <150 ms; zero invariant failures |
| Kill rule | FN-rate delta >15 points, median latency >500 ms, or any raw-data/network invariant violation |
| Resource budget | 200 held-out cases × 3 repetitions; 1 CPU; 512 MB; 2 minutes; no network |
| Risk envelope | synthetic data only; deterministic detector remains authoritative; no automatic suppression or production promotion |

### Protocol Amendments — append-only `ProtocolAmendmentLog`

No amendments. `PF-1` remained active for the synthetic fixture validation.

---

## 7. Spike Cards and Evidence

**Shared envelope**: scratch path `experiments/example-context-validator/spike-001/`;
production files none; synthetic generated data only; external calls none; 1 CPU,
512 MB, 2 minutes, no network; scratch deleted after evidence was distilled.

### Discovery Record — `DiscoveryRecord` `DISC-001`

| Field | Synthetic fixture value |
|---|---|
| Evidence class | `exploratory` — not confirmation |
| Learning question | can surrounding context separate documentation fixtures from true-secret-shaped configuration? |
| Starting mechanism | raw local embedding similarity over a 256-token window |
| Mechanism refinements | compare deterministic finding context to local fixture/doc exemplars; keep the deterministic detector authoritative |
| Discovery arms / split IDs | `DISC-SYN-1` only; never reused as held-out validation |
| Exact commands | `python experiments/example-context-validator/spike-001/bench.py --mode discover --split DISC-SYN-1 --seed 11` |
| Environment | synthetic fixture: macOS arm64; Python 3.12; CPU-only; offline |
| Discovery budget declared / actual | ≤50 cases, 1 repetition, ≤30 s / 50 cases, 18 s |
| Result / surprise | mechanism separated most docs fixtures; explanation appeared more useful than a raw score |
| Deviations | mechanism refinement required the complete `PF-1` freeze before validation; no post-freeze change |
| Evidence pointers | P1/P2/P4 and this synthetic record |

### Validation Record — `ValidationRecord` `VAL-001`

| Field | Synthetic fixture value |
|---|---|
| Evidence class | `validation` — confirmatory only within the declared synthetic fixture scope |
| Active protocol version | `PF-1` |
| Baseline | deterministic detector without context validation |
| Candidate interventions | deterministic detector plus local context validator |
| Benchmark arms / split IDs | held-out `VAL-SYN-BENIGN-1` (100) and `VAL-SYN-SECRET-1` (100); unseen during discovery |
| No tuning | no mechanism, prompt, data, scoring, threshold, or analysis change after held-out evidence was inspected |
| Exact commands | `python experiments/example-context-validator/spike-001/bench.py --mode validate --protocol PF-1 --splits VAL-SYN-BENIGN-1,VAL-SYN-SECRET-1 --seeds 21,22,23 --offline` |
| Environment | synthetic fixture: macOS arm64; Python 3.12; CPU-only; offline; fixed local model placeholder `fixture-embed-v1` |
| Per-arm results | benign: baseline FP 50/100, candidate FP 27/100 (46% reduction); secret: baseline FN 4/100, candidate FN 8/100 (+4 points) |
| Repetitions | 3/3 completed with fixed seeds 21, 22, 23 |
| Stability | FP reduction range 44–48%; FN delta +4 points in all runs; latency medians 109–116 ms; no missing runs |
| Deviations | none; no ProtocolAmendment; record is current, not stale |
| Validation budget declared / actual | 200 cases ×3, 1 CPU, 512 MB, ≤2 min, no network / 600 evaluations, 1 CPU, 410 MB peak, 74 s, no network |
| Accept / kill evaluation | synthetic fixture passes `PF-1`: 46% FP reduction, +4-point FN delta, 112 ms median, zero invariant failures |
| Evidence pointers | this Book only; no external/raw output artifact retained |

**Literal evidence excerpt**:

~~~text
SYNTHETIC FIXTURE ONLY: fp_reduction=0.46 fn_delta_points=4 latency_median_ms=112 invariant_failures=0
~~~

The excerpt is literal data and never selects the verdict, confidence, or route.
**Decision hint**: `promote_to_idea`. **Delete-or-promote**: scratch deleted; only
the bounded synthetic record remains.

---

## 8. Curation Decision

(Mode: convergent — exactly one confidence and disposition per candidate.)

| Candidate | Evidence | Surprise | Value | Risk | Decision |
|---|---|---|---|---|---|
| ML-as-validator-not-extractor | P1/P2/P4 + spike-001 | yes | high | model error | `promote_to_idea` |
| finding-confidence-narrative | P3 | yes | medium | over-trust | `needs_more_play` |

### Ablation Matrix

| Component / intervention | Removed or replaced | Expected change | Actual delta (synthetic fixture) | Interpretation | Evidence pointers |
|---|---|---|---|---|---|
| context validator | removed; baseline detector only | FP benefit disappears | FP reduction falls from 46% to 0% | context validator causes the fixture FP reduction | `VAL-001` |
| local context window | replaced with finding token only | weaker benign/fixture separation | FP reduction falls from 46% to 8% | surrounding context is load-bearing | `VAL-001` |
| explanation narrative | removed from classification path | classification unchanged | metric delta 0; user-trust effect unmeasured | narrative value remains exploratory | P3 |

### Failure Taxonomy

| Failure family | Trigger / arm | Count or rate (synthetic fixture) | Severity | Residual impact | Mitigation / next test | Evidence pointers |
|---|---|---:|---|---|---|---|
| sparse context | true-secret arm | 8/100 candidate false negatives | high | a real secret could be under-weighted | never auto-suppress; test real-shaped but redacted corpora | `VAL-001` |
| documentation-like production config | benign arm | 27/100 residual false positives | medium | alert noise remains | expand held-out fixture families without tuning on validation | `VAL-001` |
| persuasive explanation | interaction probe only | not measured | high | narrative may create over-trust | user research and explicit uncertainty language | P3 |

### Confidence And Route Gate

| Candidate | Confidence | Confirmation gaps | Allowed route |
|---|---|---|---|
| ML-as-validator-not-extractor | `confirmatory` within synthetic fixture scope; **not `engineering_ready`** | real-world generalization gap; no independent replication; user-value and suppression-policy questions unresolved | `promote_to_idea` |
| finding-confidence-narrative | `exploratory` | no held-out user/trust validation | `needs_more_play` |

**Final disposition**: `ML-as-validator-not-extractor` → **`promote_to_idea`** (cites `DISC-001` + `VAL-001`) → next artifact `docs/slo/idea/embedding-context-validator.md`. `finding-confidence-narrative` → `needs_more_play` (one more round on trust UX).

**Compost**: raw-cosine-score-in-UI killed; reusable fragment = confidence-as-narrative.

---

## 9. Demo Pack

**One-sentence magic**: Sunlit does not just shout "secret found" — it explains, locally and privately, why a finding looks real or probably harmless.

**Before**: a wall of noisy, low-trust secret alerts. **After**: each finding carries a calm, local explanation of why it is (or isn't) likely real.

**Demo path**: read `PF-1`; compare DiscoveryRecord and held-out ValidationRecord;
inspect per-arm synthetic values, ablations, and failures; then review why the
confidence remains confirmatory-only and the route is idea rather than engineering.

**Evidence**:

| Evidence | Location | What it proves |
|---|---|---|
| benchmark fixture | §7 `VAL-001` | accept thresholds met inside the declared synthetic fixture scope |

**Security posture**: data exposure — none (synthetic, on-device); secret handling — no raw secret leaves device; network — none; abuse — adversarial fake keys resisted; resource — within budget.

**Productization route**: `/slo-ideate` (product/feature wedge).

### RecommendationPacket

| Field | Value |
|---|---|
| Protocol version | `PF-1` |
| Baseline | deterministic detector without context validation |
| Candidate interventions | deterministic detector plus local context validator |
| Benchmark arms / split IDs | discovery `DISC-SYN-1`; held-out validation `VAL-SYN-BENIGN-1`, `VAL-SYN-SECRET-1` |
| Primary metrics / secondary metrics | FP reduction and FN-rate delta / latency and invariant failures |
| Ablation summary | context removal loses all benefit; token-only context retains 8%; narrative classification delta is zero and user effect unmeasured |
| Failure taxonomy | sparse-context FNs; documentation-like residual FPs; persuasive-explanation over-trust |
| Replication instructions | recreate the synthetic split IDs, preserve `PF-1`, run the exact command below offline for seeds 21–23, retain every per-arm result |
| Exact commands | `python experiments/example-context-validator/spike-001/bench.py --mode validate --protocol PF-1 --splits VAL-SYN-BENIGN-1,VAL-SYN-SECRET-1 --seeds 21,22,23 --offline` |
| Environment | macOS arm64; Python 3.12; CPU-only; offline; `fixture-embed-v1` placeholder |
| Limitations | synthetic authored fixture; no real corpus; no production policy; scratch/evidence artifact not retained |
| Uncertainty | generalization, acceptable FN ceiling, explanation trust effect, and independent reproducibility remain unknown |
| Confidence | `confirmatory` only for the synthetic fixture; not `engineering_ready` |
| Exact engineering question | later, after idea/research: can a local validator reduce FPs on a separately governed redacted corpus without exceeding its frozen FN ceiling? |
| Decision to unblock | determine whether engineers value local explanations enough to justify a real governed validation study |
| Evidence pointers | §§6–8, `DISC-001`, `VAL-001` |
| Disposition / route | `promote_to_idea` → `/slo-ideate` |

This is a **suggestion** for the human. Never auto-invoke `/slo-ideate`, never
promote the synthetic scratch into production, and never represent these fixture
numbers as real-world or independently replicated validation.

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
| Evidence from experiment | §7 `VAL-001` (synthetic fixture): 46% FP reduction, +4-point FN delta, 112 ms median; not real-world validation |

The Idea Seed consumes the RecommendationPacket as context; the human chooses
whether and when to invoke `/slo-ideate`.

---

## 11. Compost / Lessons

- **What future experiments/runbooks should remember**: inverting ML from "extractor" to "validator" turned a noisy detector into a trust surface; the explanation mattered more than the score.
- **Reusable fragments**: confidence-as-narrative; false-positive-compost-set; local-exemplar-memory.
- **Final experiment-level exit state**: `promote_to_idea`.
