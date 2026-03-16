#!/usr/bin/env bash
# =============================================================================
# run-milestones.sh — Drive GitHub Copilot CLI through RUNBOOK-UI milestones
#
# Loops until all milestones in the runbook are marked `done`.
# Each iteration tells Copilot to read the runbook and work on the next
# incomplete milestone. Verifies build + tests after each invocation.
# =============================================================================
set -euo pipefail

# ── Configuration ────────────────────────────────────────────────────────────
PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)"
RUNBOOK="docs/RUNBOOK-UI.md"
ARCHITECTURE="docs/ARCHITECTURE.md"
MAX_ATTEMPTS=150          # total Copilot invocations before giving up
COOLDOWN_SECS=5           # pause between retries
LOG_DIR="${PROJECT_DIR}/.copilot-logs"

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
      warn "You are on '${branch}'. The runbook recommends a feature branch."
    fi
  fi

  mkdir -p "${LOG_DIR}"
  success "Log directory: ${LOG_DIR}"
  echo ""
}

# ── Check if all milestones are done ─────────────────────────────────────────
all_milestones_done() {
  # Returns 0 (true) if every milestone row in the tracker has status `done`
  # Tracker rows look like: | 9 | Description | `done` | ...
  local not_done
  not_done=$(grep -E '^\| [0-9]+ \|' "${PROJECT_DIR}/${RUNBOOK}" \
    | grep -v '`done`' || true)
  [[ -z "${not_done}" ]]
}

# ── Build the prompt ─────────────────────────────────────────────────────────
build_prompt() {
  cat <<'PROMPT'
You are an expert Rust and TypeScript software engineer working on the SunLitDevOrchestrator project.
This project has a Rust backend (`sldo/`, `sldo-tauri/`) and a React+TypeScript frontend (`sldo-tauri/ui/`).

## YOUR TASK

1. Read the runbook at `docs/RUNBOOK-UI.md`. Look at the **Milestone Tracker** table.
2. Find the first milestone whose status is NOT `done` (i.e. `not_started` or `in_progress`).
3. Complete that milestone — and ONLY that milestone — following the runbook's Pre-Milestone Protocol, the milestone's Step-by-Step, BDD Acceptance Scenarios, E2E Runtime Validation, Smoke Tests, and Post-Milestone Protocol.
4. Read the architecture doc at `docs/ARCHITECTURE.md`.
5. If a lessons file exists for the previous milestone (check the Milestone Tracker), read it and apply any corrections.
6. Read the design reference files: `docs/SL_App.css` (Sunlit visual design language), `docs/SL_App.tsx` (Sunlit app structure) — these are read-only references for theming and layout.

## Workflow

- **Verify baseline**: Run `cargo test --workspace 2>&1 | tail -20` and `cargo build -p sldo` before changing anything. Also verify frontend: `cd sldo-tauri/ui && npm install && npm run build 2>&1 | tail -10`. Fix any failures first.
- **Update tracker**: Set the milestone status to `in_progress` and Started date to 2026-03-16.
- **Write tests FIRST**: Create BDD and E2E test files from the milestone's scenario tables. Backend tests go in `sldo/tests/` and `sldo-tauri/tests/`. Frontend tests go co-located or in `sldo-tauri/ui/src/e2e/`.
- **Implement**: Follow the Step-by-Step section. Make all tests pass.
- **Post-milestone**: Run smoke tests, write lessons-learned to `docs/lessons/sldo-ui-m<N>.md`, update tracker to `done`, update ARCHITECTURE.md and README.md per the Documentation Update table.
- **Final check**: Run `cargo test --workspace` AND `cd sldo-tauri/ui && npx vitest run` one last time — everything must be green. Also verify builds: `cd sldo-tauri/ui && npm run build && cd ../.. && cargo build -p sldo && cargo build -p sldo-tauri`.
- **STOP**: Do not proceed to the next milestone. Your session ends here.

## Hard rules
- Write BDD tests BEFORE production code.
- Do NOT skip any step in the Pre-Milestone or Post-Milestone protocols.
- Do NOT touch code or tests belonging to other milestones.
- All pre-existing tests must still pass (backend AND frontend).
- Follow existing code style and naming conventions.
- Do not commit secrets, API keys, or credentials.
- **TLA agents are very slow** (minutes to hours per invocation). All tests involving TLA/Phase 5 must use mock data (`MockPlanningLlm`, `DryRunAgent`, or canned responses) — NEVER invoke real TLA agents in tests. E2E tests that touch TLA should verify behavior at the library level with mocks, not by spawning the actual `tla-agent` binary.
- Do NOT work on any subsequent milestone.
- Do NOT delete the \`.sldo/\` directory or run \`rm -rf .sldo\`. It contains automation state, logs, and checkpoints. When cleaning up smoke test artifacts, only remove the specific test directories you created (e.g. \`output/smoke_*\`).
- Do NOT delete or modify any files in \`.copilot-logs/\` — the automation script writes its logs there.
- When running smoke tests that need a temporary repo, create it under \`output/\` (e.g. \`output/smoke_ui_m4/\`) and clean up only that directory afterward.
- Do NOT modify read-only reference files: `docs/SL_App.css`, `docs/SL_App.tsx`, `docs/runbook-template.md`.
- For frontend milestones (M4+), ensure `sldo-tauri/ui/package.json` dependencies are installed before building.
PROMPT
}

# ── Detect current milestone ─────────────────────────────────────────────────
current_milestone_number() {
  # Return the number of the first milestone whose status is NOT `done`
  grep -E '^\| [0-9]+ \|' "${PROJECT_DIR}/${RUNBOOK}" \
    | grep -v '\`done\`' \
    | head -1 \
    | sed 's/^| *\([0-9]*\) .*/\1/' \
    || echo "unknown"
}

current_milestone_title() {
  # Return the title of the first incomplete milestone
  grep -E '^\| [0-9]+ \|' "${PROJECT_DIR}/${RUNBOOK}" \
    | grep -v '\`done\`' \
    | head -1 \
    | awk -F'|' '{gsub(/^ +| +$/, "", $3); print $3}' \
    || echo "unknown"
}

# ── Main loop ────────────────────────────────────────────────────────────────
main() {
  header "SunLitDevOrchestrator UI — Automated Milestone Runner"
  info "Project:    ${PROJECT_DIR}"
  info "Runbook:    ${RUNBOOK}"
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

    # Show current tracker state
    grep -E '^\| [0-9]+ \|' "${PROJECT_DIR}/${RUNBOOK}" | while IFS= read -r row; do
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
      prompt="${prompt}

## RETRY CONTEXT — Attempt ${attempt}

A previous Copilot session did not fully complete a milestone. Please:
1. Run \`cargo build -p sldo\` and \`cargo test --workspace\` to see the current Rust state.
2. Run \`cd sldo-tauri/ui && npm install && npm run build\` to see the frontend state.
3. Read existing code and test files to understand what was already done.
4. Fix any issues and complete the next incomplete milestone.
5. Run \`cargo test --workspace\` AND \`cd sldo-tauri/ui && npx vitest run\` to verify everything passes.
6. STOP after that one milestone is complete."
    fi

    # Log — mkdir defensively in case a previous Copilot session deleted the dir
    mkdir -p "${LOG_DIR}"
    echo "═══════════════════════════════════════════════════" >> "${log_file}"
    echo "[$(ts)] Attempt ${attempt} — Milestone ${ms_num}: ${ms_title}" >> "${log_file}"
    echo "═══════════════════════════════════════════════════" >> "${log_file}"

    # Invoke Copilot CLI
    local exit_code=0
    copilot -p "${prompt}" \
      --model 'claude-opus-4.6' \
      "${ALLOW_FLAGS[@]}" \
      "${DENY_FLAGS[@]}" \
      2>&1 | tee -a "${log_file}" || exit_code=$?

    # Re-create log dir in case Copilot deleted it during cleanup
    mkdir -p "${LOG_DIR}"
    echo "" >> "${log_file}"
    echo "[$(ts)] Exit code: ${exit_code}" >> "${log_file}"
    echo "[$(ts)] Milestone: ${ms_num} — ${ms_title}" >> "${log_file}"

    # Verify build + tests
    info "Verifying Rust build…"
    if cargo build -p sldo 2>&1 | tee -a "${log_file}"; then
      success "sldo build OK"
    else
      warn "sldo build failed. Will retry."
    fi

    if cargo build -p sldo-tauri 2>&1 | tee -a "${log_file}"; then
      success "sldo-tauri build OK"
    else
      warn "sldo-tauri build failed. Will retry."
    fi

    info "Verifying frontend build…"
    if (cd "${PROJECT_DIR}/sldo-tauri/ui" && npm run build) 2>&1 | tee -a "${log_file}"; then
      success "Frontend build OK"
    else
      warn "Frontend build failed. Will retry."
    fi

    info "Verifying Rust tests…"
    if cargo test --workspace 2>&1 | tee -a "${log_file}"; then
      success "All Rust tests green"
    else
      warn "Rust tests failing. Will retry."
    fi

    info "Verifying frontend tests…"
    if (cd "${PROJECT_DIR}/sldo-tauri/ui" && npx vitest run) 2>&1 | tee -a "${log_file}"; then
      success "All frontend tests green"
    else
      warn "Frontend tests failing. Will retry."
    fi

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
  grep -E '^\| [0-9]+ \|' "${PROJECT_DIR}/${RUNBOOK}" | while IFS= read -r row; do
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
