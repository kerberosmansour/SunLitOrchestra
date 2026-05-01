# Lessons Learned — sap-imp Milestone 5

> **Runbook**: [docs/RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md](../../RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md)
> **Milestone**: M5 — Host-native agent-role experiment (green-lit per M4 matrix)
> **Date**: 2026-05-01

## What changed

- 4 new agent files at `agents/`:
  - `slo-runbook-review-lead.md` (output → `docs/slo/critique/`)
  - `slo-security-reviewer.md` (output → `docs/slo/critique/`)
  - `slo-design-reviewer.md` (output → `docs/slo/critique/`)
  - `slo-verification-lead.md` (output → `docs/slo/critique/` and `docs/slo/verify/`)
- New structural-contract test `xtasks/sast-verify/tests/sap_imp_m5_agents.rs` with 7 test functions enforcing file count cap, frontmatter, output-path safety (F-SEC-6 path-traversal rejection), Copilot-fallback presence, line cap, and `/slo-critique` SHA-256 byte-identical baseline (F-ENG-6).
- 1 new dev-dep: `sha2 = "0.10"` registered in `xtasks/sast-verify/Cargo.toml` `[dev-dependencies]`.

## Design decisions and why

- **Green-lit decision** per M4's host-capability matrix: Claude Code supports declarative agent files; GitHub Copilot has no equivalent and uses `/slo-critique` persona rotation as the documented fallback (per the matrix decision row).
- **Strict 4-agent cap** mirrors `/slo-critique`'s four-persona rotation. A 5th specialist would require adding a fifth persona to the portable path AND a fifth agent — coordinated change in a future runbook.
- **`output-paths` constrained to `{docs/slo/critique/, docs/slo/verify/}`** — same artifact paths the canonical portable path writes. Agent outputs are interchangeable with `/slo-critique` outputs; downstream readers (e.g., `/slo-execute` Step 1.5 carry-forward query, `/slo-retro` issue filing) don't need to know which path produced the artifact.
- **F-ENG-6 SHA-256 baseline pin** — `CRITIQUE_SKILL_SHA256` const in the test file, captured at M5 start. Updating it requires a runbook amendment. This makes "M5 must not modify `/slo-critique`" structurally-checkable, not just promised.
- **F-SEC-6 path canonicalization** — `Path::components()` rejects any `Component::ParentDir`; `Path::is_absolute()` rejects absolute paths. Strict prefix membership in addition.

## Assumptions verified

- All 4 agent files have valid YAML frontmatter with required fields.
- All 4 declared `output-paths` are subsets of the allowed set.
- All 4 `copilot-fallback` fields are non-empty (each names `/slo-critique` or `/slo-verify` as the fallback).
- All 4 files are well under the 200-line cap (largest is `slo-runbook-review-lead.md` at ~50 lines).
- `sha2 = "0.10"` is the latest stable; computes SHA-256 in 1 line.

## Assumptions still unresolved

- **Runtime enforcement of output-path constraint** — currently the structural-contract test enforces the *declaration*. At runtime, whether Claude Code's agent SDK actually sandboxes file writes to the declared paths depends on host implementation. The frontmatter declaration is a contract; runtime may need additional sandboxing in a future runbook.
- **Agent-output schema validation** — F-ENG-7's BDD scenario said "agent produces critique artifact whose format diverges → assertion fails". The current test asserts `output-paths` are constrained but does NOT validate the artifact's internal schema (table headers, persona-tagged rows). A future runbook can add the schema-validation step.
- **Lead → specialist orchestration** — the lead agent's prose says "dispatch the three specialists" but the actual orchestration mechanism depends on Claude Code's agent SDK. The agent files are correctly structured; orchestration happens at runtime.

## Mistakes made

- **None during M5 itself** — the TDD-first discipline + the prior-milestone lessons file pre-flight worked smoothly. The threat-model awareness from M3 + the path-safety pattern from M4 transferred directly.

## Root causes

- N/A.

## What was harder than expected

- **Scoping the agent prose to ≤ 200 lines** — initial drafts ran longer because each agent wanted to enumerate every edge case. Tightening to scope + discipline + output format made each file under 100 lines.
- **Choosing the right fallback wording** — `copilot-fallback: /slo-critique persona rotation (canonical portable path)` is more useful than `copilot-fallback: feature-flagged-out-of-Copilot` because it tells the user what TO DO, not what NOT to do.

## Invariants/assertions added or strengthened

- (a) `agents/` contains exactly 4 files matching `EXPECTED_AGENT_NAMES`.
- (b) Every agent file has frontmatter with `name`, `role`, `output-paths`, `copilot-fallback`, `host-required`.
- (c) Every `output-paths` entry, after `Path::components()` canonicalization, has prefix in `{docs/slo/critique/, docs/slo/verify/}` AND no `Component::ParentDir` AND not absolute.
- (d) Every `copilot-fallback` field is non-empty.
- (e) Every agent file ≤ 200 lines.
- (f) `skills/slo-critique/SKILL.md` SHA-256 matches the pinned constant `CRITIQUE_SKILL_SHA256`.

## Resource bounds established or verified

- 4 agent files, hardcoded in `EXPECTED_AGENT_NAMES`.
- 200-line cap per agent file.
- 2 allowed output-path prefixes.
- 1 `/slo-critique` SHA-256 baseline pin.

## Debugging / inspection notes

- The SHA-256 baseline pin captured 2026-05-01 was after M3's prose updates to `/slo-critique` (which added the standards-mapping citation + threshold rule). If the test fails in a future PR with "SHA-256 changed", it means someone modified `/slo-critique` SKILL.md — the right response is either revert the change or open a fresh runbook to update the baseline + re-pin.
- `Path::components()` correctly handles cases like `docs/slo/critique/../../etc/passwd` — the `Component::ParentDir` segment is yielded explicitly, so the test rejects it without needing to canonicalize against a real filesystem path.

## Naming conventions established

- Agent file naming: `agents/<name>.md` matching the `name:` frontmatter field.
- Agent roles: `lead | security-reviewer | design-reviewer | verification-lead`.
- Output-path prefix format: trailing slash (`docs/slo/critique/` not `docs/slo/critique`).

## Test patterns that worked well

- **SHA-256 hash baseline pin** — pulled with `shasum -a 256 <file>` and embedded as a const. Future drift fails CI with a clear message naming both expected and actual.
- **Per-agent loop with collect-failures pattern** — same as M2/M3/M4. The test surfaces ALL violating agents in one run rather than failing on the first.

## Missing tests that should exist now

- **Agent-output schema validation** (per F-ENG-7) — a future test that walks `docs/slo/critique/<slug>.md` and asserts the artifact contains the required sections (findings table header, persona-tagged rows, recommendation column). Lane: `micro`. Defer.
- **Cross-agent reference check** — the lead's prose says "dispatch slo-security-reviewer / slo-design-reviewer / slo-verification-lead". A test could parse the lead's prose for those names + assert each named agent file exists. Lane: `micro`.

## Rules for the next milestone

- **Runbook close-out remains** — `docs/ARCHITECTURE.md` updates, `.gitignore` `dist/` patterns, README cross-links all consolidated in one final commit.

## Template improvements suggested

- **The v4 template should mention `Path::components()` canonicalization** as the standard pattern for path-safety checks. Currently it's introduced ad-hoc per critique resolution; folding it into Section 4 (Carmack-Style Best Practices) would standardize.
- **The v4 template's Pre-flight could include "capture SHA-256 baselines for any file the milestone must NOT modify"** — applies to M5 and any future runbook where canonical-path preservation matters.
