---
name: slo-ship
description: >
  Use this skill when a runbook's milestones are all done and you're ready to
  open a PR — "ship this", "open the PR", "push and PR". Syncs main, runs the
  full non-parked test suite, confirms git state is clean-ish, pushes the
  branch, opens a PR with a runbook-aware description summarizing completed
  milestones (not line counts). Does NOT merge. Does NOT skip hooks.
---

# /slo-ship — open a PR from a completed runbook

You are the release engineer. The runbook's tracker is all `done`. Your job is to get the work onto a reviewable PR — not to merge, not to deploy, just to hand off.

## Pre-flight

1. `git status` — confirm no uncommitted changes that shouldn't be in this PR. If there are, pause and ask.
2. `git rev-parse --abbrev-ref HEAD` — confirm we are NOT on `main` or `master`. If we are, refuse.
3. Read the runbook's tracker. Refuse if any row is not `done`.
4. Run the runbook's declared baseline test command. If red, refuse and surface the failing tests.

## Method

1. `git fetch origin main` (or master, detect).
2. `git status` — ensure tree is clean after any auto-formatters from hooks. If dirty, stage + commit with a message like `chore(fmt): apply hook fixes`.
3. `git push -u origin <current-branch>` — do NOT force-push. If the push fails because the branch was never pushed, `-u` handles it. If it fails because of a non-ff condition, pause and ask the user.
4. Compose the PR title: `<prefix>: <runbook title>` (e.g., `slo-sp: skill-pack rebuild`).
5. Compose the PR body from the runbook's completion summaries:

```markdown
## Summary
<one paragraph, from runbook background + goal>

## Milestones completed
- M1 — <title> — [completion](docs/slo/completion/<prefix>-m1.md)
- M2 — <title> — [completion]...
...

## Test plan
- [ ] Reviewer: read each completion summary, skim the lessons file.
- [ ] Reviewer: run `<baseline test command>` locally.
- [ ] Reviewer: verify tracker is fully `done`.

## Deferred follow-ups
<aggregated from each completion's "Deferred follow-ups" section>

🤖 Generated with /slo-ship
```

6. `gh pr create` with the composed title/body. If `gh` isn't installed, print the manual URL.

### Optional security-summary section (gated)

If the runbook **introduced new public surface**, append a security-summary section to the PR body using [`../../references/security/security-assessment-summary-template.md`](../../references/security/security-assessment-summary-template.md). "New public surface" means any of: a new HTTP endpoint, IPC handler, CLI command, configuration key, persisted-state schema, public type, GitHub Actions workflow, agent file, or release artifact. The section structure mirrors the template: scope, findings index, residual risk, sign-off note. **Optional and gated** — for pure-refactor or doc-only runbooks (no new public surface), omit it.

A high-severity finding from `/slo-critique` or `/slo-verify` Pass 4 referenced in the section MAY also be expanded inline using [`../../references/security/security-finding-template.md`](../../references/security/security-finding-template.md) when reviewer evidence would otherwise be lost.

### Secure-release checklist + `ship_state` (Secure Value Loop)

When the runbook carried a §5B Secure Value & Security Contract, complete the secure-release checklist before opening the PR and record a `ship_state`. Canonical definition: [docs/SECURE-VALUE-LOOP.md §8](../../docs/SECURE-VALUE-LOOP.md). This is the Ship-stage security output of the Secure Value Loop overlay.

Checklist (each item: met / not-applicable-with-reason):

- tests + security scans complete; no critical/high **untriaged** finding (every `/slo-verify` Pass 4/5 finding is `pass | not_applicable | waived_with_reason`);
- every Detected Work Ledger row is disposed (`fix_now | file_github_issue | operator_action | upstream_feedback | accepted_risk`) — no row left "observed";
- **SBOM / provenance — when applicable**: required only for a milestone that builds a **released artifact** (crates.io publish, release zip, container image). For a markdown / skill-contract / docs runbook this is `not_applicable` — never a hard gate;
- deployment uses least-privilege credentials (or `not_applicable` for non-deploy);
- canary / staged rollout defined (or `not_applicable`);
- monitoring / alerts exist for new failure modes (or `not_applicable`);
- rollback path tested or documented (or `not_applicable`);
- residual risks have named owners + dates (reuse the threat-model Residual-risks convention);
- operator actions (e.g. `gh label create`, account/cred provisioning) surfaced to the user — not silently skipped.

Record a closed `ship_state` in the PR body / handoff:

~~~text
ship_state: shipped | human_review_required | blocked | canary_only | docs_only
reason: "..."
rollback: "..."
monitoring_links: ["..."]
~~~

`shipped` requires the checklist complete and no critical/high untriaged finding. A runbook that closed with a `human_review_required` / `blocked_by_operator` / `blocked_by_upstream` milestone ships as the matching non-`shipped` state, never silently `shipped`.

## Gates — refuse when

- On `main` or `master`.
- Tracker has non-done rows.
- Baseline tests red.
- Uncommitted changes not explained (hook auto-fixes are fine; random stuff is not).
- `.git/MERGE_HEAD` or rebase-in-progress — tell the user to finish what they started.

## Anti-patterns

- Skipping hooks with `--no-verify`. Hooks exist; fix what they break.
- Using `--force-push`. Never automatic; refer to `/slo-ship --force` if someone wants it, and refuse unless they explicitly pass it.
- PR description that's a diff-stat ("+2,400, -100 lines"). Use milestone titles + completion links.
- Bundling the runbook commits with unrelated work. If there's drift, surface it.

## Handoff

Print the PR URL. Stop there — do NOT merge, do NOT `--auto`, do NOT deploy. Those are separate decisions.

---

**Loops**: Sprint loop — see [docs/LOOPS-ENGINEERING.md#sprint-loop](../../docs/LOOPS-ENGINEERING.md#sprint-loop).
