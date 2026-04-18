#!/usr/bin/env bash
# mock-claude.sh — Mimics the Claude Code CLI for integration testing.
#
# Reads the -p prompt from args and writes a minimal valid runbook
# to the output file mentioned in the prompt. This allows end-to-end
# testing without requiring actual Claude API access.
set -euo pipefail

PROMPT=""
MODEL=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        -p|--print)
            PROMPT="$2"
            shift 2
            ;;
        --model)
            MODEL="$2"
            shift 2
            ;;
        --allowedTools=*|--disallowedTools=*)
            shift
            ;;
        --allowedTools|--disallowedTools)
            shift 2
            ;;
        *)
            shift
            ;;
    esac
done

# Try to extract the output path from the prompt.
# The plan prompt contains "Write the completed runbook to: `<path>`"
OUTPUT_PATH=""
if echo "$PROMPT" | grep -qoE 'Write the completed runbook.*to: `[^`]+`'; then
    OUTPUT_PATH=$(echo "$PROMPT" | grep -oE 'Write the completed runbook.*to: `[^`]+`' | grep -oE '`[^`]+`' | tail -1 | tr -d '`')
fi

# Also try the "write the runbook output to `<path>`" pattern
if [ -z "$OUTPUT_PATH" ]; then
    if echo "$PROMPT" | grep -qoE 'runbook output to `[^`]+`'; then
        OUTPUT_PATH=$(echo "$PROMPT" | grep -oE 'runbook output to `[^`]+`' | grep -oE '`[^`]+`' | tail -1 | tr -d '`')
    fi
fi

# If we found an output path and it looks like a planning prompt, write a runbook
if [ -n "$OUTPUT_PATH" ]; then
    mkdir -p "$(dirname "$OUTPUT_PATH")"
    cat > "$OUTPUT_PATH" << 'RUNBOOK'
# Mock Runbook — Test Project

> **Purpose**: Auto-generated mock runbook for integration testing.
> **How to use**: Work through milestones sequentially.

---

## Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File |
|---|---|---|---|---|---|
| 1 | Setup scaffolding | `not_started` | | | |
| 2 | Implement feature | `not_started` | | | |

---

## Pre-Milestone Protocol

1. Run the full test suite before starting.
2. Read the milestone section fully.

## Post-Milestone Protocol

1. Run the full test suite after completing.
2. Write a lessons-learned file.

## Background Context

### Current State

This is a test project with mock content.

### Problem

1. Testing the planning flow end-to-end.

## Milestone Plan

### Milestone 1 — Setup scaffolding

**Goal**: Set up the project structure.

#### BDD Acceptance Scenarios

| Scenario | Given | When | Then |
|---|---|---|---|
| Project builds | Fresh clone | Build command runs | Exit code 0 |

#### Smoke Tests

- [ ] Project builds successfully

---

### Milestone 2 — Implement feature

**Goal**: Implement the main feature.

#### BDD Acceptance Scenarios

| Scenario | Given | When | Then |
|---|---|---|---|
| Feature works | Project built | Feature is used | Expected output |

#### Smoke Tests

- [ ] Feature works correctly

---

## Documentation Update Table

| Milestone | README.md Update | Other Docs |
|---|---|---|
| 1 | Add project note | — |
| 2 | Document feature | — |
RUNBOOK
    echo "mock-claude: wrote runbook to $OUTPUT_PATH"
else
    # For run-milestones prompts, just print success
    echo "mock-claude: processed prompt (model=$MODEL)"
fi

exit 0
