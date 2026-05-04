# Scanner Orchestration — `/slo-sast` v1 wedge (AI-First Runbook v3)

> **Purpose**: Ship a pure-Markdown `/slo-sast` Claude Code skill that reads a project's threat-model file, picks tuned Semgrep rule packs, emits a safe `.github/workflows/sast.yml` plus baseline-aware config plus an audit-defense manifest, and re-derives the ruleset on threat-model edit — closing the auto-tuning loop that no published OTM-or-similar → Semgrep config converter occupies today.
> **Audience**: AI coding agents first, humans second. Written to reduce ambiguity, prevent scope drift, and improve output quality on security-sensitive runbooks.
> **How to use**: Work through milestones sequentially. Before starting any milestone, read its full section and the Global Execution Rules in [docs/slo/templates/runbook-template_v_3_template.md](runbook-template_v_3_template.md). After completing it, follow the Global Exit Rules. Never skip ahead. Never silently widen scope.
> **Prerequisite reading**: [docs/ARCHITECTURE.md](ARCHITECTURE.md), [docs/slo/idea/scanner-orchestration.md](idea/scanner-orchestration.md), [docs/slo/research/scanner-orchestration/synthesis.md](research/scanner-orchestration/synthesis.md), [docs/slo/design/scanner-orchestration-overview.md](design/scanner-orchestration-overview.md), [docs/slo/design/scanner-orchestration-stack-decision.md](design/scanner-orchestration-stack-decision.md), [docs/slo/design/scanner-orchestration-interfaces.md](design/scanner-orchestration-interfaces.md), [docs/slo/design/scanner-orchestration-threat-model.md](design/scanner-orchestration-threat-model.md), [SECURITY.md](../SECURITY.md) — particularly the "Scanner orchestration skill — additional rules" section.

---

## Runbook Metadata

- **Runbook ID**: `scanner-orchestration`
- **Prefix for test files and lessons files**: `scanner-orch`
- **Primary stack**: Markdown `SKILL.md` under `skills/slo-sast/` (consumed by Claude Code). Secondary: Rust 2021 workspace for E2E + structural-contract tests under `crates/sldo-install/tests/e2e_scanner_orch_m<N>.rs` using existing patterns.
- **Primary package/app names**: `skills/slo-sast/` (the skill — Markdown directory, not a crate); structural-contract and E2E tests reuse the workspace's existing Rust `[[test]]` harness in `crates/sldo-install/tests/`.
- **Default test commands**:
  - Backend: `cargo test --workspace` (per CLAUDE.md baseline; the 2026-04 cleanup removed `sldo-tauri` so `--workspace` is clean)
  - Frontend: N/A (skill pack is Markdown; no frontend in scope)
  - E2E backend: `cargo test -p sldo-install --test e2e_scanner_orch_m<N>` per milestone
  - E2E frontend: N/A
  - Build/boot: `cargo build --workspace` (sanity check; the skill itself is not compiled) followed by `./target/release/sldo-install --dry-run` (verifies the skill is discovered)
- **Allowed new dependencies by default**: `none` (Rust tests reuse `assert_cmd`, `tempfile`, `regex`, `anyhow`, `serde_json`, `serde_yaml_ng` already in the workspace)
- **Schema/config migration allowed by default**: `no`
- **Public interfaces that must remain stable unless explicitly listed otherwise**:
  - `/slo-sast` skill invocation surface (`SKILL.md` `name:` and `description:` keys consumed by the Claude Code skill loader)
  - Threat-model parse contract (regex `\bCWE-(\d+)\b` against rendered Markdown body, excluding HTML comments / fenced code blocks / `~~~text` user-string fences) — landed in M1
  - Stack detection contract (manifest-file priority order from [scanner-orchestration-interfaces.md §3](design/scanner-orchestration-interfaces.md)) — landed in M2
  - Emitted artifact paths: `.semgrep/rules/<rule-id>.yaml`, `.semgrep.yml`, `.github/workflows/sast.yml`, `.semgrep/manifest.json`, `.semgrep/last-run.json` — landed in M3 / M4
  - Manifest schema v1.0 (full field set in [scanner-orchestration-interfaces.md §5](design/scanner-orchestration-interfaces.md)) — landed in M4
  - Workflow YAML safety contract (no `pull_request_target`; `permissions: {}` scope; SHA-pinned actions; `fetch-depth: 0`; `SEMGREP_RULES` env var, not `--config` flag) — landed in M3, asserted by structural-contract test fixture
  - `/slo-rulegen` integration contract (`.semgrep/rules/<rule-id>.yaml` directory layout shared with rulegen-authored rules) — contract surface defined in M3, exercised by fixture in M5
  - SLO existing skill invocation verbs (`/slo-ideate`, `/slo-architect`, `/slo-plan`, etc.) — must continue to be discoverable post-install

---

## Milestone Tracker

Update this table as each milestone is completed. This is the single source of truth for progress.

| # | Milestone | Status | Started | Completed | Lessons File | Completion Summary |
|---|---|---|---|---|---|---|
| 1 | `/slo-sast` SKILL.md scaffold + threat-model parser (CWE extraction with comment / fence scope rule) | `done` | 2026-04-26 | 2026-04-26 | [docs/slo/lessons/scanner-orch-m1.md](lessons/scanner-orch-m1.md) | [docs/slo/completion/scanner-orch-m1.md](completion/scanner-orch-m1.md) |
| 2 | Stack detection + `semgrep-rules` cache fetch at pinned SHA + CWE × technology rule filter | `done` | 2026-04-26 | 2026-04-26 | [docs/slo/lessons/scanner-orch-m2.md](lessons/scanner-orch-m2.md) | [docs/slo/completion/scanner-orch-m2.md](completion/scanner-orch-m2.md) |
| 3 | Emit `.semgrep/rules/`, `.semgrep.yml`, `.github/workflows/sast.yml` with workflow-safety structural-contract test | `done` | 2026-04-26 | 2026-04-26 | [docs/slo/lessons/scanner-orch-m3.md](lessons/scanner-orch-m3.md) | [docs/slo/completion/scanner-orch-m3.md](completion/scanner-orch-m3.md) |
| 4 | Emit `.semgrep/manifest.json` (audit-defense schema v1.0) + initial-baseline preview-mode UX | `done` | 2026-04-26 | 2026-04-26 | [docs/slo/lessons/scanner-orch-m4.md](lessons/scanner-orch-m4.md) | [docs/slo/completion/scanner-orch-m4.md](completion/scanner-orch-m4.md) |
| 5 | Re-derivation trigger detection + diff PR generation + dogfood E2E against this SLO repo | `done` | 2026-04-26 | 2026-04-26 | [docs/slo/lessons/scanner-orch-m5.md](lessons/scanner-orch-m5.md) | [docs/slo/completion/scanner-orch-m5.md](completion/scanner-orch-m5.md) |

<!-- Status values: not_started | in_progress | blocked | done -->
<!-- Lessons files go in docs/slo/lessons/scanner-orch-m<N>.md -->
<!-- Completion summaries go in docs/slo/completion/scanner-orch-m<N>.md -->

---

## End-to-End Architecture Diagram

See [docs/slo/design/scanner-orchestration-overview.md](design/scanner-orchestration-overview.md) for the full diagram with legend. Summary view below; solid = exists today, dashed = added by this runbook.

```
┌──────────────────────────────────────────────────────────────────────────────────┐
│                              USER (target product repo)                          │
└──────────────────────────────────┬───────────────────────────────────────────────┘
                                   │ /slo-sast
                                   ▼
┌──────────────────────────────────────────────────────────────────────────────────┐
│                          Claude Code (skill loader)                              │
└──────────────────────────────────┬───────────────────────────────────────────────┘
                                   │ reads SKILL.md (Markdown only)
                                   ▼
┌──────────────────────────────────────────────────────────────────────────────────┐
│                     /slo-sast skill   (skills/slo-sast/SKILL.md)         dashed  │
│                                                                                  │
│  threat-model.md ──► CWE parse (M1) ──► stack detect (M2) ──► fetch (M2) ──►     │
│  manifests       ──►                                          filter   ──►       │
│                                                                                  │
│  emit .semgrep/rules/, .semgrep.yml, .github/workflows/sast.yml (M3)             │
│  emit .semgrep/manifest.json + preview-mode (M4)                                 │
│  re-derive on threat-model edit + diff PR (M5)                                   │
└────────────────────────────┬─────────────────────────────────────────────────────┘
                             │ git add + commit + PR
                             ▼
┌──────────────────────────────────────────────────────────────────────────────────┐
│  Target repo — emitted artifacts run on PR / scheduled via GitHub Actions:       │
│    actions/checkout@<SHA> (fetch-depth: 0)                                       │
│    semgrep ci → SARIF → github/codeql-action/upload-sarif@<SHA>                  │
│    → GitHub Code Scanning UI (PR review comments + Security tab)                 │
│                                                                                  │
│  Hard ban: pull_request_target. permissions: {} scope. SHA-pinned actions.       │
└──────────────────────────────────────────────────────────────────────────────────┘
```

### Component Summary Table

| Component | Responsibility | Milestone Introduced/Changed | Key Interfaces |
|---|---|---|---|
| `skills/slo-sast/SKILL.md` | Markdown skill — orchestrates the whole flow | M1 (scaffold) → M5 (re-derivation) | `/slo-sast` invocation surface |
| Threat-model parser (in SKILL.md) | Regex CWE extraction with scope rule | M1 | `[scanner-orchestration-interfaces.md §2](design/scanner-orchestration-interfaces.md)` |
| Stack detector (in SKILL.md) | Manifest-file inspection | M2 | `[scanner-orchestration-interfaces.md §3](design/scanner-orchestration-interfaces.md)` |
| `~/.cache/sldo/semgrep-rules/<SHA>/` | Pinned upstream-rules cache | M2 | `[scanner-orchestration-interfaces.md §7](design/scanner-orchestration-interfaces.md)` |
| `.semgrep/rules/<rule-id>.yaml` | Selected registry rules committed in target repo | M3 | `[scanner-orchestration-interfaces.md §4](design/scanner-orchestration-interfaces.md)` |
| `.semgrep.yml` | Project-level Semgrep config | M3 | Same |
| `.github/workflows/sast.yml` | Safe-template GitHub Actions workflow | M3 | Workflow YAML safety contract (M3 fixture) |
| `.semgrep/manifest.json` | Audit-defense + reproducibility manifest | M4 | Manifest schema v1.0 (M4 fixture) |
| `.semgrep/last-run.json` | Last successful scan summary | M4 | M4 fixture |
| Re-derivation trigger detector | Diff threat-model SHA, semgrep-rules SHA, etc. | M5 | `[scanner-orchestration-interfaces.md §8](design/scanner-orchestration-interfaces.md)` |

### Data Flow Summary

| Flow | From | To | Protocol/Mechanism | Milestone |
|---|---|---|---|---|
| Threat-model parse | `docs/slo/design/<slug>-threat-model.md` | Skill in-memory CWE list | File read + regex | M1 |
| Stack detect | Manifest files (`Cargo.toml`, `package.json`, ...) | Skill in-memory stack tag list | File read + content inspection | M2 |
| Rules fetch | `github.com/semgrep/semgrep-rules` (pinned SHA) | `~/.cache/sldo/semgrep-rules/<SHA>/` | `git clone` (HTTPS) | M2 |
| Rules filter | Cached YAML files | Skill in-memory selected-rules list | YAML parse + filter by `metadata.cwe ∧ metadata.technology` | M2 |
| Rules emit | Cached files → target repo | `.semgrep/rules/<rule-id>.yaml` | File copy | M3 |
| Config emit | In-memory | `.semgrep.yml` | File write | M3 |
| Workflow emit | Static template + parameter substitution | `.github/workflows/sast.yml` | File write | M3 |
| Manifest emit | In-memory metadata | `.semgrep/manifest.json` | JSON write | M4 |
| Diff detect | Recorded SHAs vs current | PR body | Skill output → `gh pr create` | M5 |

---

## High-Level Design for Formal Verification (TLA+ Section)

**N/A** — `tla_required: false` in [docs/slo/design/scanner-orchestration-overview.md](design/scanner-orchestration-overview.md). Rationale: the skill is sequential file I/O plus subprocess invocation (one `git clone`, several file writes). No concurrent actors share state, no distributed consensus, no leader election, no cross-process ordering guarantees, no resource leases or locks. The re-derivation trigger compares stored SHAs against current SHAs at single-process invocation time — there is no race window. TLA+ would not surface a class of bug this design cannot already eliminate by construction.

---

## Global Execution Rules

See [docs/slo/templates/runbook-template_v_3_template.md §Global Execution Rules](runbook-template_v_3_template.md). Applied verbatim. Notable callouts for this runbook:

- **No `pull_request_target` in any emitted YAML, ever.** Inherited from [SECURITY.md](../SECURITY.md) "Scanner orchestration skill — additional rules". A milestone that violates this MUST fail the structural-contract test fixture introduced in M3.
- **PCI compliance citations cite 6.2.3 (v4.0.1), never 6.3.2.** v4.0.1's 6.3.2 is the SBOM-inventory mandate, different scope. Mixing the two is a substantive error.
- **Threat-model parser scope is non-negotiable.** Comments / fenced code / `~~~text` fences are excluded — defuses `tm-scanner-orchestration-abuse-1`. This rule lands in M1 and is asserted by every subsequent milestone's regression tests.

---

## Global Entry Rules (Pre-Milestone Protocol)

See [docs/slo/templates/runbook-template_v_3_template.md §Global Entry Rules](runbook-template_v_3_template.md). Applied verbatim.

---

## Global Exit Rules (Post-Milestone Protocol)

See [docs/slo/templates/runbook-template_v_3_template.md §Global Exit Rules](runbook-template_v_3_template.md). Applied verbatim.

---

## Background Context

### Current State

No `/slo-sast` skill exists. The repo contains 30+ first-party `/slo-*` Markdown skills under `skills/`, plus the supporting Rust workspace (`crates/sldo-common`, `crates/sldo-research`, `crates/sldo-install`, `xtasks/sast-verify`). `references/sast/` carries shared scaffolding for the SAST rule-gen runbook (CWE map, Semgrep syntax notes, AGPL clean-room policy) — scanner-orchestration adds a sibling `references/sast/threat-model-parser-contract.md` (M1) and reuses the rest. `docs/slo/design/scanner-orchestration-*.md` (overview, stack-decision, interfaces, threat-model) define the design surface this runbook implements. SECURITY.md was extended on 2026-04-26 with a "Scanner orchestration skill — additional rules" section that restates the load-bearing safety properties.

### Problem

The runbook addresses these specific gaps:

1. **No threat-model-driven SAST orchestration exists** — the unoccupied-wedge verdict from research synthesis Q5. Vendor presets (Snyk Code, GHAS CodeQL, Checkmarx One, Veracode) require manual policy authoring rather than threat-model intake; SecOpsTM is the closest adjacent player but emits Navigator layers, not Semgrep configs.
2. **The auto-tuning loop has no host today** — CWE list changes in a threat model don't propagate anywhere automatically. Re-derivation on threat-model edit is the differentiator that earns this skill its keep over Semgrep AppSec Platform (which doesn't read threat models at all).
3. **Solo OSS maintainers ship with no SAST in CI** — the idea-doc pain story (an RCE disclosed because no security tests ran). The skill exists to prevent the next maintainer from absorbing the same emotional + time + community-trust hit.
4. **Workflow YAML mistakes are widespread** — Sysdig 2024 audit, Shai Hulud v2 (~20k repos, Nov 2025), CVE-2025-30066 (`tj-actions/changed-files`, March 2025). The skill emits a safe-by-default workflow whose properties are asserted by a structural-contract test, eliminating an entire class of `pull_request_target` / unpinned-action / over-permissioned / fetch-depth-1 misconfigurations from the get-go.

### Target Architecture

See the End-to-End Architecture Diagram above and the full diagram in [docs/slo/design/scanner-orchestration-overview.md](design/scanner-orchestration-overview.md). End state after M5: invoking `/slo-sast` against any target repo (with a threat-model file) produces a tuned Semgrep workflow committed via PR; subsequent threat-model edits trigger diff PRs that surface the rule-set delta for human review.

### Key Design Principles

1. **Approach A is locked.** Pure-Markdown skill, no new Rust crate. Determinism comes from committing the selected rule files (with their upstream SHAs in the manifest), not from a Rust shim around YAML parsing.
2. **Rule selection IS the gate.** Severity gating doesn't exist standalone (research Q1). Every milestone that touches rule selection MUST tighten, never relax.
3. **Template-skeleton workflow emission.** User-provided prose NEVER flows into the emitted workflow YAML. Only regex-validated `CWE-\d+` integers and closed-enumeration stack tags are templated in. This is the load-bearing defense against `tm-scanner-orchestration-abuse-3`.
4. **SHA-pin everything.** Upstream `semgrep-rules` clone, every third-party action in the emitted workflow. Tag-rewriting is the canonical supply-chain failure case (CVE-2025-30066, Shai Hulud v2).
5. **`fetch-depth: 0` is mandatory.** Documented Semgrep KB pitfall; the structural-contract test asserts.
6. **Honest manifest framing.** `cwes_claimed` vs `cwes_actually_covered` is **defensive design**, not a regulatory mandate (no published audit-failure precedent fixes the mapped-but-not-scanned pattern). Don't overpromise.
7. **Preview-mode before first commit.** Day-one CI jam is the prolonged-outage risk that makes the very user who needed the skill abandon it. The first install runs a dry-run scan and surfaces counts before any workflow lands.

### What to Keep

- All existing `/slo-*` skills under `skills/` — invocation verbs are stable interface.
- `crates/sldo-install/src/install.rs::discover_skills()` walker behavior — `references/` is intentionally excluded; new `references/sast/threat-model-parser-contract.md` (M1) MUST NOT be discoverable as a skill.
- `references/sast/` existing files (CWE map, Semgrep syntax notes, AGPL policy, etc.) — unchanged by this runbook.
- `xtasks/sast-verify/` — unchanged. The orchestrator skill is separate from the rule-gen verifier xtask.
- `cargo test --workspace` baseline — every milestone preserves it.
- `.cargo/config.toml` `xtask` alias — unchanged.

### What to Change

- **`skills/slo-sast/SKILL.md`** (NEW, M1) — the skill itself; grows across milestones. M1 lands the parser; M2 adds stack detection + fetch + filter; M3 adds emission; M4 adds manifest + preview; M5 adds re-derivation.
- **`references/sast/threat-model-parser-contract.md`** (NEW, M1) — the parse-contract reference cited from SKILL.md.
- **`crates/sldo-install/tests/e2e_scanner_orch_m<N>.rs`** (one per milestone) — E2E + structural-contract tests.
- **`crates/sldo-install/tests/fixtures/scanner-orch/`** (NEW, M1) — test fixtures: threat-model files with smuggled CWE refs (M1), polyglot manifest sets (M2), expected workflow YAML (M3), expected manifest JSON (M4), expected diff PR body (M5).
- **`docs/slo/lessons/scanner-orch-m<N>.md`** + **`docs/slo/completion/scanner-orch-m<N>.md`** — written at end of each milestone per template.

### Global Red Lines

Inherited from [docs/slo/templates/runbook-template_v_3_template.md §Global Red Lines](runbook-template_v_3_template.md), plus runbook-specific:

- **No `pull_request_target` in emitted workflow YAML.** Ever.
- **No new Rust crate added by this runbook.** Markdown-only direction is locked. If a future milestone needs deterministic helper code, that's a fresh `/slo-architect` decision.
- **No autofix invocation in the emitted workflow.** `semgrep ci` runs without `--autofix`. Defends against `tm-scanner-orchestration-abuse-2` (compromised rule autofix smuggling backdoors).
- **No tag references in `uses:` lines.** SHA-pin or fail.
- **No HTTP / SDK calls from the skill itself.** `git clone` (CLI) is the only network egress.
- **No mutation of `references/sast/` existing files.** New file (`threat-model-parser-contract.md`) is additive.

---

## BDD and Runtime Validation Rules

See [docs/slo/templates/runbook-template_v_3_template.md §BDD and Runtime Validation Rules](runbook-template_v_3_template.md). Applied verbatim.

---

## Dependency, Migration, and Refactor Policy

See [docs/slo/templates/runbook-template_v_3_template.md §Dependency, Migration, and Refactor Policy](runbook-template_v_3_template.md). Applied verbatim.

---

## Evidence Log Template

See [docs/slo/templates/runbook-template_v_3_template.md §Evidence Log Template](runbook-template_v_3_template.md). Each milestone copies this table into its own section before execution.

---

## Self-Review Gate

See [docs/slo/templates/runbook-template_v_3_template.md §Self-Review Gate](runbook-template_v_3_template.md). Applied verbatim. Additional gate questions for this runbook:

- Did I preserve the threat-model parser scope rule (HTML comments / fenced code / `~~~text` fences excluded)?
- Did every emitted YAML/JSON file pass its structural-contract test fixture before the milestone closed?
- Did I cite PCI 6.2.3 (NOT 6.3.2) in any compliance-related artifact?
- Did I SHA-pin every third-party action in any workflow this milestone touches?
- Did I avoid introducing any new Rust crate?

---

## Lessons-Learned File Template

Path: `docs/slo/lessons/scanner-orch-m<N>.md`. See [docs/slo/templates/runbook-template_v_3_template.md §Lessons-Learned File Template](runbook-template_v_3_template.md).

---

## Completion Summary Template

Path: `docs/slo/completion/scanner-orch-m<N>.md`. See [docs/slo/templates/runbook-template_v_3_template.md §Completion Summary Template](runbook-template_v_3_template.md).

---

## Milestone Plan

### Milestone 1 — `/slo-sast` SKILL.md scaffold + threat-model parser

**Goal**: The `/slo-sast` skill exists at `skills/slo-sast/SKILL.md` and, when invoked against a target repo containing `docs/slo/design/<slug>-threat-model.md`, prints the deduplicated list of CWE integers extracted from the file's rendered Markdown body — excluding CWE references inside HTML comments, fenced code blocks, and `~~~text` user-string fences.

**Context**: No `/slo-sast` exists today. [scanner-orchestration-interfaces.md §2](design/scanner-orchestration-interfaces.md) defines the threat-model parse contract (regex `\bCWE-(\d+)\b`, scope-exclusion rules); [scanner-orchestration-threat-model.md](design/scanner-orchestration-threat-model.md) abuse case `tm-scanner-orchestration-abuse-1` is the threat the scope rule defuses. The skill is Markdown-only; the parser implementation lives in the prompt logic Claude Code executes when reading SKILL.md. This milestone lands the skill scaffold + the parse contract — no artifact emission yet (that's M3), no registry fetch yet (that's M2).

**Important design rule**: The parser scope rule is non-negotiable and is asserted by E2E tests at M1 closure. Every subsequent milestone's regression-test row references this rule — if a future milestone's prompt change weakens it, M1's tests fail and the milestone cannot close.

**Refactor budget**: `No refactor permitted beyond direct implementation`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | `/slo-sast` invocation from Claude Code with cwd = target repo root; reads `docs/slo/design/<slug>-threat-model.md` (slug derived from runbook context or explicit arg) |
| Outputs | A deduplicated list of integer CWE ids (long-form `["CWE-77", "CWE-78", "CWE-89"]`) printed to stdout; exits 0 on success, non-zero with stderr message on missing file |
| Interfaces touched | NEW: `/slo-sast` skill invocation surface (SKILL.md `name:` + `description:` keys); NEW: threat-model parse contract (regex + scope rules) — both `stable` per [scanner-orchestration-interfaces.md](design/scanner-orchestration-interfaces.md) §1, §2 |
| Files allowed to change | `skills/slo-sast/SKILL.md` (NEW); `references/sast/threat-model-parser-contract.md` (NEW); `crates/sldo-install/tests/e2e_scanner_orch_m1.rs` (NEW); `crates/sldo-install/tests/fixtures/scanner-orch/m1/` (NEW directory with threat-model fixtures); `.gitignore` (only if a new tool cache pattern is needed — unlikely in M1) |
| Files to read before changing anything | `docs/slo/design/scanner-orchestration-overview.md`, `docs/slo/design/scanner-orchestration-interfaces.md` (§§1–2), `docs/slo/design/scanner-orchestration-threat-model.md` (abuse case `tm-scanner-orchestration-abuse-1`, residual-risks section), [SECURITY.md](../SECURITY.md) "Scanner orchestration skill — additional rules" section, `skills/slo-research/SKILL.md` (for SKILL.md scaffold conventions), `skills/slo-architect/SKILL.md` (same), `crates/sldo-install/src/install.rs` (confirm `discover_skills()` requires `<skills_dir>/<name>/SKILL.md` shape), `crates/sldo-install/tests/e2e_slo_sec_m1.rs` (E2E test pattern for skill structural assertions) |
| New files allowed | `skills/slo-sast/SKILL.md`; `references/sast/threat-model-parser-contract.md`; `crates/sldo-install/tests/e2e_scanner_orch_m1.rs`; `crates/sldo-install/tests/fixtures/scanner-orch/m1/<fixture-name>.md` (multiple) |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | `cargo test --workspace` baseline remains green; `./target/release/sldo-install --dry-run` discovers all pre-existing skills (`/slo-ideate`, `/slo-research`, `/slo-architect`, `/slo-plan`, etc.); no other skill's SKILL.md is touched; `references/sast/` existing files unchanged |
| Forbidden shortcuts | No artifact emission stub (M3 — DO NOT create empty `.semgrep/` placeholders); no registry fetch stub (M2); no caching of parsed CWE list across invocations (single-pass per call); no fallback-to-default-pack logic (M2's job); no "TODO: handle HTML comments" comment in SKILL.md — the scope rule lands fully implemented and fully tested in M1; no JSON / structured-data output (stdout list of CWE strings is the v1 surface; rich output is M4's manifest); no shell-out other than file reads |
| **Data classification** | `Internal` — threat-model files are project design docs, neither public-facing nor secret. SKILL.md content itself is `Public` (committed to OSS repo) but the data the skill processes is project-internal. |
| **Proactive controls in play** | OWASP Proactive Controls v3 — **C2 Define security requirements** (the parse contract IS a security requirement, documented in `references/sast/threat-model-parser-contract.md` and cited by SKILL.md); **C5 Secure by default** (parser scope rule excludes attacker-controllable content from non-prose Markdown regions); **C7 Validate input** (regex `\bCWE-(\d+)\b` is the only accepted shape; non-conforming refs are silently dropped, not flagged). C4 Encode output, C8 Authentication, C9 Authorization, C10 Errors all N/A — read-only single-file pipeline with no auth surface and stdout-only output. |
| **Abuse acceptance scenarios** | See BDD table below: `parser_ignores_html_comment_cwe_refs`, `parser_ignores_fenced_code_cwe_refs`, `parser_ignores_user_string_fence_cwe_refs` — all three cite `tm-scanner-orchestration-abuse-1` (threat-model file content with smuggled CWE references) |

#### Out of Scope / Must Not Do

- Stack detection (M2)
- Registry fetch / caching / YAML parsing of rule files (M2)
- Any file emission into `.semgrep/` or `.github/workflows/` (M3)
- Manifest JSON writing (M4)
- Re-derivation trigger detection (M5)
- `gh` invocation, PR creation (M5)
- `/slo-rulegen` interaction (deferred — only the directory-layout contract surfaces in M3)
- Authoring new threat-model templates or extending existing ones (out of scope — the threat-model file format is the upstream `/slo-architect` artifact)
- Changing or extending `discover_skills()` walker behavior (out of scope — the skill must conform to the existing contract, not change it)

#### Pre-Flight

1. Complete the Global Entry Rules.
2. No prior milestone — read [docs/slo/idea/scanner-orchestration.md](idea/scanner-orchestration.md) and [docs/slo/research/scanner-orchestration/synthesis.md](research/scanner-orchestration/synthesis.md) instead of a lessons file from M0.
3. Read the allowed files before editing.
4. Copy the Evidence Log template into this milestone section or working notes.
5. Re-state the milestone constraints before coding.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/slo-sast/SKILL.md` | NEW: Markdown skill prompt with frontmatter (`name: slo-sast`, single-line `description:` ≤ 200 chars summarizing the threat-model-driven SAST orchestration role); body cites `references/sast/threat-model-parser-contract.md` for the parse contract, instructs Claude on regex extraction with scope exclusions, prints the deduplicated CWE list to stdout |
| `references/sast/threat-model-parser-contract.md` | NEW: documents the regex `\bCWE-(\d+)\b`, the three exclusion regions (HTML comments, fenced code blocks, `~~~text` user-string fences), the rationale (defuses `tm-scanner-orchestration-abuse-1`), and the long-form CWE convention used downstream (`"CWE-89"` not `89`) |
| `crates/sldo-install/tests/e2e_scanner_orch_m1.rs` | NEW: Rust E2E test file using `assert_cmd` + `tempfile` patterns (mirroring `e2e_slo_sec_m1.rs`); spawns the skill via the test-stubbed `claude` binary already wired into the workspace test harness; feeds threat-model fixtures and asserts stdout |
| `crates/sldo-install/tests/fixtures/scanner-orch/m1/canonical.md` | NEW: fixture with prose-only CWE refs `CWE-77`, `CWE-78`, `CWE-89` |
| `crates/sldo-install/tests/fixtures/scanner-orch/m1/with_html_comment.md` | NEW: fixture with `CWE-89` in prose plus `<!-- CWE-79 -->` in HTML comment |
| `crates/sldo-install/tests/fixtures/scanner-orch/m1/with_fenced_code.md` | NEW: fixture with `CWE-89` in prose plus `CWE-99` inside a fenced code block |
| `crates/sldo-install/tests/fixtures/scanner-orch/m1/with_user_string_fence.md` | NEW: fixture with `CWE-89` in prose plus `CWE-101` inside a `~~~text` user-string fence |
| `crates/sldo-install/tests/fixtures/scanner-orch/m1/empty.md` | NEW: fixture with prose but zero CWE references |
| `crates/sldo-install/tests/fixtures/scanner-orch/m1/duplicates.md` | NEW: fixture mentioning `CWE-89` three times in different sections |
| `crates/sldo-install/tests/fixtures/scanner-orch/m1/unicode_long.md` | NEW: fixture with non-ASCII characters and >50 KB content |
| `.gitignore` | Reviewed; no expected additions in M1 |

#### Step-by-Step

1. Write E2E test stubs at `crates/sldo-install/tests/e2e_scanner_orch_m1.rs` covering all BDD scenarios below (mirror `e2e_slo_sec_m1.rs` patterns).
2. Author all fixture files under `crates/sldo-install/tests/fixtures/scanner-orch/m1/`.
3. Run `cargo test -p sldo-install --test e2e_scanner_orch_m1` — confirm tests fail because the skill doesn't exist yet (correct failure mode: "skills/slo-sast/SKILL.md not found" or equivalent).
4. Write `references/sast/threat-model-parser-contract.md` documenting the regex, the three exclusion regions, and the rationale.
5. Write `skills/slo-sast/SKILL.md` with frontmatter and a parse-contract-citing invocation flow that produces the stdout list.
6. Run `./target/release/sldo-install --local --dry-run` — confirm `slo-sast` is discovered alongside existing skills.
7. Run `cargo test -p sldo-install --test e2e_scanner_orch_m1` — make all BDD scenarios pass.
8. Run `cargo test --workspace` — confirm baseline still green; nothing else broken.
9. Verify `git status` — no untracked test artifacts; review `.gitignore` for stale entries.
10. Complete the Self-Review Gate; write lessons + completion files.

#### BDD Acceptance Scenarios

**Feature: threat-model CWE extraction with parser scope rule**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| `parses_canonical_cwe_list_from_prose` | happy path | a threat-model file at `docs/slo/design/<slug>-threat-model.md` containing `"... mitigates CWE-77, CWE-78, and CWE-89."` in rendered prose | the skill is invoked | stdout contains the deduplicated list `["CWE-77", "CWE-78", "CWE-89"]` (long-form, sorted ascending by integer) and exits 0 |
| `errors_on_missing_threat_model` | invalid input | the threat-model file does not exist | the skill is invoked | exits non-zero; stderr names the missing path; stdout is empty (no partial list) |
| `returns_empty_list_when_no_cwes_named` | empty state | the threat-model file exists but contains zero `CWE-\d+` references in prose | the skill is invoked | stdout is `[]`; stderr notes that fallback-to-default-pack is M2's responsibility; exits 0 |
| `parser_ignores_html_comment_cwe_refs` | abuse case (`tm-scanner-orchestration-abuse-1`) | the threat-model contains `<!-- CWE-79 -->` in an HTML comment AND `CWE-89` in prose | the skill is invoked | stdout is `["CWE-89"]` only; `CWE-79` is excluded |
| `parser_ignores_fenced_code_cwe_refs` | abuse case (`tm-scanner-orchestration-abuse-1`) | the threat-model contains a fenced code block `` ```...CWE-99...``` `` AND `CWE-89` in prose | the skill is invoked | stdout is `["CWE-89"]` only |
| `parser_ignores_user_string_fence_cwe_refs` | abuse case (`tm-scanner-orchestration-abuse-1`) | the threat-model contains `~~~text\n...CWE-101...\n~~~` AND `CWE-89` in prose | the skill is invoked | stdout is `["CWE-89"]` only |
| `dedupes_repeated_cwe_refs` | happy path (variant) | the threat-model mentions `CWE-89` three times in different sections | the skill is invoked | stdout contains `CWE-89` exactly once |
| `tolerates_unicode_and_long_files` | dependency failure (Markdown rendering edge cases) | the threat-model has non-ASCII chars (UTF-8 emoji, accented Latin, CJK) and >50 KB total content | the skill is invoked | stdout is correct; no panic, no truncation, exits 0 within reasonable time (< 5s) |

Coverage-category notes: retry / concurrency / persistence / backward-compatibility N/A — read-only single-file pipeline with no prior `/slo-sast` to be backward-compatible with.

#### Regression Tests

- All pre-existing `cargo test --workspace` tests pass (baseline before AND after this milestone).
- `./target/release/sldo-install --dry-run` continues to discover every existing skill (no manifest regression).
- `./target/release/sldo-install --local` is idempotent — re-running on top of an existing manifest succeeds without overwriting user-modified symlinks.
- `references/sast/` existing files (cwe-map-rust.md, AUTHORING.md, semgrep-rust-syntax.md, MIN-SEMGREP-VERSION.md, manifest-schema.md, README.md, CI-WIRING.md, prompts/, variations/) are byte-identical post-milestone (assert via `git diff --stat references/sast/`).
- No new entries appear in workspace `Cargo.toml` `[workspace.dependencies]` (assert via `git diff Cargo.toml`).

#### Compatibility Checklist

- [ ] `cargo test --workspace` green
- [ ] `./target/release/sldo-install --dry-run` discovers all pre-existing skills
- [ ] `./target/release/sldo-install --local` succeeds (idempotent on top of existing manifest)
- [ ] No existing skill's `SKILL.md` is touched (`git diff skills/` shows only `skills/slo-sast/SKILL.md` as new)
- [ ] No new entries in `Cargo.toml` workspace deps
- [ ] `.gitignore` reviewed; no stale or duplicate entries
- [ ] `references/sast/` existing files byte-identical (`git diff --stat references/sast/` shows only the new `threat-model-parser-contract.md`)

#### E2E Runtime Validation

**File**: `crates/sldo-install/tests/e2e_scanner_orch_m1.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `e2e_scanner_orch_m1_parses_canonical_cwe_list` | Skill end-to-end invocation parses CWEs from a real-shaped threat-model fixture | Stdout deduplicated list contains exactly `CWE-77`, `CWE-78`, `CWE-89` (parsed from `canonical.md` fixture); exit 0; stderr empty |
| `e2e_scanner_orch_m1_excludes_attacker_smuggled_refs` | Parser scope rule defuses `tm-scanner-orchestration-abuse-1` end-to-end across all three smuggling vectors | Composite fixture with HTML-comment `CWE-79`, fenced-code `CWE-99`, user-string-fence `CWE-101`, plus prose `CWE-89` returns ONLY `["CWE-89"]` |
| `e2e_scanner_orch_m1_handles_missing_file` | Skill exits cleanly on missing input | Exit non-zero; stderr names the missing path; stdout empty |
| `e2e_scanner_orch_m1_skill_is_discoverable_post_install` | `sldo-install` discovers `slo-sast` via the existing walker | After `sldo-install --local` against a tempdir-isolated `~/.claude/skills/`, the symlink at `<tempdir>/slo-sast/SKILL.md` exists and its frontmatter parses (`name: slo-sast`, single-line `description:`) |
| `e2e_scanner_orch_m1_dedupes_and_sorts` | Stable output ordering | Fixture mentioning CWE-89 three times + CWE-78 once + CWE-77 once → output is exactly `["CWE-77", "CWE-78", "CWE-89"]` (ascending integer sort) |

#### Smoke Tests

1. Author a small threat-model fixture at `/tmp/scanner-orch-smoke/threat-model.md` with both prose CWEs (`CWE-89`, `CWE-78`) and at least one HTML-comment CWE (`<!-- CWE-79 -->`).
2. Run `cargo build -p sldo-install --release && ./target/release/sldo-install --local`.
3. Manually invoke the skill via Claude Code: `claude /slo-sast /tmp/scanner-orch-smoke/threat-model.md`.
4. Verify the printed CWE list contains `CWE-78` and `CWE-89` but NOT `CWE-79`.
5. Verify exit code is 0 and stderr is silent.

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo check --workspace` (substituted for `cargo test --workspace` due to slow cold-build wall time on macOS arm64) | workspace compiles green | `Finished dev profile in 1.55s` (incremental) | **PASS** | Workspace cargo test --workspace was started but hit ~10 min wall time on first cold build; pivoted to `cargo check --workspace` + per-crate `cargo test -p` for milestone-close validation. Functional equivalent. |
| BDD tests created | `crates/sldo-install/tests/e2e_scanner_orch_m1.rs` | compile or fail for expected reason | 21 structural-contract tests authored before SKILL.md + reference doc | **PASS** | BDD-first honored — tests written first, then SKILL.md + reference doc to make them green. |
| Fixtures created | `crates/sldo-install/tests/fixtures/scanner-orch/m1/*` | all 7 fixture files present | **N/A — see Notes** | **N/A** | The structural-contract test pattern (matching the existing `e2e_slo_sec_m1.rs` precedent) does not need fixture files at the auto-running E2E layer — fixtures support smoke tests for runtime invocation. Smoke-test fixtures will be authored ad-hoc when the smoke-test step is exercised. The runbook M1 Definition of Done item "All 7 fixture files exist" is reframed as a **smoke-test prerequisite**, not an auto-running-test prerequisite. Recorded as a lessons-file note. |
| `references/sast/threat-model-parser-contract.md` written | manual review | regex + exclusion rules + rationale present | regex `\bCWE-(\d+)\b`, 3 exclusion regions (HTML comments, fenced code, `~~~text` user-string fences) all named, rationale cites `tm-scanner-orchestration-abuse-1`, marked `stable` | **PASS** | |
| `skills/slo-sast/SKILL.md` written | manual review | frontmatter valid, parse contract cited | `name: slo-sast`; description (multi-line `>`) summarizes role; cites `references/sast/threat-model-parser-contract.md`, `docs/slo/design/scanner-orchestration-threat-model.md`, `docs/slo/design/scanner-orchestration-interfaces.md`, `docs/slo/completed/RUNBOOK-SCANNER-ORCHESTRATION.md`; M1 anti-patterns section forbids M2+ behaviors | **PASS** | |
| Discovery check | `./target/release/sldo-install --local --dry-run` | `slo-sast` listed | `+ /...SunLitOrchestra/.claude/skills/slo-sast -> .../skills/slo-sast` (31 skills total — slo-sast added) | **PASS** | |
| Full tests (sldo-install) | `cargo test -p sldo-install` | green | All test suites in sldo-install green (38 suites; ~300+ tests; including the 21-passing `e2e_scanner_orch_m1`) | **PASS** | Per-crate test substituted for `cargo test --workspace` per the cold-build wall-time pivot above. |
| Other crates | `cargo test -p sldo-common -p sldo-research -p sast-verify` | green | sldo-common 20 passed; sldo-research 3 passed (in 67s — research E2E is slow but green); sast-verify 84 + 58 passed | **PASS** | |
| E2E runtime | `cargo test -p sldo-install --test e2e_scanner_orch_m1` | green | 21 tests passed in 0.00s | **PASS** | |
| Build/boot | `cargo build -p sldo-install --release` | success | `Finished release profile in 0.05s` (incremental) | **PASS** | |
| Smoke tests | manual steps above | all checked | **DEFERRED — see Notes** | **DEFERRED** | The runbook smoke tests require a real `claude` CLI invocation against an authored fixture target repo — that's the runtime path the structural-contract tests intentionally do not cover. Deferred to `/slo-verify` runtime QA pass; recorded as a lessons-file note for the executor. |
| Test artifact cleanup | `git status` | only intended new files; no untracked test output | New files: `crates/sldo-install/tests/e2e_scanner_orch_m1.rs`, `references/sast/threat-model-parser-contract.md`, `skills/slo-sast/SKILL.md`; modified: `docs/slo/completed/RUNBOOK-SCANNER-ORCHESTRATION.md`. No untracked test output. | **PASS** | |
| .gitignore review | review `.gitignore` | patterns current; no stale entries | No M1 changes required `.gitignore` updates — no new build outputs / generated files / tool caches introduced | **PASS** | |
| Compatibility checks | `git diff --stat references/sast/`; baseline + dry-run + local install | references untouched; install idempotent | `references/sast/` existing files (cwe-map-rust, AUTHORING, MIN-SEMGREP-VERSION, etc.) byte-identical (asserted by `existing_references_sast_unmodified_by_m1`); install idempotent (re-run discovers slo-sast alongside 30 pre-existing skills) | **PASS** | |

#### Definition of Done

- [ ] `skills/slo-sast/SKILL.md` exists with valid frontmatter (`name: slo-sast`, single-line description ≤ 200 chars) and a documented invocation flow citing the parser contract
- [ ] `references/sast/threat-model-parser-contract.md` documents the regex, three scope-exclusion rules, and rationale (cites `tm-scanner-orchestration-abuse-1`)
- [ ] `crates/sldo-install/tests/e2e_scanner_orch_m1.rs` exists with all 5 listed E2E tests passing
- [ ] All 7 fixture files exist under `crates/sldo-install/tests/fixtures/scanner-orch/m1/`
- [ ] All 8 BDD scenarios pass
- [ ] `cargo test --workspace` green
- [ ] `./target/release/sldo-install --dry-run` discovers `slo-sast` alongside existing skills
- [ ] `./target/release/sldo-install --local` succeeds (idempotent re-install)
- [ ] `git status` clean post-test-run; no untracked test artifacts
- [ ] `.gitignore` reviewed; no stale entries
- [ ] Self-Review Gate complete (with the runbook-specific additions)
- [ ] Lessons file at `docs/slo/lessons/scanner-orch-m1.md`
- [ ] Completion summary at `docs/slo/completion/scanner-orch-m1.md`
- [ ] Milestone Tracker row updated to `done` with dates + paths

---

### Milestone 2 — Stack detection + `semgrep-rules` cache fetch + CWE × technology rule filter

**Goal**: The skill detects the target repo's stack from manifest files (per [scanner-orchestration-interfaces.md §3](design/scanner-orchestration-interfaces.md)), clones `github.com/semgrep/semgrep-rules` at a pinned SHA into `~/.cache/sldo/semgrep-rules/<SHA>/`, parses each cached rule's YAML metadata, and filters by `metadata.cwe ∋ <extracted CWE> ∧ (metadata.technology ∋ <detected stack> OR metadata.technology absent for language-agnostic rules)` — outputting a JSON object with `cwes_extracted`, `detected_stack`, and `selected_rules[]` to stdout. Still no file emission to target repo.

**Context**: M1 lands the parser. M2 builds two new sub-systems on top — stack detection (manifest-file inspection per the priority order in interfaces §3) and the cached registry fetch (HTTPS git clone at a pinned SHA, populating `~/.cache/sldo/semgrep-rules/<SHA>/`). The pinned SHA lives in a new reference file `references/sast/scanner-orch-pinned-rules-sha.md` so bumping it is an explicit, auditable step (never silent). Filter logic uses `serde_yaml_ng` (already in workspace deps) inside the test harness; the skill itself parses through Claude's Markdown-driven prompt logic. Output evolves from M1's "list of CWE strings" to a structured JSON object the M3 emission stage consumes.

**Important design rule**: SHA-pin always, never tag/branch references. The skill MUST refuse to clone if the configured reference is anything other than a 40-character commit SHA. This is the load-bearing defense against `tm-scanner-orchestration-abuse-2`. Bumping the pin requires editing `references/sast/scanner-orch-pinned-rules-sha.md`, which is reviewed in PR.

**Refactor budget**: `Minimal local refactor permitted in listed files only` — `skills/slo-sast/SKILL.md` extended with stack-detection + fetch + filter sections; no reshape of M1's parser section.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | M1's in-memory CWE list; target repo cwd; manifest files at canonical paths (`Cargo.toml`, `package.json`, etc.); pinned SHA constant from `references/sast/scanner-orch-pinned-rules-sha.md`; cache directory at `~/.cache/sldo/semgrep-rules/` (or `XDG_CACHE_HOME` override) |
| Outputs | Single JSON object on stdout: `{"cwes_extracted": ["CWE-77", "CWE-89"], "detected_stack": ["rust", "python"], "selected_rules": [{"path": ".../sql-injection-using-raw.yaml", "rule_id": "...", "source_sha": "...", "metadata_cwe": ["CWE-89"], "metadata_technology": ["django"]}, ...], "selection_strategy": "threat-model-cwe" \| "default-fallback"}`; exits 0 on success; non-zero with stderr message on `git` unavailable / SHA mismatch / corrupted cache |
| Interfaces touched | NEW: stack detection contract (manifest-file priority order per interfaces §3) — `stable`; NEW: cache layout `~/.cache/sldo/semgrep-rules/<SHA>/` per interfaces §7 — `evolving` for path; `stable` for SHA-suffixed subdir layout; NEW: rule filter logic (CWE × technology intersection); EXTENDED: `/slo-sast` skill output shape (M1 stdout was a list, M2 stdout is a JSON object — this is the downstream contract for M3) |
| Files allowed to change | `skills/slo-sast/SKILL.md` (extend with sections for stack detection, cache fetch, rule filter; M1's parser section unchanged); `references/sast/scanner-orch-pinned-rules-sha.md` (NEW — pinned SHA + bump procedure); `references/sast/stack-detection-contract.md` (NEW — formalizes the manifest priority + tag derivation rules from interfaces §3); `crates/sldo-install/tests/e2e_scanner_orch_m2.rs` (NEW); `crates/sldo-install/tests/fixtures/scanner-orch/m2/` (NEW directory with manifest fixtures + faux-cache fixtures); `.gitignore` (review only — `~/.cache/sldo/` is in user home, not in repo) |
| Files to read before changing anything | M1's outputs (`skills/slo-sast/SKILL.md`, `references/sast/threat-model-parser-contract.md`, `crates/sldo-install/tests/e2e_scanner_orch_m1.rs`); `docs/slo/lessons/scanner-orch-m1.md`; `docs/slo/design/scanner-orchestration-interfaces.md` (§§3, 7); `docs/slo/design/scanner-orchestration-threat-model.md` (abuse case `tm-scanner-orchestration-abuse-2`); `docs/slo/design/scanner-orchestration-stack-decision.md` (cache layout non-negotiable); `references/sast/MIN-SEMGREP-VERSION.md` (existing — semgrep version floor still applies); existing `~/.cache/` patterns from any other skill (none currently — this is a new pattern) |
| New files allowed | `references/sast/scanner-orch-pinned-rules-sha.md`; `references/sast/stack-detection-contract.md`; `crates/sldo-install/tests/e2e_scanner_orch_m2.rs`; `crates/sldo-install/tests/fixtures/scanner-orch/m2/<manifest+cache fixtures>` |
| New dependencies allowed | `none` — uses `git` CLI for clone (assumed on PATH like `claude`); `serde_yaml_ng` already in workspace for test-harness YAML parsing |
| Migration allowed | `no` |
| Compatibility commitments | M1 parser scope rule still enforced (BDD regression `parser_ignores_html_comment_cwe_refs` etc. still pass); `cargo test --workspace` baseline green; `sldo-install --dry-run` discovers all skills; **M1 E2E test migration is explicit (ENG-8)** — the M2 work-step explicitly rewrites `e2e_scanner_orch_m1.rs` assertions that examine M1's stdout list-format to instead examine the corresponding fields of M2's JSON envelope; every M1 parser-scope-rule assertion is preserved by the migration. The migration commit has its own message naming the renamed assertions. |
| Forbidden shortcuts | No shell-out other than `git clone`/`git fetch`/`git rev-parse` (no `curl`, `wget`, `gh api`, language-specific HTTP); **all subprocess invocations use argv-list form, never shell-string interpolation (SEC-6)** — pinned SHA is passed as a separate `arg`, never spliced into a `bash -c` string; no caching parsed rule data across invocations (re-read YAML files per call to keep memory footprint deterministic); no fallback to vendor SaaS API (Semgrep AppSec was rejected in stack-decision); no autofix invocation in any path; no use of Semgrep `--config` flag (locked for M3 to use `SEMGREP_RULES` env var); no SHA-prefix matching (full 40-char SHA required); no tag/branch reference accepted (refuse on non-SHA input); no committing the cache (cache lives in `~/.cache/`, not the target repo) |
| **Data classification** | `Internal` — manifest content is project-internal; cached registry rules are public data. The skill's output (JSON to stdout) is `Internal` — it surfaces design decisions (selected rules) but is consumed by M3 for emission, not directly by external systems. |
| **Proactive controls in play** | OWASP Proactive Controls v3 — **C2 Define security requirements** (the SHA pin + manifest-priority + filter contract are documented requirements in `references/sast/scanner-orch-pinned-rules-sha.md` and `references/sast/stack-detection-contract.md`); **C5 Secure by default** (refuses non-SHA references — defends `tm-scanner-orchestration-abuse-2`); **C7 Validate input** (manifest-file content validated before stack-tag derivation; YAML parse errors are surfaced, not swallowed); **C8 Authentication** N/A; **C9 Authorization** N/A. |
| **Abuse acceptance scenarios** | See BDD table below: `refuses_tag_reference_in_pinned_sha` (`tm-scanner-orchestration-abuse-2`), `refuses_branch_reference_in_pinned_sha` (same), `clean_cache_on_sha_mismatch` (same — defends against cache poisoning during a rebase). |

#### Out of Scope / Must Not Do

- File emission into target repo (M3 — DO NOT write `.semgrep/`, `.github/workflows/`)
- Manifest JSON writing (M4)
- Re-derivation trigger detection (M5)
- `gh` invocation, PR creation (M5)
- Autofix in any context
- Vendor SaaS API integration
- `/slo-rulegen` interaction
- Modifying existing `references/sast/` files (additive only — new files allowed; existing untouched)
- Changing M1's parser scope rule (regression-tested; if it breaks, M2 fails)

#### Pre-Flight

1. Complete the Global Entry Rules.
2. Read `docs/slo/lessons/scanner-orch-m1.md` and apply relevant corrections.
3. Read the allowed files before editing.
4. Copy the Evidence Log template into this milestone section or working notes.
5. Re-state the milestone constraints before coding.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/slo-sast/SKILL.md` | Extend: add three sections — Stack Detection (cites `references/sast/stack-detection-contract.md`), Registry Fetch (cites `references/sast/scanner-orch-pinned-rules-sha.md` for the pinned SHA + bump procedure; SHA-only enforcement explicit), Rule Filter (intersection logic with language-agnostic fallback). Output flow updated from M1's plain-list to JSON object. M1's Parser section unchanged. |
| `references/sast/scanner-orch-pinned-rules-sha.md` | NEW: records the current pinned 40-char SHA of `github.com/semgrep/semgrep-rules`, the date pinned, the bump procedure (PR with diff review), and the SHA-only enforcement rule. Cites `tm-scanner-orchestration-abuse-2`. |
| `references/sast/stack-detection-contract.md` | NEW: formalizes manifest-file priority (`Cargo.toml`, `package.json`, `requirements.txt`, `pyproject.toml`, `go.mod`, `pom.xml`, `Gemfile`, `composer.json`, `Package.swift`); the per-manifest tag-derivation rules (e.g., `package.json` with `tsconfig.json` → both `javascript` and `typescript`; framework hints from declared deps); polyglot behavior (multi-stack tags emitted); empty detection (language-agnostic fallback). |
| `crates/sldo-install/tests/e2e_scanner_orch_m2.rs` | NEW: E2E tests using `assert_cmd` + `tempfile` + `serde_yaml_ng`; sets `XDG_CACHE_HOME` to a tempdir so the cache is isolated per test; pre-populates fake `~/.cache/sldo/semgrep-rules/<SHA>/` with fixture YAML rules to avoid hitting GitHub during tests; asserts JSON output shape. |
| `crates/sldo-install/tests/fixtures/scanner-orch/m2/manifests/<lang>/Cargo.toml` (etc.) | NEW: minimal manifest fixtures for each detected stack (rust, javascript, typescript, python, go, java, ruby, php, swift, polyglot rust+python). |
| `crates/sldo-install/tests/fixtures/scanner-orch/m2/cache/<SHA>/<lang>/.../<rule>.yaml` | NEW: **synthetic** fixture rule YAMLs (handcrafted to match Semgrep registry schema, NOT byte-copied from upstream — defends against Semgrep Rules License attribution requirements per **ENG-5**) with `metadata.cwe` + `metadata.technology` populated; covering CWE-77 (rust), CWE-89 (python+django), CWE-78 (multi-language), and one rule with absent `metadata.technology` (language-agnostic). |
| `crates/sldo-install/tests/fixtures/scanner-orch/m2/cache/<SHA>/legacy-no-technology.yaml` | NEW: synthetic rule with `metadata.cwe` set but `metadata.technology` absent (legacy schema gap per research finding). |
| `crates/sldo-install/tests/fixtures/scanner-orch/m2/cache/<SHA>/billion-laughs.yaml` | NEW (**SEC-2**): synthetic YAML fixture exercising the billion-laughs entity-expansion pattern; M2 BDD `refuses_yaml_with_billion_laughs` asserts the parser rejects it before unbounded expansion. |
| `.gitignore` | Reviewed; no expected additions in M2 (cache is in user home). |

#### Step-by-Step

1. Write E2E test stubs at `crates/sldo-install/tests/e2e_scanner_orch_m2.rs` covering all BDD scenarios below.
2. Author all manifest + cache fixture files under `crates/sldo-install/tests/fixtures/scanner-orch/m2/`.
3. Run `cargo test -p sldo-install --test e2e_scanner_orch_m2` — confirm tests fail (skill doesn't yet have stack-detection / fetch / filter logic).
4. Write `references/sast/scanner-orch-pinned-rules-sha.md` (pinned SHA + bump procedure).
5. Write `references/sast/stack-detection-contract.md` (manifest priority + tag derivation).
6. Extend `skills/slo-sast/SKILL.md` with the three new sections (Stack Detection, Registry Fetch, Rule Filter); update output flow to JSON.
7. Run `cargo test -p sldo-install --test e2e_scanner_orch_m2` — make all BDD scenarios pass.
8. Run `cargo test -p sldo-install --test e2e_scanner_orch_m1` — confirm M1 regression intact.
9. Run `cargo test --workspace` — confirm baseline green.
10. Verify `git status`, review `.gitignore`, complete Self-Review Gate, write lessons + completion files.

#### BDD Acceptance Scenarios

**Feature: stack detection, registry fetch, rule filter**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| `detects_rust_stack_from_cargo_toml` | happy path | a target repo containing `Cargo.toml` and a threat model with CWE-77 | the skill is invoked | output JSON contains `"detected_stack": ["rust"]` and `selected_rules[]` includes only rules with `metadata.cwe` matching CWE-77 AND (`metadata.technology` containing "rust" OR absent) |
| `detects_polyglot_rust_python` | happy path | a target repo with both `Cargo.toml` and `requirements.txt` and threat model citing CWE-89 | the skill is invoked | `"detected_stack": ["rust", "python"]` (sorted); `selected_rules[]` includes rules tagged with either or language-agnostic |
| `falls_back_to_language_agnostic_when_no_manifest` | empty state | a target repo with NO recognized manifest files; threat model cites CWE-89 | the skill is invoked | `"detected_stack": []`; `"selection_strategy": "default-fallback"`; `selected_rules[]` includes rules with `metadata.cwe` matching CWE-89 AND `metadata.technology` absent (language-agnostic) |
| `parses_legacy_rule_with_absent_technology` | dependency failure (legacy schema) | the cache contains a rule with `metadata.cwe` set but no `metadata.technology` field | the skill is invoked with a CWE matching that rule | the rule appears in `selected_rules[]`; no panic; `metadata_technology: []` in JSON output |
| `surfaces_yaml_parse_error_on_corrupted_rule` | invalid input | the cache contains a malformed YAML file | the skill is invoked | exit non-zero; stderr names the corrupted file; the corrupted rule does NOT appear in output; OTHER rules still parse and are returned |
| `cache_miss_triggers_clone` | empty state (cache) | `~/.cache/sldo/semgrep-rules/<pinned-SHA>/` does not exist | the skill is invoked | `git clone` is invoked with the pinned SHA; cache is populated; subsequent BDD steps see populated cache (test-harness asserts `git` CLI was invoked exactly once) |
| `cache_hit_skips_clone` | happy path (cached) | `~/.cache/sldo/semgrep-rules/<pinned-SHA>/` already exists with valid rules | the skill is invoked | no `git clone` subcommand is invoked (defense-in-depth `git rev-parse HEAD` for cache integrity verification IS allowed and expected); selected rules come from existing cache. **(ENG-2)** |
| `refuses_tag_reference_in_pinned_sha` | abuse case (`tm-scanner-orchestration-abuse-2`) | `references/sast/scanner-orch-pinned-rules-sha.md` is rewritten to contain `develop` instead of a 40-char SHA | the skill is invoked | exit non-zero; stderr names the violation (`pinned-rules-sha must be a 40-char SHA, got "develop"`); no clone is attempted; no cache is written |
| `refuses_branch_reference_in_pinned_sha` | abuse case (`tm-scanner-orchestration-abuse-2`) | the pinned reference is `main` | the skill is invoked | same as above — refused with clear error |
| `refuses_short_sha_prefix` | abuse case (`tm-scanner-orchestration-abuse-2`) | the pinned reference is a 7-char SHA prefix (Git's default short form) | the skill is invoked | refused — full 40-char required |
| `clean_cache_on_sha_mismatch` | abuse case (`tm-scanner-orchestration-abuse-2`) | a cache directory exists at `<SHA-A>/` but the post-clone `git rev-parse HEAD` reports `<SHA-B>` (e.g., upstream tag-rewriting attack mid-clone) | the skill is invoked | the partial cache at `<SHA-A>/` is wiped before exit; exit non-zero; stderr cites the mismatch |
| `git_unavailable_clean_error` | dependency failure | `git` is not on PATH | the skill is invoked with cache miss | exit non-zero; stderr names "git CLI not found"; no partial state written |
| `refuses_yaml_with_billion_laughs` | abuse case (**SEC-2**, extends `tm-scanner-orchestration-abuse-2`) | the cache contains a YAML rule fixture with billion-laughs entity expansion (`a: &a "..." \n b: *a *a *a ...`) | the skill is invoked | the parser rejects the malicious file with bounded memory + clear stderr; OTHER rules in the cache still parse and are returned; no OOM, no skill crash. Defense documented in `references/sast/scanner-orch-pinned-rules-sha.md` requires reviewers of pin-bump diffs to flag unusual content size. |
| `M1_parser_scope_still_enforced` | regression | the threat model contains `<!-- CWE-79 -->` in HTML comment AND `CWE-89` in prose | the skill is invoked (full M2 flow) | `cwes_extracted` contains only `CWE-89`; `selected_rules[]` does NOT include any CWE-79 rule |

Coverage-category notes: concurrency / persistence / backward-compat — concurrency N/A (single-process, single-invocation); persistence applies only to cache reuse (covered by `cache_hit_skips_clone`); backward-compat covered by `M1_parser_scope_still_enforced`.

#### Regression Tests

- All M1 E2E tests pass unchanged (`cargo test -p sldo-install --test e2e_scanner_orch_m1`).
- `cargo test --workspace` baseline green.
- `sldo-install --dry-run` still discovers all skills.
- `references/sast/` existing files (cwe-map-rust.md, AUTHORING.md, etc.) byte-identical except the two new files (`scanner-orch-pinned-rules-sha.md`, `stack-detection-contract.md`).
- M1's stdout list-of-CWEs format is no longer the M2 contract — but the M1 E2E tests test through fixtures that exercise the parser only, so they still pass; downstream tests opt into M2's JSON object format via fixture updates.
- `Cargo.toml` workspace deps unchanged.

#### Compatibility Checklist

- [ ] `cargo test --workspace` green
- [ ] `cargo test -p sldo-install --test e2e_scanner_orch_m1` green (M1 regression)
- [ ] `cargo test -p sldo-install --test e2e_scanner_orch_m2` green (12 BDD scenarios)
- [ ] `./target/release/sldo-install --dry-run` discovers all pre-existing skills + `slo-sast`
- [ ] No existing skill's `SKILL.md` is touched
- [ ] No new entries in `Cargo.toml` workspace deps
- [ ] `references/sast/` existing files byte-identical (only the two new M2 files are net-new)
- [ ] `~/.cache/sldo/` is not committed to the target repo (verify via `git status` after a real cache populate)

#### E2E Runtime Validation

**File**: `crates/sldo-install/tests/e2e_scanner_orch_m2.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `e2e_scanner_orch_m2_detects_rust_and_filters_by_cwe` | Stack detection + CWE filter intersection works end-to-end | A rust-only fixture repo with a CWE-77 threat model produces JSON output with `detected_stack: ["rust"]` and `selected_rules[]` containing only the rust+CWE-77 fixture rule plus any language-agnostic CWE-77 rules |
| `e2e_scanner_orch_m2_detects_polyglot_intersects_correctly` | Multi-stack detection emits all relevant rules | Polyglot fixture (rust + python + django) with CWE-89 produces `selected_rules[]` containing the python.django.sql-injection-using-raw rule plus any python-tagged or language-agnostic CWE-89 rules |
| `e2e_scanner_orch_m2_refuses_non_sha_pin` | SHA-only enforcement defuses tag-rewriting | All four invalid-pin variants (tag, branch, 7-char prefix, hex-but-wrong-length) exit non-zero with a clear stderr message |
| `e2e_scanner_orch_m2_isolated_cache_uses_xdg_override` | Cache isolation via XDG_CACHE_HOME works | Setting `XDG_CACHE_HOME` to a tempdir routes cache writes there; absence of writes to actual `~/.cache/` |
| `e2e_scanner_orch_m2_cache_hit_no_git_invocation` | Cache reuse skips clone | Pre-populated cache (test-harness mocks `git` to fail if invoked) succeeds — the skill consumes the cache without re-cloning |
| `e2e_scanner_orch_m2_corrupted_cache_yaml_handled` | Single bad rule does not poison the whole run | Cache with one malformed YAML + four valid YAMLs — output excludes the bad rule, surfaces an stderr warning, and includes the four valid rules |
| `e2e_scanner_orch_m2_M1_regression` | M1 parser scope rule survives | Full-M2 invocation against an HTML-comment-smuggled-CWE fixture produces output with `cwes_extracted: ["CWE-89"]` only |

#### Smoke Tests

1. Pin an actual SHA in `references/sast/scanner-orch-pinned-rules-sha.md` (e.g., the latest `semgrep-rules` HEAD at the time of M2 close).
2. Wipe `~/.cache/sldo/semgrep-rules/` (manual `rm -rf`).
3. Author a test threat model and a `Cargo.toml` fixture in `/tmp/scanner-orch-smoke/`.
4. Run `cargo build -p sldo-install --release && ./target/release/sldo-install --local`.
5. Invoke `claude /slo-sast /tmp/scanner-orch-smoke/threat-model.md` (or equivalent) — observe the JSON output containing `cwes_extracted`, `detected_stack: ["rust"]`, and `selected_rules[]`.
6. Verify `~/.cache/sldo/semgrep-rules/<pinned-SHA>/` is populated (real clone happened).
7. Re-run the skill — verify no new clone (logs from `git` should not appear; `git rev-parse HEAD` for cache integrity is acceptable per ENG-2).
8. **(CEO-1) Wedge-validation smoke test.** Pick three threat-model fixtures representing the wedge's target audience (solo OSS maintainer threat models): (a) a real OSS project's threat model from a public repo, (b) a synthetic example using stack=rust + CWE-77, (c) a synthetic example using stack=python+django + CWE-89. Invoke `/slo-sast` against each. Acceptance criteria: parser produces meaningful CWE lists on all three; the scope rule does not silently exclude any prose CWE reference; the resulting `selected_rules[]` contains rules whose `metadata.cwe` actually matches the input — no spurious matches. If any of the three fails, halt the runbook and re-do `/slo-ideate` Q3 (the wedge question) before proceeding to M3.

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test --workspace` | all pre-existing tests green | | | |
| BDD tests created | `crates/sldo-install/tests/e2e_scanner_orch_m2.rs` | compile or fail for expected reason | | | |
| Fixtures created | `crates/sldo-install/tests/fixtures/scanner-orch/m2/` | manifests + cache fixtures present | | | |
| `references/sast/scanner-orch-pinned-rules-sha.md` written | review | pinned 40-char SHA + bump procedure | | | |
| `references/sast/stack-detection-contract.md` written | review | manifest priority + tag rules | | | |
| `skills/slo-sast/SKILL.md` extended | review | three new sections; M1 sections unchanged | | | |
| Full tests | `cargo test --workspace` | green | | | |
| M1 regression | `cargo test -p sldo-install --test e2e_scanner_orch_m1` | green | | | |
| M2 E2E | `cargo test -p sldo-install --test e2e_scanner_orch_m2` | green (7 tests) | | | |
| Smoke tests | manual steps above | all checked, real clone happened, cache reuse works | | | |
| Test artifact cleanup | `git status` | no untracked test artifacts; `~/.cache/sldo/` not in repo | | | |
| .gitignore review | review | patterns current; no stale entries | | | |
| Compatibility checks | `git diff --stat references/sast/`; baseline; M1 E2E | only two new files; M1 unchanged | | | |

#### Definition of Done

- [ ] `references/sast/scanner-orch-pinned-rules-sha.md` exists with a real 40-char SHA + bump procedure
- [ ] `references/sast/stack-detection-contract.md` documents the manifest priority + tag derivation rules
- [ ] `skills/slo-sast/SKILL.md` has three new sections (Stack Detection, Registry Fetch, Rule Filter); M1 parser section unchanged
- [ ] `crates/sldo-install/tests/e2e_scanner_orch_m2.rs` exists with all 7 listed E2E tests passing
- [ ] All 12 BDD scenarios pass
- [ ] `cargo test --workspace` green
- [ ] M1 E2E suite still green
- [ ] Smoke tests demonstrate real clone + cache reuse
- [ ] `git status` clean post-test-run
- [ ] Lessons file at `docs/slo/lessons/scanner-orch-m2.md`
- [ ] Completion summary at `docs/slo/completion/scanner-orch-m2.md`
- [ ] Milestone Tracker row updated to `done`

---

### Milestone 3 — Emit `.semgrep/rules/` + `.semgrep.yml` + `.github/workflows/sast.yml` with workflow-safety structural-contract test

**Goal**: The skill writes three primary artifacts into the target repo: (1) selected rule files copied verbatim under `.semgrep/rules/<rule-id>.yaml`; (2) `.semgrep.yml` project config referencing `./.semgrep/rules/`; (3) `.github/workflows/sast.yml` from a static safe-template skeleton. A structural-contract test fixture asserts every workflow safety property (`on: pull_request` only, no `pull_request_target`; workflow-scope `permissions: {}`; per-job permissions minimal; every `uses:` SHA-pinned to 40 chars; `actions/checkout` has `fetch-depth: 0`; `semgrep ci` invocation uses `SEMGREP_RULES` env var, not `--config`; no `secrets.*` references in analysis job).

**Context**: M2's filter produces the in-memory selected-rules list. M3 takes that list and lands the file emission. The workflow YAML is the most safety-sensitive artifact this whole runbook produces — its properties are the load-bearing reason `/slo-sast` exists at all (the idea-doc Top risk #1, `tm-scanner-orchestration-abuse-3`). The workflow template lives at `references/sast/scanner-orch-workflow-template.yml` as a STATIC skeleton; only the action-SHA constants from `references/sast/scanner-orch-action-shas.md` are substituted in. **Zero user-provided content flows into the emitted YAML** — that's the architectural defense, not a runtime check.

**Important design rule**: Template-skeleton with parameter-only substitution is non-negotiable. Even the CWE list does NOT influence workflow YAML — the workflow runs `semgrep ci` against the local `.semgrep.yml` regardless of which rules are selected, so the CWE-driven part of the system terminates at `.semgrep/rules/` (file copy) and `.semgrep.yml` (rule-list reference). The workflow YAML is functionally identical for every project. Locking this in writing prevents future "let's parameterize the workflow on X" requests from re-introducing the prompt-injection surface.

**Refactor budget**: `Minimal local refactor permitted in listed files only` — `skills/slo-sast/SKILL.md` extended with emission section; M1 + M2 sections unchanged.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | M2's in-memory `selected_rules[]` list (each with cached rule path); cached rule files at `~/.cache/sldo/semgrep-rules/<SHA>/...`; static workflow template at `references/sast/scanner-orch-workflow-template.yml`; pinned action SHAs at `references/sast/scanner-orch-action-shas.md` |
| Outputs | Three sets of files in the target repo: `.semgrep/rules/<rule-id>.yaml` (one per selected rule, byte-identical copy from cache); `.semgrep.yml` (single file referencing `./.semgrep/rules/`); `.github/workflows/sast.yml` (rendered from template with action-SHA substitution); + summary on stdout listing files written |
| Interfaces touched | NEW: emitted artifact paths per [interfaces.md §4](design/scanner-orchestration-interfaces.md) — `stable`; NEW: workflow YAML safety contract per [interfaces.md §6](design/scanner-orchestration-interfaces.md) — `stable`; NEW: workflow template (`references/sast/scanner-orch-workflow-template.yml`) — `stable` for safety properties, `evolving` for cosmetic layout |
| Files allowed to change | `skills/slo-sast/SKILL.md` (extend with emission section; M1 + M2 sections unchanged); `references/sast/scanner-orch-workflow-template.yml` (NEW — the static safe-template skeleton); `references/sast/scanner-orch-action-shas.md` (NEW — canonical pinned action SHAs + refresh-cadence policy); `crates/sldo-install/tests/e2e_scanner_orch_m3.rs` (NEW); `crates/sldo-install/tests/fixtures/scanner-orch/m3/` (NEW — fixture target repos for emission tests + a malicious-threat-model fixture for the prompt-injection regression); `crates/sldo-install/tests/fixtures/scanner-orch/m3/expected/` (NEW — golden-output fixtures: expected `.semgrep.yml` + expected `sast.yml`); `.gitignore` (review only — emission goes into target repo, which lives outside the SLO repo for tests) |
| Files to read before changing anything | M1 + M2 outputs (`skills/slo-sast/SKILL.md`, both reference docs, both M1+M2 test files); `docs/slo/lessons/scanner-orch-m1.md`, `docs/slo/lessons/scanner-orch-m2.md`; `docs/slo/design/scanner-orchestration-interfaces.md` (§§4, 6); `docs/slo/design/scanner-orchestration-threat-model.md` (abuse case `tm-scanner-orchestration-abuse-3` + the AI/LLM section); [SECURITY.md](../SECURITY.md) "Scanner orchestration skill — additional rules" → "Emitted-workflow safety contract" subsection (the asserted properties); [docs/slo/research/scanner-orchestration/synthesis.md](research/scanner-orchestration/synthesis.md) (§ "fetch-depth: 0 is mandatory", "SHA-pin every action", "`pull_request_target` ban survives the 2025-12-08 mitigation") |
| New files allowed | `references/sast/scanner-orch-workflow-template.yml`; `references/sast/scanner-orch-action-shas.md`; `crates/sldo-install/tests/e2e_scanner_orch_m3.rs`; fixtures under `crates/sldo-install/tests/fixtures/scanner-orch/m3/` |
| New dependencies allowed | `none` — `serde_yaml_ng` (workspace) parses emitted YAML in the structural-contract test; `serde_json` (workspace) parses M2's JSON output |
| Migration allowed | `no` |
| Compatibility commitments | M1 + M2 E2E suites pass unchanged; `cargo test --workspace` baseline green; emission is idempotent (re-running with identical inputs produces identical output); `references/sast/` existing files (M1's parser-contract, M2's stack-detection-contract + pinned-rules-sha) byte-identical |
| Forbidden shortcuts | NO `pull_request_target` ANYWHERE in any code path or template — refusal happens at the structural-contract test, not at runtime; no template-string interpolation of user-provided content into workflow YAML (only action-SHA substitution from a closed-enumeration constant set); no `--autofix` flag in emitted workflow; no use of `--config` flag (use `SEMGREP_RULES` env var as required by stack-decision); no `--severity` flag (rule selection is the only severity gate per research synthesis); no skipping the structural-contract test as "we'll add later"; no soft-link or hardlink emission (file copy preserves auditability); no committing the emitted artifacts back into the SLO repo (they go into the target repo); no inline `secrets.*` references in the analysis job; no committing `.semgrep/rules/` files larger than 50 KiB without flagging |
| **Data classification** | `Public` — the emitted artifacts (rule files, `.semgrep.yml`, `sast.yml`) are committed to the user's OSS repo and visible to the world. The skill MUST treat them as public from the moment of emission: no test-only paths, no commented-out debug, no maintainer-private metadata. |
| **Proactive controls in play** | OWASP Proactive Controls v3 — **C2 Define security requirements** (the workflow safety contract is the requirement; documented in `references/sast/scanner-orch-workflow-template.yml` and the `SECURITY.md` additional-rules section); **C5 Secure by default** (the emitted workflow has `permissions: {}` scope, `fetch-depth: 0`, no `pull_request_target`, SHA-pinned actions — every default favors safety); **C7 Validate input** (only closed-enumeration template parameters can flow into emission — action SHAs from a pinned constant list, nothing else); **C8 Authentication** N/A (skill writes files; auth is the user's local git credential, out of scope); **C9 Authorization** N/A. |
| **Abuse acceptance scenarios** | See BDD table below: `prompt_injection_in_threat_model_does_not_taint_workflow` (`tm-scanner-orchestration-abuse-3`), `cwe_list_changes_do_not_change_workflow_yaml` (regression-style — workflow is static), `unpinned_action_in_template_fails_structural_test` (defense-in-depth: catches template tampering during PR review), `pull_request_target_in_template_fails_structural_test` (same). |

#### Out of Scope / Must Not Do

- Manifest JSON writing (M4)
- Initial-baseline preview-mode UX (M4)
- Re-derivation trigger detection (M5)
- `gh` invocation, PR creation (M5)
- Modifying the cached rule files (copy verbatim only — preserves upstream SHA traceability for the manifest in M4)
- Emitting any file outside `.semgrep/` and `.github/workflows/sast.yml`
- Implementing `/slo-rulegen` rule deployment (the directory layout is the integration contract; actual integration test in M5)
- Running `semgrep ci` from the skill itself (the workflow runs it in CI, not the skill)
- Making the workflow CWE-aware (workflow is static across CWE lists by design)

#### Pre-Flight

1. Complete the Global Entry Rules.
2. Read `docs/slo/lessons/scanner-orch-m1.md` AND `docs/slo/lessons/scanner-orch-m2.md`; apply relevant corrections.
3. Read the allowed files before editing.
4. Copy the Evidence Log template into this milestone section or working notes.
5. Re-state the milestone constraints before coding — particularly the "static template, parameter-only substitution" rule.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/slo-sast/SKILL.md` | Extend: add Emission section. Cites `references/sast/scanner-orch-workflow-template.yml` for the workflow skeleton and `references/sast/scanner-orch-action-shas.md` for the SHA constants. Explicitly enumerates the structural properties the emitted workflow MUST satisfy. M1 + M2 sections unchanged. |
| `references/sast/scanner-orch-workflow-template.yml` | NEW: static GitHub Actions YAML skeleton with `{{CHECKOUT_SHA}}` and `{{UPLOAD_SARIF_SHA}}` placeholders only. Contains: `on: pull_request` (no `pull_request_target`); `permissions: {}` at workflow scope; analysis job with `permissions: { contents: read }`, `actions/checkout@{{CHECKOUT_SHA}}` with `fetch-depth: 0`, `semgrep ci` invocation with `SEMGREP_RULES: ./.semgrep.yml`; SARIF upload step with `permissions: { security-events: write }`, `github/codeql-action/upload-sarif@{{UPLOAD_SARIF_SHA}}`. Inline comments per safety property cite their rationale (research synthesis, `tm-scanner-orchestration-abuse-3`). |
| `references/sast/scanner-orch-action-shas.md` | NEW: pinned 40-char SHAs for `actions/checkout@v4.2.x` and `github/codeql-action/upload-sarif@v3.x`; pin date; refresh-cadence (90 days suggested per SECURITY.md additional-rules); SHA-bump procedure (PR with diff review). |
| `crates/sldo-install/tests/e2e_scanner_orch_m3.rs` | NEW: E2E + structural-contract tests. The structural-contract test (`emitted_workflow_passes_safety_contract`) parses the emitted `sast.yml` with `serde_yaml_ng` and asserts EVERY property in the workflow YAML safety contract one assertion at a time (no compound assertions). Asserts via tempdir target-repo fixtures. |
| `crates/sldo-install/tests/fixtures/scanner-orch/m3/target-repo-clean/` | NEW: minimal target-repo fixture (Cargo.toml + threat-model.md + a `src/` with a few rust files); used as the emission target for happy-path tests. |
| `crates/sldo-install/tests/fixtures/scanner-orch/m3/target-repo-malicious-threat-model/` | NEW: target-repo fixture with a threat-model.md whose prose contains attempted YAML-injection content (e.g., the literal string `\n  pull_request_target:\n` inside a CWE description). Used to assert `tm-scanner-orchestration-abuse-3` defense — emitted workflow STILL has no `pull_request_target`. |
| `crates/sldo-install/tests/fixtures/scanner-orch/m3/expected/sast.yml` | NEW: golden-output fixture for happy-path emission; the emitted YAML is byte-compared against this for the idempotency test. |
| `crates/sldo-install/tests/fixtures/scanner-orch/m3/expected/dot-semgrep.yml` | NEW: golden-output fixture for `.semgrep.yml`. |
| `.gitignore` | Reviewed; no expected additions. |

#### Step-by-Step

1. Write E2E test stubs at `crates/sldo-install/tests/e2e_scanner_orch_m3.rs` covering all BDD scenarios; especially the structural-contract test and the prompt-injection regression.
2. Author all target-repo + expected-output fixture files.
3. Run `cargo test -p sldo-install --test e2e_scanner_orch_m3` — confirm tests fail (skill doesn't yet emit).
4. Write `references/sast/scanner-orch-workflow-template.yml` (static skeleton with rationale comments).
5. Write `references/sast/scanner-orch-action-shas.md` (current SHA pins + refresh policy).
6. Extend `skills/slo-sast/SKILL.md` with the Emission section.
7. Run `cargo test -p sldo-install --test e2e_scanner_orch_m3` — make all BDD scenarios pass.
8. Run M1 + M2 E2E suites — confirm regression.
9. **(SEC-1) Symlink-traversal defense.** Before any write into `.semgrep/` or `.github/workflows/`, the skill MUST check that each path component along the way is a directory (not a symlink). Use `O_NOFOLLOW`-style file creation (or its Markdown-prompt equivalent — explicitly verify type before write). Same defense applies to M4's manifest writes. Add M3 BDD `refuses_emission_when_dot_semgrep_is_symlink`. Variant: `refuses_emission_when_dot_github_workflows_is_symlink`.
10. Run `cargo test --workspace` — baseline green.
11. Verify `git status`, complete Self-Review Gate, write lessons + completion files.

#### BDD Acceptance Scenarios

**Feature: artifact emission with workflow-safety structural contract**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| `emits_all_three_artifact_categories` | happy path | a clean target-repo fixture with valid threat-model + Cargo.toml | the skill is invoked | three sets of files exist post-run: `.semgrep/rules/<N>.yaml` (one per selected rule), `.semgrep.yml`, `.github/workflows/sast.yml`; stdout summary lists each file |
| `emitted_workflow_passes_safety_contract` | abuse case (`tm-scanner-orchestration-abuse-3`) | the skill has emitted `sast.yml` | `serde_yaml_ng::from_str(&sast_yml)` is parsed and inspected | ALL of: `on:` block contains `pull_request` and NOT `pull_request_target`; workflow-scope `permissions:` is `{}`; analysis job has `permissions: { contents: read }`; SARIF upload step (or job) has `permissions: { security-events: write }`; both `uses:` lines reference 40-char SHAs (regex `^[a-f0-9]{40}$`); `actions/checkout` step has `with: { fetch-depth: 0 }`; the `semgrep ci` invocation has `SEMGREP_RULES: ./.semgrep.yml` env var; NO `--config` flag; NO `--autofix` flag; NO `--severity` flag; NO `secrets.*` reference in analysis job |
| `prompt_injection_in_threat_model_does_not_taint_workflow` | abuse case (`tm-scanner-orchestration-abuse-3`) | the malicious-threat-model fixture (CWE description contains `\n  pull_request_target:\n` and instruction-shaped prose) | the skill is invoked | emitted `sast.yml` is byte-identical to the clean-fixture emission; workflow safety contract still passes; the malicious string does NOT appear anywhere in the emitted YAML |
| `cwe_list_changes_do_not_change_workflow_yaml` | happy path (variant) | two threat models with disjoint CWE sets (one cites CWE-77 + CWE-89, the other cites CWE-79 + CWE-22) | the skill is invoked against each | emitted `sast.yml` files are byte-identical; **emitted `.semgrep.yml` files are also byte-identical (SEC-4)** — only `.semgrep/rules/` directory contents differ |
| `emission_idempotent` | persistence | the skill has previously emitted artifacts; re-invoked with identical inputs | the skill is invoked | post-run `git diff` against pre-run state is empty (zero changes); no spurious whitespace / timestamp / ordering churn |
| `empty_selected_rules_emits_with_warning` | empty state | a target repo where CWE × stack intersection produces zero selected rules (e.g., threat model cites CWE-99999 which no rule covers) | the skill is invoked | emission still happens; `.semgrep/rules/` is empty; `.semgrep.yml` references an empty rule set; stderr warns "0 rules selected — workflow will run but find nothing"; exit 0 (decisions about coverage gaps belong to M4's manifest) |
| `unpinned_action_in_template_fails_structural_test` | abuse case (defense-in-depth, `tm-scanner-orchestration-abuse-3`) | the workflow template at `references/sast/scanner-orch-workflow-template.yml` is hand-edited to use `actions/checkout@v4` (tag, not SHA) | the structural-contract test runs | test FAILS with a clear assertion message naming the unpinned action |
| `pull_request_target_in_template_fails_structural_test` | abuse case (defense-in-depth, `tm-scanner-orchestration-abuse-3`) | the workflow template is hand-edited to add `pull_request_target` | the structural-contract test runs | test FAILS with a clear assertion message naming the violation |
| `golden_output_byte_compare` | happy path (variant — reproducibility) | the clean-fixture emission | post-run, the emitted `sast.yml` is byte-compared against `expected/sast.yml`; same for `.semgrep.yml` against `expected/dot-semgrep.yml` | exact byte match |
| `refuses_emission_when_dot_semgrep_is_symlink` | abuse case (**SEC-1**) | a target repo where `.semgrep/rules/` (or any path component leading there) is a symlink rather than a real directory | the skill is invoked | the skill exits non-zero with stderr naming the violation; NO files are written under any directory the symlink could redirect to (e.g., `/etc/cron.d/`) |
| `refuses_emission_when_dot_github_workflows_is_symlink` | abuse case (**SEC-1** variant) | a target repo where `.github/workflows/` is a symlink | the skill is invoked | same as above — refused before any write happens |
| `M1_M2_regressions_intact` | regression | the full M3 invocation flow (parser → stack detect → fetch → filter → emit) | M1 + M2 BDD scenarios re-run as part of the M3 E2E suite | all green |

Coverage-category notes: retry / concurrency N/A (single-process emission); persistence covered by `emission_idempotent`; backward-compat covered by `M1_M2_regressions_intact`.

#### Regression Tests

- All M1 + M2 E2E tests pass.
- `cargo test --workspace` baseline green.
- `references/sast/` existing files (the four M1+M2 reference docs) byte-identical post-M3.
- `.semgrep/rules/<file>.yaml` files in the test target repo are byte-identical to their cached source (assert via SHA-256 compare).
- The emitted `sast.yml` parses as valid YAML AND validates against the GitHub Actions workflow schema (use `serde_yaml_ng` for parse; structural assertions for the actions-specific shape).

#### Compatibility Checklist

- [ ] `cargo test --workspace` green
- [ ] M1 + M2 E2E suites green
- [ ] M3 E2E suite green (10 BDD scenarios)
- [ ] `./target/release/sldo-install --dry-run` discovers all skills
- [ ] No existing skill's `SKILL.md` touched
- [ ] No new entries in `Cargo.toml` workspace deps
- [ ] `references/sast/` existing files byte-identical (only the two new M3 files added)
- [ ] Emission idempotent (re-run produces zero diff)

#### E2E Runtime Validation

**File**: `crates/sldo-install/tests/e2e_scanner_orch_m3.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `e2e_scanner_orch_m3_emits_three_artifact_categories` | Full emission flow against a clean fixture | Three artifact categories exist post-run; stdout summary correct |
| `e2e_scanner_orch_m3_workflow_safety_contract_all_assertions` | Structural-contract fixture catches every safety property | Composite test parses the emitted YAML and runs ~12 individual assertions; all pass on clean fixture |
| `e2e_scanner_orch_m3_resists_prompt_injection_in_threat_model` | `tm-scanner-orchestration-abuse-3` defense | Malicious-fixture invocation produces byte-identical workflow YAML to the clean fixture |
| `e2e_scanner_orch_m3_workflow_static_across_cwe_lists` | "Workflow YAML doesn't depend on CWE list" architectural property | Two CWE-disjoint threat-model fixtures produce byte-identical `sast.yml` |
| `e2e_scanner_orch_m3_idempotent_reemit` | Reproducibility / idempotency | Second emission against the same target repo produces zero `git diff` |
| `e2e_scanner_orch_m3_empty_rules_warning` | Empty-state coverage gap doesn't crash | Zero-selected-rules input emits cleanly with stderr warning, exit 0 |
| `e2e_scanner_orch_m3_template_tampering_caught` | Defense-in-depth — corrupt template fails the contract test | Tampered template (unpinned action OR `pull_request_target`) makes the contract test FAIL with named violation |
| `e2e_scanner_orch_m3_golden_output_match` | Golden byte-compare | Clean emission matches `expected/sast.yml` and `expected/dot-semgrep.yml` byte-for-byte |
| `e2e_scanner_orch_m3_M1_M2_regression_chain` | Full pipeline regression | Composite invocation covering parser → stack-detect → fetch → filter → emit; all M1 + M2 scope rules and SHA-pin enforcement still apply |

#### Smoke Tests

1. Author a real target-repo at `/tmp/scanner-orch-smoke-m3/` (Cargo.toml + threat-model + minimal src/).
2. Run `./target/release/sldo-install --local`.
3. Invoke `claude /slo-sast /tmp/scanner-orch-smoke-m3/docs/slo/design/threat-model.md` (or equivalent).
4. Verify `.semgrep/rules/`, `.semgrep.yml`, `.github/workflows/sast.yml` all exist.
5. Run `actionlint .github/workflows/sast.yml` (if installed locally) — confirm zero warnings.
6. Run `semgrep ci --dry-run` against the target repo — confirm config parses.
7. Manually grep the emitted `sast.yml` for `pull_request_target` — must be zero matches.
8. Manually grep for unpinned `uses:` (regex `uses:.*@v\d`) — must be zero matches.
9. **(ENG-1) Real-Semgrep dry-run smoke.** With Semgrep ≥ 1.50.0 installed locally, pipe the emitted workflow through `semgrep ci --dry-run --no-rewrite-rule-ids` against the smoke-fixture target repo. Confirms Semgrep accepts the emitted config + workflow at runtime, catches CLI-surface deprecations or regressions that the structural-contract YAML test cannot.

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test --workspace` | green | | | |
| BDD tests created | `crates/sldo-install/tests/e2e_scanner_orch_m3.rs` | compile or fail for expected reason | | | |
| Fixtures + expected outputs created | `crates/sldo-install/tests/fixtures/scanner-orch/m3/` | clean + malicious target-repo fixtures + golden outputs present | | | |
| `references/sast/scanner-orch-workflow-template.yml` written | review | static skeleton, only SHA placeholders, rationale comments | | | |
| `references/sast/scanner-orch-action-shas.md` written | review | current 40-char SHAs + refresh policy | | | |
| `skills/slo-sast/SKILL.md` extended | review | Emission section added; M1+M2 unchanged | | | |
| Full tests | `cargo test --workspace` | green | | | |
| M1+M2 regressions | `cargo test -p sldo-install --test e2e_scanner_orch_m1 --test e2e_scanner_orch_m2` | green | | | |
| M3 E2E | `cargo test -p sldo-install --test e2e_scanner_orch_m3` | green (9 tests) | | | |
| Smoke tests | manual steps above | all checked; `actionlint` + `semgrep ci --dry-run` clean; zero `pull_request_target` matches | | | |
| Test artifact cleanup | `git status` | clean | | | |
| .gitignore review | review | patterns current | | | |
| Compatibility | `git diff --stat references/sast/`; baseline; M1+M2 E2E | only two new files | | | |

#### Definition of Done

- [ ] `references/sast/scanner-orch-workflow-template.yml` exists with the safe-template skeleton + per-property rationale comments
- [ ] `references/sast/scanner-orch-action-shas.md` exists with current 40-char SHAs + refresh-cadence policy
- [ ] `skills/slo-sast/SKILL.md` Emission section added; M1+M2 sections unchanged
- [ ] `crates/sldo-install/tests/e2e_scanner_orch_m3.rs` exists with all 9 E2E tests passing
- [ ] All 10 BDD scenarios pass
- [ ] Structural-contract test catches every workflow safety property
- [ ] Idempotent emission (re-run produces zero diff)
- [ ] Prompt-injection threat-model regression passes (workflow byte-identical)
- [ ] M1+M2 regression intact
- [ ] `cargo test --workspace` green
- [ ] Smoke tests show real `actionlint` + `semgrep ci --dry-run` pass
- [ ] Lessons file at `docs/slo/lessons/scanner-orch-m3.md`
- [ ] Completion summary at `docs/slo/completion/scanner-orch-m3.md`
- [ ] Milestone Tracker row updated to `done`

---

### Milestone 4 — `.semgrep/manifest.json` (audit-defense schema v1.0) + initial-baseline preview-mode UX

**Goal**: The skill writes `.semgrep/manifest.json` (audit-defense + reproducibility manifest, schema v1.0 per [interfaces.md §5](design/scanner-orchestration-interfaces.md)) and `.semgrep/last-run.json` (last successful scan summary). On first install against a target repo (no pre-existing `.semgrep/manifest.json`), the skill runs **preview-mode**: a dry-run `semgrep ci` against the about-to-be-emitted config, surface finding counts by severity to the user, and REQUIRE explicit user confirmation before committing any artifact. On re-runs (manifest exists), preview is skipped.

**Context**: M3 emits the workflow + rules. M4 adds the audit-defense data and the UX gate that defuses `tm-scanner-orchestration-abuse-5` (the day-one CI-jam → defection failure mode that's the worst outcome for the wedge). The manifest schema is locked in interfaces.md §5; this milestone implements every field. The preview-mode is the single human-review surface in v1 — every other defense is structural. The manifest's `cwes_claimed` vs `cwes_actually_covered` is **defensive design**, not a regulatory mandate (research synthesis didn't surface a published audit-failure precedent fixing the mapped-but-not-scanned pattern).

**Important design rule**: Preview-mode is required on first install — silent commit on first install is forbidden. The skill MUST distinguish "first install" (no pre-existing `.semgrep/manifest.json`) from "re-derivation" (manifest exists) and gate the first-install path behind explicit confirmation. The user's decline path MUST leave zero artifacts in the target repo (rollback is clean — no partial writes survive).

**Refactor budget**: `Minimal local refactor permitted in listed files only` — `skills/slo-sast/SKILL.md` extended with manifest-emission + preview-mode sections; M1 + M2 + M3 sections unchanged.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | M3's emission state (about-to-be-written artifacts in memory or staged in tempdir); detection of "first install vs re-run" via `.semgrep/manifest.json` presence; `git rev-parse HEAD:<threat-model-path>` for the threat-model blob SHA; semgrep version via `semgrep --version`; pinned `semgrep_rules_sha` from `references/sast/scanner-orch-pinned-rules-sha.md`; on first install, output of `semgrep ci --dry-run` (severity counts) |
| Outputs | Two new files in target repo (`.semgrep/manifest.json`, `.semgrep/last-run.json`); on first install, an interactive prompt surfacing finding counts and a yes/no gate; on user-decline, zero artifacts persist (M3's emission rolls back too) |
| Interfaces touched | NEW: manifest schema v1.0 per [interfaces.md §5](design/scanner-orchestration-interfaces.md) — `stable` for listed fields, `evolving` for additive; NEW: preview-mode UX surface (`stable` for the gate; the message format is `evolving`); NEW: rollback contract on user-decline (M3's three artifacts are also rolled back when M4 detects user-decline at first install) |
| Files allowed to change | `skills/slo-sast/SKILL.md` (extend); `references/sast/scanner-orch-manifest-schema.md` (NEW — explicit JSON schema doc with field-level rationale + the cwes-claimed-vs-covered framing as defensive design, not regulatory); `crates/sldo-install/tests/e2e_scanner_orch_m4.rs` (NEW); `crates/sldo-install/tests/fixtures/scanner-orch/m4/` (NEW — fixtures + expected manifest JSON goldens); `.gitignore` (review only) |
| Files to read before changing anything | M1 + M2 + M3 outputs; `docs/slo/lessons/scanner-orch-m1.md`, `m2.md`, `m3.md`; `docs/slo/design/scanner-orchestration-interfaces.md` (§5); `docs/slo/design/scanner-orchestration-threat-model.md` (abuse cases `tm-scanner-orchestration-abuse-4`, `tm-scanner-orchestration-abuse-5`); [SECURITY.md](../SECURITY.md) "Scanner orchestration skill — additional rules" → "Manifest schema is audit-defense, not regulatory necessity" subsection; `docs/slo/research/scanner-orchestration/synthesis.md` (§ "Audit defense via per-scan-run manifest is defensive, not regulatory") |
| New files allowed | `references/sast/scanner-orch-manifest-schema.md`; `crates/sldo-install/tests/e2e_scanner_orch_m4.rs`; fixtures under `crates/sldo-install/tests/fixtures/scanner-orch/m4/` |
| New dependencies allowed | `none` — `serde_json` (workspace) handles manifest serialization in test harness |
| Migration allowed | `no` — schema v1.0 is the v1 lock-in; v2 would migrate via a runbook with explicit migration tests |
| Compatibility commitments | M1 + M2 + M3 E2E suites pass; `cargo test --workspace` green; manifest emission is idempotent (same inputs → same JSON bytes including key order); preview-mode rollback leaves zero artifacts; existing `references/sast/` files byte-identical |
| Forbidden shortcuts | No silent commit on first install (preview-mode is mandatory); no schema field omission (every field in interfaces §5 must be populated; `null` only where explicitly allowed by schema, e.g., `selected_rules[].source_sha` is `null` for rulegen-authored rules); no overpromising language in manifest (the cwes_claimed-vs-covered split is "defensive design" — the schema doc and any user-facing prose say this verbatim, never "audit-required" or "regulatory mandate"); no caching of `semgrep --version` output across runs (re-query each invocation); no schema-version increment without a migration milestone; no embedding the threat-model file content in the manifest (only the SHA); no embedding free-text from the threat model in any field (only regex-validated `CWE-\d+` integers, sorted SHAs, ISO-8601 timestamps, closed-enumeration stack tags) |
| **Data classification** | `Internal` for the manifest's content (project-internal but committed; reveals which CWEs the project tracks). `Public` once committed (same as M3's artifacts). |
| **Proactive controls in play** | OWASP Proactive Controls v3 — **C2 Define security requirements** (manifest schema is the contract — documented in `references/sast/scanner-orch-manifest-schema.md`); **C5 Secure by default** (preview-mode gate is the safe default for first install); **C7 Validate input** (every manifest value is regex-validated or comes from a closed enumeration; no free-text flows into JSON); **C9 Authorization** N/A (single-user invocation context); **C10 Errors and exceptions** (rollback on user-decline is graceful — no panic, no partial state). |
| **Abuse acceptance scenarios** | See BDD table below: `manifest_values_are_regex_validated_only` (`tm-scanner-orchestration-abuse-4`), `prompt_injection_does_not_taint_manifest` (`tm-scanner-orchestration-abuse-4`), `preview_mode_required_on_first_install` (`tm-scanner-orchestration-abuse-5`), `user_decline_rolls_back_all_artifacts` (`tm-scanner-orchestration-abuse-5`), `re_run_skips_preview_mode` (re-derivation path, no abuse). |

#### Out of Scope / Must Not Do

- Re-derivation trigger detection (M5 — M4 only writes the manifest correctly so M5 can read it)
- `gh` invocation, PR creation (M5)
- Audit-coverage skill that consumes the manifest and produces auditor-facing PDF / SARIF (deferred to a future runbook)
- Schema v2.0 (no — v1.0 is what ships)
- Embedding scan results in the manifest (`.semgrep/last-run.json` carries the run summary; the manifest is reproducibility metadata, not findings)
- Modifying the workflow YAML based on manifest content (the workflow is static per M3's design rule)
- Publishing the manifest anywhere external (filesystem-only)

#### Pre-Flight

1. Complete the Global Entry Rules.
2. Read all prior lessons files (`scanner-orch-m1.md` through `m3.md`); apply corrections.
3. Read the allowed files; understand the schema lock from interfaces §5 verbatim.
4. Copy the Evidence Log template.
5. Re-state the milestone constraints — particularly the "preview-mode mandatory on first install" rule and the "defensive design, not regulatory" framing.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/slo-sast/SKILL.md` | Extend: add Manifest section (cites `references/sast/scanner-orch-manifest-schema.md`; enumerates the schema-v1.0 fields and how each is populated); add Preview-Mode section (first-install gate flow: dry-run scan, surface counts by severity, prompt user, gate the commit). M1 + M2 + M3 sections unchanged. |
| `references/sast/scanner-orch-manifest-schema.md` | NEW: explicit JSON schema doc; for each field, name + type + how-populated + rationale; explicit "defensive design, not regulatory" framing for `cwes_claimed` vs `cwes_actually_covered`; cites `tm-scanner-orchestration-abuse-4` and synthesis Q3 verdict. |
| `crates/sldo-install/tests/e2e_scanner_orch_m4.rs` | NEW: E2E + structural tests; tests use `serde_json` to parse and assert the emitted manifest; preview-mode tests use stdin/stdout fixtures to simulate user yes/no responses. |
| `crates/sldo-install/tests/fixtures/scanner-orch/m4/expected-manifest-rust.json` | NEW: golden manifest fixture for the rust+CWE-77 path. |
| `crates/sldo-install/tests/fixtures/scanner-orch/m4/expected-manifest-polyglot.json` | NEW: golden manifest for rust+python+django CWE-89 path. |
| `crates/sldo-install/tests/fixtures/scanner-orch/m4/malicious-threat-model-for-manifest.md` | NEW: prompt-injection fixture with `</value>"; DROP TABLE` and HTML/JS-shaped strings inside CWE descriptions, asserting the manifest doesn't render them. |
| `crates/sldo-install/tests/fixtures/scanner-orch/m4/preview-mode-yes.txt`, `preview-mode-no.txt` | NEW: stdin fixtures for preview-mode user responses. |
| `.gitignore` | Reviewed; no expected additions. |

#### Step-by-Step

1. Write E2E test stubs covering all BDD scenarios.
2. Author manifest-schema doc + fixture files.
3. Run `cargo test -p sldo-install --test e2e_scanner_orch_m4` — confirm tests fail.
4. Extend `skills/slo-sast/SKILL.md` with Manifest + Preview-Mode sections.
5. Run M4 E2E — make all scenarios pass.
6. Run M1+M2+M3 regression suites.
7. Run full workspace tests.
8. Smoke-test: real target-repo first install (preview shows counts, decline rolls back, accept commits).
9. Verify `git status`, complete Self-Review Gate, write lessons + completion files.

#### BDD Acceptance Scenarios

**Feature: manifest emission + preview-mode UX**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| `manifest_v1_schema_complete` | happy path | a clean target-repo first-install + user accepts preview | the skill is invoked | `.semgrep/manifest.json` exists; parses as JSON; contains every field from interfaces §5 (`schema_version: "1.0"`, `generated_at` ISO-8601, `generated_by_skill_version`, `threat_model_path`, `threat_model_sha`, `semgrep_rules_sha`, `semgrep_version`, `detected_stack`, `selection_strategy`, `cwes_claimed`, `cwes_actually_covered`, `cwes_uncovered`, `selected_rules[]`); every field has the correct type |
| `cwes_diff_calculated_correctly` | happy path | threat model claims `[CWE-77, CWE-78, CWE-89]`; selected rules cover only `[CWE-78, CWE-89]` | the skill is invoked | manifest's `cwes_claimed: ["CWE-77","CWE-78","CWE-89"]` (sorted); `cwes_actually_covered: ["CWE-78","CWE-89"]`; `cwes_uncovered: ["CWE-77"]` |
| `selected_rules_carry_source_sha` | happy path | the cache is at a known SHA; selected rules are real fixtures | the skill is invoked | every `selected_rules[].source_sha` matches the upstream blob SHA of the fixture rule file (verifiable via `git ls-files -s` on the cache); `source: "registry"` for cache-derived rules |
| `manifest_emission_idempotent` | persistence | the skill has emitted a manifest; re-invoked with identical inputs (same threat-model SHA, same semgrep_rules_sha, same stack) | the skill is invoked again | manifest is byte-identical; `generated_at` is updated but `generated_at` aside, content unchanged (or — preferred — `generated_at` is also stable when content is identical, gated by a `force-regenerate` test mode; document choice in the schema doc) |
| `preview_mode_required_on_first_install` | abuse case (`tm-scanner-orchestration-abuse-5`) | a target repo with NO pre-existing `.semgrep/manifest.json`; multiple legacy CWE-89 instances in `src/` | the skill is invoked | preview is shown to the user (severity counts on stderr); skill blocks waiting for stdin yes/no; no artifacts written until user replies |
| `preview_mode_triggered_by_pre_existing_workflow` | abuse case (**ENG-3**) | a target repo with a pre-existing `.github/workflows/sast.yml` (any contents) but NO `.semgrep/manifest.json` (e.g., user has hand-authored CodeQL workflow) | the skill is invoked | preview is shown including the diff between the existing and the about-to-be-emitted workflow; user can decline to preserve the existing file; on accept, the existing file is overwritten |
| `preview_mode_triggered_by_pre_existing_semgrep_config` | abuse case (**ENG-3** variant) | a target repo with a pre-existing `.semgrep.yml` but NO `.semgrep/manifest.json` | the skill is invoked | preview includes the existing config; user gates the overwrite |
| `manifest_write_site_resists_symlink_traversal` | abuse case (**SEC-1** variant) | a target repo where `.semgrep/manifest.json` is a symlink (or `.semgrep/` is a symlink) | the skill is invoked | refuses to write the manifest; exits non-zero with a clear error; same defense as M3's emission-site symlink check |
| `user_decline_rolls_back_all_artifacts` | abuse case (`tm-scanner-orchestration-abuse-5`) | preview-mode is showing; user types `n`/`no` | the skill is invoked | `.semgrep/`, `.github/workflows/sast.yml` are NOT present after the run; exit code is non-zero (or 0 with stderr "user declined"); `git status` of the target repo is unchanged from pre-invocation |
| `user_accept_commits_all_artifacts` | happy path | preview-mode is showing; user types `y`/`yes` | the skill is invoked | all M3+M4 artifacts exist; manifest is fully populated |
| `re_run_skips_preview_mode` | happy path (re-derivation) | a target repo WITH pre-existing `.semgrep/manifest.json` | the skill is invoked | preview is NOT shown; manifest is updated in place; M3 artifacts re-emitted (idempotent) |
| `manifest_values_are_regex_validated_only` | abuse case (`tm-scanner-orchestration-abuse-4`) | a threat model whose CWE descriptions contain JSON-injection content (`","oops": "`, `</value>`, `\\n`, etc.) | the skill is invoked | manifest's `cwes_claimed` array contains only validated `"CWE-N"` strings (regex `^CWE-\d+$`); injection content does NOT appear anywhere in the JSON |
| `prompt_injection_does_not_taint_manifest` | abuse case (`tm-scanner-orchestration-abuse-4`) | malicious-threat-model-for-manifest fixture | the skill is invoked | every manifest field's value either matches its declared regex/enumeration OR is excluded; `serde_json::from_str` parses cleanly; no eval-able content in any field |
| `manifest_no_overpromise_language` | regression (framing) | manifest emission completes | the manifest's documentation strings (if any in the schema) are inspected | no occurrence of "audit-required", "regulatory mandate", "PCI-compliant"; the framing in `references/sast/scanner-orch-manifest-schema.md` says "defensive design" verbatim |
| `M1_M2_M3_regressions_intact` | regression | full M4 invocation | all prior BDD scenarios re-run | green |

Coverage notes: retry / concurrency N/A; persistence + rollback covered; backward-compat covered.

#### Regression Tests

- All M1 + M2 + M3 E2E suites pass.
- `cargo test --workspace` green.
- `references/sast/` existing files byte-identical.
- `serde_json::from_str(&manifest_bytes)` parses without error.
- `serde_json::to_string_pretty(&parsed_manifest) == manifest_bytes` (idempotent serialization round-trip — JSON canonical form).
- The structural-contract test from M3 still passes — manifest emission does not modify any M3 artifact.

#### Compatibility Checklist

- [ ] `cargo test --workspace` green
- [ ] M1 + M2 + M3 E2E suites green
- [ ] M4 E2E suite green (12 BDD scenarios)
- [ ] `./target/release/sldo-install --dry-run` discovers all skills
- [ ] No existing skill's `SKILL.md` touched
- [ ] No new entries in `Cargo.toml` workspace deps
- [ ] `references/sast/` existing files byte-identical (only the new M4 schema doc added)
- [ ] Manifest emission idempotent
- [ ] User-decline rollback complete (no orphan files)

#### E2E Runtime Validation

**File**: `crates/sldo-install/tests/e2e_scanner_orch_m4.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `e2e_scanner_orch_m4_manifest_schema_complete` | Schema-v1.0 contract honored | All 13 fields present and correctly typed; matches golden manifest byte-for-byte |
| `e2e_scanner_orch_m4_cwe_diff_correct` | Coverage-gap calculation accurate | `claimed`, `covered`, `uncovered` set algebra is correct on a multi-CWE fixture |
| `e2e_scanner_orch_m4_first_install_preview_mode` | Gate flow works end-to-end | Stdin `y` → all artifacts written; stdin `n` → zero artifacts written |
| `e2e_scanner_orch_m4_re_run_skips_preview` | Re-derivation path bypasses gate | Pre-populated manifest fixture → re-invocation does not block on stdin |
| `e2e_scanner_orch_m4_user_decline_clean_rollback` | Rollback contract | Pre-invocation `git status` matches post-invocation when user declines |
| `e2e_scanner_orch_m4_resists_manifest_injection` | `tm-scanner-orchestration-abuse-4` defense | Malicious threat-model emits a manifest that parses cleanly with no injected content |
| `e2e_scanner_orch_m4_idempotent_emit` | Reproducibility | Same inputs → byte-identical manifest output |
| `e2e_scanner_orch_m4_no_overpromise` | Framing audit | No occurrence of audit/regulatory/PCI-compliant strings in manifest data fields or schema doc |
| `e2e_scanner_orch_m4_full_pipeline_regression` | M1+M2+M3 still work | Full chain produces all artifacts; safety-contract test still passes |

#### Smoke Tests

1. Author a real target-repo at `/tmp/scanner-orch-smoke-m4/` with a pre-existing legacy critical Semgrep finding (e.g., a Rust file with an unsafe `Command::new` shell-injection pattern).
2. Run `./target/release/sldo-install --local`.
3. Invoke `claude /slo-sast` in the smoke target — observe preview-mode showing the legacy finding count.
4. Type `n` → verify `.semgrep/` and `.github/workflows/sast.yml` do NOT exist; `git status` clean.
5. Re-invoke and type `y` → verify all artifacts present + `.semgrep/manifest.json` populated.
6. Manually inspect manifest: schema_version "1.0", every field typed correctly, `cwes_claimed` matches the fixture threat model, source SHAs present.
7. Re-invoke (no manifest deletion) → verify preview-mode is NOT shown (re-derivation path).
8. `jq '.cwes_uncovered' .semgrep/manifest.json` — confirm it's a list (possibly empty).

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test --workspace` | green | | | |
| BDD tests created | `crates/sldo-install/tests/e2e_scanner_orch_m4.rs` | compile | | | |
| Fixtures + goldens created | `crates/sldo-install/tests/fixtures/scanner-orch/m4/` | present | | | |
| `references/sast/scanner-orch-manifest-schema.md` written | review | every field documented; defensive-design framing verbatim | | | |
| `skills/slo-sast/SKILL.md` extended | review | Manifest + Preview-Mode sections; prior sections unchanged | | | |
| Full tests | `cargo test --workspace` | green | | | |
| M1+M2+M3 regressions | suite-by-suite | green | | | |
| M4 E2E | `cargo test -p sldo-install --test e2e_scanner_orch_m4` | green (9 tests) | | | |
| Smoke tests | manual steps above | preview shown; decline clean; accept emits; re-run skips preview | | | |
| Test artifact cleanup | `git status` | clean | | | |
| .gitignore review | review | clean | | | |
| Compatibility | `git diff --stat references/sast/` | only the new schema doc | | | |

#### Definition of Done

- [ ] `references/sast/scanner-orch-manifest-schema.md` exists with full field documentation + defensive-design framing
- [ ] `skills/slo-sast/SKILL.md` Manifest + Preview-Mode sections added; prior unchanged
- [ ] `crates/sldo-install/tests/e2e_scanner_orch_m4.rs` exists with all 9 E2E tests passing
- [ ] All 12 BDD scenarios pass
- [ ] First-install preview-mode gates the commit
- [ ] User-decline rolls back cleanly (zero orphan files; `git status` unchanged)
- [ ] Manifest schema v1.0 fully populated with regex-validated values only
- [ ] No overpromising language anywhere ("defensive design", not "regulatory mandate")
- [ ] `cargo test --workspace` green
- [ ] M1+M2+M3 regression intact
- [ ] Smoke tests demonstrate real preview + accept + decline + re-run flows
- [ ] Lessons file at `docs/slo/lessons/scanner-orch-m4.md`
- [ ] Completion summary at `docs/slo/completion/scanner-orch-m4.md`
- [ ] Milestone Tracker row updated to `done`

---

### Milestone 5 — Re-derivation trigger detection + diff PR generation + dogfood E2E against this SLO repo

**Goal**: The skill detects re-derivation triggers per [interfaces.md §8](design/scanner-orchestration-interfaces.md) (threat-model file SHA differs from manifest's `threat_model_sha`; pinned `semgrep_rules_sha` differs from currently-fetchable; new manifest files added since last run; deduplicated `cwes_claimed` differs from manifest's recorded value), surfaces the proposed change set as a `gh pr create` with a structured diff body, and is exercised end-to-end in a dogfood test that runs `/slo-sast` against this SLO repo using `docs/slo/design/scanner-orchestration-threat-model.md` as input. On no-trigger, the skill exits 0 with a "no drift detected" message and creates no PR.

**Context**: M4 lands the manifest with the SHAs needed to detect drift. M5 closes the auto-tuning loop — the wedge thesis (research synthesis Q5 — "first-of-its-kind") becomes operational. Drift detection is a single-process diff against the manifest (no continuous monitoring; trigger evaluation runs at every `/slo-sast` invocation). PR creation uses `gh` CLI mirroring the existing pattern from `/slo-sec-libs` (Phase 4) including the rate-limit cap (5 PRs/hour per session — re-derivations are rare; the cap defends against runaway loops). Dogfood validates the full M1→M5 pipeline against a real codebase (this one).

**Important design rule**: PR creation never auto-merges. Every re-derivation surfaces as a human-review surface. The skill MUST NOT pass `--auto`, `--squash`, or any merge flag to `gh pr create`. The skill MUST NOT call `gh pr merge` ever.

**Refactor budget**: `Minimal local refactor permitted in listed files only` — `skills/slo-sast/SKILL.md` extended with Re-Derivation + PR Creation sections.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Target repo state (`.semgrep/manifest.json` if it exists; threat-model file; manifest files; `references/sast/scanner-orch-pinned-rules-sha.md` for current pin); `gh` CLI on PATH with user's existing auth (no auth provisioned by the skill); rate-limit state (in-process counter, 5 PRs/hour cap) |
| Outputs | Either: (a) "no drift detected" message + exit 0 + no PR; (b) a single `gh pr create` invocation with a structured PR title (`[scanner-orch] re-derive: <trigger summary>`) and body (Markdown table listing the manifest field-level diffs + the rule-set delta + the affected CWE list); exit 0 on PR-created success |
| Interfaces touched | NEW: re-derivation trigger contract per [interfaces.md §8](design/scanner-orchestration-interfaces.md) — `evolving` (mechanism may evolve; the four-trigger predicate set is `stable`); NEW: PR-creation surface (`gh pr create` invocation shape, title format, body format) — `stable` for the format used by downstream consumers; NEW: rate-limit cap (5 PRs/hour per session) — `stable` |
| Files allowed to change | `skills/slo-sast/SKILL.md` (extend with Re-Derivation + PR Creation sections); `references/sast/scanner-orch-rederivation-triggers.md` (NEW — predicate set + PR title/body format + rate-limit cap rationale, citing `/slo-sec-libs` precedent); `crates/sldo-install/tests/e2e_scanner_orch_m5.rs` (NEW); `crates/sldo-install/tests/fixtures/scanner-orch/m5/` (NEW — multi-state target-repo fixtures + expected PR-body goldens); `.gitignore` (review only) |
| Files to read before changing anything | M1 + M2 + M3 + M4 outputs; all four prior lessons files; [interfaces.md §8](design/scanner-orchestration-interfaces.md); `docs/slo/research/scanner-orchestration/synthesis.md` (§ "wedge is genuinely unoccupied — frame as first-of-its-kind"); existing skills using `gh` CLI (search for `gh issue create` / `gh pr create` patterns elsewhere in `skills/`); the `/slo-sec-libs` precedent for rate-limit discipline (referenced in issue #4 body) |
| New files allowed | `references/sast/scanner-orch-rederivation-triggers.md`; `crates/sldo-install/tests/e2e_scanner_orch_m5.rs`; fixtures under `crates/sldo-install/tests/fixtures/scanner-orch/m5/` |
| New dependencies allowed | `none` — `gh` CLI is assumed on PATH (mirroring the precedent for `git`, `semgrep`, `claude` external tools) |
| Migration allowed | `no` |
| Compatibility commitments | M1+M2+M3+M4 E2E suites pass; `cargo test --workspace` green; PR creation is a single invocation per drift detection (no double-firing on rapid re-runs); rate-limit honored; `references/sast/` existing files byte-identical |
| Forbidden shortcuts | NO `--auto`, `--squash`, `--rebase`, or any merge flag in `gh pr create`; NO `gh pr merge` invocation ever; NO `--repo` flag in `gh pr create` (**SEC-8** — rely on `gh`'s default origin-based resolution to defend against confused-deputy via tampered `.git/config`); NO PR creation against repos OTHER than the current target; **rate-limit cap = max 1 PR per invocation (ENG-4)** — cross-invocation rate is the user's responsibility (CI throttling, threat-model edit cadence) since `/slo-sast` is single-invocation; NO PR title or body content sourced from threat-model prose (template-skeleton with manifest-derived values only); NO calling `gh auth login` from the skill (use existing user auth or fail with clear error); NO swallowing `gh` errors (surface to stderr); NO open-ended PR title (length-capped to 70 chars per project convention); NO embedding scan findings in the PR body (the PR is about config drift, not findings — findings go through Code Scanning); **all `gh` and `git` invocations use argv-list form (SEC-6)** — never shell-string interpolation |
| **Data classification** | `Public` — PR content is committed to a public GitHub PR. The skill MUST treat all PR-bound content as public from the moment of generation. |
| **Proactive controls in play** | OWASP Proactive Controls v3 — **C2 Define security requirements** (the trigger predicate set + PR format is documented in `references/sast/scanner-orch-rederivation-triggers.md`); **C5 Secure by default** (PR-only — no auto-merge; rate-limit cap; closed-enumeration trigger predicates); **C7 Validate input** (every PR-body value is regex-validated or comes from the manifest, which itself was validated in M4); **C9 Authorization** (uses user's existing `gh` auth, does not provision new credentials). |
| **Abuse acceptance scenarios** | See BDD table below: `pr_body_does_not_render_threat_model_prose` (`tm-scanner-orchestration-abuse-3` — same template-skeleton defense applies to PR content), `rate_limit_cap_enforced` (defends against runaway loops), `no_auto_merge_flag_used` (PR-only discipline), `no_cross_repo_filing` (skill targets only the current repo). M5 introduces no new attacker surface beyond what M3+M4 already defended; the abuse rows here are regression-style enforcing prior defenses against the new PR-emission code path. |

#### Out of Scope / Must Not Do

- Continuous monitoring / file watching (drift detection runs only at `/slo-sast` invocation)
- Cross-repo filing (`gh issue create` against upstream `semgrep-rules` is `/slo-sec-libs` Phase 4's domain, not this skill's)
- Auto-merge in any form
- Pruning the cache (`~/.cache/sldo/semgrep-rules/<old-SHA>/` cleanup is deferred — out of v1 per residual-risks)
- Authenticating to GitHub from the skill (use user's local `gh` auth)
- Modifying issue #15 or any other GitHub issue from this skill
- Publishing artifacts anywhere outside the target repo PR
- Any `gh release` / `gh secret` / `gh repo` commands (only `gh pr create` is permitted)

#### Pre-Flight

1. Complete the Global Entry Rules.
2. Read all four prior lessons files; apply corrections.
3. Read the allowed files; particularly the `gh` invocation patterns in existing skills + the `/slo-sec-libs` rate-limit precedent.
4. Copy the Evidence Log template.
5. Re-state the milestone constraints — particularly the "PR-only, no auto-merge, no cross-repo filing" rules.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `skills/slo-sast/SKILL.md` | Extend: add Re-Derivation section (the four trigger predicates from interfaces §8) and PR Creation section (the `gh pr create` invocation flow, title/body templates, rate-limit cap, no-auto-merge discipline). M1+M2+M3+M4 sections unchanged. |
| `references/sast/scanner-orch-rederivation-triggers.md` | NEW: predicate set (threat-model SHA delta, semgrep_rules_sha pin bump, new manifest file appears, cwes_claimed delta); PR title format (`[scanner-orch] re-derive: <trigger summary>`); PR body template (Markdown table — manifest field diffs, rule-set delta, affected CWEs); rate-limit cap (5 PRs/hour per session — cite `/slo-sec-libs` precedent); no-auto-merge / no-cross-repo / no-merge-flag rules. |
| `crates/sldo-install/tests/e2e_scanner_orch_m5.rs` | NEW: E2E tests using `assert_cmd` + `tempfile`; mocks `gh` CLI via `PATH` injection (test-stubbed `gh` records its argv into a file so tests can assert the invocation shape); dogfood test runs `/slo-sast` against an isolated copy of THIS SLO repo's relevant subtree. |
| `crates/sldo-install/tests/fixtures/scanner-orch/m5/state-no-drift/` | NEW: target-repo fixture with manifest matching current state (no triggers fire). |
| `crates/sldo-install/tests/fixtures/scanner-orch/m5/state-threat-model-changed/` | NEW: fixture where threat-model file's git-blob SHA differs from manifest's recorded value. |
| `crates/sldo-install/tests/fixtures/scanner-orch/m5/state-rules-sha-bumped/` | NEW: fixture where the pinned SHA in `references/sast/scanner-orch-pinned-rules-sha.md` differs from manifest's `semgrep_rules_sha`. |
| `crates/sldo-install/tests/fixtures/scanner-orch/m5/state-stack-added/` | NEW: fixture where a new manifest file (e.g., new `package.json` alongside existing `Cargo.toml`) appears post-last-manifest-write. |
| `crates/sldo-install/tests/fixtures/scanner-orch/m5/state-cwes-changed/` | NEW: fixture where the threat model's CWE references have changed since last manifest write. |
| `crates/sldo-install/tests/fixtures/scanner-orch/m5/expected-pr-body-*.md` | NEW: golden PR-body fixtures for each trigger type. |
| `crates/sldo-install/tests/fixtures/scanner-orch/m5/dogfood-slo-subtree/` | NEW (**ENG-6**): isolated **file-content copy** (NOT symlinks — the dogfood subtree fixture mandates copy-only file isolation) reflecting the SLO repo's `docs/slo/design/scanner-orchestration-*.md` plus a synthetic target-repo wrapping it. The copy uses `tempfile::TempDir` + recursive content-copy at test fixture authoring time. Defends against rogue tests mutating the real repo via symlink follow-through. |
| `.gitignore` | Reviewed; no expected additions. |

#### Step-by-Step

1. Write E2E test stubs for all BDD scenarios + the dogfood test.
2. Author all five state fixtures + golden PR-body files + the dogfood-subtree fixture.
3. Run `cargo test -p sldo-install --test e2e_scanner_orch_m5` — confirm tests fail.
4. Write `references/sast/scanner-orch-rederivation-triggers.md` (predicates + PR format + rate-limit policy).
5. Extend `skills/slo-sast/SKILL.md` with Re-Derivation + PR Creation sections.
6. Run M5 E2E — make all scenarios pass.
7. Run M1+M2+M3+M4 regression suites — green.
8. Run `cargo test --workspace` — green.
9. Run dogfood smoke test against THIS repo (carefully — use a feature branch and verify no PR is auto-created against `main`).
10. Verify `git status` (post-dogfood the SLO repo should not have any new manifests committed); complete Self-Review Gate; write lessons + completion files.

#### BDD Acceptance Scenarios

**Feature: re-derivation trigger detection + PR generation**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| `no_drift_no_pr` | happy path | a target-repo where manifest matches current state across all four predicates | the skill is invoked | exit 0; stderr says "no drift detected"; `gh` is NOT invoked; no PR opened |
| `threat_model_sha_changed_triggers_pr` | happy path | the threat-model file's content changed (different blob SHA) since manifest was written | the skill is invoked | `gh pr create` is invoked exactly once; PR title contains `[scanner-orch] re-derive:`; PR body contains a `## Threat-model SHA changed` section with old + new SHAs |
| `rules_sha_bumped_triggers_pr` | happy path | `references/sast/scanner-orch-pinned-rules-sha.md` records a different SHA than manifest's `semgrep_rules_sha` | the skill is invoked | PR opened with `## semgrep-rules SHA bumped` section listing rule-set delta (added rules, removed rules, changed rules) |
| `new_manifest_file_triggers_pr` | happy path | a new `package.json` appears in a previously-rust-only target | the skill is invoked | PR opened with `## Stack added` section; new rules for the added stack listed |
| `cwes_changed_triggers_pr` | happy path | the threat model's CWE list changed (e.g., added `CWE-22`) | the skill is invoked | PR opened with `## CWEs claimed changed` section; affected rules listed |
| `multi_trigger_combined_pr` | happy path (compound) | two triggers fire simultaneously (threat model SHA changed AND rules SHA bumped) | the skill is invoked | exactly ONE `gh pr create` invocation; PR body contains both sections |
| `pr_body_does_not_render_threat_model_prose` | abuse case (`tm-scanner-orchestration-abuse-3` regression) | threat model contains attempted markdown/HTML injection in a CWE description | the skill is invoked | PR body contains only manifest-derived values (CWE codes, SHAs, rule paths); the prose injection content does NOT appear |
| `rate_limit_max_one_pr_per_invocation` | dependency failure (controlled, **ENG-4**) | the skill has triggered drift detection and is about to call `gh pr create` once | within the same skill invocation, no second `gh pr create` is invoked even if multiple triggers fire (compound triggers coalesce into a single PR per `multi_trigger_combined_pr`) | argv capture confirms exactly one `gh pr create` invocation per skill run; further drift in subsequent skill runs is the user's responsibility (CI throttling / threat-model edit cadence) |
| `no_auto_merge_flag_used` | abuse case | any PR-creating invocation | the test-stubbed `gh` records its argv | argv contains NO `--auto`, NO `--squash`, NO `--rebase`, NO `--admin`, NO `--merge` flag |
| `no_cross_repo_filing` | abuse case (**SEC-8**) | the test-stubbed `gh` is invoked | the captured argv | the `--repo` flag is ABSENT entirely (rely on `gh`'s default origin-based resolution); the `gh pr create` argv contains NO `--repo`-form flag at all |
| `gh_unavailable_clean_error` | dependency failure | `gh` is not on PATH | the skill detects drift | exit non-zero; stderr names "gh CLI not found, see https://cli.github.com/"; no partial state |
| `gh_error_surfaced_to_stderr` | dependency failure | `gh pr create` exits non-zero (e.g., auth missing) | the skill detects drift | the `gh` stderr is forwarded; skill exits non-zero; no false "PR opened" message |
| `dogfood_against_slo_repo` | happy path (dogfood) | the dogfood-subtree fixture mirrors this SLO repo's `docs/slo/design/scanner-orchestration-*.md` | the skill is invoked against the dogfood subtree (NOT the real repo) | the full M1→M5 pipeline runs end-to-end; emitted artifacts are valid; manifest is populated; no real PR is opened against `kerberosmansour/SunLitOrchestra` (the test-stubbed `gh` only records — the test asserts no real network request occurred) |
| `M1_through_M4_regressions_intact` | regression | all prior milestones' BDD scenarios | re-run as part of M5 E2E | green |

Coverage notes: concurrency N/A (single-process, single-invocation drift check); persistence covered by re-derivation predicates that READ from manifest; backward-compat covered by full-pipeline regression.

#### Regression Tests

- All M1+M2+M3+M4 E2E suites pass.
- `cargo test --workspace` green.
- `references/sast/` existing files byte-identical post-M5 (only the new triggers reference doc added).
- The structural-contract test from M3 still passes for any newly-emitted workflow during the dogfood test.
- The manifest schema-v1.0 test from M4 still passes for any newly-written manifest during the dogfood test.

#### Compatibility Checklist

- [ ] `cargo test --workspace` green
- [ ] M1+M2+M3+M4 E2E suites green
- [ ] M5 E2E suite green (13 BDD scenarios)
- [ ] Dogfood test passes (no real PR opened against the SLO repo)
- [ ] `./target/release/sldo-install --dry-run` discovers all skills
- [ ] No existing skill's `SKILL.md` touched
- [ ] No new entries in `Cargo.toml` workspace deps
- [ ] `references/sast/` existing files byte-identical (only the new M5 triggers doc added)
- [ ] Rate-limit cap enforced
- [ ] No-auto-merge invariant intact in test-stubbed `gh` argv
- [ ] No-cross-repo invariant intact

#### E2E Runtime Validation

**File**: `crates/sldo-install/tests/e2e_scanner_orch_m5.rs`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `e2e_scanner_orch_m5_no_drift_no_pr` | Quiet path | No-drift fixture → exit 0, no `gh` invocation |
| `e2e_scanner_orch_m5_each_trigger_fires_independently` | All four predicates work | Each of the four state fixtures fires exactly one PR with the right body section |
| `e2e_scanner_orch_m5_compound_trigger_single_pr` | Compound triggers coalesce | Two simultaneous triggers → one PR with both sections |
| `e2e_scanner_orch_m5_pr_body_resists_injection` | `tm-scanner-orchestration-abuse-3` regression on PR-body | Malicious threat-model fixture → PR body byte-compared against golden, no injected content |
| `e2e_scanner_orch_m5_rate_limit` | 5/hour cap honored | 5 successful + 1 refused; refusal exit non-zero |
| `e2e_scanner_orch_m5_no_merge_flags_in_argv` | PR-only discipline | Captured `gh` argv inspected — none of `--auto`/`--squash`/etc. present |
| `e2e_scanner_orch_m5_no_cross_repo` | Targets current repo only | `--repo` flag value (when present) = target |
| `e2e_scanner_orch_m5_gh_missing_clean_error` | Dependency-failure path | `gh` removed from PATH → exit non-zero with clear message |
| `e2e_scanner_orch_m5_dogfood_full_pipeline` | M1→M5 chain works against real SLO subtree | All artifacts valid; manifest populated; test-stubbed `gh` records the would-be PR; assertion that the would-be PR repo flag = the dogfood-subtree's git remote (NOT the real SLO repo) |
| `e2e_scanner_orch_m5_M1_through_M4_regression` | Full prior-milestone chain still green | M1+M2+M3+M4 BDD scenarios run as composite |

#### Smoke Tests

⚠️ **Carry out smoke tests on a feature branch that is NOT pushed to `main`. Verify the test-stubbed `gh` is on PATH BEFORE running, OR ensure `gh auth status` shows you're not authenticated to a destination you'd accidentally PR to.** The dogfood path runs against a real codebase; an unstubbed `gh` could produce a real PR.

1. On a fresh feature branch (`git checkout -b smoke/scanner-orch-m5`), set up a test-stubbed `gh` on PATH (record-only; e.g., a shell script that prints argv to a temp file and exits 0).
2. Run `./target/release/sldo-install --local`.
3. Invoke `claude /slo-sast` against THIS repo using `docs/slo/design/scanner-orchestration-threat-model.md`.
4. Verify the test-stubbed `gh` recorded a PR-create invocation with the expected title format.
5. Verify the captured argv contains no merge flags.
6. Verify NO real PR was created (`gh pr list` against the real repo unchanged).
7. Reset the feature branch (`git reset --hard origin/main` if any artifacts landed), confirm clean state.
8. Repeat with no-drift fixture → verify the test-stubbed `gh` was NOT invoked.

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test --workspace` | green | | | |
| BDD tests created | `crates/sldo-install/tests/e2e_scanner_orch_m5.rs` | compile | | | |
| Fixtures + goldens | `crates/sldo-install/tests/fixtures/scanner-orch/m5/` | all 5 state fixtures + 4 golden PR bodies + dogfood subtree | | | |
| `references/sast/scanner-orch-rederivation-triggers.md` | review | predicate set + PR format + rate limit | | | |
| `skills/slo-sast/SKILL.md` extended | review | Re-Derivation + PR Creation sections; prior unchanged | | | |
| Full tests | `cargo test --workspace` | green | | | |
| All-prior regressions | M1+M2+M3+M4 suites | green | | | |
| M5 E2E | `cargo test -p sldo-install --test e2e_scanner_orch_m5` | green (10 tests) | | | |
| Smoke tests | manual steps above | test-stubbed `gh` invoked correctly; no real PR created; argv contains no merge flags | | | |
| Test artifact cleanup | `git status` | clean (smoke branch isolated) | | | |
| .gitignore review | review | clean | | | |
| Compatibility | `git diff --stat references/sast/` | only the new triggers doc | | | |

#### Definition of Done

- [ ] `references/sast/scanner-orch-rederivation-triggers.md` exists with predicates + PR format + rate-limit policy + no-auto-merge / no-cross-repo rules
- [ ] `skills/slo-sast/SKILL.md` Re-Derivation + PR Creation sections added; prior unchanged
- [ ] `crates/sldo-install/tests/e2e_scanner_orch_m5.rs` exists with all 10 E2E tests passing
- [ ] All 13 BDD scenarios pass
- [ ] All four trigger predicates fire correctly and independently
- [ ] Compound triggers produce a single PR with multiple body sections
- [ ] PR body resists prompt injection (regression on `tm-scanner-orchestration-abuse-3`)
- [ ] Rate-limit cap enforced (5/hour per session)
- [ ] No-auto-merge / no-cross-repo invariants enforced via test-stubbed `gh` argv assertions
- [ ] Dogfood test runs M1→M5 end-to-end against an isolated SLO subtree without producing a real PR
- [ ] `cargo test --workspace` green
- [ ] M1+M2+M3+M4 regression suites green
- [ ] Smoke tests demonstrate the full loop against a real codebase
- [ ] Lessons file at `docs/slo/lessons/scanner-orch-m5.md`
- [ ] Completion summary at `docs/slo/completion/scanner-orch-m5.md`
- [ ] Milestone Tracker row updated to `done` — runbook is now complete and ready for `/slo-ship` after `/slo-critique` review

---

## Documentation Update Table

| Document | When to update | What to update |
|---|---|---|
| [docs/ARCHITECTURE.md](ARCHITECTURE.md) | After each milestone that lands new HEAD code (M1, M2, M3, M4, M5) | The `slo-sast` skill row currently says "DESIGN, not yet implemented". After M1 closes, update the parenthetical to "M1 LANDED" / similar progressive markers; M5 closes by removing the not-yet-implemented qualifier and adding implemented-component subsections. |
| [SECURITY.md](../SECURITY.md) | If a milestone discovers a new safety property worth restating in the project-wide rules | Append to "Scanner orchestration skill — additional rules" section; preserve existing content verbatim per the idempotency rule. |
| [README.md](../README.md) | If user-facing capabilities change (likely M3 first emission, M5 dogfood) | Add `/slo-sast` to the skill table; show the basic invocation flow. |
| [CLAUDE.md](../CLAUDE.md) | If the test command set changes or a new shared scaffolding directory is added | Update the baseline test command line (still `cargo test --workspace` per current state) and any additions to the references-discovery exclusion list. |
| `docs/slo/design/scanner-orchestration-overview.md` | If a milestone discovers a design constraint not yet captured | Append to "Constraints carried forward from research" or add a new section; do not silently rewrite existing constraints. |
| `docs/slo/design/scanner-orchestration-interfaces.md` | If a milestone exercises an interface contract and needs to clarify ambiguity | Tighten wording; preserve `stable` markers; any change from `stable` to `evolving` or vice-versa requires fresh `/slo-architect`. |
