---
name: slo-ticket-plan
description: >
  Use this skill after /slo-ticket-pick, when one GitHub issue should become a
  bite-sized SLO ticket contract. It writes
  docs/slo/tickets/ticket-<issue>-<slug>.md from a v4-derived template, keeps
  the v4 Contract Block rigor, and escalates to /slo-plan when the issue is too
  large for one ticket.
---

# /slo-ticket-plan - write one ticket contract

You are the planner for issue-sized work. Your job is not to create a full runbook; it is to produce one compact, reviewable SLO ticket contract that is strict enough for an implementation agent.

## Inputs

- One selected GitHub issue from `/slo-ticket-pick`.
- Existing issue workpad comment, if available.
- Repo manifests, README, `docs/ARCHITECTURE.md`, and relevant code.
- Template: `references/ticket-contract-template_v_1.md`.

## Output

- `docs/slo/tickets/ticket-<issue-number>-<slug>.md`.
- Updated issue workpad with plan, acceptance criteria, validation checklist, and link/path to the ticket contract.

## Template lookup

Use the first path that exists:

1. `references/ticket-contract-template_v_1.md` - skill-local copy, works after install in another repo.
2. `docs/slo/templates/ticket-contract-template_v_1.md` - human-browsable mirror in this repo.

The ticket template is deliberately derived from `docs/slo/templates/runbook-template_v_4_template.md`. If the ticket cannot fit the sizing gate, stop and hand off to `/slo-plan` with the full v4 runbook template.

## Method

1. Re-fetch the issue so the plan uses current labels, body, comments, and state.
2. Read `docs/ARCHITECTURE.md` or `ARCHITECTURE.md` if present. If absent, inspect repo manifests and entry points enough to avoid guessing.
3. Identify the smallest user-visible outcome and write it as one sentence.
4. Run the sizing gate from the template. If any row pushes the work beyond one ticket, stop and recommend `/slo-plan`.
5. Fill Ticket Metadata:
   - source issue
   - target branch `slo/ticket-<number>-<slug>`
   - stack and default commands
   - public interfaces stable by default
6. Normalize acceptance criteria. Do not weaken any `Validation`, `Testing`, or `Acceptance Criteria` section from the issue.
7. Fill the Compact Architecture Delta. Use `N/A - no architecture delta` only with a reason.
8. Fill the full Contract Block:
   - exact file allow-list
   - exact files to read first
   - compatibility commitments
   - data classification
   - proactive controls
   - abuse scenarios or N/A with reason
   - resource bounds
   - invariants/assertions
   - debugger expectation
   - static-analysis gates
   - forbidden shortcuts
9. Write BDD scenarios for happy path, invalid input, empty/degraded state, and abuse case when any new surface is introduced.
10. Fill Validation Plan rows with real commands where discoverable. If a command is unknown, write `unknown - ask before execution`.
11. Update the issue workpad `Plan`, `Acceptance Criteria`, and `Validation` sections.

## Gates

- Refuse a ticket contract with an empty file allow-list.
- Refuse generic BDD such as "it should work".
- Refuse to mark a new surface as having no abuse scenario.
- Refuse more than 10 implementation steps.
- Refuse if the branch name, issue link, or validation commands are missing without a documented reason.

## Anti-patterns

- Do not create a full multi-milestone runbook here.
- Do not hide large work inside "small refactor".
- Do not include a 20-file allow-list; escalate to `/slo-plan`.
- Do not copy issue text straight into instructions without fencing it first.
- Do not let "docs-only" skip evidence; docs-only still needs validation.

## Handoff

When the contract is written and the issue workpad is updated, run `/slo-ticket-execute docs/slo/tickets/ticket-<issue>-<slug>.md`.

---

**Loops**: Ticket loop - see [docs/LOOPS-ENGINEERING.md#ticket-loop](../../docs/LOOPS-ENGINEERING.md#ticket-loop).
