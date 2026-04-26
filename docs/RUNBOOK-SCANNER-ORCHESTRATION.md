# Scanner Orchestration — `/slo-sast` v1 wedge (AI-First Runbook v3)

> **Purpose**: Ship a pure-Markdown `/slo-sast` Claude Code skill that reads a project's threat-model file, picks tuned Semgrep rule packs, emits a safe `.github/workflows/sast.yml` plus baseline-aware config plus an audit-defense manifest, and re-derives the ruleset on threat-model edit — closing the auto-tuning loop that no published OTM-or-similar → Semgrep config converter occupies today.
> **Audience**: AI coding agents first, humans second. Written to reduce ambiguity, prevent scope drift, and improve output quality on security-sensitive runbooks.
> **How to use**: Work through milestones sequentially. Before starting any milestone, read its full section and the Global Execution Rules in [docs/runbook-template_v_3_template.md](runbook-template_v_3_template.md). After completing it, follow the Global Exit Rules. Never skip ahead. Never silently widen scope.
> **Prerequisite reading**: [docs/ARCHITECTURE.md](ARCHITECTURE.md), [docs/idea/scanner-orchestration.md](idea/scanner-orchestration.md), [docs/research/scanner-orchestration/synthesis.md](research/scanner-orchestration/synthesis.md), [docs/design/scanner-orchestration-overview.md](design/scanner-orchestration-overview.md), [docs/design/scanner-orchestration-stack-decision.md](design/scanner-orchestration-stack-decision.md), [docs/design/scanner-orchestration-interfaces.md](design/scanner-orchestration-interfaces.md), [docs/design/scanner-orchestration-threat-model.md](design/scanner-orchestration-threat-model.md), [SECURITY.md](../SECURITY.md) — particularly the "Scanner orchestration skill — additional rules" section.

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
| 1 | `/slo-sast` SKILL.md scaffold + threat-model parser (CWE extraction with comment / fence scope rule) | `not_started` | | | | |
| 2 | Stack detection + `semgrep-rules` cache fetch at pinned SHA + CWE × technology rule filter | `not_started` | | | | |
| 3 | Emit `.semgrep/rules/`, `.semgrep.yml`, `.github/workflows/sast.yml` with workflow-safety structural-contract test | `not_started` | | | | |
| 4 | Emit `.semgrep/manifest.json` (audit-defense schema v1.0) + initial-baseline preview-mode UX | `not_started` | | | | |
| 5 | Re-derivation trigger detection + diff PR generation + dogfood E2E against this SLO repo | `not_started` | | | | |

<!-- Status values: not_started | in_progress | blocked | done -->
<!-- Lessons files go in docs/lessons/scanner-orch-m<N>.md -->
<!-- Completion summaries go in docs/completion/scanner-orch-m<N>.md -->

---

## End-to-End Architecture Diagram

See [docs/design/scanner-orchestration-overview.md](design/scanner-orchestration-overview.md) for the full diagram with legend. Summary view below; solid = exists today, dashed = added by this runbook.

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
| Threat-model parse | `docs/design/<slug>-threat-model.md` | Skill in-memory CWE list | File read + regex | M1 |
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

**N/A** — `tla_required: false` in [docs/design/scanner-orchestration-overview.md](design/scanner-orchestration-overview.md). Rationale: the skill is sequential file I/O plus subprocess invocation (one `git clone`, several file writes). No concurrent actors share state, no distributed consensus, no leader election, no cross-process ordering guarantees, no resource leases or locks. The re-derivation trigger compares stored SHAs against current SHAs at single-process invocation time — there is no race window. TLA+ would not surface a class of bug this design cannot already eliminate by construction.

---

## Global Execution Rules

See [docs/runbook-template_v_3_template.md §Global Execution Rules](runbook-template_v_3_template.md). Applied verbatim. Notable callouts for this runbook:

- **No `pull_request_target` in any emitted YAML, ever.** Inherited from [SECURITY.md](../SECURITY.md) "Scanner orchestration skill — additional rules". A milestone that violates this MUST fail the structural-contract test fixture introduced in M3.
- **PCI compliance citations cite 6.2.3 (v4.0.1), never 6.3.2.** v4.0.1's 6.3.2 is the SBOM-inventory mandate, different scope. Mixing the two is a substantive error.
- **Threat-model parser scope is non-negotiable.** Comments / fenced code / `~~~text` fences are excluded — defuses `tm-scanner-orchestration-abuse-1`. This rule lands in M1 and is asserted by every subsequent milestone's regression tests.

---

## Global Entry Rules (Pre-Milestone Protocol)

See [docs/runbook-template_v_3_template.md §Global Entry Rules](runbook-template_v_3_template.md). Applied verbatim.

---

## Global Exit Rules (Post-Milestone Protocol)

See [docs/runbook-template_v_3_template.md §Global Exit Rules](runbook-template_v_3_template.md). Applied verbatim.

---

## Background Context

### Current State

No `/slo-sast` skill exists. The repo contains 30+ first-party `/slo-*` Markdown skills under `skills/`, plus the supporting Rust workspace (`crates/sldo-common`, `crates/sldo-research`, `crates/sldo-install`, `xtasks/sast-verify`). `references/sast/` carries shared scaffolding for the SAST rule-gen runbook (CWE map, Semgrep syntax notes, AGPL clean-room policy) — scanner-orchestration adds a sibling `references/sast/threat-model-parser-contract.md` (M1) and reuses the rest. `docs/design/scanner-orchestration-*.md` (overview, stack-decision, interfaces, threat-model) define the design surface this runbook implements. SECURITY.md was extended on 2026-04-26 with a "Scanner orchestration skill — additional rules" section that restates the load-bearing safety properties.

### Problem

The runbook addresses these specific gaps:

1. **No threat-model-driven SAST orchestration exists** — the unoccupied-wedge verdict from research synthesis Q5. Vendor presets (Snyk Code, GHAS CodeQL, Checkmarx One, Veracode) require manual policy authoring rather than threat-model intake; SecOpsTM is the closest adjacent player but emits Navigator layers, not Semgrep configs.
2. **The auto-tuning loop has no host today** — CWE list changes in a threat model don't propagate anywhere automatically. Re-derivation on threat-model edit is the differentiator that earns this skill its keep over Semgrep AppSec Platform (which doesn't read threat models at all).
3. **Solo OSS maintainers ship with no SAST in CI** — the idea-doc pain story (an RCE disclosed because no security tests ran). The skill exists to prevent the next maintainer from absorbing the same emotional + time + community-trust hit.
4. **Workflow YAML mistakes are widespread** — Sysdig 2024 audit, Shai Hulud v2 (~20k repos, Nov 2025), CVE-2025-30066 (`tj-actions/changed-files`, March 2025). The skill emits a safe-by-default workflow whose properties are asserted by a structural-contract test, eliminating an entire class of `pull_request_target` / unpinned-action / over-permissioned / fetch-depth-1 misconfigurations from the get-go.

### Target Architecture

See the End-to-End Architecture Diagram above and the full diagram in [docs/design/scanner-orchestration-overview.md](design/scanner-orchestration-overview.md). End state after M5: invoking `/slo-sast` against any target repo (with a threat-model file) produces a tuned Semgrep workflow committed via PR; subsequent threat-model edits trigger diff PRs that surface the rule-set delta for human review.

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
- **`docs/lessons/scanner-orch-m<N>.md`** + **`docs/completion/scanner-orch-m<N>.md`** — written at end of each milestone per template.

### Global Red Lines

Inherited from [docs/runbook-template_v_3_template.md §Global Red Lines](runbook-template_v_3_template.md), plus runbook-specific:

- **No `pull_request_target` in emitted workflow YAML.** Ever.
- **No new Rust crate added by this runbook.** Markdown-only direction is locked. If a future milestone needs deterministic helper code, that's a fresh `/slo-architect` decision.
- **No autofix invocation in the emitted workflow.** `semgrep ci` runs without `--autofix`. Defends against `tm-scanner-orchestration-abuse-2` (compromised rule autofix smuggling backdoors).
- **No tag references in `uses:` lines.** SHA-pin or fail.
- **No HTTP / SDK calls from the skill itself.** `git clone` (CLI) is the only network egress.
- **No mutation of `references/sast/` existing files.** New file (`threat-model-parser-contract.md`) is additive.

---

## BDD and Runtime Validation Rules

See [docs/runbook-template_v_3_template.md §BDD and Runtime Validation Rules](runbook-template_v_3_template.md). Applied verbatim.

---

## Dependency, Migration, and Refactor Policy

See [docs/runbook-template_v_3_template.md §Dependency, Migration, and Refactor Policy](runbook-template_v_3_template.md). Applied verbatim.

---

## Evidence Log Template

See [docs/runbook-template_v_3_template.md §Evidence Log Template](runbook-template_v_3_template.md). Each milestone copies this table into its own section before execution.

---

## Self-Review Gate

See [docs/runbook-template_v_3_template.md §Self-Review Gate](runbook-template_v_3_template.md). Applied verbatim. Additional gate questions for this runbook:

- Did I preserve the threat-model parser scope rule (HTML comments / fenced code / `~~~text` fences excluded)?
- Did every emitted YAML/JSON file pass its structural-contract test fixture before the milestone closed?
- Did I cite PCI 6.2.3 (NOT 6.3.2) in any compliance-related artifact?
- Did I SHA-pin every third-party action in any workflow this milestone touches?
- Did I avoid introducing any new Rust crate?

---

## Lessons-Learned File Template

Path: `docs/lessons/scanner-orch-m<N>.md`. See [docs/runbook-template_v_3_template.md §Lessons-Learned File Template](runbook-template_v_3_template.md).

---

## Completion Summary Template

Path: `docs/completion/scanner-orch-m<N>.md`. See [docs/runbook-template_v_3_template.md §Completion Summary Template](runbook-template_v_3_template.md).

---

## Milestone Plan

### Milestone 1 — `/slo-sast` SKILL.md scaffold + threat-model parser

**Goal**: The `/slo-sast` skill exists at `skills/slo-sast/SKILL.md` and, when invoked against a target repo containing `docs/design/<slug>-threat-model.md`, prints the deduplicated list of CWE integers extracted from the file's rendered Markdown body — excluding CWE references inside HTML comments, fenced code blocks, and `~~~text` user-string fences.

**Context**: No `/slo-sast` exists today. [scanner-orchestration-interfaces.md §2](design/scanner-orchestration-interfaces.md) defines the threat-model parse contract (regex `\bCWE-(\d+)\b`, scope-exclusion rules); [scanner-orchestration-threat-model.md](design/scanner-orchestration-threat-model.md) abuse case `tm-scanner-orchestration-abuse-1` is the threat the scope rule defuses. The skill is Markdown-only; the parser implementation lives in the prompt logic Claude Code executes when reading SKILL.md. This milestone lands the skill scaffold + the parse contract — no artifact emission yet (that's M3), no registry fetch yet (that's M2).

**Important design rule**: The parser scope rule is non-negotiable and is asserted by E2E tests at M1 closure. Every subsequent milestone's regression-test row references this rule — if a future milestone's prompt change weakens it, M1's tests fail and the milestone cannot close.

**Refactor budget**: `No refactor permitted beyond direct implementation`.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | `/slo-sast` invocation from Claude Code with cwd = target repo root; reads `docs/design/<slug>-threat-model.md` (slug derived from runbook context or explicit arg) |
| Outputs | A deduplicated list of integer CWE ids (long-form `["CWE-77", "CWE-78", "CWE-89"]`) printed to stdout; exits 0 on success, non-zero with stderr message on missing file |
| Interfaces touched | NEW: `/slo-sast` skill invocation surface (SKILL.md `name:` + `description:` keys); NEW: threat-model parse contract (regex + scope rules) — both `stable` per [scanner-orchestration-interfaces.md](design/scanner-orchestration-interfaces.md) §1, §2 |
| Files allowed to change | `skills/slo-sast/SKILL.md` (NEW); `references/sast/threat-model-parser-contract.md` (NEW); `crates/sldo-install/tests/e2e_scanner_orch_m1.rs` (NEW); `crates/sldo-install/tests/fixtures/scanner-orch/m1/` (NEW directory with threat-model fixtures); `.gitignore` (only if a new tool cache pattern is needed — unlikely in M1) |
| Files to read before changing anything | `docs/design/scanner-orchestration-overview.md`, `docs/design/scanner-orchestration-interfaces.md` (§§1–2), `docs/design/scanner-orchestration-threat-model.md` (abuse case `tm-scanner-orchestration-abuse-1`, residual-risks section), [SECURITY.md](../SECURITY.md) "Scanner orchestration skill — additional rules" section, `skills/slo-research/SKILL.md` (for SKILL.md scaffold conventions), `skills/slo-architect/SKILL.md` (same), `crates/sldo-install/src/install.rs` (confirm `discover_skills()` requires `<skills_dir>/<name>/SKILL.md` shape), `crates/sldo-install/tests/e2e_slo_sec_m1.rs` (E2E test pattern for skill structural assertions) |
| New files allowed | `skills/slo-sast/SKILL.md`; `references/sast/threat-model-parser-contract.md`; `crates/sldo-install/tests/e2e_scanner_orch_m1.rs`; `crates/sldo-install/tests/fixtures/scanner-orch/m1/<fixture-name>.md` (multiple) |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | `cargo test --workspace` baseline remains green; `./target/release/sldo-install --dry-run` discovers all pre-existing skills (`/slo-ideate`, `/slo-research`, `/slo-architect`, `/slo-plan`, etc.); no other skill's SKILL.md is touched; `references/sast/` existing files unchanged |
| Forbidden shortcuts | No artifact emission stub (M3 — DO NOT create empty `.semgrep/` placeholders); no registry fetch stub (M2); no caching of parsed CWE list across invocations (single-pass per call); no fallback-to-default-pack logic (M2's job); no "TODO: handle HTML comments" comment in SKILL.md — the scope rule lands fully implemented and fully tested in M1; no JSON / structured-data output (stdout list of CWE strings is the v1 surface; rich output is M4's manifest); no shell-out other than file reads |
| **Data classification** | `Internal` — threat-model files are project design docs, neither public-facing nor secret. SKILL.md content itself is `Public` (committed to OSS repo) but the data the skill processes is project-internal. Per [references/proactive-controls-vocabulary.md](references/proactive-controls-vocabulary.md) (cited by `/slo-plan`; if absent, OWASP Proactive Controls v3 categories are used directly). |
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
2. No prior milestone — read [docs/idea/scanner-orchestration.md](idea/scanner-orchestration.md) and [docs/research/scanner-orchestration/synthesis.md](research/scanner-orchestration/synthesis.md) instead of a lessons file from M0.
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
| `parses_canonical_cwe_list_from_prose` | happy path | a threat-model file at `docs/design/<slug>-threat-model.md` containing `"... mitigates CWE-77, CWE-78, and CWE-89."` in rendered prose | the skill is invoked | stdout contains the deduplicated list `["CWE-77", "CWE-78", "CWE-89"]` (long-form, sorted ascending by integer) and exits 0 |
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
| Baseline tests | `cargo test --workspace` | all pre-existing tests green | | | |
| BDD tests created | `crates/sldo-install/tests/e2e_scanner_orch_m1.rs` | compile or fail for expected reason (skill missing) | | | |
| Fixtures created | `crates/sldo-install/tests/fixtures/scanner-orch/m1/*` | all 7 fixture files present | | | |
| `references/sast/threat-model-parser-contract.md` written | manual review | regex + exclusion rules + rationale present | | | |
| `skills/slo-sast/SKILL.md` written | manual review | frontmatter valid, parse contract cited | | | |
| Discovery check | `./target/release/sldo-install --local --dry-run` | `slo-sast` listed | | | |
| Full tests | `cargo test --workspace` | green | | | |
| E2E runtime | `cargo test -p sldo-install --test e2e_scanner_orch_m1` | green (5 tests) | | | |
| Build/boot | `cargo build --workspace` | success | | | |
| Smoke tests | manual steps above | all checked | | | |
| Test artifact cleanup | `git status` | only intended new files; no untracked test output | | | |
| .gitignore review | review `.gitignore` | patterns current; no stale entries | | | |
| Compatibility checks | `git diff --stat references/sast/`; baseline + dry-run + local install | references untouched; install idempotent | | | |

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
- [ ] Lessons file at `docs/lessons/scanner-orch-m1.md`
- [ ] Completion summary at `docs/completion/scanner-orch-m1.md`
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
| Files to read before changing anything | M1's outputs (`skills/slo-sast/SKILL.md`, `references/sast/threat-model-parser-contract.md`, `crates/sldo-install/tests/e2e_scanner_orch_m1.rs`); `docs/lessons/scanner-orch-m1.md`; `docs/design/scanner-orchestration-interfaces.md` (§§3, 7); `docs/design/scanner-orchestration-threat-model.md` (abuse case `tm-scanner-orchestration-abuse-2`); `docs/design/scanner-orchestration-stack-decision.md` (cache layout non-negotiable); `references/sast/MIN-SEMGREP-VERSION.md` (existing — semgrep version floor still applies); existing `~/.cache/` patterns from any other skill (none currently — this is a new pattern) |
| New files allowed | `references/sast/scanner-orch-pinned-rules-sha.md`; `references/sast/stack-detection-contract.md`; `crates/sldo-install/tests/e2e_scanner_orch_m2.rs`; `crates/sldo-install/tests/fixtures/scanner-orch/m2/<manifest+cache fixtures>` |
| New dependencies allowed | `none` — uses `git` CLI for clone (assumed on PATH like `claude`); `serde_yaml_ng` already in workspace for test-harness YAML parsing |
| Migration allowed | `no` |
| Compatibility commitments | M1 parser scope rule still enforced (BDD regression `parser_ignores_html_comment_cwe_refs` etc. still pass); `cargo test --workspace` baseline green; `sldo-install --dry-run` discovers all skills; M1's stdout-format consumers (none yet — only test fixtures) explicitly opt into the new JSON shape via M2's new test fixtures |
| Forbidden shortcuts | No shell-out other than `git clone`/`git fetch`/`git rev-parse` (no `curl`, `wget`, `gh api`, language-specific HTTP); no caching parsed rule data across invocations (re-read YAML files per call to keep memory footprint deterministic); no fallback to vendor SaaS API (Semgrep AppSec was rejected in stack-decision); no autofix invocation in any path; no use of Semgrep `--config` flag (locked for M3 to use `SEMGREP_RULES` env var); no SHA-prefix matching (full 40-char SHA required); no tag/branch reference accepted (refuse on non-SHA input); no committing the cache (cache lives in `~/.cache/`, not the target repo) |
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
2. Read `docs/lessons/scanner-orch-m1.md` and apply relevant corrections.
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
| `crates/sldo-install/tests/fixtures/scanner-orch/m2/cache/<SHA>/<lang>/.../<rule>.yaml` | NEW: fixture rule YAMLs with `metadata.cwe` + `metadata.technology` populated; covering CWE-77 (rust), CWE-89 (python+django), CWE-78 (multi-language), and one rule with absent `metadata.technology` (language-agnostic). |
| `crates/sldo-install/tests/fixtures/scanner-orch/m2/cache/<SHA>/legacy-no-technology.yaml` | NEW: rule with `metadata.cwe` set but `metadata.technology` absent (legacy schema gap per research finding). |
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
| `cache_hit_skips_clone` | happy path (cached) | `~/.cache/sldo/semgrep-rules/<pinned-SHA>/` already exists with valid rules | the skill is invoked | `git clone` is NOT invoked; selected rules come from existing cache |
| `refuses_tag_reference_in_pinned_sha` | abuse case (`tm-scanner-orchestration-abuse-2`) | `references/sast/scanner-orch-pinned-rules-sha.md` is rewritten to contain `develop` instead of a 40-char SHA | the skill is invoked | exit non-zero; stderr names the violation (`pinned-rules-sha must be a 40-char SHA, got "develop"`); no clone is attempted; no cache is written |
| `refuses_branch_reference_in_pinned_sha` | abuse case (`tm-scanner-orchestration-abuse-2`) | the pinned reference is `main` | the skill is invoked | same as above — refused with clear error |
| `refuses_short_sha_prefix` | abuse case (`tm-scanner-orchestration-abuse-2`) | the pinned reference is a 7-char SHA prefix (Git's default short form) | the skill is invoked | refused — full 40-char required |
| `clean_cache_on_sha_mismatch` | abuse case (`tm-scanner-orchestration-abuse-2`) | a cache directory exists at `<SHA-A>/` but the post-clone `git rev-parse HEAD` reports `<SHA-B>` (e.g., upstream tag-rewriting attack mid-clone) | the skill is invoked | the partial cache at `<SHA-A>/` is wiped before exit; exit non-zero; stderr cites the mismatch |
| `git_unavailable_clean_error` | dependency failure | `git` is not on PATH | the skill is invoked with cache miss | exit non-zero; stderr names "git CLI not found"; no partial state written |
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
7. Re-run the skill — verify no new clone (logs from `git` should not appear).

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
- [ ] Lessons file at `docs/lessons/scanner-orch-m2.md`
- [ ] Completion summary at `docs/completion/scanner-orch-m2.md`
- [ ] Milestone Tracker row updated to `done`

---

### Milestone 3 — Emit `.semgrep/rules/` + `.semgrep.yml` + `.github/workflows/sast.yml` with safety-contract test

_Placeholder — will be authored in full once M2 is confirmed._

**Goal (preview)**: The skill writes the four primary artifacts into the target repo: selected rule files committed under `.semgrep/rules/`, project config at `.semgrep.yml`, and the safe-template GitHub Actions workflow at `.github/workflows/sast.yml`. A structural-contract test fixture asserts every safety property (no `pull_request_target`, `permissions: {}` scope, SHA-pinned actions, `fetch-depth: 0`, `SEMGREP_RULES` env var, no `secrets.*` in analysis job).

**Primary abuse case**: `tm-scanner-orchestration-abuse-3` (prompt injection via threat-model file content trying to splice `pull_request_target` into emitted YAML). Defense: template-skeleton with parameter-only substitution.

---

### Milestone 4 — `.semgrep/manifest.json` (audit-defense schema v1.0) + initial-baseline preview-mode UX

_Placeholder — will be authored in full once M3 is confirmed._

**Goal (preview)**: The skill writes `.semgrep/manifest.json` per [interfaces.md §5](design/scanner-orchestration-interfaces.md) (cwes_claimed vs cwes_actually_covered, threat_model_sha, semgrep_rules_sha, semgrep_version, selected_rules with per-rule SHAs and metadata) plus `.semgrep/last-run.json`. On first install against an existing repo, the skill runs preview-mode (dry-run scan, surface counts by severity, REQUIRE explicit user confirm before committing the workflow).

**Primary abuse cases**: `tm-scanner-orchestration-abuse-4` (manifest content injection — defended by regex-validated value population) + `tm-scanner-orchestration-abuse-5` (baseline-day CI jam — defended by preview-mode + GHA auto-baseline).

---

### Milestone 5 — Re-derivation trigger + diff PR + dogfood E2E against this SLO repo

_Placeholder — will be authored in full once M4 is confirmed._

**Goal (preview)**: The skill detects when the threat-model SHA differs from the recorded `threat_model_sha`, when the `semgrep_rules_sha` pin is bumped, when a new manifest file appears (stack added), or when `cwes_claimed` would change — and surfaces the resulting diff as a PR with the proposed changes. Dogfood: run `/slo-sast` against this SLO repo using `docs/design/scanner-orchestration-threat-model.md` as input, prove the full pipeline closes the loop end-to-end.

**Defense**: Auto-tuning loop becomes real; the wedge thesis (research synthesis Q5 verdict — "first-of-its-kind") is exercised against a real codebase.

---

## Documentation Update Table

| Document | When to update | What to update |
|---|---|---|
| [docs/ARCHITECTURE.md](ARCHITECTURE.md) | After each milestone that lands new HEAD code (M1, M2, M3, M4, M5) | The `slo-sast` skill row currently says "DESIGN, not yet implemented". After M1 closes, update the parenthetical to "M1 LANDED" / similar progressive markers; M5 closes by removing the not-yet-implemented qualifier and adding implemented-component subsections. |
| [SECURITY.md](../SECURITY.md) | If a milestone discovers a new safety property worth restating in the project-wide rules | Append to "Scanner orchestration skill — additional rules" section; preserve existing content verbatim per the idempotency rule. |
| [README.md](../README.md) | If user-facing capabilities change (likely M3 first emission, M5 dogfood) | Add `/slo-sast` to the skill table; show the basic invocation flow. |
| [CLAUDE.md](../CLAUDE.md) | If the test command set changes or a new shared scaffolding directory is added | Update the baseline test command line (still `cargo test --workspace` per current state) and any additions to the references-discovery exclusion list. |
| `docs/design/scanner-orchestration-overview.md` | If a milestone discovers a design constraint not yet captured | Append to "Constraints carried forward from research" or add a new section; do not silently rewrite existing constraints. |
| `docs/design/scanner-orchestration-interfaces.md` | If a milestone exercises an interface contract and needs to clarify ambiguity | Tighten wording; preserve `stable` markers; any change from `stable` to `evolving` or vice-versa requires fresh `/slo-architect`. |
