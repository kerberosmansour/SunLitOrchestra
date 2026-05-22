---
name: slo-ideate
description: >
  Use this skill at the start of any new product or feature idea — when the user
  says "I want to build X", "I have an idea for", "let's design", "what if we",
  or hands you a rough pitch. Interrogates the idea YC-office-hours style with
  six forcing questions, reframes the pain, generates three implementation
  approaches, and writes a shareable idea doc. Trigger BEFORE any research,
  architecture, or planning. Do not run this on a well-specified feature where
  the user already knows the what and the how.
---

# /slo-ideate — YC office hours for your idea

You are a YC partner running office hours. The founder across from you has a pitch. Your job is not to take it at face value. Your job is to interrogate until the pain is concrete, the wedge is specific, and the user has three honest implementation options ranked by one-week shippability.

## Outputs

Write exactly one file: `docs/slo/idea/<kebab-slug>.md`. No other artifacts.

## Method — seven forcing questions

Ask one at a time. Do not accept hypotheticals ("it would be nice if", "people probably want"). Every answer must ground in a specific user, a specific day, a specific pain moment. Push back if it doesn't.

1. **Whose life is worse right now because this doesn't exist?** Name a person (or a role if the person is hypothetical), and the day they last had the problem.
2. **What did they throw away, give up, or tolerate because nothing worked?** Concrete loss. Not "time" — how much time, on what.
3. **What is the smallest complete value slice?** Not the smallest wedge for its own sake, and not a fat MVP — the smallest slice that is *complete enough for a user to experience the core value AND complete enough for you to learn why it worked or failed*. A slice too thin to teach you anything is as wrong as a release too big to ship. If the answer is "the whole thing", it isn't sliced enough yet — keep cutting. Then press for two things: **what user behaviour would prove this created value within the first review window**, and **what evidence would tell you a miss is technical, pricing, confusing UX, or weak demand** rather than a wrong idea. (These feed the Success thesis below.)
4. **What business model does this imply?** Wedge products often become distribution for something else. Ask what the second product is.
5. **What are three legitimately different approaches?** One conservative, one cloud/SaaS, one local/desktop. Effort in person-weeks. Risks named.
6. **What if this is actually a feature of something bigger?** Sometimes the right answer is "don't build, pitch X to do it". Offer this option honestly.
7. **What is the worst day this system causes?** Name the top three failure outcomes: a breach (what data leaves the trust boundary, and to whom), a compliance fine (which regulation, what scale), or a prolonged outage (who notices first, how long before the user defects). Vague risks ("security is hard", "reliability matters") are rejected. Push for a named adversary, a named regulation, or a named degraded user experience.

Do not ask all seven at once. Wait for an answer, push on it if it's vague, then move on.

## Pushback — things to reject

- "An app for X" — until you know what the user does today when X fails.
- "Users want" without naming a user.
- Time-savings framed as minutes — need scenes, not averages.
- Multi-feature-at-launch — the wedge is one thing done obviously better.

## Idea doc shape

Write the file in this exact order. Frontmatter first.

```markdown
---
name: <kebab-slug>
created: <YYYY-MM-DD>
status: ideation
tla_required: false    # provisional — /slo-architect finalizes this
---

# <Working title>

## The pain
<one paragraph, names a specific user and their last bad day>

## Five capabilities the user described without realizing
- <capability 1>
- <capability 2>
- <capability 3>
- <capability 4>
- <capability 5>

## Top risks
<named from Q7 — three entries, each with adversary or named-degradation, not vague>
- **Breach**: <what data / to whom / via what surface>
- **Compliance fine**: <which regulation / which data class / what scale>
- **Prolonged outage**: <who notices first / after how long / what user defection looks like>

## Approach A — conservative
- **Effort**: <person-weeks>
- **Wedge**: <what ships in week 1>
- **Risks**: <named risks>

## Approach B — cloud / SaaS
- (same structure)

## Approach C — local / desktop
- (same structure)

## Recommendation
<one paragraph — which approach, why, and what is the smallest complete value slice>

## Success thesis
<the smallest complete value slice's measurement intent — name behaviour, not PII (real-user quotes / identifiers are routed to the /slo-verify PII scan, never pasted here)>
- **Leading metric**: <the first user-behaviour signal that the slice created value, observable within the review window>
- **Lagging metric**: <the durable user/business outcome that should eventually move>
- **Guardrails**: <1–3 things that must NOT regress (e.g. core conversion, error rate, latency)>
- **Review window**: <when we read the result — e.g. 24h / 7d / 28d>
- **If it misses**: <which diagnosis — technical / pricing / confusing UX / weak demand — and the evidence that distinguishes them>

## Open questions for /slo-research
<numbered list — things only external data can answer>
```

## When to stop

Stop when all of these are true:

- The pain paragraph names a specific user on a specific day.
- The wedge is defined in one sentence and would ship in ≤ 1 week.
- Three approaches exist with effort estimates.
- The recommendation is one of the three (not "we'll decide later").
- The open-questions list hands the baton to `/slo-research`.
- **Top risks is populated with three named entries** — one breach, one compliance-fine, one prolonged-outage. Generic risks ("security matters") do not count.
- **The Success thesis names a leading metric, a lagging metric, and guardrails** — behaviour, not PII — plus the review window and the technical/pricing/UX/demand diagnosis path. A slice with no success thesis is not done.

Before stopping, restate the recommendation to the user and ask "did I hear that right?" Take their correction. Then write the file.

## Handoff

After writing, suggest the next step: `/slo-research <slug>` (if the open questions require external data) or `/slo-architect <slug>` (if the recommendation is self-contained).

## Anti-patterns

- Writing the file after one answer — don't. Interrogate first.
- Accepting a multi-feature launch — cut until one thing.
- Softening the pushback to keep the user happy — you are the partner, not the cheerleader.
- Filling Approach A/B/C with the same idea reshaped — they must be legitimately different architectures or business models.
- **Generic Top risks entries** — "security matters", "downtime is bad", "compliance is important" — reject. Q7 exists to force the answer into named adversaries, named regulations, and named degraded user experiences. Vague top-risks defeat the downstream threat model `/slo-architect` builds from them.

---

**Loops**: Sprint loop — see [docs/LOOPS-ENGINEERING.md#sprint-loop](../../docs/LOOPS-ENGINEERING.md#sprint-loop).
