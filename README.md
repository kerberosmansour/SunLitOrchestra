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
./src/run-milestones.sh <runbook> <repo-dir> [options]
```

**Arguments:**
- `runbook` — Path to the runbook markdown file (relative to repo or absolute)
- `repo-dir` — Path to the target repository

**Options:**
- `-m, --model <model>` — Copilot model (default: `claude-opus-4.6`)
- `-a, --max-attempts <N>` — Max Copilot invocations (default: 150)
- `-c, --cooldown <secs>` — Pause between retries (default: 5)
- `--build-cmd <cmd>` — Custom build verification command (repeatable)
- `--test-cmd <cmd>` — Custom test verification command (repeatable)

**Auto-detection:** If no `--build-cmd` / `--test-cmd` are given, the script auto-detects commands from the project's build files (Cargo.toml, package.json, go.mod, Makefile, etc.).

**Safety:** Refuses to run if the repo is on `main` or `master` branch.

**Examples:**
```bash
# Auto-detect build/test commands from the repo
./src/run-milestones.sh docs/RUNBOOK.md /path/to/my-project

# Specify explicit build and test commands
./src/run-milestones.sh docs/RUNBOOK.md /path/to/my-project \
  --build-cmd "cargo build --workspace" \
  --test-cmd "cargo test --workspace"

# Multiple build and test commands
./src/run-milestones.sh docs/RUNBOOK-UI.md /path/to/my-project \
  --build-cmd "cargo build --workspace" \
  --build-cmd "cd frontend && npm run build" \
  --test-cmd "cargo test --workspace" \
  --test-cmd "cd frontend && npx vitest run"
```

## Runbook Template

See [docs/runbook-template.md](docs/runbook-template.md) for the milestone template structure used by the planning script.

## Rust Rewrite (in progress)

The project is being rewritten from Bash into Rust. The Cargo workspace lives alongside the original scripts:

```
crates/
├── sldo-common/   # Shared library (CLI parsing, colour output, git checks, runbook parsing)
├── sldo-plan/     # Binary: runbook generation (replaces plan-milestones.sh)
└── sldo-run/      # Binary: milestone execution (replaces run-milestones.sh)
```

Build and test the Rust workspace:

```bash
cargo build --workspace
cargo test --workspace
```

### `sldo-plan` — Rust CLI for Runbook Generation

Rust equivalent of `plan-milestones.sh`. Generates a milestone-based runbook using GitHub Copilot CLI.

```bash
sldo-plan <prompt-file> <repo-dir> [options]
```

**Options:**
- `-o, --output <path>` — Output runbook path (default: `<repo>/docs/RUNBOOK.md`)
- `-m, --model <model>` — Copilot model (default: `claude-opus-4.6`)
- `-n, --max-iterations <N>` — Max planning refinement passes (default: 3)

**Example:**
```bash
cargo run -p sldo-plan -- requirements.txt /path/to/repo -o docs/RUNBOOK-FEATURE.md
```

### `sldo-run` — Rust CLI for Milestone Execution

Rust equivalent of `run-milestones.sh`. Drives GitHub Copilot CLI through runbook milestones, verifying build and tests after each pass.

```bash
sldo-run <runbook> <repo-dir> [options]
```

**Options:**
- `-m, --model <model>` — Copilot model (default: `claude-opus-4.6`)
- `-a, --max-attempts <N>` — Max Copilot invocations (default: 150)
- `-c, --cooldown <secs>` — Pause between retries (default: 5)
- `--build-cmd <cmd>` — Custom build verification command (repeatable)
- `--test-cmd <cmd>` — Custom test verification command (repeatable)

**Auto-detection:** If no `--build-cmd` / `--test-cmd` are given, the tool auto-detects commands from the project's build files (Cargo.toml, package.json, go.mod, Makefile, etc.).

**Example:**
```bash
cargo run -p sldo-run -- docs/RUNBOOK.md /path/to/repo
cargo run -p sldo-run -- docs/RUNBOOK.md /path/to/repo --build-cmd "cargo build" --test-cmd "cargo test"
```

The original Bash scripts in `src/` remain fully functional during the transition.
