---
name: paradigm-over-engineering-for-simplicity
created: 2026-04-28
status: living-doc — paradigm reference, no staleness check
audience: every contributor (human or agent) authoring or modifying SLO skills, runbooks, or references
purpose: |
  Capture the design paradigm that LLM-driven workflows can sustainably carry MORE
  discipline than human-driven workflows — because LLMs do not pay the cognitive-load
  tax humans do. Apply this lens when deciding whether a given discipline (security
  step, validation gate, intake question, citation requirement) is "too much".
  Balance against context-window constraints, not against human attention spans.
---

# Over-engineering for simplicity — the paradigm

## The premise

Traditional engineering and product practice has a load-bearing constraint: **humans are the executors**. Every checklist item costs attention. Every gate slows a sprint. Every required citation multiplies code-review effort. The result is a constant trade-off: ship more discipline → people skip steps → discipline degrades into theatre. Ship less discipline → real risks slip through.

The compromise is well-documented in the security literature: tell users to do 20 things → they do 3 → the 17 they skipped are the gaps that get exploited. Same shape in code review (PRs with 200-item checklists go unreviewed), in onboarding (15-step setup procedures get cargo-culted), in reliability (SRE playbooks of 50 steps get partially-followed under stress).

LLMs change this calculation.

An LLM agent does not pay the cognitive-load tax. It does not skip step 14 because it is tired. It does not paraphrase a citation because reading the source is tedious. It does not collapse a 12-question intake to 3 questions because the founder seems impatient (assuming the skill prose says don't). Within its context window, it executes the discipline as written.

This means: **the SLO skill pack can sustainably carry more discipline than a human-driven equivalent**. Where humans optimize for the smallest viable set of practices that will actually get followed, LLMs let us optimize for the largest set that fits the context window without degrading attention.

The result, paradoxically, is **simpler** experience for the primary persona — the user (founder, engineer, employee). The user sees the OUTPUT of the disciplined process: the lawyer-review-recommended draft, the source-cited KPI dashboard, the threat-modeled architecture, the verified math. They do not see the 20 steps that produced it.

## What "over-engineering for simplicity" means concretely

Concrete patterns we apply across the SLO skill pack because the LLM is the executor:

### 1. Multi-layer defense without user friction

A human-driven workflow with 4 redundant defenses against the same risk feels paranoid; the human notices the redundancy and shortcuts to one. An LLM-driven workflow with 4 redundant defenses simply runs all 4 — the user sees one outcome.

Example: PII protection in [the biz pack](RUNBOOK-BIZ-SKILL-PACK-A.md):
- Layer 1: `tier: confidential` artifact frontmatter + auto-routing to `docs/biz/`.
- Layer 2: `.gitignore` rule excluding `docs/biz/` from version control.
- Layer 3: skill write-time warning when target dir is git-tracked AND a remote exists AND `tier: confidential`.
- Layer 4: `/slo-verify` Pass 4 PII-pattern scan over `docs/biz-public/` catches anonymisation failures.

A human pipeline would pick one and move on; the LLM pipeline runs all four. The user just sees "your interview transcript is saved confidentially."

### 2. Comprehensive, not minimum-viable, intake

Where a human-designed form might ask 3 questions and infer the rest, an LLM-designed conversational intake asks 12 questions because the marginal cost of the 9 extra questions is "the conversation lasts 30 seconds longer, and the LLM needs no extra training to ask them well". The data quality at the gate-evaluation step is dramatically better.

Example: [the legal-intake-contract](../references/biz/legal-intake-form.md)'s F1-F6 fields with explicit-comprehension follow-ups (per critique S-1). A human template would skip half; an LLM contract preserves them all because the conversational delivery makes them feel natural.

### 3. Exhaustive citation discipline

Source hierarchy (tool docs at pinned version → tool repo CHANGELOG → upstream advisory DB → conference papers → vendor blog secondary → never Stack Overflow) is unrealistic for human authors at scale — the manual fact-checking cost is prohibitive. For an LLM authoring a skill or runbook, consulting authority files is just-another-tool-call; the discipline holds across hundreds of citations per runbook.

This is why R3 (business skill improvements) can mandate verbatim quotes from `legislation.gov.uk` for every UK statute citation across 5 advisor skills + 7 generator skills, where a human team would settle for "general counsel reviewed this once".

### 4. Layered structural-contract tests

Where a human team might ship one or two integration tests per milestone, the SLO pack ships:

- BDD tests per scenario
- E2E runtime validation tests
- Structural-contract tests (markdown-shape assertions)
- Cross-skill citation tests
- Frontmatter schema validation tests
- Soft line-cap tests
- Closed-enum immutability tests

Each adds a few minutes of LLM-authoring cost; they catch a different failure class. Humans would resist 7 test categories per milestone; the LLM pipeline absorbs them.

### 5. Comprehensive abuse-case enumeration

For every new surface a milestone introduces, [`/slo-architect`](../skills/slo-architect/SKILL.md) Step 3.5 mandates THREE abuse cases (attacker, attack step, desired outcome, control, stable id). For 5 milestones with new surfaces, that's 15 abuse cases per runbook. A human team threat-models once at design time and forgets; the LLM pipeline keeps the rows in the milestone Contract Block, cited from the threat model, traced through `/slo-critique`'s security persona, asserted at `/slo-verify` Pass 4.

### 6. Verbatim discipline preservation across decomposition

R2's M2/M3 decompositions of `/slo-sast` (296 → ≤100 lines) and `/slo-tla` (323 → ≤150 lines) require **prose preservation**: every line of the old SKILL.md must appear in the new (SKILL.md ∪ methodology files), verifiable by structural-contract test. A human refactor would summarize for "readability"; the LLM-driven decomposition preserves discipline even when the lines look redundant.

## Where the paradigm balances

The over-engineering paradigm is bounded by **context window**, not by attention.

If a SKILL.md has 296 lines and a methodology spread across 8 reference files totals 1,200 lines, an LLM agent under context pressure WILL read less of it. The decomposition disciplines (R2 M2-M4) and the soft line-cap structural-contract test (R2 M4) exist because **the LLM is not infinitely-tolerant**; it is highly-tolerant relative to humans, but its limit is the context window.

Practical balance rules:

- **SKILL.md ≤ 200 lines** (soft line-cap, with `# soft-cap-exception:` pragma for documented exceptions). The orchestrator stays lean; methodology lives in references loaded on-demand.
- **Per-stage methodology files ≤ 500 lines** each. Beyond that, sub-decompose.
- **Reference files (authority docs)** can be much longer (5,000+ lines for a verbatim statute anchor file) because they are consulted by file:section, not read end-to-end.
- **Eval cases** stay small (≤ 100 lines each) because they're enumerated, not loaded into context all at once.

The pattern: **front-load the discipline into the orchestrator's instruction surface (SKILL.md, intake contracts, gates); load detail on-demand (references, methodology, evals)**. This is the LLM-equivalent of the "indirect address" pattern in software design — the orchestrator stays small; the body lives in addressable chunks.

## How to apply this paradigm in skill / runbook design

When deciding whether a given discipline is "too much", ask:

| Question | Human-driven answer | LLM-driven answer |
|---|---|---|
| Will the executor remember every step? | No, after step 8 they skip ahead. | Yes, within the context window. |
| Is the marginal-cost-per-step small or large? | Large (cognitive load + time). | Small (next-token prediction). |
| Does the user see the discipline or the output? | They see the discipline (every checkbox). | They see the output (the artifact). |
| Will the discipline degrade into theatre? | Often, after a few sprints. | Rarely, if SKILL.md prose holds. |
| Do redundant defenses add user friction? | Yes (each defense is a checkbox). | No (each defense runs silently). |

If the answers to all five favor LLM-execution, **err on the side of more discipline, not less**. The user experiences the simplicity of the output; the agent does the work.

## Domain examples

### Security

Traditional security advice ("rotate API keys quarterly, audit IAM monthly, scan dependencies daily, review SBOMs per release, threat-model every new feature, enforce least-privilege per identity, log every authn / authz decision") is famously unrealistic for human teams. Most teams follow 2-3 of these and call the rest aspirational.

The SLO security pillar (Phase 1 + R2 + R4) lets the LLM execute all of them. The user sees: a runbook with verified threat-model citations, abuse cases per surface, Pass 4 supply-chain scan, capability-gap filing. The user does not see the 20 distinct disciplines that produced the result.

### Reliability / quality

A human PR template that asks 12 questions ("backward-compatible? migration tested? observability added? alerts configured? runbook updated? rollback tested? canary plan?") gets shortcut. An LLM-driven `/slo-execute` pre-flight that runs all 12 questions plus restate-and-confirm + allow-list check + baseline test + lessons-file consultation does not get shortcut.

The result: more reliability, fewer regressions, less perceived "process overhead".

### Legal / advisor work

A human-written legal intake form with 6 fields gets filled in 2 minutes; founders skip the structured fields and write paragraphs. The LLM-driven conversational intake (R3 M2) asks 6 questions one at a time, pushes on vague answers, and synthesizes the structured `intake_summary:` block with comparable rigor to a paralegal interview. The founder experiences a 5-minute conversation instead of a 30-minute form.

## Anti-patterns

The paradigm does NOT mean:

- **Add infinite gates**. Context-window is finite. Decomposition is required.
- **Skip user agency**. Conversational intake is bidirectional; the founder corrects. Restate-and-confirm is mandatory before consequential writes. Risky actions still require user authorization.
- **Trust LLM execution unconditionally**. Structural-contract tests, abuse-case BDD scenarios, and `/slo-verify` Pass 4 catch what the LLM might silently miss.
- **Pretend the agent understands every nuance**. Source hierarchy ranks Stack Overflow as never-authoritative because LLMs WILL otherwise paraphrase SO confidently. The discipline is structural, not optimistic.
- **Replace explicit defaults with implicit "the agent will figure it out"**. Closed-enum regulator lists, fixed predicate IDs, immutable contracts protect against agent-as-judge drift.

## Cross-references

- [`docs/RUNBOOK-LOOPS-AND-LESSONS-CLOSURE.md`](RUNBOOK-LOOPS-AND-LESSONS-CLOSURE.md) — the lessons loop is itself an example of the paradigm: humans skip post-mortems, LLMs file them as tracked work.
- [`docs/RUNBOOK-ENGINEERING-SKILL-IMPROVEMENTS.md`](RUNBOOK-ENGINEERING-SKILL-IMPROVEMENTS.md) — `references/templates/` shared library + per-skill evals + research-validation discipline embody the paradigm at engineering scale.
- [`docs/RUNBOOK-BUSINESS-SKILL-IMPROVEMENTS.md`](RUNBOOK-BUSINESS-SKILL-IMPROVEMENTS.md) — conversational intake contracts + verbatim statute citations + KPI baseline source-verification embody the paradigm at biz-pack scale.
- [`docs/RUNBOOK-SLO-SEC-LIBS.md`](RUNBOOK-SLO-SEC-LIBS.md) — capability-gap filing turns library-feedback from "lessons file commentary" into structured upstream work, exactly because the LLM doesn't tire of filing.
- [`docs/templates/runbook-template_v_4_template.md`](templates/runbook-template_v_4_template.md) — the v4 template's per-milestone Contract Block (Data classification + Proactive controls + Abuse acceptance scenarios + Resource bounds + Invariants/assertions required + Debugger expectation + Static-analysis gates + extended BDD coverage categories) is the structural anchor of the paradigm. The earlier [v3 template](templates/runbook-template_v_3_template.md) remains in place as the historical artifact for runbooks already authored against it.
- [`SECURITY.md`](../SECURITY.md) — project-wide security defaults that the LLM applies to every milestone.

## When to update this doc

This is a **living doc**. Update when:

- A new SLO discipline pattern emerges that exemplifies the paradigm.
- A retrospective surfaces a case where over-engineering DID degrade user experience (counter-example worth documenting).
- The context-window calculus changes (e.g., new model versions push the soft line-cap or per-skill methodology size cap).

Anti-pattern: do NOT update with new disciplines that haven't yet shipped in a runbook. The paradigm captures patterns we've validated in practice; aspirational patterns belong in idea docs or runbook proposals.
