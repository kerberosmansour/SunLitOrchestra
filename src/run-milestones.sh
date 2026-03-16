#!/usr/bin/env bash
# =============================================================================
# run-milestones.sh — Drive GitHub Copilot CLI through runbook milestones
#
# Loops until all milestones in a runbook are marked `done`.
# Each iteration tells Copilot to read the runbook and work on the next
# incomplete milestone. Verifies build + tests after each invocation.
#
# Usage:
#   ./run-milestones.sh <runbook> <repo-dir> [options]
#
# Options:
#   -m, --model <model>          Copilot model to use (default: claude-opus-4.6)
#   -a, --max-attempts <N>       Max Copilot invocations (default: 150)
#   -c, --cooldown <secs>        Pause between retries (default: 5)
#   --build-cmd <cmd>            Custom build verification command (repeatable)
#   --test-cmd <cmd>             Custom test verification command (repeatable)
#   -h, --help                   Show this help message
#
# Examples:
#   ./run-milestones.sh docs/RUNBOOK.md /path/to/repo
#   ./run-milestones.sh docs/RUNBOOK-UI.md /path/to/repo -m claude-opus-4.6
#   ./run-milestones.sh docs/RUNBOOK.md . --build-cmd "cargo build" --test-cmd "cargo test"
# =============================================================================
set -euo pipefail

# ── Defaults ─────────────────────────────────────────────────────────────────
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
MODEL="claude-opus-4.6"
MAX_ATTEMPTS=150          # total Copilot invocations before giving up
COOLDOWN_SECS=5           # pause between retries
BUILD_CMDS=()
TEST_CMDS=()
PROJECT_DIR=""
RUNBOOK=""
LOG_DIR=""

# Tool-permission flags — broad access, deny destructive operations
# NOTE: shell(cmd:*) means "cmd + any subcommands/args". Without :* it only
#       matches the bare command with no arguments.
ALLOW_FLAGS=(
  --allow-tool='write'
  # ── Build / test toolchain ──
  --allow-tool='shell(cargo:*)'
  --allow-tool='shell(rustc:*)'
  --allow-tool='shell(rustup:*)'
  --allow-tool='shell(rustfmt:*)'
  # ── Version control ──
  --allow-tool='shell(git:*)'
  # ── File reading / searching ──
  --allow-tool='shell(cat:*)'
  --allow-tool='shell(ls:*)'
  --allow-tool='shell(find:*)'
  --allow-tool='shell(head:*)'
  --allow-tool='shell(tail:*)'
  --allow-tool='shell(grep:*)'
  --allow-tool='shell(rg:*)'
  --allow-tool='shell(ag:*)'
  # ── File manipulation ──
  --allow-tool='shell(mkdir:*)'
  --allow-tool='shell(cp:*)'
  --allow-tool='shell(mv:*)'
  --allow-tool='shell(touch:*)'
  --allow-tool='shell(rm:*)'
  --allow-tool='shell(chmod:*)'
  --allow-tool='shell(ln:*)'
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
  # ── Node.js / frontend toolchain ──
  --allow-tool='shell(node:*)'
  --allow-tool='shell(npm:*)'
  --allow-tool='shell(npx:*)'
  --allow-tool='shell(pnpm:*)'
  --allow-tool='shell(yarn:*)'
  --allow-tool='shell(tsc:*)'
  --allow-tool='shell(vite:*)'
)
DENY_FLAGS=(
  # Block destructive / irreversible operations
  --deny-tool='shell(rm -rf /)'
  --deny-tool='shell(git push --force)'
  --deny-tool='shell(git push -f)'
  --deny-tool='shell(git reset --hard)'
  --deny-tool='shell(git clean -fd)'
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
Usage: $(basename "$0") <runbook> <repo-dir> [options]

Drive GitHub Copilot CLI through milestones in a runbook, one at a time,
verifying build and tests after each pass.

Arguments:
  runbook           Path to the runbook markdown file (relative to repo or absolute).
  repo-dir          Path to the target repository.

Options:
  -m, --model <model>          Copilot model (default: claude-opus-4.6)
  -a, --max-attempts <N>       Max Copilot invocations (default: 150)
  -c, --cooldown <secs>        Pause between retries (default: 5)
  --build-cmd <cmd>            Custom build verification command (repeatable)
  --test-cmd <cmd>             Custom test verification command (repeatable)
  -h, --help                   Show this help message

If no --build-cmd / --test-cmd are given, the script auto-detects commands
from the project's build files (Cargo.toml, package.json, Makefile, etc.).

Examples:
  $(basename "$0") docs/RUNBOOK.md /path/to/repo
  $(basename "$0") docs/RUNBOOK-UI.md /path/to/repo -m claude-opus-4.6
  $(basename "$0") docs/RUNBOOK.md . --build-cmd "cargo build" --test-cmd "cargo test"
  $(basename "$0") docs/RUNBOOK.md . --build-cmd "npm run build" --test-cmd "npx vitest run"
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

  local runbook_arg="$1"
  shift
  local repo_arg="$1"
  shift

  while [[ $# -gt 0 ]]; do
    case "$1" in
      -m|--model)
        MODEL="$2"
        shift 2
        ;;
      -a|--max-attempts)
        MAX_ATTEMPTS="$2"
        shift 2
        ;;
      -c|--cooldown)
        COOLDOWN_SECS="$2"
        shift 2
        ;;
      --build-cmd)
        BUILD_CMDS+=("$2")
        shift 2
        ;;
      --test-cmd)
        TEST_CMDS+=("$2")
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

  # Resolve repo directory to absolute path
  PROJECT_DIR="$(cd "${repo_arg}" && pwd)"

  # Resolve runbook path — if absolute, use as-is; if relative, resolve from repo
  if [[ "${runbook_arg}" == /* ]]; then
    RUNBOOK="${runbook_arg}"
  else
    RUNBOOK="${runbook_arg}"
  fi

  LOG_DIR="${PROJECT_DIR}/.copilot-logs"
}

# ── Auto-detect build & test commands from the project ───────────────────────
detect_commands() {
  if [[ ${#BUILD_CMDS[@]} -gt 0 && ${#TEST_CMDS[@]} -gt 0 ]]; then
    return  # User supplied explicit commands; skip detection
  fi

  info "Auto-detecting build and test commands…"

  # Rust / Cargo
  if [[ -f "${PROJECT_DIR}/Cargo.toml" ]]; then
    if [[ ${#BUILD_CMDS[@]} -eq 0 ]]; then
      BUILD_CMDS+=("cargo build --workspace")
    fi
    if [[ ${#TEST_CMDS[@]} -eq 0 ]]; then
      TEST_CMDS+=("cargo test --workspace")
    fi
    success "Detected Cargo project"
  fi

  # Node / npm / pnpm / yarn
  for lockfile in package-lock.json pnpm-lock.yaml yarn.lock; do
    if [[ -f "${PROJECT_DIR}/${lockfile}" || -f "${PROJECT_DIR}/package.json" ]]; then
      local pkg_dir="${PROJECT_DIR}"
      local pm="npm"
      [[ -f "${PROJECT_DIR}/pnpm-lock.yaml" ]] && pm="pnpm"
      [[ -f "${PROJECT_DIR}/yarn.lock" ]] && pm="yarn"

      # Check if there's a build script
      if [[ -f "${pkg_dir}/package.json" ]] && grep -q '"build"' "${pkg_dir}/package.json" 2>/dev/null; then
        BUILD_CMDS+=("${pm} run build")
      fi
      # Check if there's a test script
      if [[ -f "${pkg_dir}/package.json" ]] && grep -q '"test"' "${pkg_dir}/package.json" 2>/dev/null; then
        TEST_CMDS+=("${pm} test")
      fi
      success "Detected ${pm} project"
      break
    fi
  done

  # Python
  if [[ -f "${PROJECT_DIR}/pyproject.toml" || -f "${PROJECT_DIR}/setup.py" || -f "${PROJECT_DIR}/setup.cfg" ]]; then
    if [[ ${#TEST_CMDS[@]} -eq 0 ]]; then
      if [[ -f "${PROJECT_DIR}/pyproject.toml" ]] && grep -q 'pytest' "${PROJECT_DIR}/pyproject.toml" 2>/dev/null; then
        TEST_CMDS+=("pytest")
      elif command -v pytest &>/dev/null; then
        TEST_CMDS+=("pytest")
      fi
    fi
    success "Detected Python project"
  fi

  # Go
  if [[ -f "${PROJECT_DIR}/go.mod" ]]; then
    if [[ ${#BUILD_CMDS[@]} -eq 0 ]]; then
      BUILD_CMDS+=("go build ./...")
    fi
    if [[ ${#TEST_CMDS[@]} -eq 0 ]]; then
      TEST_CMDS+=("go test ./...")
    fi
    success "Detected Go project"
  fi

  # Makefile
  if [[ -f "${PROJECT_DIR}/Makefile" ]]; then
    if [[ ${#BUILD_CMDS[@]} -eq 0 ]] && grep -q '^build:' "${PROJECT_DIR}/Makefile" 2>/dev/null; then
      BUILD_CMDS+=("make build")
    fi
    if [[ ${#TEST_CMDS[@]} -eq 0 ]] && grep -q '^test:' "${PROJECT_DIR}/Makefile" 2>/dev/null; then
      TEST_CMDS+=("make test")
    fi
    success "Detected Makefile"
  fi

  # Fallback: warn if nothing was detected
  if [[ ${#BUILD_CMDS[@]} -eq 0 ]]; then
    warn "No build commands detected. Use --build-cmd to specify manually."
  fi
  if [[ ${#TEST_CMDS[@]} -eq 0 ]]; then
    warn "No test commands detected. Use --test-cmd to specify manually."
  fi
}

# ── Pre-flight checks ───────────────────────────────────────────────────────
preflight() {
  header "Pre-flight checks"

  if ! command -v copilot &>/dev/null; then
    fail "GitHub Copilot CLI ('copilot') not found on PATH."
    fail "Install it: https://docs.github.com/en/copilot/concepts/agents/copilot-cli"
    exit 1
  fi
  success "copilot CLI found: $(which copilot)"

  if [[ ! -f "${PROJECT_DIR}/${RUNBOOK}" ]]; then
    fail "Runbook not found: ${PROJECT_DIR}/${RUNBOOK}"
    exit 1
  fi
  success "Runbook found: ${RUNBOOK}"

  if ! git -C "${PROJECT_DIR}" rev-parse --is-inside-work-tree &>/dev/null; then
    warn "Not a git repo — branch checks will be skipped."
  else
    local branch
    branch="$(git -C "${PROJECT_DIR}" rev-parse --abbrev-ref HEAD)"
    info "Current git branch: ${branch}"
    if [[ "${branch}" == "main" || "${branch}" == "master" ]]; then
      fail "You are on '${branch}'. Refusing to run on a protected branch."
      fail "Please create a feature branch first:"
      fail "  git checkout -b feature/my-changes"
      exit 1
    fi
    success "Branch '${branch}' is not main/master — safe to proceed."
  fi

  detect_commands

  if [[ ${#BUILD_CMDS[@]} -gt 0 ]]; then
    info "Build commands:"
    for cmd in "${BUILD_CMDS[@]}"; do info "  ${cmd}"; done
  fi
  if [[ ${#TEST_CMDS[@]} -gt 0 ]]; then
    info "Test commands:"
    for cmd in "${TEST_CMDS[@]}"; do info "  ${cmd}"; done
  fi

  mkdir -p "${LOG_DIR}"
  success "Log directory: ${LOG_DIR}"
  echo ""
}

# ── Check if all milestones are done ─────────────────────────────────────────
all_milestones_done() {
  # Returns 0 (true) if every milestone row in the tracker has status `done`
  # Tracker rows look like: | 9 | Description | `done` | ...
  # Only match rows that have a backtick-wrapped status field (the Milestone
  # Tracker), not other numbered tables (e.g. Documentation Update Table).
  local tracker_rows
  tracker_rows=$(grep -E '^\| [0-9]+ \|' "${PROJECT_DIR}/${RUNBOOK}" \
    | grep -E '`(not_started|in_progress|done)`' || true)
  if [[ -z "${tracker_rows}" ]]; then
    return 1  # No tracker rows found — not done
  fi
  local not_done
  not_done=$(echo "${tracker_rows}" | grep -v '`done`' || true)
  [[ -z "${not_done}" ]]
}

# ── Build verification command summaries for the prompt ──────────────────────
build_cmd_summary() {
  local label="$1"
  shift
  local cmds=("$@")
  if [[ ${#cmds[@]} -eq 0 ]]; then
    echo "- No ${label} commands configured."
    return
  fi
  for cmd in "${cmds[@]}"; do
    echo "- \`${cmd}\`"
  done
}

# ── Build the prompt ─────────────────────────────────────────────────────────
build_prompt() {
  local build_summary test_summary
  build_summary="$(build_cmd_summary "build" "${BUILD_CMDS[@]+"${BUILD_CMDS[@]}"}")"
  test_summary="$(build_cmd_summary "test" "${TEST_CMDS[@]+"${TEST_CMDS[@]}"}")"
  local today
  today="$(date +%Y-%m-%d)"

  cat <<PROMPT
You are an expert software engineer working on a project.

## YOUR TASK

1. Read the runbook at \`${RUNBOOK}\`. Look at the **Milestone Tracker** table.
2. Find the first milestone whose status is NOT \`done\` (i.e. \`not_started\` or \`in_progress\`).
3. Complete that milestone — and ONLY that milestone — following the runbook's Pre-Milestone Protocol, the milestone's Step-by-Step, BDD Acceptance Scenarios, E2E Runtime Validation, Smoke Tests, and Post-Milestone Protocol.
4. If an architecture doc exists (e.g. \`docs/ARCHITECTURE.md\`), read it for context.
5. If a lessons file exists for the previous milestone (check the Milestone Tracker), read it and apply any corrections.

## Workflow

- **Verify baseline**: Run the project's build and test commands before changing anything. Fix any failures first.
  Build commands:
${build_summary}
  Test commands:
${test_summary}
- **Update tracker**: Set the milestone status to \`in_progress\` and Started date to ${today}.
- **Write tests FIRST**: Create BDD and E2E test files from the milestone's scenario tables, following the test file conventions described in the runbook.
- **Implement**: Follow the Step-by-Step section. Make all tests pass.
- **Post-milestone**: Run smoke tests, write lessons-learned per the Post-Milestone Protocol, update tracker to \`done\`, update documentation per the Documentation Update table.
- **Final check**: Run all build and test commands one last time — everything must be green.
- **STOP**: Do not proceed to the next milestone. Your session ends here.

## Hard rules
- Write BDD tests BEFORE production code.
- Do NOT skip any step in the Pre-Milestone or Post-Milestone protocols.
- Do NOT touch code or tests belonging to other milestones.
- All pre-existing tests must still pass.
- Follow existing code style and naming conventions.
- Do not commit secrets, API keys, or credentials.
- Do NOT work on any subsequent milestone.
- Do NOT delete or modify any files in \`.copilot-logs/\` — the automation script writes its logs there.
- When running smoke tests that need a temporary directory, create it under \`output/\` and clean up only that directory afterward.
PROMPT
}

# ── Detect current milestone ─────────────────────────────────────────────────
current_milestone_number() {
  # Return the number of the first milestone whose status is NOT `done`
  # Only match Milestone Tracker rows (those with backtick-wrapped status)
  grep -E '^\| [0-9]+ \|' "${PROJECT_DIR}/${RUNBOOK}" \
    | grep -E '\`(not_started|in_progress|done)\`' \
    | grep -v '\`done\`' \
    | head -1 \
    | sed 's/^| *\([0-9]*\) .*/\1/' \
    || echo "unknown"
}

current_milestone_title() {
  # Return the title of the first incomplete milestone
  # Only match Milestone Tracker rows (those with backtick-wrapped status)
  grep -E '^\| [0-9]+ \|' "${PROJECT_DIR}/${RUNBOOK}" \
    | grep -E '\`(not_started|in_progress|done)\`' \
    | grep -v '\`done\`' \
    | head -1 \
    | awk -F'|' '{gsub(/^ +| +$/, "", $3); print $3}' \
    || echo "unknown"
}

# ── Run a set of verification commands ───────────────────────────────────────
verify_commands() {
  local label="$1"
  local log_file="$2"
  shift 2
  local cmds=("$@")

  if [[ ${#cmds[@]} -eq 0 ]]; then
    return
  fi

  for cmd in "${cmds[@]}"; do
    info "Running ${label}: ${cmd}"
    if eval "${cmd}" 2>&1 | tee -a "${log_file}"; then
      success "${label} OK: ${cmd}"
    else
      warn "${label} failed: ${cmd}. Will retry."
    fi
  done
}

# ── Main loop ────────────────────────────────────────────────────────────────
main() {
  parse_args "$@"

  header "Automated Milestone Runner"
  info "Repository: ${PROJECT_DIR}"
  info "Runbook:    ${RUNBOOK}"
  info "Model:      ${MODEL}"
  info "Max attempts: ${MAX_ATTEMPTS}"
  echo ""

  cd "${PROJECT_DIR}"
  preflight

  local start_time attempt=0
  start_time="$(date +%s)"

  while (( attempt < MAX_ATTEMPTS )); do
    # Check if we're done
    if all_milestones_done; then
      success "All milestones in the runbook are marked done!"
      break
    fi

    attempt=$((attempt + 1))
    local ms_num
    ms_num="$(current_milestone_number)"
    local ms_title
    ms_title="$(current_milestone_title)"
    local log_file="${LOG_DIR}/milestone-${ms_num}-attempt-${attempt}.log"

    divider
    info "Attempt ${attempt}/${MAX_ATTEMPTS} — Milestone ${ms_num}: ${ms_title}"

    # Show current tracker state (only Milestone Tracker rows)
    grep -E '^\| [0-9]+ \|' "${PROJECT_DIR}/${RUNBOOK}" \
      | grep -E '\`(not_started|in_progress|done)\`' \
      | while IFS= read -r row; do
      if echo "${row}" | grep -q '`done`'; then
        success "  ${row}"
      else
        warn "  ${row}"
      fi
    done
    divider

    local prompt
    prompt="$(build_prompt)"

    # Append retry hint after first attempt
    if (( attempt > 1 )); then
      local retry_cmds=""
      for cmd in "${BUILD_CMDS[@]+"${BUILD_CMDS[@]}"}"; do
        retry_cmds="${retry_cmds}
- Run \`${cmd}\` to check the current build state."
      done
      for cmd in "${TEST_CMDS[@]+"${TEST_CMDS[@]}"}"; do
        retry_cmds="${retry_cmds}
- Run \`${cmd}\` to check the current test state."
      done

      prompt="${prompt}

## RETRY CONTEXT — Attempt ${attempt}

A previous Copilot session did not fully complete a milestone. Please:
1. Check the current project state:${retry_cmds}
2. Read existing code and test files to understand what was already done.
3. Fix any issues and complete the next incomplete milestone.
4. Run all build and test commands to verify everything passes.
5. STOP after that one milestone is complete."
    fi

    # Log — mkdir defensively in case a previous Copilot session deleted the dir
    mkdir -p "${LOG_DIR}"
    echo "═══════════════════════════════════════════════════" >> "${log_file}"
    echo "[$(ts)] Attempt ${attempt} — Milestone ${ms_num}: ${ms_title}" >> "${log_file}"
    echo "═══════════════════════════════════════════════════" >> "${log_file}"

    # Invoke Copilot CLI
    local exit_code=0
    copilot -p "${prompt}" \
      --model "${MODEL}" \
      "${ALLOW_FLAGS[@]}" \
      "${DENY_FLAGS[@]}" \
      2>&1 | tee -a "${log_file}" || exit_code=$?

    # Re-create log dir in case Copilot deleted it during cleanup
    mkdir -p "${LOG_DIR}"
    echo "" >> "${log_file}"
    echo "[$(ts)] Exit code: ${exit_code}" >> "${log_file}"
    echo "[$(ts)] Milestone: ${ms_num} — ${ms_title}" >> "${log_file}"

    # Verify build + tests
    verify_commands "Build" "${log_file}" "${BUILD_CMDS[@]+"${BUILD_CMDS[@]}"}"
    verify_commands "Test" "${log_file}" "${TEST_CMDS[@]+"${TEST_CMDS[@]}"}"

    if (( attempt < MAX_ATTEMPTS )); then
      info "Cooling down ${COOLDOWN_SECS}s…"
      sleep "${COOLDOWN_SECS}"
    fi
  done

  local end_time
  end_time="$(date +%s)"
  local elapsed=$(( end_time - start_time ))
  local hours=$(( elapsed / 3600 ))
  local minutes=$(( (elapsed % 3600) / 60 ))

  echo ""
  header "Final Tracker State"
  grep -E '^\| [0-9]+ \|' "${PROJECT_DIR}/${RUNBOOK}" \
    | grep -E '\`(not_started|in_progress|done)\`' \
    | while IFS= read -r row; do
    if echo "${row}" | grep -q '`done`'; then
      success "  ${row}"
    else
      fail "  ${row}"
    fi
  done

  if all_milestones_done; then
    success "All milestones completed!"
  else
    fail "Not all milestones completed after ${MAX_ATTEMPTS} attempts."
  fi
  info "Total wall time: ${hours}h ${minutes}m"
}

main "$@"
