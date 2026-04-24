# TLA+ SHA Auto-Populate — Runbook (v3)

> **Purpose**: Ship `sldo-tla-sha`, a small Rust helper that computes SHA-256 for pinned entries in `skills/slo-tla/tools.toml` and prints a patch.
> **Audience**: AI coding agents first, humans second.
> **How to use**: Work milestones sequentially, respect the Global Execution Rules from [runbook-template_v_3_template.md](runbook-template_v_3_template.md).
> **Prerequisite reading**: [design](design/tla-sha-autopop-overview.md), [research synthesis](research/tla-sha-autopop/synthesis.md), [idea](idea/tla-sha-autopop.md).

---

## Runbook Metadata

- **Runbook ID**: `tla-sha-autopop`
- **Prefix**: `tla-sha`
- **Primary stack**: Rust (workspace crate)
- **Default test command**: `cargo test -p sldo-tla-sha`
- **Public interfaces that must remain stable**:
  - `skills/slo-tla/tools.toml` schema (section names + field names)
- **Allowed new deps by default**: `none` — M1 already authorized `reqwest` + `sha2` for M5 of skill-pack runbook; this runbook reuses those.

## Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons | Completion |
|---|---|---|---|---|---|---|
| 1 | `sldo-tla-sha` binary: read tools.toml, fetch URLs, compute + print patch | `done` | 2026-04-24 | 2026-04-24 | [tla-sha-m1.md](lessons/tla-sha-m1.md) | [tla-sha-m1.md](completion/tla-sha-m1.md) |
| 2 | Add `--verify` subcommand that checks current SHAs against live URLs | `not_started` | | | | |

---

## TLA+ Section

**N/A** — see [design overview](design/tla-sha-autopop-overview.md) for justification. Single-process, no concurrency, no distributed state.

---

## Global Execution Rules, Entry Rules, Exit Rules

Follow [runbook-template_v_3_template.md](runbook-template_v_3_template.md) verbatim. No local overrides.

---

## Milestone 1 — `sldo-tla-sha` binary

**Goal**: Ship a new workspace binary that reads `skills/slo-tla/tools.toml`, fetches each URL whose `sha256 = "UNSET"`, computes SHA-256 from the streamed response, and prints a TOML patch to stdout that the user applies in a commit.

**Refactor budget**: `No refactor permitted beyond direct implementation`.

### Contract Block

| Field | Value |
|---|---|
| Inputs | CLI flag: `--tools-toml <path>` (default `skills/slo-tla/tools.toml`). `--dry-run` to print the planned fetches without executing. |
| Outputs | TOML patch on stdout. No file writes in this milestone. |
| Interfaces touched | New binary `sldo-tla-sha`. `tools.toml` is read-only. |
| Files allowed to change | `Cargo.toml` (add crate to workspace), `crates/sldo-tla-sha/**` (NEW), `CLAUDE.md` (add one-line reference). |
| Files to read before changing anything | `skills/slo-tla/tools.toml`, `crates/sldo-common/src/preflight.rs`, `crates/sldo-install/src/main.rs` (as a Rust-binary style reference). |
| New files allowed | `crates/sldo-tla-sha/Cargo.toml`, `crates/sldo-tla-sha/src/main.rs`, `crates/sldo-tla-sha/tests/e2e.rs`. |
| New dependencies allowed | `reqwest` (blocking, rustls-tls, default-features=false), `sha2`, `toml`. |
| Migration allowed | `no`. |
| Compatibility commitments | `tools.toml` schema unchanged; nothing else depends on this new binary yet. |
| Forbidden shortcuts | No file writes; no hardcoded URLs; no network calls in unit tests. |

### BDD Scenarios

Critique findings f1 and f2 accepted: `redirect_to_foreign_host_aborts` and `oversize_response_aborts` added.

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| prints_patch_for_unset | happy path | `tools.toml` has two entries with `sha256 = "UNSET"` | `sldo-tla-sha` runs | stdout is a diff-style patch with two replaced lines |
| skips_already_populated | happy path | One entry has a real SHA, one has UNSET | `sldo-tla-sha` runs | Patch includes only the UNSET entry |
| dry_run_no_network | happy path | `--dry-run` passed | Run | Prints "would fetch: `<url>`" lines, no HTTP requests issued |
| missing_tools_toml | invalid input | Path does not exist | Run | Exits non-zero with clear error |
| network_failure | dependency failure | URL returns 404 | Run | Aborts with error naming the failing URL; does not print partial patch |
| redirect_to_foreign_host_aborts | dependency failure (critique f1) | URL redirects to host not on allow-list | Run | Aborts; final host is rejected; no SHA computed |
| oversize_response_aborts | dependency failure (critique f2) | Response exceeds the max-bytes cap | Run | Streaming aborts at cap; error names the limit |

### E2E Runtime Validation

**File**: `crates/sldo-tla-sha/tests/e2e.rs`

| Test | Proves | Pass |
|---|---|---|
| `dry_run_prints_plan_no_fetch` | `--dry-run` exits 0 with planning output | stdout contains "would fetch" |
| `missing_file_errors_cleanly` | Missing tools.toml fails loud | non-zero exit + stderr names the path |

### Definition of Done

- All BDD scenarios pass.
- `cargo test -p sldo-tla-sha` is green.
- No file writes at runtime.
- README/CLAUDE.md updated with one-line reference.

---

## Milestone 2 — `--verify` subcommand

**Goal**: Add `sldo-tla-sha --verify` that re-fetches every URL and confirms the computed SHA matches the stored one. Output: green OK or red mismatch. Exit 0 on all-match, non-zero on any mismatch.

**Refactor budget**: `Minimal local refactor permitted in listed files only`.

### Contract Block

| Field | Value |
|---|---|
| Inputs | `--verify` flag. |
| Outputs | Per-package PASS/FAIL on stdout. Exit code reflects overall result. |
| Files allowed to change | `crates/sldo-tla-sha/src/main.rs`, `crates/sldo-tla-sha/tests/e2e.rs`. |
| Files to read before changing anything | M1's `main.rs`. |
| New files allowed | None. |
| New dependencies allowed | None. |
| Compatibility commitments | M1's default subcommand unchanged. |
| Forbidden shortcuts | No deleting or rewriting the M1 default subcommand. |

### BDD Scenarios

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| all_match | happy path | All SHAs in tools.toml match upstream | `--verify` runs | Exit 0, lines "PASS tlc", "PASS apalache" |
| one_mismatch | partial failure | One SHA was tampered | `--verify` runs | Exit non-zero, line "FAIL tlc: expected X, got Y" |
| unset_still_present | invalid input | An entry is still UNSET | `--verify` runs | Exit non-zero with hint: "run sldo-tla-sha without --verify first" |

### Definition of Done

- All BDD scenarios pass.
- `--verify` is idempotent and network-only read.
- No regression in M1's default behavior.

---

## Documentation Update Table

| Milestone | ARCHITECTURE.md | README.md | .gitignore | Other |
|---|---|---|---|---|
| 1 | Add `sldo-tla-sha` box to the skill-pack diagram | Add helper-CLI subsection | (none) | `CLAUDE.md` one-line addition |
| 2 | — | Mention `--verify` | (none) | — |
