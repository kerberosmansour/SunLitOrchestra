---
topic: # Research brief — slo-security-embedding  ## Wedge (one sentence)  Update `/slo-architect` to produce `SECURITY.md` + `docs/design/threat-model.md` as first-class artifacts alongside `ARCHITECTURE.md…
generated_on: 2026-04-24 21:13:57 +0100
source_prompt_bytes: 5465
generator: sldo-research
---

# Research Dossier

This dossier is a structured research artifact produced by `sldo-research`. It is intended as the `prompt_file` input to `sldo-plan`.

## Repository Context

## Tech Stack

- **Primary language:** Rust 2021 edition, workspace with 7 member crates.
- **Workspace crates (`crates/`):** `sldo-common` (shared lib), `sldo-plan`, `sldo-run`, `sldo-research`, `sldo-install` (binaries), `sldo-tauri` (desktop app, currently parked), `sldo-tla-sha` (TLA+ tool-hash helper).
- **Key Rust deps** (workspace-pinned in root `Cargo.toml`): `clap 4` (derive), `anyhow 1`, `thiserror 2`, `regex 1`, `colored 2`, `chrono 0.4`, `which 7`. `sldo-install` adds `serde`/`toml 0.8`/`tempfile`. `sldo-tla-sha` adds `reqwest 0.11` (rustls-tls, blocking), `sha2 0.10`, `url 2`.
- **Desktop stack (parked):** Tauri v2 + React + TypeScript under `crates/sldo-tauri/ui/` (Node 18+, `npm`, Vite-style tooling implied by `npm run build`); uses OpenAI for voice transcription.
- **External CLI dependencies the code shells out to:** `claude` (Claude Code CLI — required on PATH by `preflight::check_claude_installed`) and `git`. README also references `copilot` for legacy bash scripts.
- **Skill pack:** Markdown `SKILL.md` files under `skills/slo-*/` consumed by Claude Code (not Rust code).
- **Legacy:** bash originals at `src/plan-milestones.sh`, `src/run-milestones.sh`.

## Project Structure

One level deep from repo root:

- `crates/` — Rust workspace members (see Tech Stack for list).
- `src/` — legacy bash scripts (`plan-milestones.sh`, `run-milestones.sh`); NOT the Rust sources.
- `tests/` — workspace-level E2E integration tests, one `e2e_*.rs` per milestone across plan/run/research/tauri/voice-tx/scaffold, plus `fixtures/`. Explicitly registered as `[[test]]` entries in the root `Cargo.toml`.
- `skills/` — the `/slo-*` skill pack (`slo-ideate`, `slo-research`, `slo-architect`, `slo-tla`, `slo-plan`, `slo-critique`, `slo-execute`, `slo-verify`, `slo-retro`, `slo-ship`, plus helpers `slo-freeze`, `slo-resume`, `slo-second-opinion`) and vendored `get-api-docs`. Each is a directory containing a `SKILL.md`.
- `docs/` — `ARCHITECTURE.md`, `MIGRATION.md`, runbook template (`runbook-template_v_3_template.md` is canonical), numerous `RUNBOOK-*.md` feature runbooks, and subdirs `completion/`, `critique/`, `design/`, `idea/`, `lessons/`, `research/` (including `research/slo-security-embedding/`).
- `output/` — default destination for `sldo-research` dossiers (gitignored).
- `target/` — Cargo build artifacts (gitignored).
- `.sldo-logs/`, `.copilot-logs/` — runtime log directories (gitignored).
- `.claude/` — local Claude Code state (gitignored).
- Top-level files: `README.md`, `CLAUDE.md` (project notes + baseline test command), `Makefile`, `Cargo.toml`, `Cargo.lock`, `.gitignore`, `.env`.

## Build & Test

- **Canonical baseline (per CLAUDE.md, because `sldo-tauri` breaks `--workspace` on macOS arm64):**
  ```bash
  cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install
  ```
- **Workspace build (non-Tauri):** `cargo build --workspace` (also `make check`). Release binaries: `cargo build -p sldo-install --release`, etc.
- **`Makefile` targets:** `dev` (`cargo tauri dev`), `build` (npm build + `cargo tauri build`), `test` (`test-backend` + `test-frontend`), `test-backend` (`cargo test --workspace` — currently red on macOS arm64 per CLAUDE.md), `test-frontend` (`cd crates/sldo-tauri/ui && npm test`), `setup` (installs frontend deps + `cargo install tauri-cli --version '^2'`), `check`, `clean`.
- **Install the skill pack locally:** `cargo build -p sldo-install --release && ./target/release/sldo-install` (writes to `~/.claude/skills/`; `--dry-run` and `uninstall` supported; manifest at `~/.sldo/install.toml`).
- **Run the binaries:** `cargo run -p sldo-plan -- …`, `cargo run -p sldo-run -- …`, `cargo run -p sldo-research -- …`. All three refuse to run if the target repo is on `main`/`master`.
- No dedicated lint target in the Makefile; `cargo fmt`/`cargo clippy` are not wired as make rules (file does not reference them).

## Existing Patterns

- **Error handling:** `anyhow::{Result, Context, bail}` on binary/glue paths; `thiserror 2` available in workspace for typed errors in libs. Frequent `.with_context(|| format!(...))` on IO and parse sites (see `sldo-common/src/logging.rs`, `sldo-tla-sha/src/lib.rs`).
- **CLI layout:** every binary defines a `clap::Parser` struct with derive, short/long flags, and `default_value` / `default_value_t`. Example: `sldo-research/src/main.rs` (`--prompt`, `--repo-dir`, `--max-iterations`, `--max-searches`).
- **Shared library layout:** `sldo-common` exposes flat modules — `color`, `copilot`, `detect`, `git`, `logging`, `preflight`, `runbook`, `toolflags` — each a single-responsibility file under `src/`.
- **External process invocation:** `std::process::Command` with stdout/stderr `Stdio::piped()` drained by **separate reader threads** joined through an `mpsc::channel` to avoid pipe-buffer deadlocks. Lines are simultaneously written to a `LogFile` via `logging::LogFile::append` (timestamped with `color::ts`).
- **Logging:** file-based, timestamped; each run creates/uses `.sldo-logs/` under the project dir (`logging::ensure_log_dir`). Per-phase scratch files for research live alongside the dossier.
- **Preflight gates:** `preflight::check_claude_installed` (via `which`), `check_file_exists`, `check_git_safety` (refuses protected branches).
- **Runbook parsing:** regex-driven table parsing of a "Milestone Tracker" with a `MilestoneStatus` enum (`not_started` / `in_progress` / `done`) implementing `Display` + `FromStr`.
- **Async:** none observed — everything is synchronous with threads where concurrency is needed. No `tokio`/`async-std` in the workspace `[workspace.dependencies]`.
- **Testing style:** BDD-flavoured comments (`// Given / When / Then`) inside `#[cfg(test)] mod tests`; each milestone has a matching top-level `tests/e2e_<area>_m<N>.rs` registered explicitly in the root `Cargo.toml` `[[test]]` table.
- **Safe HTTP (`sldo-tla-sha`):** `reqwest` blocking + rustls, explicit `ALLOWED_HOSTS` allow-list for post-redirect hosts, `DEFAULT_MAX_BYTES = 500 MiB` ceiling on streamed hashing.
- **Skills:** each `/slo-*` is a directory with a single `SKILL.md` (no code). Consumed by Claude Code's skill loader, not compiled.

## Constraints

- **Target baseline platform:** macOS arm64 (user's dev box; Darwin 25.4.0 per environment). Explicit known breakage: full-workspace `cargo test` is red on macOS arm64 because `crates/sldo-tauri/ui/node_modules/` is missing the arm64 esbuild binary (per CLAUDE.md) — **do not use `--workspace` as the baseline**; use the 5-crate list in "Build & Test".
- **`crates/sldo-tauri/` is parked (CLAUDE.md, 2026-04):** do NOT modify it; do NOT merge its branch into skill-pack work.
- **Rust edition:** 2021 across every crate. No explicit MSRV is pinned in any `Cargo.toml`, `rust-toolchain.toml`, or `Cargo.lock` metadata observed in the files read — treat MSRV as "whatever current stable supports clap 4, thiserror 2, reqwest 0.11".
- **Node version for the parked Tauri UI:** README states Node 18+, Tauri CLI `^2`.
- **External runtime prerequisites:** `claude` CLI must be on PATH for `sldo-plan`/`sldo-run`/`sldo-research`; `git` must be available and the repo must NOT be on `main`/`master` (`preflight::check_git_safety`). Research runs consume Claude API credits — quotas bound by `--max-iterations` (default 3) and `--max-searches` (default 5).
- **Security posture called out in README/code:**
  - Do not pass untrusted prompt files to `sldo-research` (Claude Code with `WebFetch`/`WebSearch` can ingest hostile content).
  - `.sldo-logs/` and research scratch files may contain proprietary source excerpts — treat as internal.
  - Do not ship a shared `OPENAI_API_KEY` in a distributed binary; key is read server-side only by the Tauri backend.
  - `sldo-tla-sha` hardens downloads via a host allow-list and a hard byte ceiling.
- **Canonical planning artifact:** every feature runbook lives at `docs/RUNBOOK-<FEATURE>.md` and must follow `docs/runbook-template_v_3_template.md` (CLAUDE.md forbids bypassing it via batch CLI shortcuts).
- **License:** no `LICENSE` file was present in the top-level listing — license is **unstated** in the repo surface I examined.

## Executive Summary

The research brief asked five questions about embedding a generated security layer (`SECURITY.md` + `docs/design/threat-model.md`) into `/slo-architect` as a first-class artifact. The findings converge on a viable — but not purely Rust-native — stack: **CycloneDX 1.7 declarations** as the capability-manifest wire format, **OWASP SecOpsTM v1.1.0** (a Python wrapper around pytm + LLM + RAG) as the closest agent-composable threat-modelling engine, **OTM 0.2.0** as the threat-model interchange, **Semgrep CE + ast-grep 0.42** as the variant-analysis pair (with CodeQL nightly), and a **client-side rate guard + SLO-owned intake repo** as the defensible default for cross-repo capability-gap filing.

Two load-bearing constraints emerged from primary sources. First, the canonical Rust crate `cyclonedx-bom` 0.8.1 still targets CycloneDX **spec 1.5**, so no Rust-native emitter for 1.6+ `declarations` exists today — any pure-Rust Option must hand-roll JSON or shell out. Second, CycloneDX declarations express *catalog/standard-level* conformance cleanly but do **not** schema-enforce parametric primitive-level claims ("Argon2id with ≥3 iterations, ≥64 MiB memory"), which must ride in `properties`, a `cryptographic-asset` component, or external references. Compliance-framework adoption data for OSS-tool users specifically does not exist in any maintainer survey found; the commercial consensus baseline (Vanta 15k+ / Drata 8k+ customers) is SOC 2 + ISO 27001 + HIPAA + PCI DSS + GDPR, and that is the strongest evidence available for a default column set. GitHub's secondary rate limit on content-creating requests (80/min, 500/hr) is authoritative, but per-endpoint point cost is deliberately undocumented — so operational prudence requires a self-imposed client-side cap.

## Topic Decomposition

The brief's five questions map to three distinct technical concerns and two operational concerns:

1. **Machine-readable capability advertising** (Q1) — what format does a library use to say "I implement control X"? Competing formats: CycloneDX 1.6+ declarations, OSCAL Component Definition v1.1.3, and the emerging TMBOM effort. SARIF and SPDX adjacent but not the primary answer.
2. **Default compliance-framework column set for the threat-model table** (Q2) — which frameworks cover ≥80% of SLO users without bloat? Evidence base: commercial compliance-automation vendor catalogs (Vanta, Drata), framework-selection heuristics from vendor blogs, and the absence of OSS-specific adoption data.
3. **LLM-agent threat-modelling prior art** (Q3) — what existing tools, schemas, and academic work can `/slo-threat-model` borrow from instead of reinventing? SecOpsTM, pytm, STRIDE-GPT, Threat Dragon, OTM, MITRE ATLAS, academic papers (Laponina 2025; arXiv 2411.17058; arXiv 2504.19956; ThreatCompute CCSW 2025).
4. **Variant-analysis tooling with a <60s interactive budget** (Q4) — which SAST/pattern tools fit agent ergonomics? Converges on Semgrep + ast-grep for interactive, CodeQL for nightly, with Opengrep as the licensing hedge post-December 2024.
5. **Cross-repo issue-filing operational limits** (Q5) — what actually constrains `gh issue create` against a repo the agent does not own? GitHub's documented rate limits, template-enforcement unreliability, and attribution semantics.

## Key Findings

### Q1 — Capability manifest formats

- **CycloneDX 1.7** is the current published spec (October 2025), codified as **ECMA-424 2nd edition (December 2025)**. CycloneDX 1.6 (April 2024) introduced the *Definitions / Declarations* surface for standards, best practices, maturity models, and signed attestation claims, and added the Cryptographic Bill of Materials (CBOM) profile. The format is JSON or XML, ECMA-standardized, and OWASP-stewarded, covering SBOM, SaaSBOM, HBOM, AI/ML-BOM, CBOM, OBOM, MBOM, VDR, and VEX. OWASP ASVS, MASVS, SCVS, and SAMM are already published in CycloneDX format.
- **CycloneDX declarations have a granularity cliff.** Catalog/standard-level claims ("component X conforms to ASVS 5.0 V2.4") are schema-native. Parametric primitive-level claims ("Argon2id with ≥3 iterations, ≥64 MiB memory, ≥4 parallelism") are not schema-enforced and must ride in `properties`, a `cryptographic-asset` component, or `externalReferences`.
- **OSCAL** is a NIST-backed XML/JSON/YAML family of schemas for publishing, implementing, and assessing security controls. The **OSCAL Foundation** was formed as an industry consortium in January 2025. The current Component Definition model is **v1.1.3**. NIST CSWP 53 ipd (2025) asserts OSCAL alignment with SPDX and CycloneDX and explicitly names agentic-AI alignment ("Autonomous Risk Reasoning with Agentic AI") as a direction.
- **CivicActions/oscal-component-definitions** is a reusable library of OSCAL component definitions, but it's small (v1.0.0, May 2024; ~12 commits, ~16 stars) — useful as template material rather than a living dependency.
- **OSCAL ↔ CycloneDX crosswalk is asserted but not published as a lossless table.** NIST's documented control-mapping model concerns OSCAL's *internal* catalog crosswalks (NIST 800-53 ↔ CIS ↔ ISO 27001), not a bidirectional OSCAL↔CycloneDX mapping. Sbomify publishes an SPDX↔CycloneDX crosswalk but not OSCAL↔CycloneDX.
- **Emerging: TMBOM** (Threat Model Bill of Materials), a joint effort across CycloneDX, OWASP Threat Dragon, and pytm to standardize threat-model interchange. Not yet a stable spec.
- **SARIF 2.1.0** is the interchange for static-analysis results and travels alongside CycloneDX in production pipelines (e.g., Nox emits both; ReARM's 2025-09-01 release ingests SARIF + CycloneDX VDR/BOV).
- **VEX / VDR** are the CycloneDX fields already used to state "control X mitigates CVE Y for component Z" — the natural surface for the capability-gap issue flow.
- **Rust tooling gap (load-bearing).** `cyclonedx-bom` 0.8.1 (2026-03-19) still targets **CycloneDX spec 1.5**, with no upstream 1.6+ `declarations` support. `cargo-cyclonedx` 0.5.9 generates SBOMs for Rust workspaces but not declarations. A pure-Rust Option A would have to upstream 1.6 support, hand-roll declarations JSON, or shell out to Node/Python toolchains.

### Q2 — Compliance framework defaults

- **Commercial consensus baseline.** Vanta (15,000+ customers, 35+ frameworks) and Drata (8,000+ customers, 26+ frameworks) both treat **SOC 2, ISO 27001, HIPAA, PCI DSS, and GDPR** as the common core for SaaS/web-service teams.
- **Framework-selection heuristic** (repeated across vendor blogs in 2025–2026): SOC 2 first for US SaaS → enterprise; ISO 27001 first for EU/global; HIPAA if PHI is touched; PCI DSS if payments are in scope. SOC 2 and ISO 27001 overlap roughly 70–80%, making them economical to pair.
- **Newer frameworks** entering both platforms' 2026 catalogs: **ISO 42001** (AI management), **DORA** (EU financial), **NIS2** (EU critical infrastructure), **FedRAMP**.
- **OWASP ASVS 5.0.0** was released in May 2025 at Global AppSec EU Barcelona, with ~350 requirements across 17 chapters.
- **OSS-maintainer survey data does not cover per-framework adoption.** Tidelift's 2024 state-of-maintainers reports Scorecard awareness at 40%, NIST SSDF at 39%, SLSA at 23% — but publishes no SOC 2 / ISO 27001 / HIPAA numbers for OSS maintainers. Snyk's 2024 OSS Security report covers practices (SBOM generation at ~62%, pipeline security at ~50%) not frameworks. This evidence gap is structural.
- Earlier research baseline (NIST 800-53 + IEC 62443 + SOC 2) under-serves EU users (no GDPR) and does not name ASVS — both surfaced as gaps in the deepened analysis.

### Q3 — LLM-agent threat-modelling prior art

- **MITRE ATLAS v5.1.0** (November 2025) is the canonical AI-adversary TTP knowledge base: 16 tactics, 84 techniques, 32 mitigations, 42 case studies. October 2025 added 14 AI-agent-specific techniques in collaboration with Zenity Labs (including `AML.T0058 AI Agent Context Poisoning`, Memory Manipulation, Thread Injection, Modify AI Agent Configuration); tactic `AML.TA0015 Command and Control` was added. February 2026 added further agentic-AI techniques.
- **OWASP SecOpsTM** has real tagged releases: **v1.0.0 (2025-08-13)** and **v1.1.0 (2025-08-26)** — a significant maturity signal. It fuses three engines: the pytm rule engine for deterministic STRIDE identification; a component-level LLM (Ollama offline, Gemini, OpenAI, Mistral) for AI-enriched threats; and a RAG pipeline (ChromaDB + HuggingFace embeddings) for system-level cross-boundary threats. It emits mappings to MITRE ATT&CK, CAPEC, CVE, D3FEND, CIS Controls, and NIST 800-53, plus SVG/HTML reports.
- **pytm** is at v1.3.1 (April 25, 2024), with no release in ~24 months — stagnant standalone. Multiple posts note pytm lacks direct NIST 800-53 mappings; SecOpsTM adds them on top.
- **STRIDE-GPT** (mrwadams) is an active OSS LLM-driven STRIDE threat modeler, recently extended with OWASP LLM Top 10 (LLM01–LLM10) mode. Comparative analyses (Pure Storage, FuzzingLabs, 2025) flag **unstable categorization** across runs — unsuitable as a dependency without a validation gate.
- **OWASP Threat Dragon** is an active OWASP Lab project supporting STRIDE, LINDDUN, CIA, DIE, PLOT4ai with a rule engine; LLM extensions exist (`threat-dragon-ai-tool` by InfosecOTB, `threat-dragon-llm` by otisthescribe).
- **OTM 0.2.0 (Open Threat Model)** from IriusRisk is the only platform-independent threat-model interchange with a published JSON schema (Apache 2.0). Schema has been stable for ~2 years — treat as frozen, not stalled.
- **Other tools surveyed:** OWASP Threat Model Library (curated reference corpus); OWASP Threat Modeling Cheat Sheet; Microsoft Threat Modeling Tool; ThreatFinderAI; IriusRisk and SecuriCAD (commercial).
- **Methodology catalog** surfaced in 2025 trend reporting: STRIDE, LINDDUN, PASTA, STRIDE-AI, PLOT4ai, ADMIn, MAESTRO.
- **Academic prior art (2024–2025):**
  - Laponina, *"Threat Modeling Software Development for LLM-Agent-Based Systems"*, Int. J. of Open Information Technologies, 2025.
  - arXiv 2411.17058 — *"ThreatModeling-LLM: Automating Threat Modeling using LLMs for Banking System"*.
  - arXiv 2504.19956 — *"Securing Agentic AI: A Comprehensive Threat Model and Mitigation Framework"* (SHIELD).
  - ResearchGate 396511419 — *"Extending STRIDE and MITRE ATLAS for AI-Specific Threat Landscapes"*.
  - ACM CCSW 2025 — *ThreatCompute*, combining LLMs with attack graphs for Kubernetes; finds hybrid approaches reduce manual effort but struggle with stable categorization.
- **Consensus compliance triad for AI/agent-focused threat models:** OWASP LLM Top 10 + MITRE ATLAS + NIST AI RMF (Giskard, Straiker, AWS, multiple 2025–2026 posts).
- **Cross-cutting OSS reality (USENIX Security 2025 prepub, Kaur et al.):** OSS threat-modeling is "almost always ad hoc" because structured TM is perceived as high-cost / low-benefit by volunteer maintainers — direct support for a generated-not-prompted approach.

### Q4 — Variant analysis outside a monorepo

- **Semgrep CLI** emits `--sarif` / `--sarif-output=<file>` natively; also supports text, json, gitlab-sast, gitlab-secrets, junit-xml, emacs, vim. 2026 data puts median scans at ~10 s, memory at ~150 MB, and language coverage at 30+ (including Bash, Dart, Elixir, Lua, OCaml, PHP, Rust, Scala, Solidity, Terraform, Dockerfile, YAML). Cross-file analysis is absent for Ruby, PHP, Swift, Rust. Semgrep v1.160.0 (2026-04-16) moved Scala parsing to tree-sitter and improved pro-engine taint for variadic functions.
- **Licensing shift:** In December 2024 Semgrep moved several previously-open features behind commercial license. In January 2025, ~10 vendors forked Semgrep CE into **Opengrep**. The core engine remains LGPL-2.1, but pro rules are gated.
- **CodeQL** covers ~12 languages (C, C++, C#, Go, Java, Kotlin, JS, TS, Python, Ruby, Swift, Rust) with semantic data-flow/taint analysis via a relational database build. Scans take minutes to 30+ minutes, use ~450 MB, and require a buildable environment. Variant analysis has identified **400+ CVEs in OSS projects**. Free for OSS via GitHub Advanced Security. GitHub added incremental analysis for all supported languages in September 2025.
- **ast-grep** is Rust-native, MIT-licensed, tree-sitter based. Current version: **0.42.0 (2026-03-16)**; depends on `serde-sarif ^0.8.0`; **native SARIF output landed in 0.40.0 (November 2025)**. Positioned as a lighter-weight structural search tool without a cloud tether. Rule ecosystem is smaller than Semgrep's; lacks taint and equivalences.
- **Combined-usage pattern** (recurring industry recommendation): Semgrep in CI on every PR for ~10 s feedback; CodeQL on nightly/weekly for depth. Both emit SARIF, so results merge cleanly in GitHub's Security tab.
- **mrva** (Trail of Bits, 2025-12-11) brings CodeQL multi-repo variant analysis to the terminal — directly addresses the "variant analysis outside a monorepo" question.
- **`sarifw`** (lambdasawa) is a third-party SARIF wrapper that emits SARIF from ripgrep and ast-grep output — a fallback if a tool's native SARIF isn't sufficient.

### Q5 — Cross-repo issue filing

- **Primary rate limits** (GitHub Docs): 5,000 REST req/hr for a standard PAT/OAuth user token; 1,000 req/hr for `GITHUB_TOKEN`; 15,000 req/hr for GitHub Apps owned by a GHEC org; 2,000 OAuth token requests/hr.
- **Secondary rate limit on content creation:** **80 content-creating requests per minute, 500 per hour**, applied uniformly to issues, comments, PRs, and commits across REST, GraphQL, and the web UI. Exceeding returns HTTP 403 or 429 with a `retry-after` header. **Per-endpoint point cost is not publicly disclosed.** There is no documented way to bypass or raise secondary limits — only wait for reset.
- **Cross-repo filing is not documented as separate from same-repo filing.** No evidence that cross-repo creation has its own threshold.
- **Abuse / spam heuristics** are deliberately undocumented. Third-party clients (e.g., octokit throttling plugin, issue #108) report generic "abuse detection" 403s with no explicit thresholds; recommended mitigation is to respect `retry-after` and back off. No evidence that low-volume (per-week) authenticated traffic triggers abuse filters.
- **OAuth scopes:** `gh` ships a default bundle (`repo`, `read:org`, `gist`); `repo` (or `public_repo` for public-only) is sufficient to open issues on a repo the user does not own. `workflow` is needed only for Actions workflows. Known friction in cross-repo scope handling: `cli/cli#9380`; ongoing effort to issue tokens with more restricted scopes: `cli/cli#10500`.
- **Attribution:** `gh issue create` authenticates as the local user's GitHub identity; the issue is authored by that user. No supported "file-as-service-account" path via `gh` without a separate GitHub App install.
- **Enforced issue templates:** repositories can mandate `.github/ISSUE_TEMPLATE/*.yml` forms with `validations: required: true`. However, enforcement is **known to be unreliable in organization-owned repos** (community discussion #43859), and `required` validation on checkboxes/dropdowns is inconsistent (discussion #45084). REST `POST /repos/{owner}/{repo}/issues` bypasses the web form; an agent cannot rely on template enforcement as a guardrail.
- **Alternative channels identified:** fork + PR (requires `repo` scope + push to fork; much higher upstream acceptance rate because it delivers a concrete contribution); GitHub Discussions (GraphQL-only `createDiscussion` mutation, no REST parity, requires category to exist); SLO-owned intake repo (decouples from upstream rate-limit pressure).
- **Operational implication:** "A few capability-gap issues per week" is ~3–4 orders of magnitude below the 80/min, 500/hr thresholds, so primary-volume rate limits are not the binding constraint at that frequency. The binding constraints are attribution semantics, template validation reliability, and — at larger session volumes — unpublished point costs for content endpoints.

## Library & Tool Evaluations

### Capability-manifest formats

| Candidate | Version | License | Rust support | Assessment |
|---|---|---|---|---|
| **CycloneDX 1.7 + Declarations** | spec 1.7 (Oct 2025), ECMA-424 2nd ed. (Dec 2025) | Apache 2.0 / ECMA | **Partial gap** — `cyclonedx-bom` 0.8.1 still tracks spec 1.5 | Purpose-built for catalog-level conformance claims; attestation + claims + evidence model supports signed assertions. Primitive-level parametric claims require `properties`, `cryptographic-asset`, or external references — not schema-enforced. Lowest lift if an SBOM is already emitted. Active ecosystem, standardized as ECMA-424. |
| **OSCAL Component Definition** | v1.1.3 | Public domain (NIST) | No mature Rust crate surfaced | Most mature "component → controls" manifest. First-class catalog linking (NIST 800-53, CIS, custom). OSCAL Foundation industry-owned since Jan 2025. Verbose per file; authoring cost higher than CycloneDX. Cross-walked with SPDX + CycloneDX per NIST CSWP 53 ipd, but no published lossless mapping table. |
| **CivicActions oscal-component-definitions** | v1.0.0 (May 2024), ~12 commits, ~16 stars | Public domain | n/a | Template source / reference material; too small to be a live dependency. |
| **OTM 0.2.0** | frozen 0.2.0 | Apache 2.0 | Go ref. lib exists (`threatcl/go-otm`) | Threat-model *interchange*, not a capability manifest. Best used as `/slo-threat-model` output schema with a validation gate. |
| **TMBOM** | in development | n/a | n/a | Joint CycloneDX × Threat Dragon × pytm effort. Watch; don't bet. |
| **SPDX 3.0** | 3.0 published; tooling lag at 2.3 | — | — | License-centric; 3.0 adds security profiles but narrower security coverage than CycloneDX. Not the primary candidate for capability manifests. |
| **SARIF 2.1.0** | 2.1.0 (OASIS) | — | `serde-sarif` crate family used by ast-grep | Interchange format for static-analysis findings; travels *alongside* capability manifests, not a substitute. |
| **`cyclonedx-bom` (Rust)** | 0.8.1 (2026-03-19) | Apache 2.0 | native | Tracks CycloneDX spec 1.5 only. No 1.6+ declarations support in the Rust ecosystem today. |
| **`cargo-cyclonedx`** | 0.5.9 (2026-03-19) | Apache 2.0 | native | Generates SBOMs for Rust workspaces; adjunct to declarations, not a replacement. |

### Threat-modelling engines

| Candidate | Version | License | Assessment |
|---|---|---|---|
| **OWASP SecOpsTM** | **v1.1.0 (2025-08-26)** | OSS / OWASP | Closest agent-composable precedent for `/slo-threat-model`. Python stack (venv + ChromaDB + HuggingFace + optional LLM backends). Emits STRIDE with MITRE ATT&CK, D3FEND, CAPEC, CIS, NIST 800-53 mappings; supports Ollama offline + Gemini + OpenAI + Mistral. Platform coverage and install footprint not characterized in the findings. |
| **pytm** | v1.3.1 (April 2024) | MIT | Canonical Python threat-model-as-code; ~24 months since last release — stagnant. No native NIST 800-53 mappings. Use via SecOpsTM wrapper only. |
| **STRIDE-GPT (mrwadams)** | Active OSS; no pinned version surfaced | MIT | Reference for prompt patterns; added OWASP LLM Top 10 mode. Documented categorization instability makes it unsuitable as a direct dependency. |
| **OWASP Threat Dragon** | Active OWASP Lab; no version surfaced | Apache 2.0 | STRIDE/LINDDUN/CIA/DIE/PLOT4ai rule engine. GUI-first; its JSON DFD format is a potential input for agent composition. LLM extensions exist (`threat-dragon-ai-tool`, `threat-dragon-llm`). |
| **OTM 0.2.0** | frozen | Apache 2.0 | Validation-schema role for `/slo-threat-model` outputs. |
| **MITRE ATLAS** | **v5.1.0 (Nov 2025)**, with Oct 2025 + Feb 2026 agentic updates | CC / MITRE | Reference knowledge base, not a tool. Citation target for AI-specific threats. |
| **Microsoft Threat Modeling Tool / IriusRisk / SecuriCAD** | — | commercial / free-tier | Surfaced as landscape context; no integration path proposed in findings. |

### Variant-analysis tooling

| Candidate | Version / status | License | Assessment |
|---|---|---|---|
| **Semgrep CE** | v1.160.0 (2026-04-16); LGPL core unchanged post-Dec 2024 relicense | LGPL-2.1 core / proprietary Pro | Primary candidate for <60 s budget. ~10 s median scan, ~150 MB RAM, 30+ languages, native `--sarif`. Cross-file analysis absent for Ruby/PHP/Swift/Rust. |
| **Opengrep** | Forked Jan 2025 (~10-vendor coalition) | LGPL-2.1 | Hedge against Semgrep commercial gating. Rule ecosystem still tracking Semgrep CE. |
| **CodeQL** | Actively maintained; incremental analysis all-languages since Sep 2025 | proprietary; free for public OSS | Nightly-only for the SLO interactive budget. Deeper data-flow/taint; ~400+ OSS CVEs found via variant analysis. Minutes–30+ min scans, ~450 MB DB. |
| **ast-grep** | **0.42.0 (2026-03-16)**; SARIF since 0.40.0 (Nov 2025) | MIT | Rust-native companion; fast structural search without a cloud tether. Pair with Semgrep rather than substitute. MSRV not surfaced. |
| **`mrva` (Trail of Bits)** | Announced 2025-12-11 | — | Terminal-first multi-repo CodeQL variant analysis; directly relevant but outside the interactive budget. |
| **`sarifw` (lambdasawa)** | OSS | — | SARIF wrapper for ripgrep / ast-grep; fallback if native SARIF is insufficient. |

### Agent-filing channels

| Channel | Auth | Friction | Rate pressure | Upstream cost |
|---|---|---|---|---|
| `gh issue create` (direct) | user token, `repo` or `issues:write` | low | shared 80/min, 500/hr secondary pool; template validation advisory only | high if many gaps filed (spam shape); authored as user |
| `gh pr create` (fork + implementation) | same + write to fork | high (agent must implement the capability) | same pool | **low** — concrete contribution, higher acceptance rate |
| SLO-owned intake repo | PAT or webhook | medium | none upstream | lowest — decouples from third-party rate limits and upstream maintainer attention |
| GitHub Discussions | GraphQL `createDiscussion` only (no REST) | medium | shared pool; requires categories to exist | medium |

## Architecture Options

Three options surfaced in the raw findings. All three use the same `/slo-threat-model` interchange (OTM 0.2.0) and the same variant-analysis pairing (Semgrep + ast-grep, with CodeQL nightly). They differ on the capability-manifest format, the threat-model engine, and the default filing channel.

### Option A — "CycloneDX-native with Rust escape hatch"

- **Capability manifest:** CycloneDX 1.7 BOM with hand-rolled `declarations` JSON (or a thin new Rust crate emitting declarations against the 1.6 schema), because `cyclonedx-bom` 0.8.1 still targets 1.5. Primitive-level claims (e.g., Argon2id parameters) encoded under `properties` in a vendored SLO property-taxonomy namespace, validated against an internal schema at emit time.
- **Threat model:** `/slo-threat-model` shells out to **SecOpsTM v1.1.0** in a vendored Python venv; post-processes SecOpsTM output into OTM 0.2.0 JSON and validates against the OTM schema before writing.
- **Variant analysis:** Semgrep CE + ast-grep 0.42 both emit SARIF; results merged into a CycloneDX VDR sidecar.
- **Filing:** `gh issue create` with a client-side ≤40/hr per-session guard; SLO-owned intake repo as the fallback channel.
- **Trade-offs:** Pragmatic, schema-agnostic, ships fastest. Introduces a Python subprocess dependency (SecOpsTM). Property-taxonomy is a local convention until the upstream CycloneDX Property Taxonomy absorbs it.
- **Best fit per findings:** SLO today — Rust-first, solo-operator, prioritizes working artifacts this quarter.

### Option B — "OSCAL-anchored, compliance-defensible"

- **Capability manifest:** OSCAL Component Definition v1.1.3 per library, referencing NIST 800-53 + ASVS 5.0 + CIS v8. Capability claims live in `implemented-requirements.statements.by-component`; primitives expressed via `props` under an SLO-controlled vocabulary. CycloneDX declarations generated as a derived view.
- **Threat model:** SecOpsTM → OTM JSON plus an OSCAL `assessment-plan` linking threats to the `implemented-requirements` records.
- **Variant analysis:** Semgrep + CodeQL (nightly) emitting SARIF.
- **Filing:** Prefer PR-to-fork; `gh issue create` as fallback.
- **Trade-offs:** Audit-ready; strongest fit if SLO users pursue FedRAMP/SOC 2 evidence collection downstream. Heavier per-library authoring cost; no Rust OSCAL crate surfaced (pure JSON authoring, or Python subprocess via `compliance-trestle` if proven suitable). Capability claim is duplicated across two formats.
- **Best fit per findings:** SLO users in regulated or government-adjacent SaaS.

### Option C — "Rust-pure, schema-emitting core"

- **Capability manifest:** A small Rust crate `sldo-capmanifest` defines an internal `CapabilityClaim` struct and emits either CycloneDX 1.6 declarations or OSCAL component-definition JSON from the same source. Primitive-level parameters are first-class fields in the internal struct; lossy when serialized into either wire format, by design.
- **Threat model:** Rust service calls Claude directly with a deterministic prompt, validates against an internal schema (superset of OTM 0.2.0), rejects malformed output. No SecOpsTM / pytm dependency.
- **Variant analysis:** **ast-grep 0.42** as primary (Rust-native, fast, MIT, SARIF-capable); Semgrep optional.
- **Filing:** SLO-owned intake repo is the default; `gh issue create` against third-party repos requires explicit user confirmation.
- **Trade-offs:** Single-language stack, no Python, zero upstream rate-limit pressure by default, primitive-level claims first-class. Reinvents parts of SecOpsTM's rule engine; LLM-only threat-model path inherits the categorization-instability risk documented for STRIDE-GPT and similar tools unless the prompt + validator pairing is tested. Smaller rule ecosystem than Semgrep.
- **Best fit per findings:** If Rust purity of the SLO toolchain outweighs reuse of OWASP engines.

### Orthogonal decision: default framework columns

Independent of A/B/C and applicable to all: render **SOC 2 + OWASP ASVS 5.0** by default in the threat-model compliance column. Make **GDPR, HIPAA, PCI DSS, NIST 800-53 moderate, ISO 27001** opt-in via a `compliance:` front-matter list in the runbook. GDPR should additionally be a *section*, not only a column, because it encodes principles and data-processing obligations rather than a discrete control catalog.

## API & SDK Documentation

The raw findings cite the following primary API/SDK surfaces rather than fetching their contents directly. Treat this as a pointer list for later lookup rather than an inlined reference.

- **CycloneDX 1.6 / 1.7 JSON schema and reference docs** — the `declarations`, `definitions`, CBOM, and VEX/VDR surfaces. Schemas and examples published at `cyclonedx.org/specification/overview/` and `cyclonedx.org/docs/1.6/json/`. ECMA-424 2nd edition (December 2025) is the normative reference.
- **OSCAL Component Definition v1.1.3** — JSON / XML / YAML models and reference examples at `pages.nist.gov/OSCAL-Reference/models/v1.1.3/component-definition/json-reference/`. Control-mapping model documented at `pages.nist.gov/OSCAL/learn/concepts/layer/control/mapping/`.
- **Semgrep CLI reference** — flags (`--sarif`, `--sarif-output`), output formats (text, json, sarif, gitlab-sast, gitlab-secrets, junit-xml, emacs, vim), release notes. Primary docs at `semgrep.dev/docs/cli-reference`, `semgrep.dev/docs/getting-started/cli`, `semgrep.dev/docs/release-notes`, and the Semgrep↔CodeQL comparison at `semgrep.dev/docs/faq/comparisons/codeql`.
- **ast-grep** — homepage and tool-comparison page; depends on `serde-sarif ^0.8.0` for SARIF output (added in 0.40.0). Rust crate on crates.io / docs.rs; Python binding on PyPI (`ast-grep-cli`).
- **GitHub REST API** — rate limits for REST, OAuth apps, and GitHub Apps; OAuth scopes; issue-form syntax (`validations: required: true`). Primary docs on `docs.github.com`, supplemented by community discussions (#32120, #43859, #45084) for enforcement behaviour.
- **GitHub CLI (`gh`)** — scope handling and cross-repo friction documented in open issues `cli/cli#10500` (restricted-scope tokens) and `cli/cli#9380` (cross-repo PR scope gap).
- **OWASP SecOpsTM** — GitHub repo (`ellipse2v/SecOpsTM`) and OWASP project page; v1.0.0 / v1.1.0 tags published Aug 2025; Python runtime with pytm + LLM + RAG engines.
- **OTM 0.2.0** — schema in `iriusrisk/OpenThreatModel/otm_schema.json`.
- **MITRE ATLAS** — knowledge base at `atlas.mitre.org/`; agentic-AI technique catalog.

No deeper third-party API/SDK reference was pulled into the findings for this section; the brief explicitly excluded "deep API reference for specific libraries" from scope.

## Design Recommendations

Each recommendation is tagged by how strongly the raw findings support it.

1. **Adopt OTM 0.2.0 as the `/slo-threat-model` output schema, with a strict validator gate** `(confidence: high)`. The only platform-independent threat-model interchange with a stable published schema; Apache 2.0; multiple tools already import/export it. STRIDE-GPT and SecOpsTM both show LLM categorization instability, which the gate is designed to catch.
2. **Use CycloneDX 1.7 declarations as the primary capability-manifest format for Hulumi / SunLitSecureLibraries** `(confidence: high)`. Purpose-built for catalog-level compliance claims; active ecosystem; ECMA-424 standardized; already composes with VEX/VDR for capability-gap tracking. OSCAL Component Definition is a strictly stronger choice *only* when FedRAMP/SOC 2 evidence collection is a live requirement.
3. **Pair Semgrep CE + ast-grep 0.42 for interactive variant analysis; keep CodeQL nightly-only** `(confidence: high)`. Directly supported by 2026 head-to-heads and by ast-grep's November 2025 native SARIF landing. CodeQL's 400+ OSS CVE record is strong evidence it stays in the pipeline, just not on the interactive path.
4. **Render SOC 2 + OWASP ASVS 5.0 as the default compliance-framework columns; make GDPR/HIPAA/PCI DSS/NIST 800-53/ISO 27001 opt-in via runbook front-matter** `(confidence: medium)`. Best effort given the complete absence of OSS-tool-user per-framework adoption data. Grounded in the Vanta/Drata commercial consensus and the 70–80% overlap between SOC 2 and ISO 27001.
5. **Treat GDPR as a section *and* a column in the threat-model file, not a column alone** `(confidence: medium)`. Supported by the observation that GDPR encodes principles and data-processing obligations rather than discrete control IDs, but the need for a separate section is a design inference from the findings rather than a directly cited design pattern.
6. **Adopt Option A ("CycloneDX-native with Rust escape hatch") as the default architecture** `(confidence: medium)`. Supported by SLO's stated Rust-first, solo-operator context and by the documented Rust tooling gap (no 1.6+ `cyclonedx-bom`), which means any option must either hand-roll JSON or shell out. A is the least ambitious of the three and preserves an upgrade path to B or C.
7. **Use an SLO-owned intake repo as the primary capability-gap filing channel; `gh issue create` against third-party repos only with explicit user confirmation and a per-session client-side cap (≤40 issues/hr is a defensible half-budget)** `(confidence: medium)`. Supported by the 80/min + 500/hr secondary rate limit, the undocumented point cost, attribution ambiguity, and template-validation unreliability in org repos. The specific cap is a defensive inference.
8. **Depend on SecOpsTM v1.1.0 (via Python subprocess) as the threat-model engine in Option A** `(confidence: medium)`. Tagged release; actively maintained; emits the mapping columns the brief asks for. Risk: cross-platform maturity (macOS / Windows) and ChromaDB + Ollama install footprint not characterized in the findings — see Open Questions.
9. **Cite MITRE ATLAS + OWASP LLM Top 10 + NIST AI RMF as the standard triad whenever the runbook target involves an AI/LLM component** `(confidence: high)`. Repeatedly recommended across 2025–2026 vendor and independent sources (Giskard, Straiker, AWS, Practical DevSecOps, Vectra).
10. **Use Opengrep as a licensing hedge, not a default** `(confidence: low)`. Forked Jan 2025 after Semgrep's Dec 2024 relicense; backed by ~10 vendors; rule ecosystem still tracks Semgrep CE. Findings support it as a fallback if Semgrep commercial gating becomes a real blocker.
11. **Do not depend on pytm directly; wrap it only via SecOpsTM** `(confidence: high)`. pytm v1.3.1 has had no release in ~24 months; SecOpsTM supersedes the output stack and adds the NIST 800-53 mappings pytm lacks.
12. **Treat the OSCAL ↔ CycloneDX mapping as lossy by default; do not promise round-trip fidelity** `(confidence: medium)`. NIST CSWP 53 ipd asserts alignment; no published lossless mapping exists. An SLO-authored crosswalk will likely be lossy at the `claims.evidence` ↔ `by-component.statements.remarks` boundary.

## Risks & Open Questions

Ordered by impact on the `/slo-security-embedding` design.

1. **No Rust-native emitter exists for CycloneDX 1.6+ `declarations`.** `cyclonedx-bom` 0.8.1 still targets spec 1.5; no crate in the findings speaks 1.6 declarations. Every Option A path depends on either hand-rolling JSON against the schema, upstreaming 1.6 support, or shelling out to Node/Python. Directly blocks the claim of "drop-in" Rust fit.
2. **CycloneDX declarations do not schema-enforce primitive-level parametric claims.** Claims like "Argon2id with ≥3 iterations, ≥64 MiB memory" must ride in `properties`, a `cryptographic-asset` component, or `externalReferences`. **Open:** does the public CycloneDX Property Taxonomy already define crypto-primitive namespaces (e.g., `cdx:crypto:argon2id:iterations`)? If yes, SLO's "vendored namespace" becomes a Taxonomy contribution; if no, SLO owns the vocabulary.
3. **Per-framework compliance adoption data among OSS-tool users is structurally absent.** No survey found (Tidelift 2024, Snyk 2024, Stack Overflow, JetBrains) reports framework-level adoption for OSS maintainers. The recommended default column set rests on commercial-vendor catalog inference (Vanta/Drata), not direct evidence. A targeted search into 2025 OpenSSF / Linux Foundation / Tidelift maintainer reports *might* close this; prior rounds did not.
4. **SecOpsTM cross-platform maturity is not characterized.** v1.1.0 is a real tagged release, but macOS and Windows support, ChromaDB install footprint, and Ollama fallback behaviour were not surfaced in any search round. Blocks making SecOpsTM a user-machine dependency with high confidence.
5. **OSCAL ↔ CycloneDX crosswalk is asserted (NIST CSWP 53 ipd) but not published as a lossless table.** NIST's documented mapping model concerns OSCAL internal catalog crosswalks. An SLO that wants round-trip between the two must author the crosswalk and assume it will be lossy.
6. **GitHub content-creation per-endpoint point cost is deliberately undocumented.** An SLO agent session cannot predict exactly how many capability-gap issues it can file before throttling. The only way to know is empirical probing against a disposable repo. A ≤40/hr client-side cap is a defensive workaround, not a guarantee.
7. **Issue-template `required` validation is unreliable in org-owned repos** (community discussion #43859; #45084 for checkbox/dropdown inconsistency). An agent-authored filing path must produce a fully valid body itself rather than trusting the template as a guardrail. Not confirmed whether `kerberosmansour/hulumi` specifically honors `validations: required: true`.
8. **SecOpsTM and related LLM threat-modellers show unstable STRIDE categorization across runs** (Pure Storage 2025, FuzzingLabs 2025, Laponina 2025, ThreatCompute CCSW 2025). Any `/slo-threat-model` built on LLM emission needs a deterministic validation gate, and Option C's LLM-only path inherits this risk most directly.
9. **Is `compliance-trestle`** (IBM's OSCAL Python toolkit) mature enough to be the Option B subprocess backend? Not evaluated in any research round. Blocks Option B from being a concrete recommendation.
10. **MSRV for `ast-grep` not surfaced.** The crate depends on `serde-sarif ^0.8.0`; verify the minimum Rust toolchain before committing if SLO's MSRV policy tightens.
11. **STRIDE-GPT, SecOpsTM, Threat Dragon, pytm, ASVS, OpenThreatModel — exact tagged-version pins are incomplete across rounds.** SecOpsTM (v1.1.0) and OWASP ASVS (5.0.0) and pytm (1.3.1) are pinned; Threat Dragon, STRIDE-GPT, Semgrep CLI, CodeQL CLI, and gh CLI have no pinned versions surfaced. Needed before shipping any of them as named dependencies.
12. **CycloneDX Property Taxonomy upstream contribution process** is not documented in the gathered material. Low priority post-MVP.
13. **Rust-native OSCAL writer crate** — whether a mature one exists (vs. hand-rolling) was not evaluated. Affects Option B authoring cost.
14. **Node (`@cyclonedx/cyclonedx-npm`) or Python (`cyclonedx-python-lib`) 1.6 declarations support** — a one-shot check there might unblock an Option-A shortcut (call out to Node) vs. hand-rolling JSON.
15. **agentskills.io `SKILL.md` frontmatter and MCP server capability descriptors** as capability-manifest formats were listed in the brief but did not surface in any search round. Unknown whether they express security controls in any structured way.
16. **SPDX 3.0 security profiles** adoption and tooling support — noted as closing the gap with CycloneDX but still lagging at 2.3 in practice. Not evaluated in depth.

## References

- [A Deeper Look at Modern SAST Tools — Going Beyond Grep](https://goingbeyondgrep.com/posts/a-deeper-look-at-modern-sast-tools/)
- [AI Code Review — Semgrep vs CodeQL: Patterns vs Semantic Analysis](https://aicodereview.cc/blog/semgrep-vs-codeql/)
- [AI Threat Modeling (Security Compass)](https://www.securitycompass.com/blog/ai-threat-modeling/)
- [AI Threat Modeling in Practice: STRIDE + MITRE ATLAS workshop](https://aiq.hu/en/ai-threat-modeling-in-practice-a-stride-and-mitre-atlas-workshop-guide/)
- [AppSec Santa — OpenGrep vs Semgrep (2026)](https://appsecsanta.com/sast-tools/opengrep-vs-semgrep)
- [AppSec Santa — Semgrep Review 2026 (CE vs AppSec Platform)](https://appsecsanta.com/semgrep)
- [AppSec Santa — Semgrep vs CodeQL (2026)](https://appsecsanta.com/sast-tools/semgrep-vs-codeql)
- [ast-grep — Comparison With Other Frameworks](https://ast-grep.github.io/advanced/tool-comparison.html)
- [ast-grep — DeepWiki](https://deepwiki.com/ast-grep/ast-grep)
- [ast-grep — GitHub](https://github.com/ast-grep/ast-grep)
- [ast-grep — Releases](https://github.com/ast-grep/ast-grep/releases)
- [ast-grep — CHANGELOG](https://github.com/ast-grep/ast-grep/blob/main/CHANGELOG.md)
- [ast-grep — Homepage](https://ast-grep.github.io/)
- [ast-grep on docs.rs](https://docs.rs/crate/ast-grep/latest)
- [ast-grep-cli on PyPI](https://pypi.org/project/ast-grep-cli/)
- [arXiv 2411.17058 — ThreatModeling-LLM (banking)](https://arxiv.org/html/2411.17058v2)
- [arXiv 2504.19956 — Securing Agentic AI (threat model + SHIELD)](https://arxiv.org/html/2504.19956v2)
- [cdxgen / CycloneDX .NET in GitHub Secure Open Source Fund (Aug 2025)](https://owasp.org/blog/2025/08/11/cyclonedx-projects-in-github-sosf)
- [CivicActions/oscal-component-definitions](https://github.com/CivicActions/oscal-component-definitions)
- [cli/cli #10500 — Issuing OAuth tokens with more restricted scopes](https://github.com/cli/cli/issues/10500)
- [cli/cli #9380 — Missing auth scope on cross-repo PR](https://github.com/cli/cli/issues/9380)
- [code-safety — Pysa vs CodeQL vs Semgrep taint benchmark](https://github.com/laugiov/code-safety)
- [Code Security with Semgrep (SmartTECS)](https://blog.smarttecs.com/posts/2024-006-semgrep/)
- [Comparing Semgrep and CodeQL — Doyensec](https://blog.doyensec.com/2022/10/06/semgrep-codeql.html)
- [CycloneDX — Bill of Materials Standard](https://cyclonedx.org/)
- [CycloneDX — Specification Overview](https://cyclonedx.org/specification/overview/)
- [CycloneDX — Tool Center](https://cyclonedx.org/tool-center/)
- [CycloneDX — VEX Capability](https://cyclonedx.org/capabilities/vex/)
- [CycloneDX — OWASP Developer Guide](https://devguide.owasp.org/en/05-implementation/02-dependencies/03-cyclonedx/)
- [CycloneDX — v1.6 release announcement (OWASP)](https://owasp.org/blog/2024/04/09/CycloneDX-v1.6-Released)
- [CycloneDX — v1.6 release announcement (CycloneDX news)](https://cyclonedx.org/news/cyclonedx-v1.6-released/)
- [CycloneDX — v1.7 release notes (SBOM Observer, 2026-03-25)](https://docs.sbom.observer/release-notes/2026-03-25-cyclonedx-1.7)
- [CycloneDX — Cryptography (CBOM) post](https://owasp.org/blog/2023/10/03/CycloneDX-Cryptography-CBOM.html)
- [CycloneDX — Specification repository](https://github.com/CycloneDX/specification)
- [CycloneDX — Specification releases](https://github.com/CycloneDX/specification/releases)
- [CycloneDX — 1.6 JSON Reference](https://cyclonedx.org/docs/1.6/json/)
- [cyclonedx-rust-cargo — Releases](https://github.com/CycloneDX/cyclonedx-rust-cargo/releases)
- [cyclonedx-rust-cargo — cyclonedx-bom CHANGELOG](https://github.com/CycloneDX/cyclonedx-rust-cargo/blob/main/cyclonedx-bom/CHANGELOG.md)
- [DeepTeam — MITRE ATLAS framework](https://www.trydeepteam.com/docs/frameworks-mitre-atlas)
- [DEV.to — Semgrep vs CodeQL lightweight patterns (2026)](https://dev.to/rahulxsingh/semgrep-vs-codeql-lightweight-patterns-vs-semantic-analysis-for-sast-2026-412k)
- [DEV.to — Unlocking the Power of SARIF](https://dev.to/shivasurya/unlocking-the-power-of-sarif-the-backbone-of-modern-static-analysis-9lc)
- [DEV.to — How to Set Up Semgrep in 2026](https://dev.to/rahulxsingh/how-to-set-up-semgrep-in-2026-complete-installation-and-configuration-guide-5emm)
- [DevSecOps Pipelines: Semgrep Python SAST Scans 2026](https://www.johal.in/devsecops-pipelines-semgrep-python-sast-scans-2026/)
- [ECMA-424 2nd edition, December 2025 (CycloneDX)](https://ecma-international.org/wp-content/uploads/ECMA-424_2nd_edition_december_2025.pdf)
- [ellipse2v/SecOpsTM — Releases](https://github.com/ellipse2v/SecOpsTM/releases)
- [ellipse2v/SecOpsTM — Repository](https://github.com/ellipse2v/SecOpsTM)
- [Empirical Study of Security-Policy Issues in OSS (Springer 2025)](https://link.springer.com/chapter/10.1007/978-3-032-12089-2_43)
- [episki — Compliance Framework Comparison](https://episki.com/now/compliance-framework-comparison)
- [Extending STRIDE and MITRE ATLAS for AI-Specific Threat Landscapes (ResearchGate)](https://www.researchgate.net/publication/396511419_Extending_STRIDE_and_MITRE_ATLAS_for_AI-Specific_Threat_Landscapes)
- [FedTech — What Is OSCAL? (Feb 2025)](https://fedtechmagazine.com/article/2025/02/what-is-oscal-perfcon)
- [FOSSA — The Complete Guide to CycloneDX](https://fossa.com/learn/cyclonedx/)
- [FuzzingLabs — AI-Driven Threat Modeling](https://fuzzinglabs.com/ai-threat-modeling-arrows/)
- [GitHub community discussion #32120 — Secondary rate limit](https://github.com/orgs/community/discussions/32120)
- [GitHub community discussion #43859 — required validation in org repos](https://github.com/orgs/community/discussions/43859)
- [GitHub Docs — Rate limits for OAuth apps](https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/rate-limits-for-oauth-apps)
- [GitHub Docs — Rate limits for the REST API](https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api)
- [GitHub Docs — Rate limits for GitHub Apps](https://docs.github.com/en/apps/creating-github-apps/registering-a-github-app/rate-limits-for-github-apps)
- [GitHub Docs — Scopes for OAuth apps](https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/scopes-for-oauth-apps)
- [GitHub Docs — Syntax for issue forms](https://docs.github.com/en/communities/using-templates-to-encourage-useful-issues-and-pull-requests/syntax-for-issue-forms)
- [Giskard — Risk assessment for LLMs and AI agents (OWASP, MITRE ATLAS, NIST AI RMF)](https://www.giskard.ai/knowledge/risk-assessment-for-llms-and-ai-agents-owasp-mitre-atlas-and-nist-ai-rmf-explained)
- [Harness — Ingest SARIF scan results](https://developer.harness.io/docs/security-testing-orchestration/custom-scanning/ingest-sarif-data/)
- [HeroDevs — SPDX vs CycloneDX](https://www.herodevs.com/blog-posts/spdx-vs-cyclonedx-choosing-the-right-sbom-format-for-your-software-supply-chain)
- [Interlynk — All about CycloneDX 1.6](https://medium.com/@interlynkblog/all-about-cyclonedx-1-6-6af35df675de)
- [IriusRisk — 11 Recommended Threat Modeling Tools](https://www.iriusrisk.com/resources-blog/recommended-threat-modeling-tools)
- [iriusrisk/OpenThreatModel — Repository](https://github.com/iriusrisk/OpenThreatModel)
- [iriusrisk/OpenThreatModel — otm_schema.json](https://github.com/iriusrisk/OpenThreatModel/blob/main/otm_schema.json)
- [KICS results documentation](https://docs.kics.io/latest/results/)
- [Konvu — Semgrep vs CodeQL (2026)](https://konvu.com/compare/semgrep-vs-codeql)
- [lambdasawa/sarifw — SARIF wrapper for ripgrep / ast-grep](https://github.com/lambdasawa/sarifw)
- [mikeroyal/Open-Source-Security-Guide](https://github.com/mikeroyal/Open-Source-Security-Guide)
- [MITRE ATLAS — Home](https://atlas.mitre.org/)
- [MITRE ATLAS Framework 2026 Guide (Practical DevSecOps)](https://www.practical-devsecops.com/mitre-atlas-framework-guide-securing-ai-systems/)
- [MITRE ATLAS — Vectra overview](https://www.vectra.ai/topics/mitre-atlas)
- [NIST CSWP 53 ipd — Charting the Course for NIST OSCAL](https://nvlpubs.nist.gov/nistpubs/CSWP/NIST.CSWP.53.ipd.pdf)
- [Nightfall — MITRE ATLAS Essential Guide](https://www.nightfall.ai/ai-security-101/mitre-atlas)
- [NoComplexity — Threat Models / Security Reference Architecture](https://nocomplexity.com/documents/securityarchitecture/architecture/threadmodels.html)
- [Nox — SARIF + CycloneDX + AI inventory scanner](https://github.com/Nox-HQ/nox)
- [octokit/plugin-throttling.js #108 — Abuse detection on GitHub APIs](https://github.com/octokit/plugin-throttling.js/issues/108)
- [OpenText Core Application Security 26.1 release notes](https://community.opentext.com/cybersec/fortify/w/tips/53185/opentext-core-application-security-fortify-on-demand-26-1-release-notes)
- [OSCAL — Open Security Controls Assessment Language (NIST)](https://pages.nist.gov/OSCAL/)
- [OSCAL — Control Mapping Model (NIST)](https://pages.nist.gov/OSCAL/learn/concepts/layer/control/mapping/)
- [OSCAL Component Definition v1.1.3 JSON reference](https://pages.nist.gov/OSCAL-Reference/models/v1.1.3/component-definition/json-reference/)
- [OWASP ASVS 5.0.0 PDF](https://raw.githubusercontent.com/OWASP/ASVS/v5.0.0/5.0/OWASP_Application_Security_Verification_Standard_5.0.0_en.pdf)
- [OWASP ASVS 5.0.0 — asvs.dev preface](https://asvs.dev/v5.0.0/Preface/)
- [OWASP CycloneDX Project (ECMA-424)](https://owasp.org/www-project-cyclonedx/)
- [OWASP pytm — Releases](https://github.com/OWASP/pytm/releases)
- [OWASP pytm — Developer Guide](https://devguide.owasp.org/en/04-design/01-threat-modeling/02-pytm/)
- [OWASP SecOpsTM Project](https://owasp.org/www-project-secopstm/)
- [OWASP Threat Dragon](https://owasp.org/www-project-threat-dragon/)
- [OWASP Threat Model Library](https://owasp.org/www-project-threat-model-library/)
- [OWASP Threat Modeling Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Threat_Modeling_Cheat_Sheet.html)
- [Practical DevSecOps — MITRE ATLAS Framework 2026](https://www.practical-devsecops.com/mitre-atlas-framework-guide-securing-ai-systems/)
- [Pure Storage — Leveraging LLMs for STRIDE Threat Modeling](https://blog.purestorage.com/purely-technical/leveraging-large-language-models-for-stride-threat-modeling/)
- [ReARM release — SARIF, CycloneDX VDR/BOV (2025-09-01)](https://rearmhq.com/blog/2025-09-01-rearm-sarif-cyclonedx-vdr-bov-artifact-versioning/)
- [Sanj.dev — AI Code Security Tools Comparison 2025](https://sanj.dev/post/ai-code-security-tools-comparison)
- [SARIF glossary — Aptori](https://www.aptori.com/glossary/static-analysis-results-interchange-format-sarif)
- [Sbomify — CycloneDX vs SPDX (Jan 2026)](https://sbomify.com/2026/01/15/sbom-formats-cyclonedx-vs-spdx/)
- [Sbomify — SBOM schema crosswalk](https://sbomify.com/compliance/schema-crosswalk/)
- [SD Times — The State of Open Source Maintainers (Tidelift)](https://sdtimes.com/open-source/the-state-of-open-source-maintainers/)
- [Semgrep — CLI reference](https://semgrep.dev/docs/cli-reference)
- [Semgrep — Community Edition](https://semgrep.dev/products/community-edition/)
- [Semgrep — Compare to CodeQL](https://semgrep.dev/docs/faq/comparisons/codeql)
- [Semgrep — GitHub](https://github.com/semgrep/semgrep)
- [Semgrep — Local CLI scans](https://semgrep.dev/docs/getting-started/cli)
- [Semgrep — Release notes](https://semgrep.dev/docs/release-notes)
- [Snyk — 2024 Open Source Security Report blog](https://snyk.io/blog/2024-open-source-security-report-slowing-progress-and-new-challenges-for/)
- [Softwaremill — What's new in ASVS 5.0](https://softwaremill.com/whats-new-in-asvs-5-0/)
- [Sprinto — Top-10 Compliance Standards](https://sprinto.com/blog/compliance-standards/)
- [Straiker — Comparing AI Security Frameworks](https://www.straiker.ai/blog/comparing-ai-security-frameworks-owasp-csa-nist-and-mitre)
- [STRIDE-GPT (mrwadams) — Repository](https://github.com/mrwadams/stride-gpt)
- [System Weakness — Threat Modeling for AI Systems: STRIDE, DREAD, and Beyond](https://systemweakness.com/threat-modeling-for-ai-systems-stride-dread-and-beyond-8a3996a0ab43)
- [threat-dragon-ai-tool (InfosecOTB)](https://github.com/InfosecOTB/threat-dragon-ai-tool)
- [threat-dragon-llm (otisthescribe)](https://github.com/otisthescribe/threat-dragon-llm)
- [Threat Modeling Dev — A Dragon and Python walk into an OWASP card game](https://threatmodeling.dev/dragpyt/)
- [Threat Modeling Software Development for LLM-Agent-Based Systems (Laponina, IJOIT)](https://www.injoit.ru/index.php/j1/article/view/2178/0)
- [ThreatBandit — 2025 Threat Modeling Trends](https://www.threatbandit.com/about-threat-modeling.html)
- [ThreatCompute — ACM CCSW 2025](https://dl.acm.org/doi/10.1145/3733812.3765533)
- [Trail of Bits — Introducing mrva (2025-12-11)](https://blog.trailofbits.com/2025/12/11/introducing-mrva-a-terminal-first-approach-to-codeql-multi-repo-variant-analysis/)
- [TuxCare — 2025 Open-Source Security Best Practices](https://tuxcare.com/blog/open-source-security/)
- [usnistgov/OSCAL on GitHub](https://github.com/usnistgov/OSCAL)
- [Virtual Cyber Labs — Threat Modeling for LLM-Powered Chatbots](https://virtualcyberlabs.com/threat-modeling-for-llm-powered-chatbots/)
- [Vanta vs Drata 2026 Comparison — ztekcyber](https://www.ztekcyber.com/resources/vanta-vs-drata-2026-comparison)
- [Drata blog — Secureframe vs Vanta vs Drata](https://drata.com/blog/secureframe-vs-vanta-vs-drata)
- [AWS Security Blog — Threat modeling your generative AI workload](https://aws.amazon.com/blogs/security/threat-modeling-your-generative-ai-workload-to-evaluate-security-risk/)
- [USENIX Security 2025 — Investigating Threat Modeling Practices in Open-Source (prepub)](https://www.usenix.org/system/files/conference/usenixsecurity25/sec25cycle1-prepub-294-kaur.pdf)
- [VulnCheck — Vulnerability Exchange Formats (CycloneDX, SPDX, VDR, VEX)](https://www.vulncheck.com/blog/vulnerability-exchange-formats)
- [William Ogou — Mapping MITRE ATLAS to OWASP LLM Top 10](https://blog.ogwilliam.com/post/mapping-mitre-atlas-mitigations-owasp-top-10-llms)
- [Wiz — Standard SBOM Formats Guide](https://www.wiz.io/academy/application-security/standard-sbom-formats)
