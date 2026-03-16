#!/usr/bin/env bash
# =============================================================================
# plan-milestones.sh — Generate a milestone-based runbook from a prompt file
#
# Takes a text file (describing desired software changes) and a repo location
# as input. Uses GitHub Copilot CLI to analyse the repo and produce a runbook
# markdown file following the project's runbook template.
#
# Usage:
#   ./plan-milestones.sh <prompt-file> <repo-dir> [options]
#
# Options:
#   -o, --output <path>     Output runbook path (default: <repo>/docs/RUNBOOK.md)
#   -m, --model <model>     Copilot model to use (default: claude-opus-4.6)
#   -n, --max-iterations <N> Max planning refinement iterations (default: 3)
#   -h, --help              Show this help message
#
# Examples:
#   ./plan-milestones.sh requirements.txt /path/to/repo
#   ./plan-milestones.sh spec.md /path/to/repo -o docs/RUNBOOK-FEATURE.md
# =============================================================================
set -euo pipefail

# ── Defaults ─────────────────────────────────────────────────────────────────
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
TEMPLATE_PATH="${SCRIPT_DIR}/../docs/runbook-template.md"
MODEL="claude-opus-4.6"
MAX_ITERATIONS=3
OUTPUT_PATH=""
COOLDOWN_SECS=3
LOG_DIR=""

# Tool-permission flags — read-only exploration + write for the output file
ALLOW_FLAGS=(
  --allow-tool='write'
  # ── File reading / searching ──
  --allow-tool='shell(cat:*)'
  --allow-tool='shell(ls:*)'
  --allow-tool='shell(find:*)'
  --allow-tool='shell(head:*)'
  --allow-tool='shell(tail:*)'
  --allow-tool='shell(grep:*)'
  --allow-tool='shell(rg:*)'
  --allow-tool='shell(ag:*)'
  # ── Text processing ──
  --allow-tool='shell(echo:*)'
  --allow-tool='shell(printf:*)'
  --allow-tool='shell(sed:*)'
  --allow-tool='shell(awk:*)'
  --allow-tool='shell(wc:*)'
  --allow-tool='shell(sort:*)'
  --allow-tool='shell(uniq:*)'
  --allow-tool='shell(cut:*)'
  --allow-tool='shell(tr:*)'
  --allow-tool='shell(diff:*)'
  --allow-tool='shell(tee:*)'
  --allow-tool='shell(xargs:*)'
  --allow-tool='shell(basename:*)'
  --allow-tool='shell(dirname:*)'
  --allow-tool='shell(realpath:*)'
  # ── Misc utilities ──
  --allow-tool='shell(which:*)'
  --allow-tool='shell(env:*)'
  --allow-tool='shell(test:*)'
  --allow-tool='shell(true:*)'
  --allow-tool='shell(false:*)'
  --allow-tool='shell(cd:*)'
  --allow-tool='shell(pwd:*)'
  --allow-tool='shell(python:*)'
  --allow-tool='shell(python3:*)'
  --allow-tool='shell(tree:*)'
  # ── Version control (read-only operations) ──
  --allow-tool='shell(git:*)'
  # ── Build / package managers (for discovery only) ──
  --allow-tool='shell(cargo:*)'
  --allow-tool='shell(rustc:*)'
  --allow-tool='shell(node:*)'
  --allow-tool='shell(npm:*)'
  --allow-tool='shell(npx:*)'
  --allow-tool='shell(pnpm:*)'
  --allow-tool='shell(yarn:*)'
  --allow-tool='shell(tsc:*)'
  --allow-tool='shell(pip:*)'
  --allow-tool='shell(pip3:*)'
  --allow-tool='shell(go:*)'
  --allow-tool='shell(make:*)'
  --allow-tool='shell(cmake:*)'
  --allow-tool='shell(mkdir:*)'
)
DENY_FLAGS=(
  # Block destructive / irreversible operations
  --deny-tool='shell(rm -rf /)'
  --deny-tool='shell(git push --force)'
  --deny-tool='shell(git push -f)'
  --deny-tool='shell(git reset --hard)'
  --deny-tool='shell(git clean -fd)'
  --deny-tool='shell(rm -rf:*)'
)

# ── Colours & helpers ────────────────────────────────────────────────────────
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
BOLD='\033[1m'
RESET='\033[0m'

ts()      { date "+%Y-%m-%d %H:%M:%S"; }
info()    { echo -e "${BLUE}[$(ts)] ℹ ${RESET} $*"; }
success() { echo -e "${GREEN}[$(ts)] ✔ ${RESET} $*"; }
warn()    { echo -e "${YELLOW}[$(ts)] ⚠ ${RESET} $*"; }
fail()    { echo -e "${RED}[$(ts)] ✖ ${RESET} $*"; }
header()  { echo -e "\n${BOLD}${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${RESET}"; \
            echo -e "${BOLD}${CYAN}  $*${RESET}"; \
            echo -e "${BOLD}${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${RESET}\n"; }
divider() { echo -e "${CYAN}──────────────────────────────────────────────────${RESET}"; }

# ── Usage ────────────────────────────────────────────────────────────────────
usage() {
  cat <<EOF
Usage: $(basename "$0") <prompt-file> <repo-dir> [options]

Generate a milestone-based runbook from a requirements prompt and a repository.

Arguments:
  prompt-file       Path to a text/markdown file describing the desired changes.
  repo-dir          Path to the target repository to plan changes for.

Options:
  -o, --output <path>       Output runbook path (default: <repo>/docs/RUNBOOK.md)
  -m, --model <model>       Copilot model (default: claude-opus-4.6)
  -n, --max-iterations <N>  Max planning refinement iterations (default: 3)
  -h, --help                Show this help message

Examples:
  $(basename "$0") requirements.txt /path/to/repo
  $(basename "$0") spec.md /path/to/repo -o docs/RUNBOOK-FEATURE.md
  $(basename "$0") changes.txt . --model claude-opus-4.6 -n 5
EOF
  exit 0
}

# ── Parse arguments ──────────────────────────────────────────────────────────
parse_args() {
  if [[ $# -lt 2 ]]; then
    fail "Missing required arguments."
    echo ""
    usage
  fi

  PROMPT_FILE="$1"
  shift
  REPO_DIR="$1"
  shift

  while [[ $# -gt 0 ]]; do
    case "$1" in
      -o|--output)
        OUTPUT_PATH="$2"
        shift 2
        ;;
      -m|--model)
        MODEL="$2"
        shift 2
        ;;
      -n|--max-iterations)
        MAX_ITERATIONS="$2"
        shift 2
        ;;
      -h|--help)
        usage
        ;;
      *)
        fail "Unknown option: $1"
        echo ""
        usage
        ;;
    esac
  done

  # Resolve paths
  PROMPT_FILE="$(cd "$(dirname "${PROMPT_FILE}")" && pwd)/$(basename "${PROMPT_FILE}")"
  REPO_DIR="$(cd "${REPO_DIR}" && pwd)"

  if [[ -z "${OUTPUT_PATH}" ]]; then
    OUTPUT_PATH="${REPO_DIR}/docs/RUNBOOK.md"
  elif [[ "${OUTPUT_PATH}" != /* ]]; then
    # Relative path — resolve relative to repo dir
    OUTPUT_PATH="${REPO_DIR}/${OUTPUT_PATH}"
  fi

  LOG_DIR="${REPO_DIR}/.copilot-logs"
}

# ── Pre-flight checks ───────────────────────────────────────────────────────
preflight() {
  header "Pre-flight checks"

  # Check copilot CLI
  if ! command -v copilot &>/dev/null; then
    fail "GitHub Copilot CLI ('copilot') not found on PATH."
    fail "Install it: https://docs.github.com/en/copilot/concepts/agents/copilot-cli"
    exit 1
  fi
  success "copilot CLI found: $(which copilot)"

  # Check prompt file exists
  if [[ ! -f "${PROMPT_FILE}" ]]; then
    fail "Prompt file not found: ${PROMPT_FILE}"
    exit 1
  fi
  success "Prompt file found: ${PROMPT_FILE} ($(wc -c < "${PROMPT_FILE}" | tr -d ' ') bytes)"

  # Check repo directory exists
  if [[ ! -d "${REPO_DIR}" ]]; then
    fail "Repository directory not found: ${REPO_DIR}"
    exit 1
  fi
  success "Repository directory: ${REPO_DIR}"

  # Check it's a git repo and NOT on main/master
  if ! git -C "${REPO_DIR}" rev-parse --is-inside-work-tree &>/dev/null; then
    fail "Not a git repository: ${REPO_DIR}"
    fail "The planning script requires a git repo to avoid working on unversioned code."
    exit 1
  fi

  local branch
  branch="$(git -C "${REPO_DIR}" rev-parse --abbrev-ref HEAD)"
  info "Current git branch: ${branch}"

  if [[ "${branch}" == "main" || "${branch}" == "master" ]]; then
    fail "You are on '${branch}'. Refusing to plan on a protected branch."
    fail "Please create a feature branch first:"
    fail "  git checkout -b feature/my-changes"
    exit 1
  fi
  success "Branch '${branch}' is not main/master — safe to proceed."

  # Check template exists
  if [[ ! -f "${TEMPLATE_PATH}" ]]; then
    warn "Runbook template not found at ${TEMPLATE_PATH} — will use built-in template."
  else
    success "Runbook template found: ${TEMPLATE_PATH}"
  fi

  # Ensure output directory exists
  local output_dir
  output_dir="$(dirname "${OUTPUT_PATH}")"
  mkdir -p "${output_dir}"
  success "Output will be written to: ${OUTPUT_PATH}"

  # Set up log directory
  mkdir -p "${LOG_DIR}"
  success "Log directory: ${LOG_DIR}"
  echo ""
}

# ── Read the runbook template ────────────────────────────────────────────────
read_template() {
  if [[ -f "${TEMPLATE_PATH}" ]]; then
    cat "${TEMPLATE_PATH}"
  else
    # Fallback: minimal built-in template structure
    cat <<'FALLBACK_TEMPLATE'
# [Runbook Title] — [Project Name]

> **Purpose**: [One-sentence description]
> **How to use**: Work through milestones sequentially.

---

## Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File |
|---|---|---|---|---|---|

---

## Pre-Milestone Protocol
## Post-Milestone Protocol
## Background Context
## Milestone Plan

### Milestone N — [Title]

**Goal**: [description]
**Context**: [description]

#### Files Most Likely Touched
#### Step-by-Step
#### BDD Acceptance Scenarios
#### Regression Tests
#### E2E Runtime Validation
#### Smoke Tests

---

## Documentation Update Table
FALLBACK_TEMPLATE
  fi
}

# ── Build the planning prompt ────────────────────────────────────────────────
build_planning_prompt() {
  local iteration="$1"
  local user_prompt
  user_prompt="$(cat "${PROMPT_FILE}")"
  local template
  template="$(read_template)"

  cat <<PROMPT
You are an expert software architect and planning agent. Your job is to analyse a
repository and produce a detailed, actionable runbook of milestones that an AI
coding agent can follow to implement the requested changes.

## INPUT

### User Requirements

The user has described the following desired changes:

<requirements>
${user_prompt}
</requirements>

### Runbook Template

The output MUST follow this exact template structure. Fill in every placeholder.
Do not remove any sections — populate them all with concrete, repo-specific content.

<template>
${template}
</template>

## YOUR TASK

1. **Explore the repository thoroughly.** Read key files: README, package.json /
   Cargo.toml / pyproject.toml / Makefile (whichever exist), directory structure,
   architecture docs, existing test structure, CI config. Understand the tech stack,
   project layout, build commands, and test commands.

2. **Analyse the requirements** against the current codebase. Identify:
   - Which files and modules need to change
   - What new files need to be created
   - What the correct build and test commands are
   - What dependencies might need to be added
   - What existing tests and features must not break

3. **Decompose into milestones.** Each milestone should be:
   - Small enough for one focused coding session (1-3 hours of AI agent work)
   - Self-contained: builds, tests pass, and the new feature works at the end
   - Ordered by dependency — earlier milestones provide foundations for later ones
   - Concrete: reference real file paths, real function names, real test commands

4. **For each milestone, fill in the template completely:**
   - **Goal**: One clear sentence.
   - **Context**: Reference specific files, modules, and current state.
   - **Files Most Likely Touched**: Real file paths with specific changes.
   - **Step-by-Step**: Numbered, actionable steps. Not vague — reference files and functions.
   - **BDD Acceptance Scenarios**: Concrete Given/When/Then with realistic test data.
   - **E2E Runtime Validation**: Tests that prove the system works at runtime.
   - **Regression Tests**: List pre-existing tests that must still pass.
   - **Smoke Tests**: Manual or scripted verification steps.

5. **Fill in background sections:**
   - **Current State**: Describe the real repo state (reference actual files).
   - **Problem**: Number the actual gaps the requirements describe.
   - **Target Architecture**: ASCII diagram or description of the end state.
   - **Key Design Principles**: Derive from the codebase style and requirements.
   - **What to Keep / What to Change**: Concrete lists of files and modules.
   - **Documentation Update Table**: What docs need updating per milestone.

6. **Populate protocols with real commands:**
   - Replace placeholder test/build commands with the actual commands for this repo.
   - Use the correct package manager, test runner, and build system.

7. **Write the completed runbook** to: \`${OUTPUT_PATH}\`

## HARD RULES

- Explore the repo BEFORE writing the runbook. Do not guess file paths or commands.
- Every file path in the runbook must be a real path in the repo (or a clearly marked new file).
- Every build/test command must be the actual command for this project's tech stack.
- Milestones must be strictly sequential — no circular dependencies.
- Each milestone must leave the project in a buildable, testable state.
- Do NOT implement any code changes. This is a PLANNING session only.
- Do NOT modify any existing source files in the repository.
- Do NOT commit or push anything.
- Write the runbook output to \`${OUTPUT_PATH}\`.
- The runbook file is the ONLY file you should create or modify.
PROMPT

  # Add iteration context for refinement passes
  if (( iteration > 1 )); then
    cat <<RETRY

## REFINEMENT PASS ${iteration}

A previous planning pass has already written a draft runbook to \`${OUTPUT_PATH}\`.
Please:

1. **Read the existing draft** at \`${OUTPUT_PATH}\`.
2. **Re-explore the repo** to verify all file paths and commands in the draft are accurate.
3. **Improve the runbook**:
   - Fix any incorrect file paths, commands, or tech stack references.
   - Add missing BDD scenarios or E2E tests.
   - Ensure step-by-step instructions are concrete enough for an AI agent to follow.
   - Verify milestone ordering makes sense (no forward dependencies).
   - Fill in any placeholder text that was left from the template.
4. **Overwrite** \`${OUTPUT_PATH}\` with the improved version.
RETRY
  fi
}

# ── Validate the generated runbook ───────────────────────────────────────────
validate_runbook() {
  local runbook_path="$1"
  local issues=0

  info "Validating generated runbook…"

  if [[ ! -f "${runbook_path}" ]]; then
    fail "Runbook file was not created at: ${runbook_path}"
    return 1
  fi

  local size
  size=$(wc -c < "${runbook_path}" | tr -d ' ')
  if (( size < 500 )); then
    warn "Runbook is suspiciously small (${size} bytes). May be incomplete."
    issues=$((issues + 1))
  else
    success "Runbook size: ${size} bytes"
  fi

  # Check for key sections
  local required_sections=(
    "Milestone Tracker"
    "Pre-Milestone Protocol"
    "Post-Milestone Protocol"
    "Background Context"
    "Current State"
    "BDD Acceptance Scenarios"
  )

  for section in "${required_sections[@]}"; do
    if grep -q "${section}" "${runbook_path}"; then
      success "Found section: ${section}"
    else
      warn "Missing section: ${section}"
      issues=$((issues + 1))
    fi
  done

  # Check milestone tracker has entries
  local milestone_count
  milestone_count=$(grep -cE '^\| [0-9]+ \|' "${runbook_path}" || echo "0")
  if (( milestone_count > 0 )); then
    success "Milestone count: ${milestone_count}"
  else
    warn "No milestones found in tracker table."
    issues=$((issues + 1))
  fi

  # Check for unfilled template placeholders
  local placeholder_count
  placeholder_count=$(grep -cE '\[.*\.(rs|ts|tsx|js|json|toml|yaml|md)\]' "${runbook_path}" \
    | grep -c '\[.*file path\]' || echo "0")
  if (( placeholder_count > 0 )); then
    warn "Found ${placeholder_count} possible unfilled placeholders."
    issues=$((issues + 1))
  fi

  # Check that status values are not_started
  local done_count
  done_count=$(grep -cE '`done`' "${runbook_path}" || echo "0")
  if (( done_count > 0 )); then
    warn "Some milestones are marked 'done' — they should all be 'not_started'."
    issues=$((issues + 1))
  fi

  if (( issues == 0 )); then
    success "Runbook validation passed — no issues found."
    return 0
  else
    warn "Runbook validation found ${issues} issue(s). Will attempt refinement."
    return 1
  fi
}

# ── Main ─────────────────────────────────────────────────────────────────────
main() {
  parse_args "$@"

  header "Milestone Planner — Runbook Generator"
  info "Prompt file:    ${PROMPT_FILE}"
  info "Repository:     ${REPO_DIR}"
  info "Output:         ${OUTPUT_PATH}"
  info "Model:          ${MODEL}"
  info "Max iterations: ${MAX_ITERATIONS}"
  echo ""

  preflight

  local start_time iteration=0
  start_time="$(date +%s)"

  cd "${REPO_DIR}"

  while (( iteration < MAX_ITERATIONS )); do
    iteration=$((iteration + 1))
    local log_file="${LOG_DIR}/plan-iteration-${iteration}.log"

    divider
    if (( iteration == 1 )); then
      info "Iteration ${iteration}/${MAX_ITERATIONS} — Initial planning pass"
    else
      info "Iteration ${iteration}/${MAX_ITERATIONS} — Refinement pass"
    fi
    divider

    local prompt
    prompt="$(build_planning_prompt "${iteration}")"

    # Log
    mkdir -p "${LOG_DIR}"
    echo "═══════════════════════════════════════════════════" >> "${log_file}"
    echo "[$(ts)] Planning iteration ${iteration}" >> "${log_file}"
    echo "═══════════════════════════════════════════════════" >> "${log_file}"

    # Invoke Copilot CLI
    local exit_code=0
    copilot -p "${prompt}" \
      --model "${MODEL}" \
      "${ALLOW_FLAGS[@]}" \
      "${DENY_FLAGS[@]}" \
      2>&1 | tee -a "${log_file}" || exit_code=$?

    mkdir -p "${LOG_DIR}"
    echo "" >> "${log_file}"
    echo "[$(ts)] Exit code: ${exit_code}" >> "${log_file}"

    if (( exit_code != 0 )); then
      warn "Copilot exited with code ${exit_code}."
    fi

    # Validate the output
    if validate_runbook "${OUTPUT_PATH}"; then
      success "Runbook generated successfully after ${iteration} iteration(s)."
      break
    fi

    if (( iteration < MAX_ITERATIONS )); then
      info "Cooling down ${COOLDOWN_SECS}s before refinement…"
      sleep "${COOLDOWN_SECS}"
    fi
  done

  local end_time
  end_time="$(date +%s)"
  local elapsed=$(( end_time - start_time ))
  local minutes=$(( elapsed / 60 ))
  local seconds=$(( elapsed % 60 ))

  echo ""
  header "Planning Complete"

  if [[ -f "${OUTPUT_PATH}" ]]; then
    success "Runbook written to: ${OUTPUT_PATH}"

    # Show summary
    local milestone_count
    milestone_count=$(grep -cE '^\| [0-9]+ \|' "${OUTPUT_PATH}" || echo "0")
    info "Milestones planned: ${milestone_count}"

    echo ""
    info "Milestone Tracker:"
    grep -E '^\|' "${OUTPUT_PATH}" | head -20
    echo ""

    success "You can now run the milestones with:"
    info "  src/run-milestones.sh"
    info "  (after updating RUNBOOK path in the script to point to ${OUTPUT_PATH})"
  else
    fail "Runbook was not generated after ${MAX_ITERATIONS} iterations."
    fail "Check logs in ${LOG_DIR}/ for details."
  fi

  info "Total wall time: ${minutes}m ${seconds}s"
}

main "$@"
