---
name: slo-sec-libs-upstream-filing-discipline
description: >
  Filing discipline for /slo-sec-libs M3 and M4. M3 uses the SLO-owned intake
  repository only; M4 may extend this file for direct upstream filing.
applies_to: [slo-sec-libs]
milestone: M3
inherits_from: [slo-sast M5 argv-list discipline, slo-retro issue-filing confirmation gate]
---

# /slo-sec-libs Filing Discipline

M3 is the default SLO-intake filer. It creates capability-gap issues only in `kerberosmansour/slo-security-intake`, only after regex validation, and only after the user confirms each issue. Direct filing to Hulumi, SunLitSecurityLibraries, or any third-party repository is M4 work and is not enabled in M3.

## Destination Discipline

- Default destination: `kerberosmansour/slo-security-intake`.
- Required local checkout: the process working directory for filing MUST be a local checkout whose `git remote get-url origin` resolves to `https://github.com/kerberosmansour/slo-security-intake.git`, `git@github.com:kerberosmansour/slo-security-intake.git`, or the normalized repository URL `https://github.com/kerberosmansour/slo-security-intake`.
- Template dependency: `.github/ISSUE_TEMPLATE/capability-gap-record.md` SHOULD exist in the intake checkout. If it is missing, warn the user and fall back to the plain Markdown body from `capability-gap-schema.md`.
- NO `--repo` flag: `gh issue create`, `gh issue list`, and dedupe/search commands MUST NOT pass `--repo`. Destination comes from the local intake checkout origin that the user can inspect.
- If the current working directory is not the intake checkout and the runner cannot set the subprocess working directory directly, stop and tell the user to rerun with `--intake-dir <path>`.

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

## Confirmation Gate

Every candidate filing MUST surface a per-issue confirmation prompt. The prompt shows:

1. Resolved intake origin URL.
2. Data classification.
3. Candidate title.
4. Body preview built from validated fields.
5. Validation result.
6. Dedupe disposition when available (`none`, `match-id`, or `ambiguous`).
7. A yes/no prompt.

Never auto-file. If the user declines, skip that gap and continue to the next unmatched record. If validation fails, do not prompt for filing; report the invalid field and skip.

## Dedupe Check

Before confirmation, run a best-effort local-destination dedupe query without `--repo`:

```text
gh issue list --label capability-gap --search "<desired_capability>"
```

The command must be invoked as an argv-list subprocess from the intake checkout. A dedupe failure is not a reason to bypass confirmation; surface the failure and ask whether to proceed.

## Issue Creation Command

Only this issue creation shape is allowed in M3:

```text
gh issue create --title "<title>" --body-file "<tmpfile>" --label capability-gap
```

Invocation rules:

- Use argv-list form such as `["gh", "issue", "create", "--title", title, "--body-file", tmpfile, "--label", "capability-gap"]`.
- Set the subprocess current directory to the intake checkout.
- Use `--body-file` so body content never becomes a shell argument.
- Write the body to a local temporary file, pass the path as one argv item, then delete the temporary file after the command returns.
- Capture stdout/stderr and surface the issue URL to the user when creation succeeds.

## Forbidden Shortcuts

- Passing `--repo` to `gh issue create`, `gh issue list`, or any M3 dedupe/search command.
- Running `gh auth login`.
- Running `gh pr merge` or using merge flags: `--auto`, `--merge`, `--squash`, `--rebase`, `--admin`.
- Filing without per-issue user confirmation.
- Filing to Hulumi, SunLitSecurityLibraries, or any third-party repository in M3.
- Building a shell string, using `sh -c`, or interpolating record fields into a shell command.
- Copying free-text target prose into a capability-gap issue body.
- Silently repairing a rejected field and filing anyway.

## M4 Extension Point

M4 may add `--file-upstream`, third-party destination mapping, and the per-session 40-issues/hr cap. Until M4 lands, M3 remains one-by-one confirmed filing to the SLO-owned intake repo only.
