---
name: slo-critique
description: >
  Use this skill after /slo-plan produces a runbook, BEFORE /slo-execute runs
  any milestone. Orchestrates four adversarial review passes (CEO, eng-lead,
  security, design) against the plan. Auto-fixes mechanical issues, surfaces
  scope concerns for user approval, rejects vague findings. Every finding must
  include a concrete exploit or failure scenario — theoretical risks are not
  accepted. Skip the design pass automatically if the runbook has no UI surface.
---

# /slo-critique — four adversarial reviews on a runbook

You are not a single reviewer. You are four specialists, each rotating into the chair in turn: CEO, engineering lead, security officer, designer. Each gets the whole runbook. Each produces findings in one of three categories — auto-fix, ask-to-fix, hold-scope — and records them in a shared summary file.

## Inputs

- `docs/slo/current/RUNBOOK-<feature>.md` — the target runbook.
- `ARCHITECTURE.md`, `docs/slo/design/*.md` — context.
- Prior `docs/slo/lessons/*.md` — the last lessons file(s) tell you what kinds of findings have been relevant on this codebase before.

## Outputs

- Inline edits to the runbook (for auto-fix category).
- `docs/slo/critique/<runbook-slug>.md` — findings summary, one row per finding, cross-referenced to runbook line numbers.
- Expanded security finding appendices MAY use [`../../references/security/security-finding-template.md`](../../references/security/security-finding-template.md) when a compact row would hide evidence, standards mapping, or remediation detail.

## Rotation order

Run the personas in this order, one at a time. Do not interleave:

1. **CEO** (`personas/ceo.md`) — scope challenge. Is there a 10-star product hiding inside? Should we expand, hold, or reduce?
2. **Eng lead** (`personas/eng.md`) — architecture pokes. Hidden assumptions, missing failure modes, test gaps, orthogonal edits.
3. **Security** (`personas/security.md`) — class elimination + variant analysis + threat-model citation. Every finding names a bug class from [`references/bug-class-catalog.md`](references/bug-class-catalog.md), cites a row from `docs/slo/design/<slug>-threat-model.md`, answers whether the class is eliminated / mitigated / residual, and includes a variant-analysis pointer from [`references/variant-analysis-playbook.md`](references/variant-analysis-playbook.md). Concrete exploit scenarios only.
4. **Design** (`personas/design.md`) — only if the runbook has a UI surface. AI-slop detection, interaction gaps, empty-state handling. Skip with a note if N/A.

## Finding format

Every finding is one row in `docs/slo/critique/<slug>.md`:

| id | persona | category | runbook section | finding | concrete scenario | recommendation |
|----|---------|----------|-----------------|---------|-------------------|----------------|

- **category** is one of `auto-fix`, `ask`, `hold-scope`, `reduce-scope`, `defer`.
- **concrete scenario** is mandatory. If the reviewer can't write one sentence describing an actor + action + bad outcome, the finding is not accepted — ask for specificity or drop it.
- **recommendation** is actionable: "add BDD scenario for X", "rename interface Y in M3", not "consider improving Z."
- **security persona rows** must include the most specific applicable standard or bug-class mapping in the `finding` or `recommendation` cell: local bug-class id, CWE, OWASP LLM / Top 10, ASVS, or OpenCRE. If the row needs more than one sentence of evidence, add an appendix entry using `references/security/security-finding-template.md` and link to it from the row.

## What `auto-fix` means

Only mechanical, uncontroversial fixes:

- Missing Compatibility Checklist rows that the contract requires.
- Test naming that doesn't follow the runbook's declared convention.
- `.gitignore` patterns missing for declared artifacts.
- Cross-reference typos (e.g., link to `docs/slo/current/RUNBOOK-OLD.md` in a section about a new file).

Non-mechanical fixes (architecture changes, new test scenarios, scope changes) are always `ask` — the user must approve before they land.

## Gates — refuse a finding when

- It is vague ("this might have a race"). Ask for the concrete scenario or drop it.
- It is stylistic preference dressed as risk ("I'd prefer a different pattern here").
- It is a theoretical OWASP category with no concrete surface in this plan.
- It proposes a scope change without a one-sentence business justification.

## Scope change discipline

When the CEO persona proposes expansion or reduction:

- Never auto-apply. Always `ask`.
- Name the opportunity cost in one sentence ("adding X means M3 slips a week").
- Defer the decision to the user. Do not argue.

## Anti-patterns

- Running all four personas in one pass and blending findings. The value comes from rotation — each persona gets the whole plan with their lens uncontaminated.
- Filling every category with at least one finding so the critique "looks thorough." Some categories will come up empty on some runbooks; that's honest.
- Translating OWASP categories into a generic checklist. "A01 — Broken Access Control" is not a finding; "the M3 IPC handler exposes user IDs without verifying the caller's session" is.
- Writing findings the user will immediately waive. Before writing, ask: "would the user actually change the plan for this?" If no, cut it.

## Handoff

After the summary file is written and the user has accepted or declined each `ask` finding, suggest `/slo-execute M1` to start the first milestone.

---

**Loops**: Sprint loop, Security-tuning loop — see [docs/LOOPS-ENGINEERING.md#sprint-loop](../../docs/LOOPS-ENGINEERING.md#sprint-loop).
