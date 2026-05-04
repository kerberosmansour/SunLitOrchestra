---
name: tool-safety-section
status: stable-template
created: 2026-05-04
audience: skills that invoke subprocesses, remote tools, scanners, package managers, or GitHub CLI
purpose: Shared tool pre-flight and subprocess discipline.
---

# Tool Safety Section Template

Tool-backed skills must prove what they are about to run. This template cites `references/templates/citation-discipline.md` because tool behavior claims need current docs or pinned repo references.

## Required Pre-Flight

1. Locate the executable with `command -v <tool>` or a repo-pinned absolute path. Prefer `command -v`; avoid `which`.
2. Run `<tool> --version` when supported and record stdout, stderr, and exit code separately.
3. Run `<tool> --help` before dispatch when flags are uncertain, recently changed, or part of the skill contract.
4. Prefer dry-run or read-only modes before mutating modes.
5. Name whether the command is read-only, writes local files, writes remote state, or can execute target code.

## Subprocess Discipline

Use argv-list form. Never splice user strings into a shell command. Capture stdout, stderr, and exit code separately so tool errors are not mistaken for findings.

## Mutating Tool Rule

Before a mutating command, restate:

- target path or remote;
- exact command shape;
- rollback or no-rollback status;
- evidence row to fill afterward.

## Failure Handling

If the tool is missing, unavailable, or returns a tool error, record a skipped/tool-error row. Do not convert transient network or advisory DB errors into security findings.
