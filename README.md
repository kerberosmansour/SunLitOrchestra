# SunLitOrchestrate

A toolkit for orchestrating AI-driven software development through structured, milestone-based runbooks.

## Installation (Rust CLI — recommended)

Install the Rust binaries with Cargo:

```bash
cargo install --path crates/sldo-plan
cargo install --path crates/sldo-run
```

Or build from source:

```bash
cargo build --workspace --release
# Binaries at target/release/sldo-plan and target/release/sldo-run
```

## Rust CLI

The Rust implementation is the preferred way to use SunLitOrchestrate. It provides type-safe argument parsing, structured error handling, and cross-platform support.

### `sldo-plan` — Generate a Runbook

Generates a milestone-based runbook from a requirements prompt and a repository using GitHub Copilot CLI.

```bash
sldo-plan <prompt-file> <repo-dir> [options]
```

**Arguments:**
- `<prompt-file>` — Path to a text/markdown file describing the desired changes
- `<repo-dir>` — Path to the target repository

**Options:**
- `-o, --output <path>` — Output runbook path (default: `<repo>/docs/RUNBOOK.md`)
- `-m, --model <model>` — Copilot model (default: `claude-opus-4.6`)
- `-n, --max-iterations <N>` — Max planning refinement passes (default: 3)
- `-h, --help` — Show help

**Safety:** Refuses to run if the repo is on `main` or `master` branch.

**Example:**
```bash
sldo-plan requirements.txt /path/to/my-project -o docs/RUNBOOK-FEATURE.md
# Or via cargo:
cargo run -p sldo-plan -- requirements.txt /path/to/repo -o docs/RUNBOOK-FEATURE.md
```

### `sldo-run` — Execute a Runbook

Drives GitHub Copilot CLI through the milestones in an existing runbook, one at a time, verifying build and tests after each pass.

```bash
sldo-run <runbook> <repo-dir> [options]
```

**Arguments:**
- `<runbook>` — Path to the runbook markdown file (relative to repo or absolute)
- `<repo-dir>` — Path to the target repository

**Options:**
- `-m, --model <model>` — Copilot model (default: `claude-opus-4.6`)
- `-a, --max-attempts <N>` — Max Copilot invocations (default: 150)
- `-c, --cooldown <secs>` — Pause between retries (default: 5)
- `--build-cmd <cmd>` — Custom build verification command (repeatable)
- `--test-cmd <cmd>` — Custom test verification command (repeatable)
- `-h, --help` — Show help

**Auto-detection:** If no `--build-cmd` / `--test-cmd` are given, the tool auto-detects commands from the project's build files (Cargo.toml, package.json, go.mod, Makefile, etc.).

**Safety:** Refuses to run if the repo is on `main` or `master` branch.

**Examples:**
```bash
# Auto-detect build/test commands from the repo
sldo-run docs/RUNBOOK.md /path/to/my-project

# Specify explicit build and test commands
sldo-run docs/RUNBOOK.md /path/to/my-project \
  --build-cmd "cargo build --workspace" \
  --test-cmd "cargo test --workspace"

# Or via cargo:
cargo run -p sldo-run -- docs/RUNBOOK.md /path/to/repo
```

### Project Structure

```
crates/
├── sldo-common/   # Shared library (CLI parsing, colour output, git checks, runbook parsing)
├── sldo-plan/     # Binary: runbook generation (replaces plan-milestones.sh)
├── sldo-run/      # Binary: milestone execution (replaces run-milestones.sh)
└── sldo-tauri/    # Desktop app: Tauri v2 + React GUI for planning and execution
```

Build and test the workspace:

```bash
cargo build --workspace
cargo test --workspace
```

## Desktop App

SunLitOrchestrate includes a Tauri v2 desktop application that provides a graphical interface for AI-driven planning and execution.

### UI Overview

The desktop app features a chatbot-style interface:
- **Home screen** — centered prompt input with sample prompt chips and hero branding
- **Conversation view** — scrollable message thread with user/assistant messages and input pinned at bottom
- **Sidebar** — navigation with logo, session management, and settings access
- **Plan editor** — Markdown editor with edit/preview toggle, milestone tracker sidebar, and validation warnings
- **Execution view** — live streaming agent output, build/test results, milestone progress, and cancel button
- **Settings panel** — provider/model selection, tool flags editor, execution parameters, and repository directory
- **Voice input** — microphone button in the chat input area for speech-to-text transcription
- **Error boundary** — graceful fallback UI when components crash, with "Try Again" recovery

### Keyboard Shortcuts

| Shortcut | Action |
|---|---|
| `Cmd/Ctrl+Enter` | Submit prompt |
| `Cmd/Ctrl+N` | New session |
| `Cmd/Ctrl+,` | Open settings |
| `Escape` | Close settings panel |
| `Shift+Enter` | Insert newline in prompt |
| `Cmd/Ctrl+S` | Save runbook (in editor) |

### Workflow

1. **Prompt** — Type or speak your requirements on the home screen
2. **Plan** — The app invokes the coding agent to generate a milestone-based runbook
3. **Review** — Edit the runbook in the Markdown editor; review milestones in the tracker
4. **Execute** — Click "Execute Plan" to run milestones; monitor live output and build/test results
5. **Cancel** — Click "Cancel Execution" at any time to stop

### Configuration

Open Settings (`Cmd/Ctrl+,`) to configure:

| Setting | Default | Description |
|---|---|---|
| Provider | `copilot` | Coding agent backend |
| Model | `claude-opus-4.6` | AI model for planning/execution |
| Max Attempts | `150` | Maximum execution attempts per run |
| Cooldown | `5` seconds | Delay between execution attempts |
| Max Iterations | `3` | Planning refinement iterations |
| Repository Directory | _(none)_ | Target repo for planning/execution |
| Allow Flags | Tool permissions | Copilot CLI `--allow-tool` flags |
| Deny Flags | Tool restrictions | Copilot CLI `--deny-tool` flags |

### Prerequisites

- [Node.js](https://nodejs.org/) v18+ (for the React frontend)
- Rust toolchain with Tauri CLI: `cargo install tauri-cli --version '^2'`

### Voice Input Setup

To enable speech-to-text, set your OpenAI API key in a `.env` file at the project root:

```bash
echo 'OPENAI_API_KEY=sk-your-key-here' >> .env
```

The API key is read by the Tauri backend only — it is never sent to the frontend.

### Development

```bash
# Install frontend dependencies (first time only)
cd crates/sldo-tauri/ui && npm install

# Launch the desktop app in development mode
cargo tauri dev
```

### Build

```bash
# Build the frontend
cd crates/sldo-tauri/ui && npm run build

# Build the full app
cargo tauri build
```

### Testing

```bash
# Run frontend unit and component tests (90 tests)
cd crates/sldo-tauri/ui && npm test

# Run all backend tests including Tauri E2E (200 tests)
cargo test --workspace
```

### Troubleshooting

| Issue | Solution |
|---|---|
| `cargo tauri dev` fails | Ensure Node.js v18+ is installed and `cd crates/sldo-tauri/ui && npm install` has been run |
| Voice input doesn't work | Set `OPENAI_API_KEY` in `.env` file at project root |
| Settings not persisting | Check Tauri app data directory permissions |
| Build warnings about unused fields | These are intentional — fields used at runtime via serialization |

### Migrating from Bash

See [docs/MIGRATION.md](docs/MIGRATION.md) for a complete migration guide with flag mapping and behavioral differences.

## Legacy Bash Scripts

> **Note:** The Bash scripts below are the original implementation. They remain functional but the Rust CLI above is the preferred implementation.

### `src/plan-milestones.sh` — Generate a Runbook (legacy)

```bash
./src/plan-milestones.sh <prompt-file> <repo-dir> [options]
```

**Options:**
- `-o, --output <path>` — Output runbook path (default: `<repo>/docs/RUNBOOK.md`)
- `-m, --model <model>` — Copilot model (default: `claude-opus-4.6`)
- `-n, --max-iterations <N>` — Max planning refinement passes (default: 3)
- `-h, --help` — Show this help message

### `src/run-milestones.sh` — Execute a Runbook (legacy)

```bash
./src/run-milestones.sh <runbook> <repo-dir> [options]
```

**Options:**
- `-m, --model <model>` — Copilot model (default: `claude-opus-4.6`)
- `-a, --max-attempts <N>` — Max Copilot invocations (default: 150)
- `-c, --cooldown <secs>` — Pause between retries (default: 5)
- `--build-cmd <cmd>` — Custom build verification command (repeatable)
- `--test-cmd <cmd>` — Custom test verification command (repeatable)
- `-h, --help` — Show this help message

## Runbook Template

See [docs/runbook-template.md](docs/runbook-template.md) for the milestone template structure used by both implementations.
