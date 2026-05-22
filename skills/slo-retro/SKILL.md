---
name: slo-retro
# soft-cap-exception: carries milestone closeout, lessons, and issue-filing discipline
description: >
  Use this skill at the END of every milestone, after /slo-execute and
  /slo-verify have finished. Writes the milestone's lessons-learned file,
  completion summary, and updates the runbook's Milestone Tracker. Requires a
  filled-in Evidence Log. Refuses to run on milestones whose Evidence Log has
  blank "Actual Result" cells. Also usable standalone to close out a milestone
  that was executed by hand.
---

# /slo-retro — close out a milestone

You are the engineering manager running the post-mortem. The milestone just finished. You have one job: turn the evidence log plus the observed behavior into two files and a tracker update, following the v3 runbook templates literally.

## Inputs

- A runbook at `docs/slo/current/RUNBOOK-<feature>.md` with a current milestone in `in_progress` state.
- That milestone's Evidence Log (inside the runbook).
- Optional: a verification report at `docs/slo/verify/<prefix>-m<N>.md` if `/slo-verify` ran.
- Optional: the previous milestone's lessons file, for comparison.

## Outputs

Write exactly three things, then run the additive issue-filing flow (see "Issue filing" below):

1. `docs/slo/lessons/<prefix>-m<N>.md` — lessons-learned file (use the matching runbook template — v4 by default for new runbooks; v3 when the runbook was authored against v3). **Always written first**, even when `gh` is unavailable.
2. `docs/slo/completion/<prefix>-m<N>.md` — completion summary (same template version as the runbook).
3. Inline edits to the runbook: Milestone Tracker row updated to `done`, with Completed date and paths.

After those three are on disk, run the issue-filing flow as described under "Issue filing". If issue filing fails for any reason, the three artifacts above are still safely written — issue filing is strictly additive.

## Pre-conditions

Refuse to run and list the blockers if any of these are true:

- The Evidence Log has blank "Actual Result" cells.
- A **Kani-obligation Evidence-Log row is blank** (the milestone had `kani_required` proof obligations but the `cargo kani` verdict / bound was never recorded) — same refusal as any blank Evidence row.
- The Self-Review Gate questions in the runbook template are not all "yes".
- Any BDD scenario in the milestone is still marked pending.
- `git status` shows untracked test artifacts.

When a milestone had Kani proof obligations, record the proved properties, assumptions, bounds, stubs/contracts, and what remains unproved in the lessons file (sourced from `docs/slo/verify/<slug>-kani.md`).

Do not fix these yourself. Tell the user which rows are blank, which questions are not answered, which scenarios are pending. They decide whether to go finish execution or override.

## Method

1. Read the runbook's milestone section top to bottom. Extract: goal, files changed, BDD scenarios, Evidence Log, Definition of Done.
2. Read the previous lessons file (if one exists). Note which rules it established — were they followed? Note in the new lessons.
3. Diff the actual files changed vs. "Files Allowed To Change" in the contract. Flag any out-of-scope edits in the lessons file.
4. Write the lessons file per the template below.
5. Write the completion summary per the template below.
6. Edit the runbook: change the milestone tracker row, add the Completed date, record the paths to both new files.
7. Run the runbook's post-flight commands (tests, build) one more time and record in the evidence log if they haven't been already.

## Lessons file template

```markdown
# Lessons Learned — <prefix> Milestone <N>

## What changed
- <summary>

## Design decisions and why
- <decision> — <reason>

## Mistakes made
- <mistake>

## Root causes
- <root cause>

## What was harder than expected
- <note>

## Naming conventions established
- <types, files, tests, events, commands>

## Test patterns that worked well
- <pattern>

## Missing tests that should exist now
- <test>

## Rules for the next milestone
- <rule>

## Template improvements suggested
- <improvement>
```

## Completion summary template

```markdown
# Completion Summary — <prefix> Milestone <N>

## Goal completed
- <what capability now exists>

## Files changed
- <file>

## Tests added
- <test file>

## Runtime validations added
- <e2e file>

## Compatibility checks performed
- <check>

## Documentation updated
- <doc and section>

## .gitignore changes
- <patterns added or removed>

## Test artifact cleanup verified
- <confirmation that git status is clean>

## Deferred follow-ups
- <follow-up>

## Known non-blocking limitations
- <limitation>
```

## Issue filing

After the lessons file, completion summary, and tracker update are on disk, classify each lesson and file it as a tracked issue with explicit user confirmation. **Issue filing is strictly additive** — if anything fails here, the three on-disk artifacts above are still safe.

Read [`references/issue-filing-discipline.md`](references/issue-filing-discipline.md) for the locked rules. The procedure below is the in-prose summary; the reference file is the authoritative source.

### Step 1 — classify each lesson

For every flagged lesson in `docs/slo/lessons/<prefix>-m<N>.md`, decide one of:

- **`product`** — lesson applies to the current target product / repo. Filed against the current repo (resolved via `git config remote.origin.url`).
- **`upstream-OSS`** — lesson applies to a third-party tool (Semgrep, Playwright, `cargo audit`, etc.). Filed against the resolved upstream repo via `.sldo/upstream-mapping.toml` (with crates.io / npm fallback).
- **`slo-process`** — lesson applies to SunLitOrchestra itself (skill prose, runbook template, the lessons loop mechanism). Filed against `kerberosmansour/SunLitOrchestra`.

If a lesson does not fit any of the three, ask the user. Do not invent a fourth classification.

### Step 2 — three-strike dedupe

For each candidate filing, run **three** `gh search issues --label retro-derived` queries:

1. Literal title search.
2. NFKC-normalized title search.
3. ASCII-collapsed (lowercase, whitespace-collapsed, non-ASCII stripped) search.

If any strike returns a hit, surface it and skip filing unless the user explicitly says "file new anyway". Reject candidates with U+202E / U+202D (RTL / LTR override codepoints) outright — escalate to the user.

Also check `LESSONS-BACKLOG.md` for matching `body_sha256` rows (cross-session dedupe).

### Step 3 — confirm with user

Before any `gh issue create`, surface a confirmation prompt with: classification, resolved destination URL, dedupe disposition (`none` / `match-id` / `ambiguous`), candidate title, candidate body preview. **Never auto-file.** Issue creation is publicly visible; user gate is non-negotiable.

### Step 4 — file with argv-list discipline

```
gh issue create --title "<title>" --body "<body>" --label retro-derived
```

**Rules**:
- argv-list form only — no shell-string interpolation of lesson body. Inherits from `/slo-sast` M5.
- **NO `--repo` flag** — confused-deputy defense. Rely on `gh`'s default origin-based resolution.
- Wrap the lesson body in `~~~text` fence (per `/slo-architect` user-string-fence rule) so downstream skills (M4 carry-forward, `/slo-resume`) treat it as literal text.
- Truncate body at 65,536 chars with a `... [truncated; full body in lessons file at <path>]` footer.
- Cap: 40 issues per session per hour. Spill remaining lessons to `LESSONS-BACKLOG.md` with `disposition: spilled-cap`.
- Adaptive backoff on `gh` rate-limit responses — read `Retry-After` (default 60 s); never retry blind.

### Step 5 — record in lessons file frontmatter

Append a `filed_issues:` frontmatter block to `docs/slo/lessons/<prefix>-m<N>.md` listing each filing's URL, classification, disposition (`filed` / `skipped-dupe` / `skipped-user` / `spilled-cap`), and `body_sha256` (first 12 hex chars).

### Fallback — `gh` unavailable

If `which gh` returns nothing, OR `gh auth status` returns unauth, append the lesson to a top-level `LESSONS-BACKLOG.md` file using the 12-field audit row schema in `references/issue-filing-discipline.md`. The lessons file is still written first (the graceful-degradation rule). Notify the user with a `gh auth login` install hint.

### Forbidden in this flow

- Auto-filing without confirmation.
- `--repo` flag.
- Shell-string interpolation of lesson body.
- Skipping dedupe.
- Replacing or skipping the lessons-file write — issue filing is ALWAYS additive, after the file is on disk.

## Anti-patterns

- Writing platitudes — "it went well", "nothing to note". The template fields exist because honest post-mortems find things. If a field is truly N/A, write the one-line reason it's N/A.
- Closing a milestone with out-of-scope edits un-flagged — the lessons file is where scope breaches get documented so the next milestone's allow-list can learn.
- Skipping the "rules for the next milestone" section. This is the single most valuable part of the lessons file and the next milestone will read it first.
- Auto-filing issues. Issue creation is publicly visible — confirmation is non-negotiable.
- Filing without `--label retro-derived`. The label is the canonical marker `/slo-execute` M4 carry-forward queries against.

## Handoff

After writing, suggest the next step: `/slo-execute M<N+1>` or, if the runbook's last milestone is now done, `/slo-ship` to open the PR.

---

**Loops**: Sprint loop, Lessons loop, Library-feedback loop — see [docs/LOOPS-ENGINEERING.md#lessons-loop](../../docs/LOOPS-ENGINEERING.md#lessons-loop).
