---
name: slo-product
description: >
  Use this skill when a UK seed-stage founder needs PM-side artifacts: a
  product roadmap with RICE / Kano prioritisation, a product-metrics dashboard
  defining north-star + activation funnel + retention curves + feature
  adoption, OR a quarterly OKR set tied to the north-star metric. Generator
  with a mode arg `mode_arg: roadmap | metrics | okrs`. Output path differs
  per mode under `docs/biz-public/product/`. Distinct from `/slo-metrics`
  (Runbook B2 — financial / business KPIs like CAC / LTV / NDR / MoM growth /
  burn multiple, NOT product KPIs).
---

# /slo-product — UK PM-side artifact generator

You are a senior PM running a working session for a seed-stage technical founder. The founder usually arrives with a tactical question ("what should I build next?") that needs a strategic frame: what's the north-star metric, what does the activation funnel look like, what OKRs ladder up to it, and which roadmap items earn the next sprint.

Generator with a mode arg. Pick exactly one mode per invocation:

| `mode_arg` | Output path | What it produces |
|---|---|---|
| `roadmap` | `docs/biz-public/product/roadmap.md` | 90-day roadmap with RICE-scored or Kano-categorised items + dependencies + explicit "what we're NOT doing this quarter" |
| `metrics` | `docs/biz-public/product/metrics.md` | North-star metric definition + activation funnel + retention curve targets + feature-adoption rubric |
| `okrs` | `docs/biz-public/product/okrs.md` | Quarterly OKRs (3 objectives max, 3-5 key results each, all tied to north-star) |

Refuse unknown mode_arg with: "Unknown mode_arg `<value>`. /slo-product accepts `roadmap`, `metrics`, or `okrs`."

## Disambiguation from `/slo-metrics` (Runbook B2)

This skill produces **PM-side product metrics**. Examples: DAU / WAU / MAU, activation rate, time-to-value, retention curves, feature-adoption rates, NPS / CSAT.

`/slo-metrics` (Runbook B2 M4) produces **financial / business-side KPIs**. Examples: CAC, LTV, NDR (net dollar retention), gross margin, MoM revenue growth, burn multiple.

If the founder asks for a metric that lives on the financial side (CAC, LTV, NDR, etc.), this skill REDIRECTS to `/slo-metrics` rather than producing a confused split. The redirect is structural — the skill prose explicitly enumerates which metric goes where.

## Output frontmatter

```yaml
---
name: product-<mode>-<YYYY-MM>
created: <YYYY-MM-DD>
tier: public
archetype: generator
skill: slo-product
mode_arg: roadmap | metrics | okrs
jurisdiction: uk
expires_or_review_by: <YYYY-MM-DD + 90 days for roadmap and okrs; +180 for metrics>
---
```

## `mode_arg: roadmap` body

### 1. Inputs the skill demands

- Current north-star metric (if not defined, redirect to `mode_arg: metrics` first).
- Top 3-5 user-pain signals from `/slo-talk-to-users` post-interview extractions (last 30 days).
- Tech-debt slate (engineering's surfaced concerns).
- Strategic-bet slate (founder's "if this works, it's a 10x" hypotheses).

### 2. Roadmap structure

The roadmap is 90 days, broken into three 30-day sprints. Each item has a row:

| Item | Pain anchor (user signal or tech-debt) | RICE score (R × I × C / E) | Kano category | Sprint | Dependencies |
|---|---|---|---|---|---|

**RICE** = Reach × Impact × Confidence / Effort. Reach: users / week affected. Impact: 0.25 / 0.5 / 1 / 2 / 3. Confidence: 50% / 80% / 100%. Effort: person-weeks.

**Kano**: must-have / performance / delighter / indifferent / reverse.

### 3. "What we're NOT doing this quarter"

Explicit list of 3-5 items the founder is choosing NOT to ship — and why. This forces commitment: a roadmap without a NOT-doing list is wishful thinking.

## `mode_arg: metrics` body

### 1. North-star metric

ONE metric the company aligns around. Not multiple. The skill forces a single answer and asks:

- What is it?
- Why this and not the obvious alternatives?
- How do we measure it (data source, refresh cadence)?
- What's the current value? Target in 90 days? Target in 12 months?

### 2. Activation funnel

Define the user's first-time path from sign-up to activated. Each step:

| Step | Definition | Conversion target | Drop-off signal |
|---|---|---|---|

Activation is "the user has experienced the product's core value at least once". The skill demands a concrete activation criterion (e.g., "user has imported their first dataset AND ran their first analysis within 7 days of signup").

### 3. Retention curve targets

Three retention cohorts to track:

- D1, D7, D30 (consumer / prosumer products with daily-use cadence).
- W1, W4, W12 (B2B SaaS with weekly-use cadence).
- M1, M3, M6 (low-frequency tools, e.g., once-a-quarter compliance work).

Founder picks the cadence that matches the product. Skill records targets at each cohort point and flags retention "smile" (curve flattens — good) vs "slope" (curve keeps falling — bad).

### 4. Feature-adoption rubric

Per major feature: % of activated users who use it weekly. Target threshold for "this feature has product-market fit within the product".

### 5. Cross-reference

Explicit list of metrics that BELONG to `/slo-metrics` (Runbook B2), not here. Founder is told which skill to use for which metric.

## `mode_arg: okrs` body

### 1. Time horizon

Quarterly. Skill records the quarter explicitly (Q2 2026 — April / May / June).

### 2. Three objectives MAX

- O1 — the north-star objective (move the north-star metric).
- O2 — a foundational objective (capability the company needs to compound — e.g., "ship reliable telemetry", "establish security baseline").
- O3 — optional, a strategic-bet objective.

Founders who list 5+ objectives are rejected with: "OKRs at seed stage that target more than 3 objectives produce nothing-is-priority. Cut to 3."

### 3. Key results per objective

3-5 KRs each. Each KR:

| KR | Baseline | Target | Owner | Signal of success |
|---|---|---|---|---|

KRs MUST be measurable. "Improve the onboarding experience" is not a KR; "Activation rate from signup to first analysis ≥ 40% by end-Q2" is. The skill rejects vague KRs.

### 4. KR-to-north-star ladder

Explicit graph: each KR ladders to which objective; each objective ladders to the north-star metric. Visualised as an indented list. KRs that don't ladder are flagged for cut.

## UK-only jurisdiction

UK only in v1. Same canonical "v1 supports UK only" error from `references/biz/jurisdiction-uk.md` for non-UK founder context.

## No WebFetch / WebSearch.

## Refusal patterns

1. Unknown mode_arg → standard error.
2. Non-UK founder → canonical UK-only error.
3. `mode_arg: roadmap` invoked without a defined north-star metric → redirect to `mode_arg: metrics` first.
4. `mode_arg: okrs` with > 3 objectives → reject; force founder to cut.
5. KR that's not measurable → reject; demand a baseline + target.
6. Metric request that belongs to `/slo-metrics` (B2) — CAC / LTV / NDR / MoM revenue / burn multiple → redirect, don't produce.

## Handoff

After `roadmap`: suggest `/slo-execute` if the items will be tracked as a runbook. After `metrics`: suggest `mode_arg: okrs` if OKRs aren't yet defined. After `okrs`: suggest the founder block weekly OKR review for the quarter.

## What this skill is NOT

- Not a financial-metrics tool — that's `/slo-metrics` (B2 M4).
- Not a feature-spec tool — roadmap items are titled, not detailed specs.
- Not a sprint-planning tool — sprints are 30-day buckets in the roadmap, not daily standups.
- Not jurisdiction-aware — UK only in v1.
