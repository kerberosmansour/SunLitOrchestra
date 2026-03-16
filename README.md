# SunLitOrchestrate

A toolkit for orchestrating AI-driven software development through structured, milestone-based runbooks.

## Scripts

### `src/plan-milestones.sh` — Generate a Runbook

Takes a requirements prompt (text file) and a repository, then uses GitHub Copilot CLI to analyse the codebase and produce a milestone-based runbook following the [runbook template](docs/runbook-template.md).

```bash
./src/plan-milestones.sh <prompt-file> <repo-dir> [options]
```

**Arguments:**
- `prompt-file` — Path to a text/markdown file describing the desired changes
- `repo-dir` — Path to the target repository

**Options:**
- `-o, --output <path>` — Output runbook path (default: `<repo>/docs/RUNBOOK.md`)
- `-m, --model <model>` — Copilot model (default: `claude-opus-4.6`)
- `-n, --max-iterations <N>` — Max planning refinement passes (default: 3)

**Safety:** Refuses to run if the repo is on `main` or `master` branch.

**Example:**
```bash
./src/plan-milestones.sh requirements.txt /path/to/my-project -o docs/RUNBOOK-FEATURE.md
```

### `src/run-milestones.sh` — Execute a Runbook

Drives GitHub Copilot CLI through the milestones in an existing runbook, one at a time, verifying build and tests after each pass.

```bash
./src/run-milestones.sh
```

## Runbook Template

See [docs/runbook-template.md](docs/runbook-template.md) for the milestone template structure used by the planning script.
