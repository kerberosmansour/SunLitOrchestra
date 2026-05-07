---
name: slo-ticket-execute
description: >
  Use this skill to implement one docs/slo/tickets/ticket-*.md contract. It is
  the ticket-sized analogue of /slo-execute: BDD tests first, exact file
  allow-list enforcement, smallest safe change, v4 evidence log discipline,
  and no scope widening without user approval.
---

# /slo-ticket-execute - implement one ticket contract

You are the implementer for one issue-sized SLO contract. Your job is to satisfy the ticket contract exactly, not to improve adjacent systems.

## Inputs

- `docs/slo/tickets/ticket-<issue>-<slug>.md`.
- The linked GitHub issue and workpad comment, if available.
- Files listed in the contract's read-list and allow-list.

## Output

- Code, tests, docs, and evidence required by the ticket contract.
- Updated ticket contract Validation Plan actual results.
- Updated issue workpad progress.

## Pre-flight

1. Read the entire ticket contract.
2. Re-fetch the GitHub issue/workpad if available and note any changes since planning.
3. Run the baseline command from the contract. If red before work starts, stop and record the baseline blocker.
4. Read every file in "Files to read before changing".
5. Restate constraints in chat:
   - goal
   - allowed files
   - forbidden changes
   - public interface compatibility
   - resource bounds
   - invariants/assertions
   - reversibility / rollback path
   - exemplar / anti-exemplar constraints
   - refactoring discipline
   - AI tolerance contract, when present
   - static-analysis gates
   - validation commands
6. Run the Repo hygiene gate before file edits. Record git state, confirm the current branch is not the default/protected branch, create/switch to the target task branch when needed, and update the issue workpad with the branch name once selected.

## Pre-flight: Repo hygiene gate

This gate runs before file edits. It is allowed to switch branches, but it must not edit project files until branch state is safe.

### Commands to record

Run and record:

```
git status --short --branch
git rev-parse --abbrev-ref HEAD
git symbolic-ref --short refs/remotes/origin/HEAD
```

If `origin/HEAD` is unavailable, detect the default branch from local context and fall back to checking both `main` and `master`. Treat the current branch as unsafe when it is the default/protected branch or when local policy marks it protected.

### Branch rule

Use the ticket contract's target branch when it is present. If the contract does not name one, derive:

```
ticket/<issue>-<slug>
```

Do not include the agent name, host name, or model name in the branch. Branch names are task-scoped, not agent-scoped.

If execution is on the default/protected branch, stop before file edits and create or switch to the task branch unless the user explicitly instructed execution to remain there. If uncommitted work already exists on the default branch, preserve it by switching to a new branch immediately, then record the remediation. Do not stash, discard, or reset user work unless the user explicitly asks.

### Evidence and workpad row

Add or fill a Repo hygiene row in the ticket Validation Plan and issue workpad with:

- branch before
- branch after
- dirty-tree state
- remediation needed
- remediation taken

The issue workpad must include the selected branch name. Execution may prepare the working tree; commits and pushes happen only when the active workflow or the user explicitly asks for them.

## Allow-list rule

If the fix requires editing a file not listed in `Files allowed to change`:

1. Stop coding.
2. Name the file and change required.
3. Explain why the current contract excludes it.
4. Ask the user whether to extend the contract, split the ticket, or escalate to `/slo-plan`.
5. Do not proceed until the contract is updated.

## Method

1. Write or update BDD/unit tests first for each acceptance scenario.
2. Run the new tests and confirm they fail for the expected reason, not a compile/setup error.
3. Add runtime validation stubs if the contract requires them.
4. Implement the smallest safe change inside the allow-list.
5. Encode required invariants/assertions and resource bounds.
6. Follow the exemplar / anti-exemplar, reversibility, refactoring discipline, and AI tolerance rows when they are not N/A.
7. Make the tests pass.
8. Run formatter, typecheck/build, static analysis/lint, unit/BDD tests, and any dependency/security audit required by the contract.
9. Run runtime validation if the ticket touches behavior that can be exercised outside unit tests.
10. Fill Actual Result and Status cells in the Validation Plan and update the issue workpad `Evidence` section with command names and outcomes.

## Gates

- Do not proceed from red tests unless the failure is the expected pre-implementation failure.
- Do not leave placeholders, fake implementations, or temporary proof edits.
- Do not claim a command passed unless it was run. If skipped, record why.
- Do not widen the branch into unrelated cleanup.
- Do not mark complete while any Validation Plan row is `pending`.

## Anti-patterns

- Treating the issue body as stronger than the SLO contract.
- Writing production code before tests for a behavior change.
- Fixing unrelated warnings in untouched files.
- Adding a dependency because it is convenient.
- Recording "looks good" as evidence.

## Handoff

When every contract row is satisfied and evidence is filled, run `/slo-ticket-verify <ticket-contract-path>`.

---

**Loops**: Ticket loop - see [docs/LOOPS-ENGINEERING.md#ticket-loop](../../docs/LOOPS-ENGINEERING.md#ticket-loop).
