# Lessons Learned — slo-sp Milestone 10

## What changed
- Vendored `skills/get-api-docs/SKILL.md` verbatim from `andrewyng/context-hub@596506e`.
- `skills/get-api-docs/UPSTREAM.md` records attribution, MIT license, fetch date, and a one-line `curl | diff` recipe for verifying the copy.
- `CLAUDE.md` authored (new) listing all first-party `/slo-*` skills and the third-party `get-api-docs` skill with its `chub` prerequisite.
- 4 E2E tests: SKILL.md validity, UPSTREAM.md attribution, generic-pickup install through the M1 code path, CLAUDE.md listing.

## Design decisions and why
- **Installation path is identical to first-party.** The installer discovers any `skills/<name>/SKILL.md` and installs it; third-party skills get no special-case treatment. This validates M1's generic symlink logic end-to-end. Rationale: specialization at the install layer for "third-party" would be over-engineering for a category we expect to stay small.
- **Vendor verbatim, no local tweaks.** Upstream is the source of truth. If we drift, we lose compatibility with the `chub` CLI's evolving conventions. Rationale: the skill's value depends on staying current with upstream; a fork would bit-rot fast.
- **Commit hash recorded in `UPSTREAM.md`.** Not just "vendored from main" — the specific SHA, so anyone can verify. Rationale: "main" drifts; SHAs don't.
- **`chub` install is the user's job.** The skill body assumes `chub` exists on PATH and tells the agent how to detect its absence. We do NOT auto-run `npm install -g` from the installer. Rationale: global npm installs are user state; auto-running one is too invasive.

## Mistakes made
- Nearly duplicated the upstream SKILL.md body into M10's milestone spec section of the runbook. Caught before committing: the runbook references the vendored file at `skills/get-api-docs/SKILL.md`, doesn't inline it.

## Root causes
- N/A.

## What was harder than expected
- Writing the E2E test for "CLAUDE.md lists get-api-docs" without making it a fragile keyword search. Landed on: the skill name appears (lowercased), and the prerequisite keyword `chub` appears. Anything more specific would break when we update the table format.

## Naming conventions established
- Third-party vendored skills: dir name matches upstream skill name (`get-api-docs`, not `slo-get-api-docs` or `chub-get-api-docs`). First-party skills keep the `slo-` prefix.
- Attribution files: `skills/<name>/UPSTREAM.md`. Not `LICENSE.md`, because that would confuse cargo into thinking the skill dir is its own crate.

## Test patterns that worked well
- Keyword-and-commit-hash test for attribution — catches accidental deletion of the commit hash if someone edits UPSTREAM.md.
- Reusing M1's tempdir install harness to prove generic pickup works on a third-party skill. Zero new install-code paths needed.

## Missing tests that should exist now
- A CI job that runs the `curl | diff` verification to detect silent upstream drift. Deferred until there's CI for this repo.
- A test that installing `get-api-docs` with `chub` absent surfaces the install hint (runtime, needs harness).

## Rules for the next milestone (M9 — self-hosting)
- The skill pack is complete. M9 is validation, not construction. Resist adding features.
- Self-hosting means: run the full pipeline on a real SLO feature. The candidate feature is "auto-populate SHA-256 in `skills/slo-tla/tools.toml` on first maintainer run" — small enough to actually ship in one sitting, real enough to exercise M1-M8 end to end.

## Template improvements suggested
- Add an explicit "External / vendored skills" section pattern to `skills/README.md` describing the vendoring policy (verbatim, record SHA, MIT preserved). This pattern will recur.
