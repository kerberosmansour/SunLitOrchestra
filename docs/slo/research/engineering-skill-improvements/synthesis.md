---
name: engineering-skill-improvements
researched: 2026-04-27
incomplete: false
note: |
  Skill-pack improvement runbook — primary research input is the 2026-04-27 skill-pack
  review (in-conversation artifact, summarized below) + issues #21 and #22 + the
  project owner's explicit research-validation discipline for security-engineering
  citations. This file is the synthesis; the equivalent dossier content is the review
  + issues + the existing references/sast/ and references/ trees.
---

# Synthesis — engineering skill improvements

## What the design must handle (and why)

### 1. Decomposition must preserve install-symlink semantics

The design must handle install-symlink continuity because [`crates/sldo-install/src/install.rs`](../../../crates/sldo-install/src/install.rs)'s `discover_skills()` walks `skills/<name>/SKILL.md` and creates a symlink chain at `~/.claude/skills/<name>/`. If a SKILL.md decomposition moves content to `skills/slo-sast/references/methodology-m1-parser.md`, the install must still surface it. The existing pattern in `skills/slo-critique/personas/` (7 files inside the skill dir) and `skills/slo-tla/tools.toml` shows the per-skill `references/` subdir is the correct location — sibling to SKILL.md, included by the install.

### 2. `references/templates/` library belongs at the repo root, not under `skills/`

The design must handle templates being at repo root because [`crates/sldo-install/src/install.rs:44-71`](../../../crates/sldo-install/src/install.rs#L44)'s `discover_skills()` ignores `references/` at the repo root (this is documented behavior — see the existing `references/biz/` pattern in CLAUDE.md). Putting templates under `skills/` would either (a) accidentally install them as a fake skill, or (b) require special-casing in the installer. Repo-root `references/templates/` matches the existing `references/biz/` and `references/sast/` pattern.

### 3. The research-validation prereq is a milestone, not a checklist item

The design must handle this because the project owner's stated concern (security-engineering citations must be source-verified) applies to multiple downstream milestones in this runbook — sast/tla/plan decomposition all carry security-engineering content. The prereq is not "remember to source-verify"; it is a discrete M1-or-M0 milestone that produces `references/templates/citation-discipline.md` AND validates every existing security-engineering claim in the SKILL.md files about to be decomposed.

### 4. Source hierarchy is non-negotiable

The design must handle the source hierarchy explicitly:

1. Tool's own documentation at the pinned version (e.g., `cargo audit --help` on the documented version)
2. Tool repo's `README` / `CHANGELOG` at the pinned commit
3. Upstream advisory database documentation (RustSec, OSV, NVD, GHSA — each has its own structure)
4. Conference talks / academic papers when foundational (named author + year)
5. Vendor blog posts — flag as secondary
6. Stack Overflow / random commentary — never authoritative

Bright-line rule: **unverifiable claims are removed, not weakened**. "Approximate, please verify" is exactly the failure mode the runbook closes.

### 5. `/slo-freeze` hook must reuse `update-config`, not invent settings.json mutation

The design must handle settings.json mutation through the existing `update-config` skill ([per the project skill list](../../../CLAUDE.md)). Inventing a parallel mutation path violates the canonical-mutation-surface principle and creates a second place to keep secure (argv-list discipline, etc.).

### 6. PreToolUse hook is per-project (`.claude/settings.json`), not global

The design must handle per-project scope because (a) the freeze scope is session-scope, (b) global settings would freeze every project the user touches simultaneously, (c) the project-level `.claude/settings.json` is git-trackable so the discipline travels with the repo.

### 7. Per-skill evals must use the documented-expectation shape until the runtime harness lands

The design must handle the staged-rollout because the "Runtime Claude Code harness" deferred-follow-up from [issue #4](https://github.com/kerberosmansour/SunLitOrchestra/issues/4) hasn't shipped. Until it does, evals are documented expectations + manual run, not executable. The shape (`evals/<case-name>.md` with input + expected-behavior frontmatter) lets the harness consume them later without rewrites.

### 8. Soft line-cap structural-contract test must allow justified exceptions

The design must handle exceptions because some SKILL.md content (e.g., `/slo-tla`'s elicitation Q1-Q6 + state-explosion triage) is genuinely sequential — splitting fragments the methodology. Test passes with `# soft-cap-exception: <reason>` frontmatter; CI rejects exceptions without the comment. This matches the existing `pii_scan_override:` + `tier_override_reason:` pattern in [`references/biz/artifact-schema.md`](../../../references/biz/artifact-schema.md).

## Open questions that research did not answer

1. **Apalache's distribution channel** — does it use SHA-256 like `tla2tools.jar` or release-asset checksums? Verify against https://github.com/apalache-mc/apalache/releases at runbook-author time and capture the pinned version + SHA in [`skills/slo-tla/tools.toml`](../../../skills/slo-tla/tools.toml).
2. **`~~~text` user-string fence rule provenance** — is this a documented Markdown convention or a defense-in-depth invention? Critical for the `references/templates/citation-discipline.md` file's authoritative voice.
3. **Soft line-cap threshold** — 200 lines as the cap is the review's recommendation, but `/slo-tla` may legitimately need 250-280 even after decomposition. The structural-contract test should be calibrated against the post-decomposition SKILL.md sizes.
4. **Eval-runner harness API** — currently undefined; the deferred-follow-up in [issue #4](https://github.com/kerberosmansour/SunLitOrchestra/issues/4) lists it as blocked. The eval file shape must be forward-compatible without locking in the runner's API today.

## Source pointers

- 2026-04-27 skill-pack review (in-conversation artifact)
- [Issue #21](https://github.com/kerberosmansour/SunLitOrchestra/issues/21) — SKILL.md decomposition + shared templates
- [Issue #22](https://github.com/kerberosmansour/SunLitOrchestra/issues/22) — per-skill evals + hard-enforcement hooks + polish
- [`skills/slo-sast/SKILL.md`](../../../skills/slo-sast/SKILL.md) (296 lines), [`skills/slo-tla/SKILL.md`](../../../skills/slo-tla/SKILL.md) (323 lines), [`skills/slo-plan/SKILL.md`](../../../skills/slo-plan/SKILL.md) (132 lines) — primary decomposition targets
- [`references/sast/`](../../../references/sast/) — existing per-stage references (parser-contract, stack-detection-contract, etc.) that the new `methodology-m1..m5.md` orchestration files complement
- [`docs/slo/completed/RUNBOOK-SLO-SECURITY-EMBEDDING.md`](../../completed/RUNBOOK-SLO-SECURITY-EMBEDDING.md) (if present) — Phase 1 program that established the security-aware skill flow this runbook extends

## Note on chub / get-api-docs

Use [`/get-api-docs`](../../../skills/get-api-docs/SKILL.md) for any third-party API reference encountered during execution — but the runbook itself doesn't pre-declare API integrations; the polish items (E5 in [issue #22](https://github.com/kerberosmansour/SunLitOrchestra/issues/22)) extend `chub`'s failure-mode handling.
