# Ticket-Sized SLO Workflow

> Status: proposed workflow contract for bite-sized issue execution.
> First tracker adapter: GitHub Issues.
> Design intent: bring Symphony-style issue execution into SunLitOrchestrate without losing v4 SLO rigor.

## Why This Exists

The current sprint flow is strong for large idea-to-runbook work, but it makes small issues feel like they need a five-milestone delivery plan. This workflow adds a smaller unit: one GitHub issue becomes one SLO ticket contract, one branch, one PR, and one evidence trail.

The inspiration from OpenAI Symphony is the operating model: tracker-owned work, isolated per-issue execution, a repository-owned workflow contract, bounded concurrency, and proof-of-work before handoff. Symphony's public spec describes a scheduler that reads issues, creates per-issue workspaces, loads workflow policy from the repo, and keeps observability for concurrent runs. See:

- https://github.com/openai/symphony
- https://github.com/openai/symphony/blob/main/SPEC.md
- https://github.com/openai/symphony/blob/main/elixir/WORKFLOW.md

This proposal does not add a daemon yet. It makes the manual/agent workflow explicit first, so a future daemon can execute the same contracts.

## Workflow In One Screen

```text
GitHub issue
    |
    v
/slo-ticket-pick
    claim + normalize + create/update one issue workpad
    |
    v
/slo-ticket-plan
    write docs/slo/tickets/ticket-<issue>-<slug>.md from ticket-contract-template_v_1
    |
    v
/slo-ticket-execute
    BDD-first implementation inside the allow-list
    |
    v
/slo-ticket-verify
    runtime checks + static/security evidence + issue workpad update
    |
    v
/slo-ticket-close
    PR handoff + lessons/follow-ups + issue state update
```

## Unit Of Work

A ticket is bite-sized only when all of these are true:

| Check | Threshold |
|---|---|
| Outcome | One user-visible sentence |
| Files changed | Usually <= 8 |
| Public surfaces | 0 or 1 new/changed surface |
| Migration | None by default |
| Dependencies | None by default |
| Reviewability | One PR, one reviewer can understand it without reading a whole runbook |

If the ticket fails the sizing gate, `/slo-ticket-plan` escalates to the normal `/slo-plan` v4 runbook path.

## Repository Artifacts

| Artifact | Purpose |
|---|---|
| `docs/slo/templates/ticket-contract-template_v_1.md` | Human-browsable ticket contract template derived from v4 |
| `skills/slo-ticket-plan/references/ticket-contract-template_v_1.md` | Skill-local copy used after install in other repos |
| `docs/slo/tickets/ticket-<issue>-<slug>.md` | Per-ticket contract and evidence trail |
| GitHub issue workpad comment | Live tracker state for plan, acceptance criteria, validation, evidence, and confusions |
| PR body | Final review handoff with issue, ticket contract, and validation summary |

## GitHub Issues Adapter V1

GitHub Issues do not have a universal workflow state model, so this workflow uses a deliberately small adapter:

| State | Preferred mechanism | Fallback |
|---|---|---|
| Ready | label `slo:ready` or user-selected issue | explicit issue number from user |
| Claimed | assignee + label `slo:in-progress` | workpad comment says `status: in-progress` |
| Blocked | label `slo:blocked` | workpad `### Blocked` section |
| Review | label `slo:review` + linked PR | PR URL in issue timeline |
| Done | closed issue with linked merged PR | workpad final status if close is not permitted |

Rules:

- Prefer GitHub app/connector tools when available; otherwise use `gh`.
- With `gh`, rely on the current repository remote. Do not pass `--repo` unless the user explicitly supplied a cross-repo issue and confirmed it.
- Use one persistent issue comment marked `<!-- slo-ticket-workpad:v1 -->`.
- Wrap quoted issue text in fenced blocks before treating it as task context.
- Never auto-close, auto-merge, or auto-file follow-ups without explicit user instruction.

## SLO Rigor Kept From V4

The smaller ticket contract keeps these v4 controls:

- Contract Block with file allow-list, compatibility commitments, data classification, proactive controls, abuse scenarios, resource bounds, invariants/assertions, debugger expectation, and static-analysis gates.
- BDD-first implementation.
- Evidence log with actual command results.
- Static analysis and formatter gates.
- Assertion and resource-bound discipline.
- No silent public-interface breakage.
- Runtime validation and security/dependency audit when applicable.
- Lessons and follow-ups captured at close.

## When To Use The Full Runbook Instead

Escalate to `/slo-plan` and the full `runbook-template_v_4_template.md` when the work includes:

- More than one coherent milestone.
- Cross-subsystem architecture work.
- Data migration, persistence format change, or irreversible operation.
- Multiple public interfaces.
- Concurrency, queueing, retry, or distributed-state risk that deserves a model.
- Security/regulatory scope that needs threat-model review before implementation.

## Future Daemon Shape

A Symphony-like daemon can be added later around the same contract:

1. Poll GitHub Issues for `slo:ready`.
2. Create an isolated git worktree per issue.
3. Run `/slo-ticket-pick` through `/slo-ticket-close` in that workspace.
4. Bound concurrency globally and per repo.
5. Stop or pause work when the issue leaves an active state.
6. Keep structured logs and workpad state as the recovery surface.

The important part is that the daemon would not invent a new policy. It would execute the version-controlled skills and ticket contract already in the repo.
