---
name: slo-sec-libs-upstream-filing-discipline
description: >
  Filing discipline for /slo-sec-libs M3 and M4. M3 uses the SLO-owned intake
  repository by default; M4 adds explicit direct upstream filing with a
  per-session cap and spillover path.
applies_to: [slo-sec-libs]
milestone: M3-M4
inherits_from: [slo-sast M5 argv-list discipline, slo-retro issue-filing confirmation gate]
---

# /slo-sec-libs Filing Discipline

M3 is the default SLO-intake filer. It creates capability-gap issues in `kerberosmansour/slo-security-intake`, only after regex validation, and only after the user confirms each issue. M4 keeps that default unchanged and adds direct upstream filing only when the user passes `--file-upstream`.

## Destination Discipline

- Default destination: `kerberosmansour/slo-security-intake`.
- Required local checkout: the process working directory for filing MUST be a local checkout whose `git remote get-url origin` resolves to `https://github.com/kerberosmansour/slo-security-intake.git`, `git@github.com:kerberosmansour/slo-security-intake.git`, or the normalized repository URL `https://github.com/kerberosmansour/slo-security-intake`.
- Template dependency: `.github/ISSUE_TEMPLATE/capability-gap-record.md` SHOULD exist in the intake checkout. If it is missing, warn the user and fall back to the plain Markdown body from `capability-gap-schema.md`.
- NO `--repo` flag: `gh issue create`, `gh issue list`, and dedupe/search commands MUST NOT pass `--repo`. Destination comes from the local intake checkout origin that the user can inspect.
- If the current working directory is not the intake checkout and the runner cannot set the subprocess working directory directly, stop and tell the user to rerun with `--intake-dir <path>`.

## Upstream Destination Gate

`--file-upstream` is an explicit opt-in gate. Without it, every valid gap follows the default SLO-intake path even when `expected_library_owner` names a known library.

Allowed upstream mappings:

| `expected_library_owner` | Upstream repository | Accepted checkout origins |
|---|---|---|
| `hulumi` | `kerberosmansour/hulumi` | `https://github.com/kerberosmansour/hulumi.git`, `git@github.com:kerberosmansour/hulumi.git`, `https://github.com/kerberosmansour/hulumi` |
| `SunLitSecurityLibraries` | `kerberosmansour/SunLitSecurityLibraries` | `https://github.com/kerberosmansour/SunLitSecurityLibraries.git`, `git@github.com:kerberosmansour/SunLitSecurityLibraries.git`, `https://github.com/kerberosmansour/SunLitSecurityLibraries` |
| `unknown` | none | upstream filing refused |

Rules:

- The upstream checkout MUST be supplied by `--upstream-dir <path>`.
- The upstream checkout origin MUST match the mapped repository before any issue body is previewed for upstream filing.
- Ambiguous, unknown, legacy, or mismatched owner values MUST NOT file upstream. Offer the SLO-intake fallback with a note instead.
- If the user declines upstream filing, offer the SLO-intake fallback and require a separate confirmation before filing there.
- Do not infer upstream filing from owner names, component names, package names, or repository prose.

## Pre-Flight

Run these checks before reading any M2 unmatched records:

```text
gh --version
gh auth status
git remote get-url origin
test -f .github/ISSUE_TEMPLATE/capability-gap-record.md
```

Never run `gh auth login` from this skill. If `gh auth status` fails, stop with this hint:

```text
GitHub CLI is not authenticated. Run `gh auth login`, then rerun /slo-sec-libs --file-gaps.
```

When `--file-upstream` is present, additionally run `git remote get-url origin` from `--upstream-dir` and compare the normalized URL to the mapping above. Still do not use `--repo`.

## Confirmation Gate

Every candidate filing MUST surface a per-issue confirmation prompt. The prompt shows:

1. Resolved intake origin URL.
2. Data classification.
3. Candidate title.
4. Body preview built from validated fields.
5. Validation result.
6. Dedupe disposition when available (`none`, `match-id`, or `ambiguous`).
7. Cap counter (`filed_this_session` / 40) when `--file-upstream` is active.
8. A yes/no prompt.

Never auto-file. If the user declines the default intake filing, skip that gap and continue to the next unmatched record. If the user declines upstream filing, offer the SLO-intake fallback and ask again. If validation fails, do not prompt for filing; report the invalid field and skip.

## Dedupe Check

Before confirmation, run a best-effort local-destination dedupe query without `--repo`:

```text
gh issue list --label capability-gap --search "<desired_capability>"
```

The command must be invoked as an argv-list subprocess from the intake checkout. A dedupe failure is not a reason to bypass confirmation; surface the failure and ask whether to proceed.

When `--file-upstream` is active, run the same command from the upstream checkout for upstream candidates, then run it from the intake checkout only if the flow falls back to intake.

## Issue Creation Command

Only this issue creation shape is allowed in M3-M4:

```text
gh issue create --title "<title>" --body-file "<tmpfile>" --label capability-gap
```

Invocation rules:

- Use argv-list form such as `["gh", "issue", "create", "--title", title, "--body-file", tmpfile, "--label", "capability-gap"]`.
- Set the subprocess current directory to the intake checkout.
- Use `--body-file` so body content never becomes a shell argument.
- Write the body to a local temporary file, pass the path as one argv item, then delete the temporary file after the command returns.
- Capture stdout/stderr and surface the issue URL to the user when creation succeeds.

For M4, the subprocess current directory is:

- the intake checkout when filing the default SLO-intake issue;
- the mapped upstream checkout when `--file-upstream` is active, mapping succeeds, the cap allows the filing, and the user confirms upstream filing.

## Rate-Limit Cap and Spillover

M4 enforces a defensive cap of **40 issues per session per hour** for direct upstream filing. The counter is named `filed_this_session`.

Counter rules:

- Initialize `filed_this_session = 0` at the start of each `/slo-sec-libs` invocation.
- Increment only after `gh issue create` succeeds for an upstream issue.
- Allow upstream filings while `filed_this_session < 40`.
- The 41st and later upstream candidates MUST NOT call `gh issue create`.
- Spill capped candidates to `LESSONS-BACKLOG.md` in the original invocation repo root with `disposition: spilled-cap`.
- MUST NOT persist cap state to disk, environment variables, git config, temp files, or any global cache.
- The cap resets when the invocation/session ends.

Spillover records append a Markdown table row with every `capability-gap-schema.md` field plus these M4 fields:

| Field | Validation |
|---|---|
| `filed_to` | `kerberosmansour/hulumi`, `kerberosmansour/SunLitSecurityLibraries`, or `kerberosmansour/slo-security-intake` |
| `disposition` | `spilled-cap` |
| `spillover_reason` | `upstream-cap-40-per-session` |

If `gh issue create` returns a GitHub secondary rate-limit response before the local cap fires, do not retry blindly. Surface the response to the user and either wait with a fresh confirmation or spill to `LESSONS-BACKLOG.md` with `disposition: spilled-cap`.

## Forbidden Shortcuts

- Passing `--repo` to `gh issue create`, `gh issue list`, or any M3 dedupe/search command.
- Running `gh auth login`.
- Running `gh pr merge` or using merge flags: `--auto`, `--merge`, `--squash`, `--rebase`, `--admin`.
- Filing without per-issue user confirmation.
- Filing to Hulumi, SunLitSecurityLibraries, or any third-party repository without `--file-upstream`.
- Inferring upstream filing from owner names without explicit user opt-in.
- Persisting the `filed_this_session` cap counter across sessions.
- Filing the 41st upstream issue in a session instead of spilling to `LESSONS-BACKLOG.md`.
- Building a shell string, using `sh -c`, or interpolating record fields into a shell command.
- Copying free-text target prose into a capability-gap issue body.
- Silently repairing a rejected field and filing anyway.

## M5 Extension Point

M5 dogfoods the full read -> match -> file flow against real targets. M5 may record filed issue URLs, but it must not change this filing discipline.
