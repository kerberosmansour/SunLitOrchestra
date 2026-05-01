# Completion Summary — sap-imp Milestone 5

> **Runbook**: [docs/RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md](../../RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md)
> **Milestone**: M5 — Host-native agent-role experiment (gated on M4 host-capability matrix)
> **Decision**: GREEN-LIT — 4 agents shipped with Copilot fallback documented per the matrix.
> **Started**: 2026-05-01
> **Completed**: 2026-05-01

## Goal completed

Four host-native specialist agents under `agents/` provide an additive Claude-Code-only enhancement to the canonical portable critique flow. Each agent declares `output-paths` constrained to `{docs/slo/critique/, docs/slo/verify/}`; each documents a Copilot fallback (`/slo-critique` persona rotation or `/slo-verify` for the verification specialist). A structural-contract test enforces frontmatter, file count, output-path safety (path traversal + absolute paths rejected), Copilot-fallback presence, line cap, and `/slo-critique` SKILL.md byte-identical baseline.

## Files changed

- `agents/slo-runbook-review-lead.md` (NEW) — lead agent.
- `agents/slo-security-reviewer.md` (NEW) — security specialist.
- `agents/slo-design-reviewer.md` (NEW) — design specialist (UI-only; returns N/A for non-UI runbooks).
- `agents/slo-verification-lead.md` (NEW) — verification specialist (review-time + runtime modes).
- `xtasks/sast-verify/tests/sap_imp_m5_agents.rs` (NEW) — 7-test structural-contract test.
- `xtasks/sast-verify/Cargo.toml` — added `sha2 = "0.10"` to `[dev-dependencies]`.
- `docs/RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md` — milestone tracker updated.

## Tests added

`xtasks/sast-verify/tests/sap_imp_m5_agents.rs` (NEW) with 7 test functions:

- `agents_directory_passes_vacuously_or_strictly`
- `at_most_four_agent_files`
- `every_agent_has_required_frontmatter`
- `every_output_path_in_allowed_set` (per F-SEC-6)
- `copilot_fallback_documented`
- `agent_file_under_line_cap`
- `slo_critique_skill_md_unchanged` (per F-ENG-6 — SHA-256 baseline pin)

## Runtime validations added

- Test command: `cargo test -p sast-verify --test sap_imp_m5_agents` — green: `7 passed; 0 failed`.
- Tests run against actual `agents/<name>.md` files at HEAD plus the live `skills/slo-critique/SKILL.md` for SHA-256 comparison.

## Static analysis and formatter evidence

- `cargo fmt --all` — clean.
- `cargo build --workspace` — clean.
- `cargo test -p sast-verify --tests` — `34 passed; 0 failed` (gate_e2e + M1 + M2 + M3 + M4 + M5 = 34).
- `cargo test -p sldo-common -p sldo-install -p sldo-research` (runbook baseline) — green.

## Compatibility checks performed

- M1's `sap_imp_m1_citations` test still passes (verified via `cargo test -p sast-verify --tests`).
- M2's `sap_imp_m2_examples` test still passes.
- M3's `sap_imp_m3_standards` test still passes.
- M4's `sap_imp_m4_workflow_pinning` test still passes.
- `skills/slo-critique/SKILL.md` byte-identical to its pinned baseline (SHA-256: `8b1c1db40706f6168fa30c2d8cc44b9caa0356a2c1e5f7c7797f1b4e00184d65`).
- `sldo-install --dry-run` ignores `agents/` (installer walks `skills/<name>/SKILL.md` only).
- All shipped `skills/<name>/SKILL.md` files unchanged in M5.
- GitHub Copilot install path unchanged; documented fallback for each agent.

## Invariants/assertions added

7 invariants encoded:

1. Exactly 4 agent files in `agents/` matching `EXPECTED_AGENT_NAMES`.
2. Cap-of-4 enforced.
3. Frontmatter required fields present on every agent.
4. Every `output-paths` entry path-safe (no traversal, not absolute, in allowed prefix set).
5. Every `copilot-fallback` non-empty.
6. Every agent file ≤ 200 lines.
7. `/slo-critique` SKILL.md SHA-256 matches pinned constant (canonical portable path preserved).

## Resource bounds added or verified

- 4 agent files (cap-of-4 enforced).
- 200-line cap per agent file.
- 2 allowed output-path prefixes (`docs/slo/critique/`, `docs/slo/verify/`).
- 1 SHA-256 baseline pin for `/slo-critique` SKILL.md.

## Documentation updated

- `docs/RUNBOOK-SECURE-AGENT-PLAYBOOK-IMPORTS.md` — milestone tracker updated.
- `docs/ARCHITECTURE.md` "Agent roles" subsection — pending close-out.
- `docs/skill-pack-catalog.md` Agent roles subsection — pending close-out.
- `CLAUDE.md` and `copilot-instructions.md` overlay updates — pending close-out.

## .gitignore changes

None — `agents/` is committed; no generated outputs.

## Test artifact cleanup verified

`git status` clean after M5 work.

## Deferred follow-ups

- **`docs/ARCHITECTURE.md` Agent roles + Distribution channels subsections** — runbook close-out task.
- **`docs/skill-pack-catalog.md` Agent roles subsection** — close-out.
- **`CLAUDE.md` and `copilot-instructions.md` overlay updates** — close-out.
- **Agent-output schema validation** (F-ENG-7 deferred) — future micro task.
- **Cross-agent reference check** — verify lead's named-specialists exist as files. Lane: `micro`.
- **Runtime sandboxing of agent file writes** — currently structural-contract test enforces declaration; runtime enforcement depends on Claude Code agent SDK. Future runbook.

## Known non-blocking limitations

- **Agent-runtime orchestration** depends on Claude Code's agent SDK; the agent files are structurally correct, but the lead → specialist dispatch happens at runtime by host invocation, not by file-level wiring.
- **`output-paths` runtime enforcement** is host-dependent. The structural-contract declaration is a contract; runtime sandboxing is up to the host. Mitigation: the constraint is documented and structurally enforced at design-time.
- **`copilot-fallback` is documented prose**, not a runtime feature flag. Copilot users running `/slo-critique` get the same artifact format; the fallback is informational, not enforced. Acceptable given the canonical portable path is the actual fallback mechanism.
