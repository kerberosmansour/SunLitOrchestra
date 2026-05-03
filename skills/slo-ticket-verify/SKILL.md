---
name: slo-ticket-verify
description: >
  Use this skill after /slo-ticket-execute. It performs ticket-sized runtime QA
  and security/static evidence checks against docs/slo/tickets/ticket-*.md,
  writes regression tests before fixes for any bug found, and updates the issue
  workpad. It does not implement unrelated fixes.
---

# /slo-ticket-verify - prove the ticket works

You are the QA lead for one ticket contract. The implementation claims to be done; your job is to prove the behavior and evidence hold.

## Inputs

- `docs/slo/tickets/ticket-<issue>-<slug>.md`.
- Changed files and tests on the current branch.
- GitHub issue workpad, if available.

## Output

- Completed Validation Plan rows in the ticket contract.
- Optional `docs/slo/verify/ticket-<issue>-<slug>.md` report when runtime QA is non-trivial.
- Regression tests for any bug found, written before the fix.
- Updated issue workpad validation/evidence.

## Method

1. Read the ticket contract, especially BDD Acceptance Scenarios, Validation Plan, resource bounds, invariants/assertions, and compatibility commitments.
2. Inspect the diff against the base branch and confirm changed files are inside the allow-list.
3. Run the validation commands recorded in the contract:
   - formatter
   - typecheck/build
   - static analysis/lint
   - unit/BDD tests
   - runtime validation
   - dependency/security audit when applicable
4. Exercise each BDD scenario at runtime when possible:
   - happy path
   - invalid input
   - empty/degraded state
   - abuse case when applicable
5. If UI is touched, use Playwright or the repo's existing browser harness and capture evidence.
6. Check resource bounds and invariants:
   - confirm tests/assertions exist
   - confirm behavior at limit is visible, not silent
7. Check compatibility:
   - public API/CLI/event/config/persisted-state commitments still hold
8. Update Actual Result and Status cells.
9. Update the issue workpad `Validation` and `Evidence` sections.

## Bug-found flow

When you find a real bug:

1. Stop.
2. Write a regression test that fails today for the bug.
3. Record the failure in the ticket contract and workpad.
4. Hand the fix back to `/slo-ticket-execute`.
5. After the fix, re-run the failed test and the full ticket validation.

Do not fix bugs inline unless the user explicitly asks you to combine verify and execute for this ticket.

## Gates

- Do not mark verified if any acceptance scenario lacks evidence.
- Do not mark verified if static analysis/lint was skipped without a reason.
- Do not mark verified if a high/critical security finding is unresolved.
- Do not mark verified if out-of-scope files changed.
- Do not mark verified if screenshots/traces are missing for failed UI paths.

## Anti-patterns

- Re-running unit tests and calling that runtime verification.
- Fixing bugs without first adding a regression test.
- Treating tool network errors as security findings; record them as skipped/tool-error unless the scanner produced a real finding.
- Skipping abuse scenarios because the happy path passed.

## Handoff

When every validation row is pass/N/A with reason and the workpad is current, run `/slo-ticket-close <ticket-contract-path>`.

---

**Loops**: Ticket loop - see [docs/LOOPS-ENGINEERING.md#ticket-loop](../../docs/LOOPS-ENGINEERING.md#ticket-loop).
