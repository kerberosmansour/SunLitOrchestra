---
topic: # Research brief — scanner-orchestration  ## Wedge (one sentence)  A pure-Markdown Claude Code skill (`/slo-sast`) that reads a project's `docs/slo/design/<slug>-threat-model.md`, extracts the named CWE l…
generated_on: 2026-04-26 00:39:05 +0100
source_prompt_bytes: 7193
generator: sldo-research
---

# Research Dossier

This dossier is a structured research artifact produced by `sldo-research`. It is intended as the `prompt_file` input to `sldo-plan`.

## Repository Context

I have enough context to write the report.

## Tech Stack

- **Language:** Rust 2021 edition (workspace, resolver = "2"). Stable toolchain; no `rust-toolchain.toml` pin.
- **Workspace members:** `crates/sldo-common`, `crates/sldo-research`, `crates/sldo-install`, `xtasks/sast-verify`.
- **Core dependencies (pinned at workspace root):** `clap 4` (derive), `anyhow 1`, `thiserror 2`, `colored 2`, `regex 1`, `chrono 0.4`, `which 7`, `serde 1` (derive), `serde_yaml_ng 0.10`, `serde_json 1`, `tempfile 3`. Per-crate extras: `toml 0.8` (sldo-install), `dotenvy 0.15` + `base64 0.22` (workspace E2E tests).
- **External tooling:** Claude Code CLI (`claude`) shelled to by `sldo-research`; Semgrep (≥ 1.50.0) shelled to by `sast-verify`. No async runtime in the surviving code.
- **Skill pack:** Markdown SKILL.md files under `skills/` consumed by Claude Code (no Rust at runtime for those).
- **Pre-commit:** `.pre-commit-config.yaml` runs `semgrep/pre-commit@v1.156.0` against `.semgrep/`.

## Project Structure

- `crates/sldo-common/` — shared library (`color`, `copilot`, `detect`, `git`, `logging`, `preflight`, `runbook`, `toolflags`).
- `crates/sldo-research/` — CLI binary that wraps Claude Code to produce research dossiers (`main`, `dossier`, `prompt`, `research`).
- `crates/sldo-install/` — symlinks `skills/*` into `~/.claude/skills/` or project-local `.claude/skills/` (`main`, `install`, `manifest`, `paths`).
- `xtasks/sast-verify/` — Semgrep rule-pack gate (`validate`, `test`, `check_coverage`, `check_clean`, `gate`, `tier_detect`, `validate_file_paths`, `yaml_schema`, `semgrep_runner`).
- `skills/` — 30+ first-party `/slo-*` skills plus vendored `get-api-docs`. Markdown only.
- `.semgrep/rust/` — 10 paired CWE rule YAMLs + Rust fixtures (cwe-20, 79, 125, 190, 295, 416, 672, 697, 755, 787).
- `references/biz/`, `references/sast/` — shared scaffolding read by skills (NOT installed; ignored by `discover_skills()`).
- `docs/` — runbooks (`RUNBOOK-*.md`), runbook v3 template, and `idea/`, `design/`, `research/`, `critique/`, `verify/`, `completion/`, `lessons/`, `legal/` subtrees.
- `tests/` — workspace-level E2E tests (`e2e_common_m2`, `e2e_research_m1`–`m7`, `e2e_sast_rulegen_a_m2`/`m3`) plus `tests/fixtures/`.
- `xtasks/` — Cargo xtask alias target (only `sast-verify` lives here).
- `.github/workflows/` — single CI workflow `semgrep.yml`.
- Top-level governance: `CLAUDE.md`, `SECURITY.md`, `README.md`, `LICENSE` (Apache-2.0 OR MIT).

## Build & Test

- **Workspace build:** `cargo build --workspace` (or `cargo build -p sldo-install -p sast-verify --release` per README).
- **Baseline test command (per `CLAUDE.md`):** `cargo test --workspace`.
- **xtask alias** (`.cargo/config.toml`): `cargo xtask <args>` ⇒ `cargo run --package sast-verify --`. Example: `cargo xtask sast-verify gate .semgrep/rust/cwe-755-panic-on-result-fn.yaml`.
- **Skill installer:** `./target/release/sldo-install` (default global), `--local`, `--dry-run`, or `uninstall`.
- **Pre-commit hooks:** `pre-commit run` or the Rust drop-in `prek run` (Semgrep against `.semgrep/`, Rust files only, fixtures excluded).
- **No `Makefile`, no `package.json`, no `pyproject.toml`, no `go.mod`** at the repo root.

## Existing Patterns

- **Error handling:** `anyhow::Result` for binary call sites with `Context` / `with_context` for path-rich error messages; `thiserror` for typed errors in libraries (`sldo-common`, `sast-verify`). `bail!` used to abort with formatted messages (e.g., `preflight::check_git_safety`).
- **CLI shape:** `clap 4` derive with a top-level `Cli` struct and `Subcommand` enum; `#[command(name=…, about=…, version)]`. Global flags via `global = true` (see `sldo-install/src/main.rs:30-47`, `sast-verify/src/main.rs:37-56`).
- **Module layout:** thin `main.rs` parses CLI then dispatches to sibling modules (`mod install; mod manifest; mod paths;`). Library entry `lib.rs` is a `pub mod` index plus a tiny `version()` helper.
- **Logging:** custom timestamped append-only `LogFile` under a `.sldo-logs/` directory (`sldo_common::logging`); colored stderr via `sldo_common::color` (`header`, `info`, `success`, `warn`, `divider`, `ts`). No `tracing`/`log` crates.
- **Preflight gating:** every CLI calls `sldo_common::preflight` to check the `claude` binary is on PATH, that paths exist, and that the current branch is not protected before doing work.
- **External-process invocation:** binaries shell out (Claude Code CLI from `sldo-research`; Semgrep from `sast-verify`); `which` resolves binaries with `--*-bin` flags as overrides.
- **Tests:** unit tests are `#[cfg(test)] mod tests` at the bottom of each file with explicit Given/When/Then comments; workspace-level E2E lives in `tests/e2e_*.rs` declared as `[[test]]` entries on the root `sunlit-orchestrate-tests` package.
- **Skill discovery:** `crates/sldo-install/src/install.rs` `discover_skills()` walks `skills/` only — `references/` is intentionally excluded.
- **Async:** none in the surviving code (the legacy CLIs and Tauri UI were removed in 2026-04 cleanup, per `CLAUDE.md`).

## Constraints

- **License:** dual Apache-2.0 OR MIT (NOT AGPL). SAST rules must be clean-room re-authored from `references/sast/variations/` — Trail of Bits' AGPL `semgrep-rules` may NOT be copied (`references/sast/AUTHORING.md`).
- **MSRV:** not pinned in `Cargo.toml` or `rust-toolchain.toml`; `SECURITY.md` records "Rust stable, latest supporting `clap 4` / `thiserror 2` / `reqwest 0.11`."
- **Semgrep version floor:** `references/sast/MIN-SEMGREP-VERSION.md` requires ≥ 1.50.0; pre-commit pin is `v1.156.0`.
- **Network posture (per `SECURITY.md` + `xtasks/sast-verify/src/main.rs:1-12`):** `sast-verify` never reaches the network; SLO stores no confidential data; only crypto operation in surviving code is SHA-256 integrity hashing.
- **Branch protection:** `preflight::check_git_safety` refuses to run on protected branches — work on a feature branch.
- **Skill-pack scope rules (per `CLAUDE.md`):** every feature must live in `docs/RUNBOOK-<slug>.md` per the v3 template; UK-only for the biz pack v1; broad GDPR `draft` hard-block locked 2026-04-25.
- **PII discipline:** outputs split into `docs/biz/` (gitignored, real data) vs `docs/biz-public/` (tracked, placeholders); `/slo-verify` Pass 4 PII scan covers the public tier.
- **Dropped surfaces:** `sldo-plan` / `sldo-run` CLIs, the Tauri desktop UI, and `sldo-tla-sha` were removed in the 2026-04 cleanup. Skills are now the canonical interface — research must not propose work that depends on those binaries.
- **Exit codes:** `sast-verify` reserves codes 0–7; ≥64 reserved for unrecoverable crashes (per `xtasks/sast-verify/src/main.rs` header).
- **Files not present at repo root:** `Makefile`, `package.json`, `pyproject.toml`, `go.mod`, `rust-toolchain.toml` — none exist.

## Executive Summary

The research validates a pure-Markdown `/slo-sast` skill as the right v1 wedge: a Claude Code skill that reads `docs/slo/design/<slug>-threat-model.md`, extracts CWE references, queries Semgrep registry rule metadata to derive a tuned ruleset, and emits a safe `pull_request`-only GitHub Actions workflow plus a baselined `.semgrep.yml`. Five high-confidence anchors emerged: (1) `semgrep ci` auto-derives baselines from PR event payloads on GitHub Actions, so `SEMGREP_BASELINE_REF` / `--baseline-commit` is irrelevant; the real footgun is `actions/checkout`'s default `fetch-depth: 1`. (2) Severity-based finding gating does not exist for stand-alone (non-AppSec-Platform) Semgrep users — every selected rule's findings block, so rule *selection* is the only gate. (3) The Semgrep registry's `metadata.cwe` field is mandatory for `category: security` rules and queryable by direct YAML inspection, so the skill can derive CWE→rule mappings programmatically rather than maintaining a hand-curated table. (4) GitHub's 2025-12-08 change forces `pull_request_target` workflow source to default branch, but the wedge's hard ban on `pull_request_target` remains correct — `on: pull_request` with `permissions: {}` and SHA-pinned actions is the safe template. (5) No published OTM-or-similar → Semgrep config converter exists in OSS or commercial tooling — the threat-model-edit-to-ruleset-rederivation loop is unoccupied. One material correction surfaced: PCI DSS 6.3.2 in the brief refers to v3.2.1 numbering; the current v4.0.1 requirement for code review is **6.2.3** (6.3.2 in v4.0.1 is the new SBOM-inventory mandate). The wedge addresses 6.2.3.

## Topic Decomposition

The brief decomposes into five research questions, each with distinct information needs:

- **Q1 — Semgrep CI integration mechanics:** baseline behavior, severity gating, fetch-depth, pull-request-comment paths for OSS users without Semgrep AppSec Platform, common pitfalls.
- **Q2 — CWE → Semgrep rule pack mapping:** is the registry's `metadata.cwe` tagging dense and queryable enough to be canonical, or must the skill author its own table?
- **Q3 — Auditable SAST coverage claims for PCI DSS / SOC 2:** what evidence formats, retention windows, and "addressed-finding" definitions auditors accept; whether claiming CWE coverage that wasn't actually scanned is a documented audit-failure pattern.
- **Q4 — `pull_request_target` security posture:** when (if ever) it's needed for SAST, minimum-permissions block, two-workflow split for fork-PR comments, action-SHA pinning, post-2025-12-08 GitHub semantics.
- **Q5 — Prior art:** has anyone built threat-model-driven scanner configuration; OWASP Threat Dragon, pytm, OTM, Threagile, vendor presets (Snyk, GHAS CodeQL, Checkmarx, Veracode), academic work; defensible answer to "why doesn't this already exist?"

Findings are strongest on Q1, Q2, Q4, and Q5; weakest on Q3 (PCI DSS / SOC 2 specifics). Q3 needs primary-source reads of the PCI Council document library (v4.0.1 PDF), AICPA TSC 2017+2022, and NIST SSDF SP 800-218 PW.7/PW.8 verbatim text — none of which web search returned in extractable form across multiple research rounds.

## Key Findings

### PCI DSS numbering correction (high confidence)

The brief and idea doc cite **PCI DSS 6.3.2** for code review; that number is from v3.2.1. In the current **v4.0.1** standard, the code-review requirement is **6.2.3** (*"Bespoke and custom software is reviewed prior to release into production or to customers, to identify and correct potential coding vulnerabilities"*), with sub-requirement **6.2.3.1** for code changes. **6.3.2 in v4.0.1 is a different requirement** — the new bespoke / third-party software inventory (SBOM-adjacent) mandate. Testing procedure 6.2.3.a expects the reviewer to be other than the originating author, knowledgeable in code review and secure coding, with management approval prior to release; manual / automated / hybrid all acceptable. Sources: VISTA InfoSec, securityreview.ai, Halock, Cybeats, KirkpatrickPrice, Pen Test Partners.

### Q1 — Semgrep CI integration

- **`semgrep ci` auto-detects baselines on GitHub Actions and GitLab CI.** The `SEMGREP_BASELINE_REF` env var (and the `--baseline-commit` CLI flag) is reserved for "other" CI providers; on GHA the baseline is computed from the PR event payload's merge base. The `--baseline-ref` form sometimes used in older guides is not the canonical flag — current docs name `--baseline-commit`. Sources: Semgrep CLI reference, CI environment variables, Add Semgrep to CI/CD, configuration reference.
- **Fetch-depth is the actual operational footgun.** `actions/checkout@v4` defaults to `fetch-depth: 1`, which prevents `semgrep ci` from reaching the merge base; the result is "failed to run a git command." Mitigations: emit `fetch-depth: 0` on the checkout step, or set `SEMGREP_GHA_MIN_FETCH_DEPTH`. Sources: Semgrep KB git command errors, CI environment variables.
- **Severity-based blocking does not exist for stand-alone (non-Semgrep AppSec Platform) users.** Verbatim from Semgrep's "Configure blocking findings" doc: *"If you do not use Semgrep AppSec Platform with Semgrep in CI or Semgrep Managed Scans (that is, you are using a stand-alone setup), all Semgrep findings are blocking findings."* Implication: rule *selection* is the only gating mechanism — the skill cannot say "include these rules but only block on ERROR."
- **`--config` is documented as "not supported in ci mode"** in the Semgrep CLI reference, but real-world workflows use it and it works in practice. Recommended workaround for the v1 template: use the `SEMGREP_RULES` env var rather than the flag. Confidence: medium-high; empirical confirmation still warranted.
- **Recommended adoption pattern for noisy existing repos:** scheduled full-scan on `main` (e.g., weekly) plus diff-aware `pull_request` job that only blocks on newly-introduced findings. Sources: Add Semgrep to CI/CD, Trail of Bits Testing Handbook continuous-integration guide.
- **OSS-only PR-feedback path:** SARIF upload to GitHub Code Scanning via `github/codeql-action/upload-sarif`. Findings appear in the repo's Security tab and as PR review comments without requiring Semgrep AppSec Platform. Source: 0xdbe walkthrough.
- **`semgrep ci` exit codes:** `0` clean, `1` findings present, `2` fatal error, `3` invalid target code, `4` invalid pattern, `5` unparseable YAML, `7` missing config, `8` invalid language, `13` invalid API key. `--suppress-errors` only suppresses error exit codes, not finding exit codes.

### Q2 — CWE → Semgrep rule pack mapping (verdict: registry metadata is canonical)

- **CWE is required metadata for security rules.** The Semgrep registry contributing guide lists `cwe`, `owasp`, `confidence`, `likelihood`, `impact`, `subcategory`, and `vulnerability_class` as mandatory for any rule submitted under `category: security`. CWE values are the long form (`"CWE-89: Improper Neutralization..."`), not the bare integer.
- **A `metadata-cwe.yaml` linter in the registry source enforces the tag at contribution time.** Sources: returntocorp/semgrep-rules `metadata-cwe.yaml` file; February 2026 release notes describe an improvement that makes the CWE tooltip show the specific CWE name on findings — implying older rules may have had imprecise labels even when the field was populated.
- **Concrete proof from a live registry rule** (`semgrep-rules/python/django/security/injection/sql/sql-injection-using-raw.yaml`): metadata block includes `cwe: ["CWE-89: ..."]`, `owasp: [A01:2017, A03:2021, A05:2025]`, `technology: [django]`, `cwe2022-top25: true`, `cwe2021-top25: true`, `subcategory: [vuln]`, `likelihood: MEDIUM`, `impact: HIGH`, `confidence: MEDIUM` — but no `vulnerability_class` field, evidence that legacy rules predate the schema tightening and the skill must tolerate field absence.
- **OWASP Top 10 2025 re-mapping** completed in early 2026: all 4,000+ registry rules carry OWASP 2025 + CWE metadata. Curated packs `p/owasp-top-ten` and `p/cwe-top-25` are coherent against the latest taxonomy. Sources: Semgrep blog "OWASP Top 10 2025: What's New," cwe-top-25 ruleset page.
- **No external "CWE → Semgrep pack" mapping document was found** at MITRE, OWASP, or third parties. The registry tag pages and the `semgrep-rules` raw repo are the canonical sources.
- **Caveat — historical SARIF/array-CWE bug:** issue [semgrep/semgrep #4673](https://github.com/semgrep/semgrep/issues/4673) documented `cwe`/`owasp` as arrays breaking SARIF export. Verify on current Semgrep ≥ 1.161 before relying on multi-CWE rules in SARIF uploads.

### Q3 — Audit evidence (partial; primary-source still required)

- **PCI DSS 6.2.3 (v4.0.1) expects two artifacts:** evidence the review happened (tool output OR peer-review record) and evidence each finding was either remediated or formally risk-accepted. Source: securityreview.ai, paraphrased verbatim. Pen Test Partners' v4.0 evidence checklist explicitly lists *"Code review records: evidence of peer reviews or automated scans of application code"* as acceptable.
- **No public source fixed format (SARIF vs PDF vs CSV) or retention window** for SAST artifacts under either PCI DSS 6.2.3 or SOC 2 CC7.1. Two rounds of search returned only generic compliance-vendor content.
- **NIST SSDF SP 800-218 PW.7 / PW.8** map to PCI 6.2.3 + SOC 2 CC7.1: PW.7 = review/analyze human-readable code; PW.8 = test executable code; implementation examples include automated static analysis in CI; retention is referenced abstractly as "establishing and enforcing security and retention policies for artifact data" without a specific window. PDF text extraction failed in WebFetch — secondary summaries align consistently.
- **"Mapped-but-not-scanned CWE coverage claim" as audit-failure pattern is intuitive but unsourced.** Multiple search rounds did not surface a published QSA writeup of an audit failing on this exact pattern. Treat the risk as plausible-but-unproven.
- **What the v1 wedge should capture per scan run** (synthesized; the inferred items are flagged): scan timestamp + git SHA scanned (high confidence on auditor relevance); rule pack identifier(s) + Semgrep version (high); SARIF output (high on format choice, medium on auditor acceptance); finding statuses (`open`/`fixed`/`risk_accepted`) (high); `cwes_claimed` from threat model vs `cwes_actually_covered` from union of `metadata.cwe` of selected rules (inference — defensive, not regulatory); threat-model file SHA at time of derivation (inference); reviewer / management-approval reference (high — the GitHub PR review record is the management-approval artifact for OSS practice).

### Q4 — `pull_request_target` posture

- **Material 2025 GitHub change.** The 2025-11-07 changelog announced (effective **2025-12-08**) that `pull_request_target` workflow file and checkout commit are forced to come from the **default branch**, regardless of the PR base branch. `GITHUB_REF` resolves to default branch; `GITHUB_SHA` to its latest commit. This kills the "outdated workflow on a stale base branch is still exploitable" attack class. Pre-Dec-2025 docs assuming base-branch sourcing are stale.
- **The wedge's hard ban on `pull_request_target` for SAST remains correct.** Even with the 2025-12-08 mitigation, `pull_request_target` confers full secret access on PR runs; secure-by-default is `on: pull_request` with no checkout of untrusted code into a privileged context. Sources: GitHub Security Lab "Preventing pwn requests," GitHub Docs Secure use reference, Wiz hardening guide.
- **`pull_request_target` is only required for SAST when results must be posted as PR comments using base-repo `GITHUB_TOKEN`** — fork PRs' default `GITHUB_TOKEN` cannot do that. The canonical alternative is the **two-workflow split**: Workflow A (`on: pull_request`, no secrets) runs the analysis and uploads SARIF as an artifact; Workflow B (`on: workflow_run`, `permissions: pull-requests: write`) downloads the artifact and posts the comment with elevated perms.
- **Minimum-permissions template** (cited rationale per line):
  - top-level `permissions: {}` — least-privilege default per Wiz / GitHub Security Lab
  - per-job `contents: read` — for `actions/checkout`
  - `security-events: write` only on the SARIF-upload job — required by `github/codeql-action/upload-sarif`
  - All third-party actions SHA-pinned (the **tj-actions/changed-files** compromise — CVE-2025-30066, March 2025 — is the canonical "tag rewriting" failure case)
- **Real-world incidents confirming widespread misconfiguration:** Sysdig (2024 audit found insecure `pull_request_target` patterns at MITRE, Splunk, others); pytorch/pytorch self-hosted runner takeover (Oct 2024); Shai Hulud v2 self-replicating worm via `pull_request_target` (Nov 2025, ~20k repos); CVE-2025-61671 Microsoft `pull_request_target` advisory (CVSS 9.3).
- **First-time-contributor handling:** GitHub's "Require approval for first-time contributors" repo setting plus `on: pull_request` (no fork-secret access) is sufficient — no `pull_request_target` needed. Defense in depth, not a primary control.
- **Forthcoming GitHub Actions 2026 security roadmap** introduces a `dependencies:` lockfile concept for action SHAs (analogous to `go.sum`) — not GA yet; manual SHA pinning remains v1 reality.

### Q5 — Threat-model-driven scanner configuration (verdict: wedge unoccupied)

- **No published OTM (or similar) → Semgrep config converter exists** in OSS or commercial tooling, across multiple search rounds.
- **OTM 0.2.0 (released 2023-08-30, license CC-BY-SA-4.0) has a `threats[].cwes` field** — structurally compatible with downstream CWE consumption. The OTM repo (iriusrisk/OpenThreatModel) does not document any Semgrep / SAST consumer. The associated **StartLeft** converter only transforms diverse inputs (MTMT, Terraform, Visio, Draw.io) **into** OTM, not out of it. OTM has been stagnant since 2023.
- **OWASP Threat Dragon's TMF format is incompatible with pytm, Threagile, and OTM** per its own wiki, and is being **superseded by CycloneDX TM-BOM** (OWASP Threat Model Library project) — but TM-BOM has no shipping consumers as of 2026-04.
- **OWASP pytm** uses an internal threat-ID scheme (`INP01`, `CR01`...) rather than CWE references; outputs JSON, Graphviz/DOT, PlantUML, Markdown — no Semgrep / CodeQL / Snyk integration in-tree.
- **Adjacent prior art:**
  - **Autogrep** (LambdaSec) — automates Semgrep rule *generation* from CVE patches using LLMs; orthogonal to threat-model-driven *selection*.
  - **AppSec Untangled "Threat Modeling Handbook #5"** — manual practice of converting a threat model to custom Semgrep rules in CI; not an automated pipeline.
  - **SecOpsTM** (ellipse2v) — STRIDE-as-code with MITRE ATT&CK / D3FEND / CAPEC mapping; outputs reports / Navigator layers, not Semgrep configs.
  - **Phoenix Security Semgrep integration** — ASPM correlation/enrichment over scan output; not config generation.
  - **Community Claude skills** `sast-semgrep` (AgentSecOps) and `claude-build-workflow-sast-semgrep` (rohunj) — wire Semgrep into a project but neither reads a threat model.
  - **GitHub CodeQL config**, **Snyk Code policy as code**, **Checkmarx One presets**, **Veracode policy framework** — manual policy/config files; none consume threat models.
  - **Academic:** "Semgrep*: Improving the Limited Performance of SAST Tools" (EASE 2024) covers hand-authored rule additions for +181% detection, not threat-model-driven selection. RealVuln (arXiv) is a benchmarking paper.
- **Defensible "why doesn't this exist?" answer:** (1) threat-model formats remain fragmented (pytm / Threat Dragon / OTM / Threagile incompatible; CycloneDX TM-BOM is the in-flight unifier); (2) Semgrep registry only got dense, queryable CWE metadata in the last ~24 months; (3) the diff-and-rederive loop is workflow-design work, not algorithm work — small surface area that vendors don't ship and OSS hadn't yet built because constituent pieces only just arrived.

## Library & Tool Evaluations

### Semgrep CLI (CE) `1.161.0` — April 2026

- License: LGPL-2.1 (CLI) + Semgrep Rules License for community rules.
- Pros: monthly cadence; SARIF output; native GHA baseline detection; dense queryable `metadata.cwe` / `technology`; OWASP 2025 re-mapping completed early 2026; OSS-installable (`pip install semgrep`, `uv` since Jan 2026).
- Cons: no severity-based gating for stand-alone users (rule selection IS the gate); `--config` officially "not supported in ci mode" but works in practice (docs vs behavior conflict); historical SARIF/array-CWE bug ([#4673](https://github.com/semgrep/semgrep/issues/4673)) — verify on installed version; CLI surface is large.
- Fit: primary engine, no substitutes. Pin minor in emitted workflow; bump deliberately. Use `SEMGREP_RULES` env var rather than `--config` flag in `semgrep ci` to dodge the docs-vs-behavior risk.

### `semgrep-rules` raw repo (`github.com/semgrep/semgrep-rules`)

- License: Semgrep Rules License (review for downstream OSS use).
- Pros: ground truth; queryable as files; lets the skill commit `.semgrep/rules/` with explicit pinned rule SHAs (auditable); enables programmatic `cwes_actually_covered` calculation.
- Cons: ~4k YAML files (needs an index step); legacy rules drift on schema (e.g., missing `vulnerability_class`).
- Fit: primary CWE-mapping source. Query at scan time, emit explicit rule list, make audit defense possible.

### Semgrep curated packs `p/cwe-top-25`, `p/owasp-top-ten`, `p/security-audit`, `p/github-actions`

- Pros: curated; recently re-mapped to OWASP 2025; `cwe2022-top25` / `cwe2021-top25` boolean tags allow fast filtering.
- Cons: opaque pack identifier conflicts with audit reproducibility; pulling all rules conflicts with the wedge's "tuned, not generic" promise.
- Fit: fallback default when threat-model parsing yields nothing, or when stack/CWE intersection is empty.

### `actions/checkout` v4

- License: MIT.
- Latest pinnable v4.2.x SHA referenced across guides; canonical pin currently in repo SECURITY.md is `692973e3d937129bcbf40652eb9f2f61becf3332` (v4.1.7); newer guides cite `d632683dd7b4114ad314bca15554477dd762a938` (v4.2.0) and `11bd71901bbe5b1630ceea73d27597364c9af683` (v4.2.2). Confirm the live SHA at execute-time.
- Pros: GitHub-maintained; SHA-pinnable.
- Cons: default `fetch-depth: 1` breaks Semgrep diff-aware scans → must emit `fetch-depth: 0`.

### `github/codeql-action/upload-sarif@v3`

- License: MIT.
- Pros: native Code Scanning UI integration without Semgrep AppSec Platform; surfaces findings as PR review comments automatically.
- Cons: requires `security-events: write`, scoped to that job only; SHA must be re-pinned periodically.

### Open Threat Model (OTM) spec `0.2.0`

- License: CC-BY-SA-4.0; released 2023-08-30; stagnant since.
- Pros: explicit `threats[].cwes` array; tool-agnostic.
- Cons: stagnant; no documented SAST consumer; ecosystem fragmented vs. CycloneDX TM-BOM (in flight, no shipping consumers).
- Fit: optional v1 input behind Markdown; do not depend on it.

### OWASP Threat Dragon

- License: Apache-2.0; active OWASP flagship.
- Pros: produces JSON + OTM exports; threats can carry `cwe` references.
- Cons: TMF format incompatible with pytm / Threagile / OTM per its own wiki; out-of-the-box threat objects don't always populate CWE; UI-driven, not file-first; not the format SLO emits.
- Fit: secondary input behind Markdown; accept its OTM export but don't require it.

### OWASP pytm

- License: GPL-3.0; active OWASP project.
- Pros: code-as-threat-model; programmatic CWE export possible in principle.
- Cons: Python dependency; uses internal threat IDs (INP01...) rather than CWE; no Semgrep adapter in-tree.
- Fit: out of v1 scope; reasonable v2 additional input.

### CycloneDX Threat-Model BOM (TM-BOM)

- Status: in-flight standard; no shipping consumers as of 2026-04.
- Fit: not v1; potential v2 swap-in if it stabilizes and Threat Dragon / pytm both adopt it. Re-check 2026-10.

### SecOpsTM (ellipse2v)

- STRIDE-as-code framework with MITRE ATT&CK / D3FEND / CAPEC mappings; outputs reports / Navigator layers, not Semgrep configs. Closest adjacent prior art on threat-model→automated-security-artifact spectrum but not a direct competitor.

### Autogrep (LambdaSec)

- Research artifact only; no packaged release. Automates Semgrep rule *generation* from CVE patches via LLMs. Orthogonal to v1 (selection, not generation); tangentially relevant to the future `/slo-rulegen` integration.

### Community Claude skills (`sast-semgrep`, `claude-build-workflow-sast-semgrep`)

- Adjacent prior art that wires Semgrep into a project; neither is threat-model-driven.

## Architecture Options

Three options surfaced from the consolidated findings.

### Option A — Pure Markdown skill, registry-query at scan time, deterministic rule manifest (recommended by the synthesis)

- Shape: SKILL.md drives Claude to (1) parse `docs/slo/design/<slug>-threat-model.md` for `CWE-\d+` mentions and stack hints, (2) detect stack from manifests (`Cargo.toml`, `package.json`, `requirements.txt`, `go.mod`, `pom.xml`, `pyproject.toml`), (3) clone-or-cache `semgrep-rules` at a pinned SHA and filter by `metadata.cwe ∋ CWE-NN:` AND `metadata.technology ∋ <stack>`, (4) emit `.semgrep/rules/` with selected rule files committed, (5) emit `.semgrep.yml` referencing local rules, (6) emit `.github/workflows/sast.yml` with the safe template (single `pull_request` workflow, `permissions: {}` top + per-job overrides, SHA-pinned actions, `fetch-depth: 0`, SARIF upload), (7) write `.semgrep/manifest.json` (`cwes_claimed`, `cwes_actually_covered`, threat-model SHA, `semgrep-rules` SHA, generated-at timestamp), (8) on threat-model edit, re-derive and surface a diff PR.
- Trade-offs: matches CLAUDE.md's "skills are the canonical interface" direction; ~0.5–1 person-week per the idea doc; deterministic enough because the rule set is committed back into the repo as concrete files; manifest defends against the inferred mapped-but-not-scanned audit pattern; no new Rust crate. Cons: registry filtering happens via Claude prompt logic (less crisp than typed code); `cwes_actually_covered` calculation is prompt-driven and must be covered by test fixtures; skill must include a fallback when registry is unreachable.
- Best when: v1 wedge for solo OSS maintainers — consistent with the idea doc's Approach A.

### Option B — Markdown skill + tiny `crates/sldo-sast-index` helper (rejected for v1)

- Shape: Same as A, but a small Rust binary (~200–400 LOC) builds a deterministic sqlite/JSON index of `(cwe, technology) → rule-path` from a pinned `semgrep-rules` SHA and exposes a query subcommand. Skill shells out for the lookup; everything else stays Markdown.
- Trade-offs: deterministic / unit-testable / byte-stable rule queries (strongest defense against the mapped-but-not-scanned audit pattern). Cons: re-introduces the `cargo install` bootstrap that the 2026-04 cleanup explicitly removed; conflicts with the stated "skills are canonical" direction.
- Best when: a future enterprise / regulated v2 where reproducibility evidence is required. Not v1.

### Option C — Two-workflow split (analysis + commenter), pre-templated (deferred)

- Shape: Skill emits two workflow files: `sast-analysis.yml` (`on: pull_request`, no secrets, runs `semgrep ci`, uploads SARIF as an artifact) and `sast-comment.yml` (`on: workflow_run`, `permissions: pull-requests: write`, downloads artifact, posts a single PR comment summarizing findings).
- Trade-offs: matches the GitHub Security Lab canonical pattern for SAST that needs to comment on fork PRs without `pull_request_target`. Cons: SARIF-to-Code-Scanning already surfaces findings in PR review without an explicit comment — most solo-OSS-maintainer wedges don't need this; doubles the YAML.
- Best when: v2 add-on for projects with meaningful fork-PR traffic that prefer a single bot comment per scan over the Code Scanning tab.

## API & SDK Documentation

The research surfaced documentation surfaces sufficient to design the wedge; deeper `chub` / `get-api-docs` sweeps will be needed at architect / execute time for exact flag tables and SHA pins. Key documentation entry points already collected in raw findings:

- **Semgrep CLI / CI:** "Add Semgrep to CI/CD" (canonical recipe), "CI configuration reference" (flag surface), "CI environment variables" (`SEMGREP_BASELINE_REF`, `SEMGREP_BASELINE_COMMIT`, `SEMGREP_GHA_MIN_FETCH_DEPTH`, `SEMGREP_RULES`), "Findings in CI" (blocking-finding semantics), "Configure blocking findings" (the "all findings block in stand-alone setup" verbatim text), "Sample CI configurations," KB "git command errors during PR/MR scan." Release-notes index for current minor version pinning.
- **Semgrep registry rule schema:** "Rule structure syntax," "Contribute rules to the Semgrep Registry" (mandatory metadata fields for `category: security`), `metadata-cwe.yaml` linter, registry tag pages, `p/cwe-top-25` and `p/owasp-top-ten` ruleset pages, OWASP Top 10 2025 re-mapping blog post.
- **GitHub Actions:** Changelog 2025-11-07 (`pull_request_target` defaults change effective 2025-12-08), Secure use reference, Actions 2026 security roadmap (action SHA lockfile, native egress controls), `pull_request_target` policy SHA-pinning changelog (2025-08-15), Security Lab "Preventing pwn requests" Parts 1–4.
- **OTM spec:** `iriusrisk/OpenThreatModel` repo + `otm_schema.json` (0.2.0); StartLeft documentation (input-only converter).
- **Compliance:** PCI Council document library (v4.0.1 PDF — assessor-side procedures need primary-source read); NIST CSRC SP 800-218 page (PDF text-extraction failed via WebFetch — needs alternate fetch); AICPA TSC 2017 + 2022 revisions (not surfaced in search).

The exact flag table for `semgrep ci` (especially the `--config` vs `SEMGREP_RULES` ambiguity), the verbatim PCI 6.2.3 testing procedure text, and the verbatim NIST PW.7 / PW.8 sub-tasks are open documentation gaps to close at architect time.

## Design Recommendations

1. **Adopt Option A (pure-Markdown skill, registry-query at scan time, deterministic rule manifest) as the v1 wedge.** Matches the idea doc's Approach A and CLAUDE.md's "skills are the canonical interface" direction. The manifest extension (Option 3 in earlier rounds) folds in, not as an alternative. **(confidence: high)**

2. **Hard-ban `pull_request_target` in the emitted workflow template; default to `on: pull_request`.** Even after the 2025-12-08 mitigation, `pull_request_target` confers full secret access; no SAST step the wedge generates needs that. **(confidence: high)**

3. **Emit `permissions: {}` at workflow scope and minimal per-job permissions:** `contents: read` for the analysis job, `security-events: write` only on the SARIF-upload step. Cite the rationale per line in inline comments. **(confidence: high)**

4. **SHA-pin every third-party action in the emitted template.** The `tj-actions/changed-files` (CVE-2025-30066, March 2025) and Shai Hulud v2 (Nov 2025) incidents make this non-negotiable. Establish an SLO refresh cadence (suggested 90 days) as project policy. Confirm exact pins at execute-time, not research-time. **(confidence: high)**

5. **Emit `fetch-depth: 0` on `actions/checkout`.** Default `fetch-depth: 1` causes "failed to run a git command" on `semgrep ci` diff-aware scans — documented in Semgrep's KB as the most common pitfall. **(confidence: high)**

6. **Treat the Semgrep registry's `metadata.cwe` field as canonical for CWE → rule mapping; query it at scan time rather than maintaining a hand-curated table.** Tagging is mandatory for `category: security` rules and enforced by a registry-side linter. **(confidence: high)**

7. **Tolerate legacy registry-rule schema gaps in the skill's parser.** The live Django SQLi rule example shows older rules can be missing `vulnerability_class` — assume any non-required field may be absent. **(confidence: high)**

8. **Use `SEMGREP_RULES` env var rather than `--config` flag inside `semgrep ci` invocations.** The CLI reference says `--config` is "not supported in ci mode," even though it works empirically — env-var path is the future-proof choice. **(confidence: medium-high)**

9. **Treat rule selection as the only severity gate; do not promise users "include this rule but only block on ERROR" semantics.** Stand-alone Semgrep blocks on every finding from every selected rule per the official "Configure blocking findings" doc. The skill's promise must be "we picked tight rules so the findings you get are the findings worth blocking on." **(confidence: high)**

10. **Pin the threat-model code-review compliance claim to PCI DSS 6.2.3 (v4.0.1), not 6.3.2.** The brief's number is from v3.2.1; 6.3.2 in v4.0.1 is the SBOM-inventory mandate. Update idea doc, threat-model template, and any future coverage doc. **(confidence: high)**

11. **Capture per-scan-run manifest data** (timestamp, git SHA scanned, rule pack identifiers, Semgrep version, SARIF artifact, finding statuses, `cwes_claimed` vs `cwes_actually_covered`, threat-model file SHA). Frame the `cwes_claimed` vs `cwes_actually_covered` divergence as **defensive design**, not regulatory necessity, until the mapped-but-not-scanned audit-failure pattern can be sourced. **(confidence: medium)**

12. **Default SARIF artifact retention to GitHub's default (90 days for free orgs, 400 for paid) and surface retention as a user-configurable knob in the workflow.** No public source fixed a specific retention window for PCI 6.2.3 / SOC 2 CC7.1 evidence; making it configurable defers the question to the user's auditor. **(confidence: medium)**

13. **Defer the two-workflow split (Option C) to v2.** SARIF upload to Code Scanning already surfaces findings in PR review for the wedge's solo-OSS-maintainer audience; the `workflow_run` commenter is overhead until fork-PR traffic justifies it. **(confidence: medium)**

14. **Accept Markdown threat models as the v1 input; treat OTM 0.2.0 `threats[].cwes` as an optional parse path, not a requirement.** OTM is stagnant since 2023 and has no SAST consumers; SLO already emits Markdown via `/slo-architect`. **(confidence: high)**

15. **Document the wedge's competitive position as "first-of-its-kind" rather than "displaces vendor X."** No published OTM-or-similar → Semgrep config converter exists; SecOpsTM is the closest adjacent player; vendor presets (Snyk Code, GHAS CodeQL, Checkmarx, Veracode) require manual policy authoring rather than threat-model intake. **(confidence: medium-high)**

16. **Re-check CycloneDX TM-BOM trajectory in 2026-10** (~6 months) for shipping consumers in Threat Dragon / pytm; it may become the v2 canonical input format. **(confidence: medium)**

## Risks & Open Questions

Ordered by what most blocks the architect / execute phases.

1. **`--config` vs `SEMGREP_RULES` env var on `semgrep ci`.** CLI docs say `--config` is "not supported in ci mode," but real-world workflows use it and it works. Empirical confirmation needed (run `semgrep ci --config p/cwe-top-25` against a fixture; check release notes for any forthcoming deprecation). Blocks workflow-template authoring.
2. **Verbatim PCI DSS v4.0.1 testing procedure 6.2.3.a/.b text.** Two rounds of public-web search returned paraphrases; authoritative version requires the PCI Council document library (membership / PDF download). Blocks any post-v1 coverage doc claiming "supports 6.2.3 evidence."
3. **NIST SSDF SP 800-218 PW.7 / PW.8 verbatim sub-tasks and notional implementation examples.** PDF binary-stream extraction failed via WebFetch. Re-fetch via `chub` / direct download or pull `nist.sp.800-218.ssdf-table.xlsx`. Blocks a defensible mapping table from "what the wedge captures" → "what NIST PW.7 expects."
4. **Real-world `metadata.cwe` coverage gaps in `semgrep-rules` per language.** A one-time empirical sweep — `% rules with cwe field, by language, by year` — would tell us whether registry-query alone is sufficient for v1's stacks (Python, JS/TS, Go, Java, Rust, Ruby, PHP) or whether a registered fallback to `p/security-audit` is needed. Achievable via a small script during the architect phase.
5. **Is "mapped-but-not-scanned CWE coverage claim" a documented audit-failure pattern?** Plausible, still unsourced after multiple rounds. Worth a targeted search of QSA-firm postmortems (Coalfire, Schellman, A-LIGN). If none surfaces, the manifest's `cwes_actually_covered` field is defensive, not required.
6. **SOC 2 CC7.1 specific evidence formats and retention.** AICPA TSC 2017 / 2022 needs primary read; Vanta / Drata template artifacts may surface a concrete artifact requirement. Blocks a defensible retention default in the workflow template (currently inferring "GitHub default 90 days").
7. **Severity-gating mechanism precise wiring on packs.** The CLI reference confirms stand-alone users have no severity gate; verify by running `semgrep ci` with a mixed-severity pack to confirm every finding blocks regardless of severity.
8. **`actions/checkout` and `github/codeql-action/upload-sarif` SHA refresh cadence.** Pin current v4.2.x at execute time; establish a 90-day refresh cadence as SLO project policy.
9. **CycloneDX TM-BOM trajectory.** Re-check 2026-10 for shipping consumers in Threat Dragon / pytm; affects whether v2 OTM-optional path swaps to TM-BOM.
10. **Semgrep SARIF + multi-CWE round-trip.** Issue [semgrep/semgrep #4673](https://github.com/semgrep/semgrep/issues/4673) historically broke array-typed `cwe` / `owasp` in SARIF. Verify on installed Semgrep ≥ 1.161 before relying on multi-CWE rules in SARIF uploads.
11. **Exact current Semgrep stable version.** Earlier rounds variously cite 1.161.0 (2026-04-22) and "version not surfaced." Pin via PyPI / GitHub releases at execute time.
12. **CVE/incident citations to verify.** `pytorch/pytorch` self-hosted runner takeover (Oct 2024) and CVE-2025-61671 Microsoft `pull_request_target` advisory were cited at medium confidence — confirm exact CVE IDs / advisory URLs before quoting in design docs.

## References

- [Add Semgrep to CI/CD](https://semgrep.dev/docs/deployment/add-semgrep-to-ci)
- [CI environment variables | Semgrep](https://semgrep.dev/docs/semgrep-ci/ci-environment-variables)
- [CI configuration reference | Semgrep](https://semgrep.dev/docs/semgrep-ci/configuration-reference/)
- [Sample CI configurations | Semgrep](https://semgrep.dev/docs/semgrep-ci/sample-ci-configs)
- [Findings in CI | Semgrep](https://semgrep.dev/docs/semgrep-ci/findings-ci)
- [Configure blocking findings | Semgrep](https://semgrep.dev/docs/semgrep-ci/configuring-blocking-and-errors-in-ci)
- [Semgrep CE in CI | Semgrep](https://semgrep.dev/docs/deployment/oss-deployment)
- [Semgrep CLI reference](https://semgrep.dev/docs/cli-reference)
- [Semgrep in CI | Semgrep KB](https://semgrep.dev/docs/kb/semgrep-ci)
- [Semgrep in CI | overview](https://semgrep.dev/docs/semgrep-ci/overview/)
- [Failed to run a git command during a pull request or merge request scan | Semgrep](https://semgrep.dev/docs/kb/semgrep-ci/git-command-errors)
- [Contribute rules to the Semgrep Registry | Semgrep](https://semgrep.dev/docs/contributing/contributing-to-semgrep-rules-repository)
- [Rule structure syntax | Semgrep](https://semgrep.dev/docs/writing-rules/rule-syntax)
- [Semgrep Registry — Tags](https://registry.semgrep.dev/tag)
- [`p/cwe-top-25` ruleset](https://semgrep.dev/p/cwe-top-25)
- [`p/owasp-top-ten` ruleset](https://semgrep.dev/p/owasp-top-ten)
- [`p/github-actions` ruleset](https://semgrep.dev/p/github-actions)
- [returntocorp/semgrep-rules `metadata-cwe.yaml`](https://github.com/returntocorp/semgrep-rules/blob/develop/yaml/semgrep/metadata-cwe.yaml)
- [Semgrep Release Notes (index)](https://semgrep.dev/docs/release-notes)
- [Semgrep Release Notes — February 2025](https://semgrep.dev/docs/release-notes/february-2025)
- [Semgrep Release Notes — December 2025](https://semgrep.dev/docs/release-notes/december-2025)
- [Semgrep Release Notes — January 2026](https://semgrep.dev/docs/release-notes/january-2026)
- [Semgrep Release Notes — February 2026](https://semgrep.dev/docs/release-notes/february-2026)
- [Semgrep Release Notes — March 2026](https://semgrep.dev/docs/release-notes/march-2026)
- [OWASP Top 10 2025: What's New | Semgrep](https://semgrep.dev/blog/2026/owasp-top-10-2025-whats-new/)
- [Semgrep — Imagine zero false positive SAST (2025)](https://semgrep.dev/blog/2025/making-zero-false-positive-sast-a-reality-with-ai-powered-memory/)
- [semgrep on PyPI](https://pypi.org/project/semgrep/)
- [semgrep/semgrep on GitHub](https://github.com/semgrep/semgrep)
- [semgrep/semgrep releases (GitHub)](https://github.com/semgrep/semgrep/releases)
- [semgrep/semgrep-rules (GitHub)](https://github.com/semgrep/semgrep-rules)
- [semgrep/semgrep issue #4673](https://github.com/semgrep/semgrep/issues/4673)
- [How To Enable Code Scanning With Semgrep — 0xdbe](https://0xdbe.github.io/GitHub-HowToEnableCodeScanningWithSemgrep/)
- [Sample Semgrep CI (j3ssie)](https://github.com/j3ssie/sample-semgrep-ci)
- [Continuous integration | Trail of Bits Testing Handbook](https://appsec.guide/docs/static-analysis/semgrep/continuous-integration/)
- [How to Set Up Semgrep in 2026 (DEV community)](https://dev.to/rahulxsingh/how-to-set-up-semgrep-in-2026-complete-installation-and-configuration-guide-5emm)
- [DevOps Daily — Semgrep SAST guide](https://devops-daily.com/guides/sast-tools/03-semgrep)
- [GitHub Changelog — Actions pull_request_target and environment branch protections changes (2025-11-07)](https://github.blog/changelog/2025-11-07-actions-pull_request_target-and-environment-branch-protections-changes/)
- [GitHub Changelog — Actions policy now supports blocking and SHA-pinning actions (2025-08-15)](https://github.blog/changelog/2025-08-15-github-actions-policy-now-supports-blocking-and-sha-pinning-actions/)
- [GitHub Actions 2026 security roadmap](https://github.blog/news-insights/product-news/whats-coming-to-our-github-actions-2026-security-roadmap/)
- [Complete Guide to GitHub Actions 2026 Security Roadmap (dev.to)](https://dev.to/x4nent/complete-guide-to-github-actions-2026-security-roadmap-dependency-locking-native-egress-5aap)
- [GitHub Docs — Secure use reference](https://docs.github.com/en/actions/reference/security/secure-use)
- [GitHub Security Lab — Preventing pwn requests (Part 1)](https://securitylab.github.com/resources/github-actions-preventing-pwn-requests/)
- [GitHub Security Lab — New vulnerability patterns and mitigation strategies (Part 4)](https://securitylab.github.com/resources/github-actions-new-patterns-and-mitigations/)
- [Wiz — Hardening GitHub Actions: Lessons from Recent Attacks](https://www.wiz.io/blog/github-actions-security-guide)
- [Sysdig — Insecure GitHub Actions in MITRE, Splunk, and other OSS repos](https://www.sysdig.com/blog/insecure-github-actions-found-in-mitre-splunk-and-other-open-source-repositories)
- [Orca Security — GitHub Actions security risks](https://orca.security/resources/blog/github-actions-security-risks/)
- [Orca Security — GitHub Actions Hardening](https://orca.security/resources/blog/github-actions-hardening/)
- [Orca Security — pull_request_nightmare Part 2 exploits](https://orca.security/resources/blog/pull-request-nightmare-part-2-exploits/)
- [AquilaX — GitHub Actions Security Hardening](https://aquilax.ai/blog/github-actions-security-hardening)
- [Arctiq — Top 10 GitHub Actions Security Pitfalls](https://arctiq.com/blog/top-10-github-actions-security-pitfalls-the-ultimate-guide-to-bulletproof-workflows)
- [GitGuardian — GitHub Actions Security Best Practices cheat sheet](https://blog.gitguardian.com/github-actions-security-cheat-sheet/)
- [Towards a secure-by-default GitHub Actions (community discussion #179107)](https://github.com/orgs/community/discussions/179107)
- [Improve Actions security (community discussion #157949)](https://github.com/orgs/community/discussions/157949)
- [GHSA-mrrh-fwg8-r2c3 — tj-actions/changed-files compromise (CVE-2025-30066)](https://github.com/advisories/ghsa-mrrh-fwg8-r2c3)
- [iriusrisk/OpenThreatModel — OTM specification repo](https://github.com/iriusrisk/OpenThreatModel)
- [iriusrisk/OpenThreatModel — `otm_schema.json`](https://github.com/iriusrisk/OpenThreatModel/blob/main/otm_schema.json)
- [Open Threat Model (OTM) — StartLeft documentation](https://iriusrisk.github.io/startleft/Open-Threat-Model-(OTM)/)
- [OWASP/threat-dragon — TMF format wiki](https://github.com/OWASP/threat-dragon/wiki/Threat-Model-File-(TMF)-format)
- [OWASP Threat Dragon project page](https://owasp.org/www-project-threat-dragon/)
- [OWASP/threat-dragon (GitHub)](https://github.com/OWASP/threat-dragon)
- [OWASP/pytm — Pythonic threat-modeling framework](https://github.com/OWASP/pytm)
- [OWASP pytm project page](https://owasp.org/www-project-pytm/)
- [OWASP Developer Guide — pytm chapter](https://devguide.owasp.org/en/04-design/01-threat-modeling/02-pytm/)
- [OWASP Threat Model Library (TM-BOM successor)](https://owasp.org/www-project-threat-model-library/)
- [OWASP Ontology Driven Threat Modeling Framework](https://owasp.org/www-project-ontology-driven-threat-modeling-framework/)
- [OWASP Threat Modeling Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Threat_Modeling_Cheat_Sheet.html)
- [SecOpsTM — STRIDE-as-code framework](https://github.com/ellipse2v/SecOpsTM)
- [A Dragon and Python walk into an OWASP card game (DragPyT)](https://threatmodeling.dev/dragpyt/)
- [Threat Modeling Handbook #5 — AppSec Untangled](https://medium.com/appsec-untangled/threat-modeling-handbook-5-convert-your-threat-model-into-an-automated-pentest-using-devsecops-84efcd138202)
- [Threat Modeling, Ch. 4 "Automated Threat Modeling" (O'Reilly)](https://www.oreilly.com/library/view/threat-modeling/9781492056546/ch04.html)
- [Autogrep — Automated Generation and Filtering of Semgrep Rules from Vulnerability Patches](https://lambdasec.github.io/AutoGrep-Automated-Generation-and-Filtering-of-Semgrep-Rules-from-Vulnerability-Patches/)
- [Phoenix Security — Semgrep ASPM integration](https://phoenix.security/phoenix-security-integration-semgrep/)
- [Semgrep* paper (ACM DL, EASE 2024)](https://dl.acm.org/doi/10.1145/3661167.3661262)
- [Semgrep* paper (ACM DL fullHtml)](https://dl.acm.org/doi/fullHtml/10.1145/3661167.3661262)
- [RealVuln benchmarking paper (arXiv)](https://arxiv.org/html/2604.13764)
- [GitLab — Customize SAST rulesets](https://docs.gitlab.com/user/application_security/sast/customize_rulesets/)
- [sast-semgrep agent skill (claude-plugins.dev)](https://claude-plugins.dev/skills/@AgentSecOps/SecOpsAgentKit/sast-semgrep)
- [Lobehub — claude-build-workflow-sast-semgrep](https://lobehub.com/skills/rohunj-claude-build-workflow-sast-semgrep)
- [IBM/mcp-context-forge issue #259 — Semgrep+ZAP Makefile/Actions targets](https://github.com/IBM/mcp-context-forge/issues/259)
- [PCI Security Standards Council — Document Library](https://www.pcisecuritystandards.org/document_library/)
- [VISTA InfoSec — PCI DSS Requirement 6 changes from v3.2.1 to v4.0](https://vistainfosec.com/blog/pci-dss-requirement-6-changes-from-v3-2-1-to-v4-0-explained/)
- [GuidePoint — PCI DSS 4.0 future-dated requirements](https://www.guidepointsecurity.com/blog/pci-dss-4-0-major-future-dated-requirements/)
- [Linford & Co — PCI DSS 4.0 Mandatory Requirements 2025 Guide](https://linfordco.com/blog/pci-dss-4-0-requirements-guide/)
- [Halock — PCI DSS v4.0.1 software catalog mandate](https://www.halock.com/what-is-the-new-pci-dss-v4-0-1-software-catalog-mandate/)
- [Cybeats — PCI DSS 4.0 SBOMs 2025 readiness](https://www.cybeats.com/blog/pci-dss-4-0-sboms-a-2025-readiness-guide)
- [KirkpatrickPrice — PCI Requirement 6.3.2 review of custom code prior to release (video)](https://kirkpatrickprice.com/video/pci-requirement-6-3-2-review-custom-code-prior-release/)
- [Pen Test Partners — PCI DSS v4.0 evidence and documentation requirements checklist](https://www.pentestpartners.com/security-blog/pci-dss-v4-0-evidence-and-documentation-requirements-checklist/)
- [securityreview.ai — Is your code or your pipeline the bigger PCI DSS 4.0 risk?](https://www.securityreview.ai/blog/is-your-code-or-your-pipeline-the-bigger-pci-dss-4-0-risk)
- [Strike Graph — PCI DSS vs SOC 2](https://www.strikegraph.com/blog/pci-dss-vs-soc-2)
- [SOC 2 Audit Checklist (2026)](https://soc2auditors.org/insights/soc-2-audit-checklist/)
- [Dsalta — SOC 2 compliance in 2025](https://www.dsalta.com/resources/articles/soc-2-compliance-in-2025-requirements-readiness-and-audit-success)
- [NIST SP 800-218 (CSRC)](https://csrc.nist.gov/pubs/sp/800/218/final)
- [NIST SP 800-218 SSDF table (xlsx)](https://csrc.nist.gov/files/pubs/sp/800/218/final/docs/nist.sp.800-218.ssdf-table.xlsx)
- [Aikido — NIST SSDF explained](https://www.aikido.dev/learn/compliance/compliance-frameworks/nist-ssdf)
- [OWASP Top 10 — 2025 edition](https://owasp.org/Top10/2025/)
