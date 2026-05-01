---
topic: # Research brief — sast-rulegen-skill-pack  ## Wedge (one sentence)  `/slo-rulegen` v1 — a Claude Code skill that produces a Semgrep rule pack for Rust covering the top-10 CWE classes that idiomatic s…
generated_on: 2026-04-25 01:43:10 +0100
source_prompt_bytes: 7850
generator: sldo-research
---

# Research Dossier

This dossier is a structured research artifact produced by `sldo-research`. It is intended as the `prompt_file` input to `sldo-plan`.

## Repository Context

## Tech Stack

- **Primary language:** Rust 2021 edition (workspace), `resolver = "2"`. No MSRV declared in `Cargo.toml`.
- **Workspace crates:** `sldo-common` (lib), `sldo-plan`, `sldo-run`, `sldo-research`, `sldo-install`, `sldo-tla-sha` (active); `sldo-tauri` (parked per `CLAUDE.md`, 2026-04).
- **Key Rust deps (workspace-pinned):** `clap 4` (derive), `anyhow 1`, `thiserror 2`, `colored 2`, `regex 1`, `chrono 0.4`, `which 7`. Per-crate: `serde 1` + `toml 0.8` (install/tla-sha), `reqwest 0.11/0.12` w/ `rustls-tls`, `sha2 0.10`, `url 2`, `tempfile 3` (dev), `dotenvy 0.15`, `base64 0.22`.
- **Desktop (parked):** Tauri v2 backend with `tokio (full)`, `serde_json`, `rig-core 0.33` (audio); React 18 + TypeScript 5.6 + Vite 6 + Vitest 4 frontend in `crates/sldo-tauri/ui/` (Node v18+ per README).
- **External CLIs invoked:** `claude` (Claude Code CLI) via `sldo-common::copilot::ClaudeInvocation`; `git`; optionally `gh` and `chub` (for `/get-api-docs`).
- **Skill pack:** Markdown skills under `skills/<name>/SKILL.md` with YAML frontmatter (`name`, `description`); no code, installed by `sldo-install` as symlinks.

## Project Structure

- `crates/sldo-common/` — shared library: modules `color`, `copilot`, `detect`, `git`, `logging`, `preflight`, `runbook`, `toolflags`.
- `crates/sldo-plan/` — binary, generates v3 runbooks via Claude CLI.
- `crates/sldo-run/` — binary, drives Claude through milestones with build/test verification.
- `crates/sldo-research/` — binary, multi-phase research dossier pipeline (`main.rs`, `dossier.rs`, `prompt.rs`, `research.rs`).
- `crates/sldo-install/` — binary + lib, symlinks `skills/*` into `~/.claude/skills/`; manifest at `~/.sldo/install.toml`.
- `crates/sldo-tla-sha/` — TLA+ tooling helper (binary + lib).
- `crates/sldo-tauri/` — Tauri v2 desktop app (parked). Subdirs: `src/`, `ui/`, `capabilities/`, `gen/`, `icons/`, `Info.plist`, `tauri.conf.json`.
- `skills/` — `slo-{ideate,research,architect,tla,plan,critique,execute,verify,retro,ship,resume,freeze,second-opinion}` plus vendored `get-api-docs/`.
- `src/` — legacy Bash: `plan-milestones.sh`, `run-milestones.sh` (superseded by Rust CLI).
- `docs/` — `ARCHITECTURE.md`, `MIGRATION.md`, `runbook-template_v_3_template.md` (canonical), per-feature runbooks `RUNBOOK-*.md`, plus `idea/`, `research/`, `design/`, `critique/`, `lessons/`, `completion/`, `legal/`.
- `tests/` — workspace-level integration tests `e2e_*.rs` (scaffold/common/plan/run/integration/tauri/voice/research milestones) declared in root `Cargo.toml`.
- `output/` — generated research dossiers (gitignored).
- `.sldo-logs/`, `.copilot-logs/`, `.claude/`, `.sldo/`, `target/` — gitignored runtime/build artefacts.

## Build & Test

- **Skill-pack baseline (per `CLAUDE.md`, authoritative):**
  ```bash
  cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install
  ```
  `--workspace` is **not** the baseline because parked `sldo-tauri` is red on macOS arm64 (missing esbuild arm64 binary in its `node_modules/`).
- **Makefile targets:** `make dev` (Tauri dev), `make build`, `make test` (= `test-backend` + `test-frontend`), `make test-backend` (`cargo test --workspace`), `make test-frontend` (`cd crates/sldo-tauri/ui && npm test`), `make check` (`cargo build --workspace`), `make setup`, `make clean`. README/Makefile reference `cargo test --workspace`, but `CLAUDE.md` overrides this for skill-pack work.
- **Install skills:** `cargo build -p sldo-install --release && ./target/release/sldo-install [--dry-run|uninstall|status|verify]`.
- **Frontend (parked):** `cd crates/sldo-tauri/ui && npm install && npm test` (Vitest); build via `npm run build` then `cargo tauri build`.
- **No dedicated lint target** in Makefile/Cargo (no `cargo clippy`/`cargo fmt` wrappers found).

## Existing Patterns

- **Error handling:** `anyhow::Result` with `Context`/`bail!` at binary boundaries; `thiserror 2` available workspace-wide for typed errors (declared as workspace dep but minimally used in inspected files).
- **CLI parsing:** `clap 4` derive macros (`#[derive(Parser)]`) on per-binary `Cli` structs (see `sldo-research/src/main.rs`).
- **Logging/UX:** custom helpers in `sldo_common::color` (`info`, `success`, `warn`, `fail`, `header`, `divider`, `ts`) printing to **stderr** with `colored`+`chrono` timestamps; file logs via `sldo_common::logging::LogFile` writing timestamped lines into `.sldo-logs/`.
- **Subprocess execution:** synchronous `std::process::Command` with `Stdio::piped()`; stdout/stderr drained on **separate threads** to avoid pipe-buffer deadlocks (`copilot.rs`).
- **Async:** no Tokio in active crates (sync `std::process` + `std::thread`); `tokio (full)` only inside parked `sldo-tauri`.
- **Tool-permission allowlists:** centralised in `sldo_common::toolflags` (`plan_*`, `run_*`, `research_*` allow/deny vectors injected as `--allowedTools=…`).
- **Pre-flight pattern:** every binary calls `sldo_common::preflight` to verify `claude` on PATH, file existence, and **refuse to run on `main`/`master`** branches.
- **Runbook parsing:** regex-driven Milestone Tracker parser in `sldo_common::runbook` (`MilestoneStatus { NotStarted, InProgress, Done }`).
- **Test style:** unit tests `#[cfg(test)] mod tests` co-located in modules; integration/E2E tests at workspace root `tests/e2e_*_m<N>.rs`, one per milestone of each feature runbook.
- **Comment style:** `//!` crate/module-level doc comments; modules cross-reference Bash predecessors (e.g. `detect.rs` → `run-milestones.sh`).

## Constraints

- **Branch safety:** active CLIs refuse to operate on `main`/`master` (`preflight::check_git_safety`). Current branch in this snapshot: `feature/biz-skill-pack`; PR target main branch is `skill-pack`.
- **Canonical planning artifact:** every feature runbook lives at `docs/RUNBOOK-<FEATURE>.md` and must conform to `docs/slo/templates/runbook-template_v_3_template.md`; `/slo-plan` is the authoring path — do not bypass it for batch CLI shortcuts.
- **Parked `sldo-tauri`:** do **not** modify it; do **not** merge its branch into skill-pack work; full `--workspace` baseline stays red on macOS arm64 until Tauri is unparked. Frontend assumes Node.js v18+.
- **External-CLI dependency:** `claude` CLI must be on `PATH`; some skills also require `gh` and `chub` (for `/get-api-docs`).
- **Untrusted-prompt security:** `sldo-research` warns that Claude with `WebFetch`/`WebSearch` can ingest hostile content; prompt files must be trusted. Logs/scratch files may contain proprietary excerpts.
- **Cost control:** research defaults capped at `--max-iterations 3`, `--max-searches 5`; high-quota runs can be expensive.
- **Secrets:** `OPENAI_API_KEY` via `.env` is for local dev only; README explicitly warns against shipping a shared key in distributed binaries.
- **MSRV:** none declared in any `Cargo.toml`; edition `2021`.
- **License:** **no `LICENSE` file present in the repo root** — license obligations cannot be confirmed from the source tree.
- **Platform note:** primary development target is macOS arm64 (Darwin 25.4.0 in current env); Tauri build targets per `tauri.conf.json` not inspected in detail.

## Executive Summary

`/slo-rulegen` v1 targets a real and currently unfilled gap: 2026 commercial AI-SAST tools (Semgrep Assistant, Snyk DeepCode AI, GitHub Copilot Autofix, SonarQube AI CodeFix, Veracode Fix) do **AI-triage or AI-autofix**, not per-bug rule-synthesis with corpus-driven verification, and none integrate as a Claude Code skill. Semgrep Assistant is the closest precedent — it does run an internal fire-on-bad/silent-on-good gate — but its input contract is `(description, 1 bad, 1 good) → 1 rule` authored in its SaaS UI, not `(bug_summary, fix_diff, file_paths) → N variation rules` callable in-session.

Semgrep's Rust frontend is GA in 2026 with working `pattern-either`, `pattern-inside`, taint mode (used in Semgrep's own Supply Chain reachability for Rust), and macro-arg taint propagation. Known gaps: proc-macro expansion (axum/tokio/`tracing::instrument` handlers), taint through `format!` and intermediate `let`, and undocumented `metavariable-type` behaviour on Rust generics. The Trail of Bits `panic-in-function-returning-result.yaml` rule is the gold-standard precedent: it enumerates 4 fn-context variations inside a single `pattern-either` and tags as **CWE-755**, not CWE-248 — establishing both the rule shape and the canonical CWE for panic-DoS.

CWE frequency-in-the-wild for Rust (RustSec→GHSA-OSV→`cwe_ids` join, n=8 sample, ~95% join rate expected): **CWE-416 (UAF), CWE-697 (incorrect comparison), CWE-125 (OOB read), CWE-787, CWE-190, CWE-295, CWE-1289, CWE-20, CWE-672**. CWE-416 surfaces as a top-3 class that was absent from the user's prior expectation. RustSec stores **no CWE field** natively — the join goes via `aliases[GHSA-*]`.

Corpus convention is settled: paired `<rule-id>.yaml` + `<rule-id>.rs` co-located, with `// ruleid:` / `// ok:` inline annotations, run by `semgrep --test`. GitHub AUP explicitly permits educational vulnerability content — the conservative `.gitignore` default is over-cautious; tracked-and-labelled is upstream convention. Clippy's `unwrap_used` / `indexing_slicing` / `arithmetic_side_effects` are all `restriction`-group, default `Allow`, no CWE tags — useful as a coarse fence to compose with, not replace. `pre-commit` (Python) is canonical; `prek 0.3.10` (MIT, 2026-04-21, used by CPython and Apache Airflow) is a drop-in Rust alternative.

## Topic Decomposition

The brief decomposes into ten sub-questions, of which the deepening passes prioritised five:

- **SQ1 — RustSec/OSV CWE distribution (2024–2026).** Frequency-in-the-wild count by CWE for Rust ecosystem advisories; requires the RustSec→GHSA→OSV `cwe_ids` two-hop join because RustSec's native schema has no CWE field.
- **SQ2 — Where idiomatic-safe-Rust panics live.** Sink shapes that recur in non-`unsafe` advisories: `unwrap`/`expect`/`?`, slice indexing, integer overflow in capacity/length, recursion/stack-overflow on attacker input, decompression/regex/serde untagged-recursive DoS.
- **SQ3 — Webapp-shape CVEs in axum/actix/hyper/sqlx/reqwest/serde.** Whether CWE-89/CWE-79/CWE-918 deserve top-10 frequency placement (web classes are present but lower-frequency than panic-DoS in absolute counts).
- **SQ4 — Direct competitors in LLM-driven SAST rule generation.** Semgrep Assistant, Snyk DeepCode AI, GitHub Copilot Autofix + custom CodeQL queries, SonarQube AI CodeFix, Veracode Fix, `kerberosmansour/SAST.GEN`, OSS LLM rule synthesisers.
- **SQ5 — Semgrep Rust frontend capabilities at HEAD (2026).** `pattern-either`, `pattern-inside`, `pattern-not`, `metavariable-pattern`, `metavariable-type`, taint mode, `pattern-inside unsafe { ... }`, macro expansion behaviour.
- **SQ6 — Adjacent Rust security tools and integrate-vs-replace decisions.** `cargo-audit`, `cargo-deny`, `cargo-vet`, `cargo-geiger`, Clippy security lints, miri, kani, prusti, creusot, MIRAI, rudra.
- **SQ7 — Corpus / test layout convention.** Per-rule co-located `.test.yaml`, single `tests/` dir, inline `metadata.examples`, or inline `// ruleid:` / `// ok:` annotations.
- **SQ8 — CI and dev-loop entry points for Semgrep on a Rust repo (2026).** `returntocorp/semgrep` action, GitLab templates, BuildKite plugin; local: `pre-commit`, `lefthook`, `prek`, IDE plugins, possible `cargo-semgrep`.
- **SQ9 — Publication-risk precedent for vulnerable-snippet corpora.** GitHub AUP "Active Malware or Exploits" clause, `semgrep-rules` precedent, takedown-notice history.
- **SQ10 — `cargo xtask` precedent for security-leaning verifiers.** Canonical layout (matklad), workspace-vs-single-crate trade-off, security-themed xtask precedents.

Priority during deepening: SQ1+SQ2 (drives `cwe-map-rust.md`), SQ5 (gates which rule primitives templates may use), SQ4 (gap justification), SQ6 (Clippy compose-vs-replace), SQ9 (publication-risk default), SQ7, SQ8, SQ3, SQ10.

## Key Findings

### CWE susceptibility ranking for Rust in 2026 (SQ1, SQ2, SQ3)

- **RustSec advisory schema has no CWE field.** The TOML schema defines `id`, `package`, `categories`, `keywords`, `aliases`, `cvss` — no `cwe`. CWE mapping requires a two-hop join: RustSec → `aliases[GHSA-*]` → GHSA-OSV record → `database_specific.cwe_ids`. The RUSTSEC-side OSV record itself does not carry `cwe_ids` even when its GHSA alias does.
- **GHSA-side coverage is high.** In an 8-advisory sample of 2024–2026 vuln-class advisories, 8/8 GHSA aliases returned populated `database_specific.cwe_ids`. Expect ~95%+ GHSA-side CWE coverage for `informational: vulnerability` advisories; the residual gap is the GHSA-less subset (informational/unmaintained-crate advisories).
- **RustSec's own taxonomy** is 10 categories — `code-execution`, `crypto-failure`, `denial-of-service`, `file-disclosure`, `format-injection`, `malicious-code`, `memory-corruption`, `memory-exposure`, `privilege-escalation`, `thread-safety`. RustSec explicitly tracks panic-on-attacker-input as DoS, including panics in code "advertised as panic-free."
- **OSV.dev does not natively expose CWE/CVSS/EPSS for Rust advisories.** Tracked as feature request in google/osv.dev#3245.
- **CWE classes confirmed in the deepened sample (n=8):**
  - **CWE-416 (use-after-free)** — 3 hits (cassandra-rs RUSTSEC-2024-0017, mio RUSTSEC-2024-0019, wasmtime RUSTSEC-2026-0090). **Was absent from the user's prior top-7; should be promoted into top-3.**
  - **CWE-697 (incorrect comparison)** — 2 hits (idna RUSTSEC-2024-0421, hpke-rs). **Net-new vs. prior expectation.**
  - **CWE-125 (OOB read)** — 2 hits (ruzstd RUSTSEC-2024-0400, scaly RUSTSEC-2026-0080).
  - Single hits: **CWE-787 (OOB write), CWE-190 (integer overflow), CWE-295 (cert validation), CWE-1289 (improper validation of specified entity input), CWE-20 (improper input validation), CWE-672 (operation on resource after expiration/release)**.
- **CWE-248 (panic-DoS) is not natively tagged.** Trail of Bits' production Rust panic rule uses **CWE-755 (improper handling of exceptional conditions)**. Adopt CWE-755 as the canonical tag for the panic-DoS class in the wedge.
- **The RustXec dataset (Virginia Tech, 2026 paper)** analyses 515 RustSec advisories from Jan 2021 – Apr 2025 and reports memory-corruption (31 cases) and DoS (28 cases) as leading classes; the most frequent specific CWEs are **CWE-787 (9), CWE-415 (8), CWE-400 (7)**.
- **2025 CWE Top 25 (MITRE/CISA, 2025-12-11)** is the cross-language baseline — top entries are CWE-79 (XSS), CWE-787, CWE-89 (SQLi), CWE-352 (CSRF), CWE-22 (path traversal). A KEV-aligned Top 10 was also published in 2025.
- **Web-class advisories are NOT well-represented in Rust frequency data.** Searches for axum/sqlx-specific SQLi/XSS/SSRF in 2024–2025 returned no major framework-level advisories; RUSTSEC-2024-0421 (idna/Punycode confusion in `url`/`sqlx`/`reqwest`) maps to CWE-20/CWE-1007 (homograph) — phishing/SSRF-adjacent, not classic SQLi/stored XSS. Including CWE-89/CWE-79/CWE-918 in a Rust top-10 is justified by user pain ("missing input sanitisation"), not by frequency in the public advisory corpus.

### Direct competitors in AI-driven SAST rule generation (SQ4)

- **Semgrep Assistant (2026)** is the closest precedent and the most-likely-to-be-confused-with the wedge:
  - Rule-generation input contract: `(description, 1 bad, 1 good) → 1 rule` — corpus pairs are user-supplied, not auto-derived.
  - Internal verification gate exists: raises `AiNonFatalResponseParseError` when "Rule matched the good code that it should not have matched against," then retries.
  - Self-reported quality bar: rules generated correctly are "around a 3 out of 10 on a complexity scale."
  - Distribution: Semgrep Cloud Platform + GitHub PR flow. **No mention of Claude Code skill integration, MCP exposure, or in-session callability** in the source pages.
  - Pricing: Semgrep Code $30/contributor/month (2026).
  - 2026 Assistant focus is **finding triage** (claims 96% alignment with human triage) and a "Memories" feature, **not** per-bug rule synthesis.
- **Snyk Code (DeepCode AI).** Symbolic-AI-based, not rule-based. Custom rules are **Enterprise-only Early Access**, written in a proprietary Datalog DSL against an internal "event graph" — opaque, not human-editable. Snyk advanced SAST entry ~$50/mo per developer.
- **GitHub Code Scanning + Copilot Autofix.** Code Scanning runs CodeQL (different language than Semgrep). Copilot Autofix generates *fix patches*, not new queries. Custom CodeQL queries are author-it-yourself; no "bug → query" auto-generator in GHAS as of 2026.
- **SonarQube AI CodeFix.** Generates fixes, not rules.
- **Veracode Fix.** Fix-suggestion AI, not rule-generation.
- **`kerberosmansour/SAST.GEN`** — user's own 2024 prior art; specific implementation/gating not extracted in this research run.
- **OSS LLM rule synthesisers.** No project surfaced that integrates as a Claude Code skill with corpus-gating.
- **Gap-to-wedge.** None of the surveyed 2026 tools advertise a workflow that takes `(bug_summary, fix_diff, file_paths)` and emits Rust-idiomatic Semgrep rules with auto-derived corpus pairs and a fire/silent verification gate, callable inside a Claude Code session.

### Adjacent Rust security tools (SQ6)

| Tool | 2026 status | Decision | Why |
|---|---|---|---|
| `rustsec/advisory-db` | active, daily updates | INTEGRATE — primary corpus source | Each advisory's `url` links to the upstream fix PR — diff = ground-truth bug+fix |
| `cargo-audit` 0.22.1 (2026-02) | Apache/MIT, active | INTEGRATE | Emits JSON and SARIF (`OutputFormat::{Json,Sarif}`) — clean CI trigger |
| `cargo-deny` 0.19.4 (2026-04) | Apache, active | INTEGRATE (advisories channel only) | `--format=json/sarif` confirmed |
| `cargo-vet` 0.10.0 (2024-10) | Apache | IGNORE | Trust attestation, not bug findings — no fix-diff stream |
| `cargo-geiger` 0.13.0 (2025-08) | active | IGNORE for findings; prioritisation signal only | Counts `unsafe` *presence*, not bug patterns; most `unsafe` is benign FFI |
| `miri` | active (rustc-tracking) | BACKLOG | UB findings are CWE-119/416/787 class; require triggering test + diagnostic-JSON parsing |
| `kani` 0.67.0 (2026-01) | active | IGNORE for v1 | Heavyweight CBMC; harness authoring required |
| `prusti` / `creusot` | active | IGNORE | Verification, not detection; Creusot is **LGPL-2.1** (vendoring caution) |
| MIRAI (facebookexp) | **archived 2024-08-22** | IGNORE | Tiny endorlabs fork exists; Kani is practical successor |
| `clippy` security restriction lints | shipped | INTEGRATE as coarse fence; layer Semgrep on top | See below |

- **Clippy security restriction lints (`unwrap_used`, `expect_used`, `indexing_slicing`, `arithmetic_side_effects`, `panic`, `missing_panics_doc`, `unimplemented`, `todo`)** are all **restriction group, default `Allow`** — opt-in only. `clippy::indexing_slicing` source comment: "Checks for usage of indexing or slicing that may panic at runtime… To avoid implicit panics from indexing and slicing." `clippy::arithmetic_side_effects` covers `+ - * <<` overflow and `/ %` divide-by-zero panics. The arxiv study "Unleashing the Power of Clippy in Real-World Rust Projects" finds `arithmetic_side_effects` patterns "pose a considerable challenge for automatic fixing by Clippy."
- **Clippy attaches no CWE IDs.** The CWE mapping has to be added by `/slo-rulegen`. Issue rust-clippy#6636 ("forbid all expect and unwrap use") shows the lint is debated for production opt-in.
- **No source claims a quantified ≥60% sink-shape overlap** between Clippy lints and CWE-248/CWE-190 sinks; that number would have to be measured against a corpus. At sink-shape level Clippy covers the surface broadly, but fires on every occurrence with no taint, no source/sink reasoning, no per-handler context. Composition (Clippy as coarse fence + Semgrep taint on top) is supported by evidence; replacement is not.

### Semgrep Rust frontend reality in 2026 (SQ5)

- **GA status.** Rust support reached GA in Semgrep Code by 2026, with 40+ Pro rules, cross-function dataflow analysis, and Supply Chain reachability + malicious-dependency detection. Beta declared February 2023. Kudelski Security contributed CST→AST and tree-sitter-rust grammar fixes in 2021.
- **Capability matrix:**

| Primitive | Status | Confidence | Source / note |
|---|---|---|---|
| `pattern: unsafe { ... }` | Works | HIGH | `semgrep-rules/rust/lang/security/unsafe-usage.yml` |
| `pattern-inside: unsafe { ... }` | Inconclusive | LOW | No public example; not documented as restricted; smoke-test required |
| `metavariable-type` on concrete types | Works | HIGH | Rust listed on the experiments docs page |
| `metavariable-type` w/ generics & trait bounds | Likely partial | LOW | Open issues #10380, #11150; no Rust example in docs; fall back to `metavariable-pattern` + regex |
| Patterns through proc-macros (`#[axum::debug_handler]`, `#[tokio::main]`, `#[tracing::instrument]`) | Negative | MEDIUM | Open issues #10471, #10362, #3600, #5221 |
| Taint source→sink (basic) | Works | HIGH | `bcder` example in 2026 SCA blog; `rust.hyper.sql.diesel-taint` exists |
| Taint through `format!` / intermediate `let` | Negative | HIGH | Issues #10757, #10900 — confirmed FNs in `diesel-taint` |
| Macro-arg taint propagation | Works | HIGH | Added July 2023 release notes; `foo!(&x)`, `foo!(*x)` propagate |
| `axum::extract::Query` → `sqlx::query` taint rule in the wild | Absent | HIGH | None found — would be net-new content |
| `semgrep --validate` non-zero exit on bad YAML | Works | HIGH | Exit code 5 (bad config), 7 (invalid rule), 4 (bad pattern) |
| `semgrep --test` non-zero on assertion failure | Works (with caveat) | HIGH | PR #6070 made it return 1; **caveat #10319**: returns 0 on *invalid rule itself* — must run `--validate` first |

- **Macro/false-positive rate** on heavy macro code (axum handlers, `tracing::instrument`, tokio `select!`) is **not quantified** in any source surfaced.
- **Upstream `rust/lang/security/` is exactly 10 rules + 10 fixtures** (`args-os`, `args`, `current-exe`, `insecure-hashes`, `reqwest-accept-invalid`, `reqwest-set-sensitive`, `rustls-dangerous`, `ssl-verify-none`, `temp-dir`, `unsafe-usage`). Each is a `.yml` + `.rs` pair co-located in the same directory — no `tests/` subdir, no per-rule subdir. **Zero overlap with the wedge's CWE-755/CWE-416/CWE-125/CWE-787/CWE-190/CWE-697 classes** — the wedge is genuinely net-new content.

### Trail of Bits panic-DoS rule as gold-standard precedent

- **File:** `panic-in-function-returning-result.yaml` + paired `panic-in-function-returning-result.rs` (3,676 bytes) at `https://github.com/trailofbits/semgrep-rules/tree/main/rs`.
- **Variation enumeration shape.** One rule enumerates **4 fn-context variations** in a single `pattern-either`: direct `Result<T1, T2>`, direct `Result<T>`, type-alias to `Result<T1, T2>`, type-alias to `Result<T>`. Plus **3 `pattern-not-inside`** exclusions for `#[cfg(test)] mod tests { ... }` blocks.
- **Implication.** "3–5 variation rules per bug" target is achievable inside one Semgrep rule's `pattern-either` — variation enumeration is a YAML structural concern, not a multi-file concern. 1 rule = 1 fixture file with N inline `// ruleid:` annotations.
- **CWE tag.** `cwe: "CWE-755: Improper Handling of Exceptional Conditions"` (not CWE-248). Metadata: `category: security`, `subcategory: [audit]`, `confidence: HIGH`, `likelihood: MEDIUM`, `impact: LOW`.
- **License caveat.** Trail of Bits semgrep-rules repo is **AGPL-3.0** — copy-pasting YAML wholesale would inherit AGPL. Treat as reference precedent only.
- **Trail of Bits' rule does not use `metavariable-type`** — it uses purely structural `pattern-inside fn ... -> Result<$T1, $T2> { ... }`. Result-type discrimination falls out of the structural pattern. The wedge does not need `metavariable-type` for v1.

### Corpus / test layout convention (SQ7)

- **Co-located paired files.** Semgrep's official testing docs require a rule at `path/to/rule.yaml` with sibling `path/to/rule.<lang-ext>` (e.g. `rule.py`, `rule.rs`) — not a separate `tests/` tree. Test filename must match the rule filename, only the extension differs.
- **Inline annotation convention.** `// ruleid: <id>` on the line above bad code, `// ok: <id>` for negative cases — used inline in the test file rather than as separate `bad.rs` / `good.rs` files. `semgrep --test` is the canonical runner (issue #2799).
- **`.test.yaml` extension** is reserved for the case where the rule's target language is YAML itself (avoids ambiguity with the rule's own `.yaml` file). Not relevant for Rust corpora.
- **Cross-pack consistency.** Same convention used by `semgrep/semgrep-rules` (3,966+ commits), `trailofbits/semgrep-rules`, `0xdea/semgrep-rules` (which documents the strongest version: "Each rule is accompanied by an actual vulnerable source code that was targeted by an exploit, with vulnerable lines marked with `// ruleid: ...`").
- **`// ruleid:` on `.rs` files is confirmed** — all 10 upstream Semgrep Rust fixtures use it; no Rust-specific quirks; `--test` accepts `//` comments verbatim.

### CI and dev-loop entry points (SQ8)

- **Canonical CI:** `returntocorp/semgrep` GitHub Action / `semgrep ci` workflow command. Free for OSS scans, paid Semgrep Cloud for centralised findings. GitLab has first-class Semgrep templates. BuildKite has a community plugin.
- **Canonical local-dev:** `pre-commit` framework still dominant in 2026; Semgrep ships a first-party hook repo at `semgrep/pre-commit`.
- **Lefthook** (Go, 2019): faster, parallel-by-default, but does **not** auto-install tools or pull remote hook configs.
- **`prek` (j178/prek):** Rust-native drop-in `pre-commit` replacement. Latest release **0.3.10 (2026-04-21)**, license **MIT**, reads `.pre-commit-config.yaml` unchanged. Production users include CPython and Apache Airflow. Pre-1.0 with partial language-hook parity.
- **No `cargo-semgrep` subcommand wrapper** surfaced in any search.
- **IDE plugins:** VS Code Semgrep extension; Cursor + IntelliJ-Rust integration via Semgrep LSP. Rust-analyzer integration with Semgrep is not native.

### Publication-risk precedent (SQ9)

- **GitHub AUP carves out educational vulnerability content explicitly.** Quoted policy text: *"GitHub allows dual-use content and supports the posting of content that is used for research into vulnerabilities, malware, or exploits, as the publication and distribution of such content has educational value."*
- **No takedown notices** surfaced against semgrep-rules-style repos for deliberately-vulnerable test snippets.
- **Upstream convention is tracked-and-labelled.** `semgrep/semgrep-rules` is fully public on GitHub and ships hundreds of deliberately-vulnerable test files; convention is per-rule paired test file with the same basename, lines tagged with `// ruleid: my-rule` (fire-on-bad) and `// ok: my-rule` (silent-on-good). Repos following the same pattern: Trail of Bits, 0xdea, elttam, CodeVigilant, Decurity.
- **The compliance-fine framing in the original idea doc is not corroborated** by any takedown evidence.
- **US EAR 5D002 / "intrusion software" controls** — enforcement on snippet collections is essentially zero; audit-tool source code (Metasploit, Nuclei, Semgrep itself) is publicly distributed.
- **Two-tier model is defensible:** snippets in a *user's application repo* default to `.gitignore`'d (compliance-finding mitigation in a non-security-tooling repo); snippets in *the rule-pack repo itself* track and label, following upstream convention.

### `cargo xtask` precedent (SQ10)

- **Canonical skeleton (matklad/cargo-xtask).** Root with `.cargo/config.toml`, root `Cargo.toml` declaring a workspace, sibling `xtask/` and `<main-crate>/` directories each with `Cargo.toml` and `src/`. Cargo alias is one line: `[alias] xtask = "run --package xtask --"` in `.cargo/config.toml`. Alias works from any subdirectory only when xtask is a workspace member.
- **Adoption.** Cargo itself uses xtasks; rust-analyzer ships an `xtask` crate. Tokio, ripgrep, clap, wasmtime, bevy, OpenVMM. The pattern is not officially blessed by the cargo team but is widely adopted.
- **Single-binary-with-subcommands** is the dominant convention (vs. multiple `xtask-foo` aliases). OpenVMM precedent uses `clean`, `fmt`, `fuzz`, `install-git-hooks`; bare `cargo xtask` prints the list.
- **Workspace vs single-crate.** matklad explicitly recommends workspace membership; non-workspace fallback (`run --manifest-path ./xtask/Cargo.toml --`) is documented but discouraged.
- **Helper crates** (`xtaskops`, `xtasks`, `tracel-xtask`) exist but are not canonical; matklad's spec doesn't bless any helper crate.
- **No widely-cited "security xtask" template** surfaced; OpenVMM's `fuzz` subcommand is the closest documented example.

### Repo-state constraints

- **No `xtasks/`, no `crates/sldo-sast/`, no `.cargo/config.toml`** — all are net-new infrastructure.
- Workspace deps pinned: `clap 4`, `anyhow 1`, `thiserror 2`, `colored 2`, `regex 1`, `chrono 0.4`, `which 7`. Any new verifier crate must reuse these.
- `--workspace` baseline is parked because of `sldo-tauri`; new crates must be appended explicitly to `cargo test -p ...` in `CLAUDE.md`.
- Existing skill-pack pattern (`skills/<name>/SKILL.md` + optional `references/`) requires no scaffolding code — `/slo-rulegen` and `/slo-ruleverify` can ship as Markdown.
- `/slo-critique` already has a `variant-analysis-playbook.md` that is prior art for variation enumeration logic.
- No Semgrep configuration exists in the repo today — M3 CI wiring is greenfield for the user.

## Library & Tool Evaluations

### Semgrep CLI (verifier engine)
- **Status / version:** Rust frontend GA; specific 2026 CLI version not pinned in this research.
- **License:** LGPL-2.1 (engine), Semgrep Rules License (registry rules).
- **Pros:** stable `--validate` and `--test` exit codes; first-party `pre-commit` hook repo; SARIF output; Rust taint mode usable; macro-arg taint propagation since July 2023; used internally for Rust SCA reachability.
- **Cons:** only 10 Rust rules in the upstream first-party tree (thin prior-art well); proc-macro parsing brittle (multiple open issues); `metavariable-type` semantics on Rust generics under-documented; `--test` returns 0 on invalid rule itself (issue #10319) — must run `--validate` first.
- **Fit:** required.

### Trail of Bits `semgrep-rules` (CWE-755 panic precedent)
- **Status:** live, `main` branch.
- **License:** **AGPL-3.0** — vendoring concern.
- **Pros:** only public production-grade Rust panic-DoS rule; demonstrates variation-enumeration shape (`pattern-either` + `pattern-not-inside` test-block exclusions); validates CWE-755 framing.
- **Cons:** AGPL forces re-implementation rather than vendoring; only one panic-class rule.
- **Fit:** reference precedent only; re-author rules independently to keep the pack's license clean.

### Upstream `semgrep/semgrep-rules` Rust pack
- **Status:** continuously updated `develop` branch; 3,966+ commits across language directories.
- **License:** Semgrep Rules License.
- **Pros:** defines the layout convention (paired `.yml` + `.rs` co-located); confirms `// ruleid:` annotation; covers crypto/TLS/process-arg classes the wedge does not need.
- **Cons:** zero overlap with wedge CWE classes — complementary, not extensible.
- **Fit:** layout-convention reference + complementary install.

### Semgrep Assistant
- **Status:** GA since 2024 launch; iterated through 2025–2026 docs.
- **Distribution:** Semgrep Cloud Platform + GitHub PR flow.
- **Pros:** internal fire-on-bad/silent-on-good gate; broadly similar input contract.
- **Cons:** input contract is `(description, 1 bad, 1 good) → 1 rule` (no auto-derivation from a found bug); SaaS-only, not callable as a Claude Code skill or via MCP; 2026 focus is finding triage, not rule synthesis.
- **Fit:** competitor — the wedge's differentiation rests on auto-corpus-derivation, variation enumeration, and in-session callability.

### Snyk Code (DeepCode AI)
- **Status:** GA; symbolic-AI engine.
- **Pros:** strong AI-fix workflow.
- **Cons:** custom rules **Enterprise-only Early Access**; proprietary Datalog DSL against opaque event graph; not human-editable in Semgrep sense; ~$50/mo per developer entry.
- **Fit:** competitor — different design direction (closed/opaque vs. open/inspectable).

### GitHub Code Scanning + Copilot Autofix
- **Status:** GA.
- **Pros:** native to GitHub; CodeQL is powerful for cross-procedural analysis.
- **Cons:** different query language (CodeQL ≠ Semgrep); Autofix generates fix patches, not new queries; no "bug → query" auto-generator as of 2026.
- **Fit:** orthogonal — not in the same product category.

### SonarQube AI CodeFix / Veracode Fix
- **Fit:** orthogonal — fix-suggestion AI, not rule-generation. No rule-synthesis surface.

### `cargo-audit` 0.22.1
- **License:** Apache-2.0 OR MIT.
- **Pros:** confirmed JSON+SARIF emit (`OutputFormat::{Json,Sarif}`); CWE-via-GHSA path is the same join `/slo-rulegen` already needs.
- **Cons:** emits findings against `Cargo.lock`, not source AST — does not directly drive a Semgrep rule.
- **Fit:** CI integration partner; trigger for the per-bug *extend* loop.

### `cargo-deny` 0.19.4 (2026-04)
- **License:** Apache.
- **Pros:** `--format=json/sarif`.
- **Fit:** integrate (advisories channel only).

### `cargo-vet` 0.10.0 (2024-10)
- **Fit:** ignore — trust attestation, no fix-diff stream.

### `cargo-geiger` 0.13.0 (2025-08)
- **Fit:** ignore for findings; useful as prioritisation signal only — counts `unsafe` *presence*, not bug patterns.

### `miri` / `kani` 0.67.0 / `prusti` / `creusot` / MIRAI / `rudra`
- **Fit:** miri = backlog; kani = ignore for v1 (heavyweight); prusti/creusot = ignore (verification, not detection; Creusot is LGPL-2.1); MIRAI = ignore (archived 2024-08-22).

### Clippy security restriction lints
- **Lints:** `unwrap_used`, `expect_used`, `indexing_slicing`, `arithmetic_side_effects`, `panic`, `missing_panics_doc`, `unimplemented`, `todo`, `integer_arithmetic` (deprecated, replaced by `arithmetic_side_effects`).
- **Status:** stable; restriction group; default `Allow`.
- **Pros:** zero new infrastructure; covers CWE-248/CWE-755/CWE-190 sink shapes broadly.
- **Cons:** no taint, no per-handler context, no CWE tags; fires on every occurrence — noisy without Semgrep layered on top.
- **Fit:** integrate as coarse fence, not replacement.

### `pre-commit` (Python, canonical)
- **Pros:** ecosystem default; first-party Semgrep hook at `semgrep/pre-commit`.
- **Cons:** Python runtime dep in a Rust-only repo.
- **Fit:** default.

### `prek` (j178/prek) — Rust-native pre-commit replacement
- **Version:** **0.3.10**, released **2026-04-21**.
- **License:** **MIT**.
- **Pros:** drop-in for `.pre-commit-config.yaml`; no Python bootstrap; production users include CPython and Apache Airflow.
- **Cons:** pre-1.0; partial language-hook parity.
- **Fit:** document as supported alternative; emit `.pre-commit-config.yaml` that works under either runner.

### `lefthook`
- **Pros:** parallel by default.
- **Cons:** does not auto-install tools or pull remote hook configs.
- **Fit:** out — adds friction without a clear win for this skill pack.

### `xtaskops` / `xtasks` / `tracel-xtask` (helper crates)
- **Fit:** do not adopt. Not canonical; not blessed by matklad's spec; introduces unpinned deps; current SLO crates use bare `clap 4` everywhere.

## Architecture Options

The deepening passes surfaced four shapes for delivering `/slo-rulegen` and `/slo-ruleverify`. They are not milestones; they are mutually-exclusive design defaults the runbook author can pick from.

### Option α — Skill-only (no Rust crate, no xtask)
- **Shape:** `/slo-rulegen` and `/slo-ruleverify` ship as `skills/<name>/SKILL.md` + `references/sast/`. No new Cargo crate. Skills drive Claude to shell out to `semgrep --validate` and `semgrep --test` via Bash, parsing exit codes inline.
- **Pros:** matches existing `slo-*` skill-pack pattern; no new Cargo infra; ships fastest.
- **Cons:** verifier gate is prompted Bash sequence — Claude can skip a step. Variation-coverage minimum is policy not code. Silent degradation if `semgrep` is not on PATH.
- **Best fit:** ship-speed wins; gate enforced by skill discipline + canonical Semgrep exit codes alone.

### Option β — Hybrid skill pack + `xtasks/sast-verify/` workspace member
- **Shape:** skills do LLM-heavy work. New `xtasks/sast-verify/` crate added to workspace + `.cargo/config.toml` declaring `[alias] xtask = "run --package xtasks/sast-verify --"`. Single binary with subcommands `validate`, `test`, `check-coverage`, each returning structured exit codes. Skills shell to `cargo xtask sast-verify <subcommand>`.
- **Pros:** deterministic gate — `/slo-rulegen` cannot append a rule until `cargo xtask sast-verify check-coverage --rule <path>` exits 0. Variation-coverage minimums enforceable in code. Reuses workspace deps.
- **Cons:** net-new `.cargo/config.toml` + `xtasks/` dir; must be appended to `CLAUDE.md`'s explicit `cargo test -p` baseline; xtask scope-creep risk flagged in idea doc.
- **Best fit:** variation-coverage-minimum determinism > ship-speed.

### Option γ — Hybrid skill pack + `crates/sldo-sast/` first-party crate
- **Shape:** thin skills shelling out to a `sldo-sast` binary, mirroring the `sldo-research` precedent. Crate owns advisory-db ingestion, GHSA→OSV CWE join, snippet-corpus management, verifier loop.
- **Pros:** deepest backstop; CWE map can be regenerated from data on a schedule; matches familiar codebase shape.
- **Cons:** ~3–4 weeks; couples bug-found→rule-extended loop to a `cargo build` cycle — directly opposes the user's "fix the regression now, not after a release tag" pain. Idea doc explicitly rejected this shape.
- **Best fit:** only if the project commits to a recurring advisory-db ingestion pipeline.

### Option δ — Skill-only + upstream Semgrep `--test` runner in CI
- **Shape:** `/slo-rulegen` writes rules, paired snippets, and a `.github/workflows/semgrep-test.yml` that invokes Semgrep's first-party `--test` runner in CI. Local dev uses `pre-commit` with `semgrep/pre-commit`. The "gate" lives in upstream Semgrep tooling, not a new component.
- **Pros:** minimum surface area; matches semgrep-rules upstream convention exactly (tracked corpus + `// ruleid:` + `semgrep --test`); zero new in-repo infrastructure; reinforced by F9 (GitHub AUP allows tracked corpora) — generated pack runs under any consumer's `semgrep ci` with no rewriting.
- **Cons:** verification responsibility pushed to user's CI provider; no in-repo deterministic fail-fast for variation-coverage minimum.
- **Best fit:** portability of the generated pack outranks in-repo gate.

### Consolidation note from the deepening pass
The "1 rule = 1 `pattern-either` with N variations = 1 paired `.rs` fixture with N `// ruleid:` annotations" shape (from F3, F5) means corpus layout is settled and the gate can be canonical Semgrep `--validate` + `--test` for v1, with no custom verifier required. The deepened evidence pushes the default toward **δ**, with **β** as a hardening upgrade if variation-coverage minimums ever justify net-new Cargo infra.

## API & SDK Documentation

The current research run did not exercise any third-party SDK or HTTP API beyond ad-hoc fetches against the OSV.dev REST endpoint and GitHub's tree API; full SDK contracts were not extracted. The data points surfaced are:

- **OSV.dev REST API.** `GET https://api.osv.dev/v1/vulns/<ID>` returns a JSON OSV record. For RUSTSEC IDs, the record contains `aliases[]` (often a `GHSA-*`) and `database_specific.categories` + `database_specific.cvss`, but **not** `cwe_ids`. For GHSA IDs, the same endpoint returns `database_specific.cwe_ids` populated. The two-hop join is the only documented data path for CWE on Rust advisories; OSV `POST /v1/query` with `package.ecosystem=crates.io` was referenced but not exercised. Issue google/osv.dev#3245 tracks adding native CWE/CVSS/EPSS metadata.
- **GitHub git-tree API.** `GET https://api.github.com/repos/<owner>/<repo>/git/trees/<branch>?recursive=1` returns the full tree blob list — used to enumerate `semgrep/semgrep-rules` `rust/` directory contents.
- **Semgrep CLI surface.** `semgrep --validate <rule.yaml>` (exit codes: 5 bad config, 7 invalid rule, 4 bad pattern); `semgrep --test <rule-or-dir>` (exit 1 on assertion failure since PR #6070; caveat: returns 0 on invalid rule itself per issue #10319 — must `--validate` first); `semgrep ci` for CI-mode runs. Rule schema entry points: `pattern`, `pattern-either`, `pattern-not`, `pattern-inside`, `pattern-not-inside`, `metavariable-pattern`, `metavariable-regex`, `metavariable-type` (experimental); taint mode via `mode: taint` with `pattern-sources` / `pattern-sinks` / `pattern-sanitizers`.
- **Pre-commit hook spec.** `semgrep/pre-commit` is a pre-commit-hooks-style YAML repo consumed by `.pre-commit-config.yaml`. `prek 0.3.10` reads the same config format unchanged.
- **No Anthropic SDK / Claude API surface** was exercised — `/slo-rulegen` is a Markdown skill that drives the existing `claude` CLI, not a new SDK consumer.
- **No `cargo-audit` / `cargo-deny` programmatic API** was exercised beyond the documented `--format=json/sarif` flags.

Detailed SDK call patterns, request/response schemas, and authentication contracts were not produced for this section in the current research run.

## Design Recommendations

Recommendations are drawn directly from the raw findings; they are not milestones or a runbook. The runbook author owns sequencing.

- **(confidence: high) Use CWE-755, not CWE-248, for the panic-DoS class.** Trail of Bits' production rule tags `panic-in-function-returning-result` as `CWE-755: Improper Handling of Exceptional Conditions`. RustSec/GHSA do not natively tag panic-DoS at all; CWE-755 is the operational consensus tag.
- **(confidence: high) Adopt the co-located paired `<rule-id>.yaml` + `<rule-id>.rs` convention with inline `// ruleid:` / `// ok:` annotations.** This is uniformly used by upstream `semgrep/semgrep-rules`, Trail of Bits, 0xdea, and is the documented contract in Semgrep's `--test` runner. No Rust-specific quirks; `//` comments are accepted verbatim.
- **(confidence: high) Build `cwe-map-rust.md` from a two-hop RustSec → GHSA → OSV `cwe_ids` join.** RustSec has no native CWE field; GHSA-side coverage is high (8/8 in the deepened sample). Plan for a category-only crosswalk fallback for the GHSA-less informational/unmaintained advisory tail.
- **(confidence: high) Promote CWE-416 (use-after-free), CWE-697 (incorrect comparison), and CWE-125 (OOB read) into the top-level wedge scope alongside CWE-787, CWE-190, CWE-755.** All three have multiple hits in the deepened sample; CWE-416 and CWE-697 were absent from the user's prior expectation.
- **(confidence: high) Track vulnerable corpus snippets in the rule-pack repo, labelled with `// ruleid:` annotations, with a top-level disclaimer.** GitHub AUP explicitly carves out educational vulnerability content; semgrep-rules / Trail of Bits / 0xdea all ship public tracked corpora; no takedowns surfaced. The conservative `.gitignore`'d default in the idea doc is over-cautious for the pack repo itself.
- **(confidence: high) For corpora generated inside a *user's application repo*, default to `.gitignore`'d.** That repo is not a security-tooling repo; tracked vuln snippets there would be a compliance finding even if portable. Two-tier convention: rule-pack repo tracks-and-labels; user app repo gitignores.
- **(confidence: high) Emit a `.pre-commit-config.yaml` that works under both `pre-commit` (canonical) and `prek` (Rust drop-in).** `prek 0.3.10` is MIT-licensed, used by CPython and Apache Airflow, and reads the same config format unchanged. Default install instructions to `pre-commit`; document `prek` as alternative.
- **(confidence: high) Compose with Clippy's `unwrap_used` / `indexing_slicing` / `arithmetic_side_effects` rather than replace them.** All three are restriction-group `Allow`-by-default and have no CWE tags; layering Semgrep taint/context-aware rules on top adds the source/sink reasoning Clippy lacks.
- **(confidence: high) Re-author rules independently rather than vendoring Trail of Bits patterns.** Trail of Bits' semgrep-rules repo is AGPL-3.0; copying YAML wholesale would inherit AGPL. Use the structural shape (variation-enumeration via `pattern-either`, `pattern-not-inside #[cfg(test)]` exclusions) as inspiration; write fresh.
- **(confidence: high) Adopt the single-binary-with-subcommands xtask shape if any xtask is introduced.** matklad's spec and OpenVMM precedent both endorse `cargo xtask <subcommand>` over multiple `xtask-foo` aliases. Hand-roll a `main.rs` rather than depending on `xtaskops` / `xtasks` / `tracel-xtask` (none are canonical).
- **(confidence: medium) Use `pattern-either` with N variations inside a single rule, paired with one fixture file containing N `// ruleid:` annotations, as the default rule shape for v1.** Trail of Bits' panic rule is the existence proof; the wedge's "3–5 variations per CWE" target maps cleanly. Reach for taint mode only when structural enumeration fails.
- **(confidence: medium) Run `semgrep --validate` *before* `semgrep --test` in any verifier sequence.** Issue #10319 documents that `--test` returns 0 on an invalid rule itself; `--validate` is the only way to catch bad YAML/pattern syntax up-front. Exit codes 5/7/4 are stable.
- **(confidence: medium) Treat `metavariable-type` on Rust generics as unreliable for v1.** Docs are silent on Rust examples; open issues #10380 and #11150 suggest partial support. Trail of Bits' precedent uses purely structural `pattern-inside fn ... -> Result<$T1, $T2>` and works without it. Fall back to `metavariable-pattern` + regex if needed.
- **(confidence: medium) Treat proc-macro patterns as unreliable for v1.** Open issues #10471, #10362, #3600, #5221 confirm the gap; `#[axum::debug_handler]`, `#[tokio::main]`, `#[tracing::instrument]`-decorated handlers will not match through the macro. Macro-arg taint propagation does work (since July 2023). Document the limitation in the rule pack's README; do not assume FP/FN parity with upstream.
- **(confidence: medium) Wire `cargo-audit` (JSON or SARIF emit) as the CI trigger for the per-bug *extend* loop.** It already does the GHSA-CWE join `/slo-rulegen` needs and emits structured output; pairing it with `/slo-rulegen` against the patch diff is the natural integration.
- **(confidence: medium) Default to Architecture Option δ (skill-only + upstream `--test` runner in CI) for v1.** Minimum surface area, matches upstream convention, generated pack is portable under any consumer's `semgrep ci`. Reserve Option β (xtask) as a hardening upgrade only if a deterministic in-repo variation-coverage gate becomes load-bearing.
- **(confidence: low) Skip `metavariable-type` filtering on trait bounds entirely.** Inferred from absence of public examples and the precedent rule's avoidance of it; needs a smoke test before being declared a hard rule.
- **(confidence: low) Skip a `crates/sldo-sast/` first-party crate (Architecture Option γ).** Justified by the user's explicit "fix the regression now" pain and the idea doc's prior rejection; not justified by hard data.

## Risks & Open Questions

Ordered by impact on the wedge's design.

1. **`pattern-inside: unsafe { ... }` is undocumented for Rust.** No public example in `semgrep/semgrep-rules`; no documented restriction. A 5-line smoke rule + fixture is the only way to confirm before templating any unsafe-FFI rule (CWE-119/787) into `references/sast/`. Fallback if it fails: `pattern-inside: fn $F(...) { ... unsafe { ... } ... }` or `metavariable-pattern` over the function body.
2. **Variation-template content for CWE-416 / CWE-190 / CWE-787 / CWE-125 / CWE-697 is not yet sourced.** Trail of Bits gives the CWE-755 shape; equivalent shapes for memory-class and comparison-class CWEs need real fix-diff sourcing. Concrete fix-diff sources to mine: `cassandra-rs` UAF (RUSTSEC-2024-0017), `mio` named-pipe-token UAF (RUSTSEC-2024-0019), `wasmtime::Linker::clone` UAF (RUSTSEC-2026-0090), `hpke-rs` underflow (RUSTSEC-2026-0070), `ruzstd` OOB-read (RUSTSEC-2024-0400), `idna` Punycode comparison (RUSTSEC-2024-0421). Each diff produces 1–2 idiom shapes for the variation backbone.
3. **Full RustSec→GHSA join coverage % over a 24-month window is unmeasured.** Sample (n=8) shows 100% GHSA-side CWE coverage, but bottleneck is the GHSA-alias-presence rate among RustSec advisories. A one-shot script over `rustsec/advisory-db` (2024-04 → 2026-04) is needed to report the join hit-rate and the size of the residual category-only fallback.
4. **CWE-89 / CWE-79 / CWE-918 in a Rust top-10 lacks frequency support.** No major framework-level advisories surfaced for axum/sqlx SQLi/XSS/SSRF in 2024–2025. Inclusion is justifiable on threat-class grounds (axum apps absolutely can be vulnerable when devs hand-roll SQL or HTML), not frequency. Open question: should the wedge's top-10 be frequency-ranked (omits these classes), threat-class-ranked (includes them), or hybrid?
5. **Whether Clippy's restriction lints cover ≥60% of CWE-755/CWE-190 sink shapes is unmeasured.** No source quantifies the overlap; the ≥60% assumption from the brief would have to be measured against a real corpus, not asserted from documentation. The compose-not-replace decision stands either way, but the exact division of labour between Clippy and Semgrep is not pinned.
6. **`metavariable-type` on Rust generics and trait bounds — actual behaviour is unverified.** Open issues #10380 and #11150 suggest partial support. Fallback path (`metavariable-pattern` + regex) is the recommendation, but breaks under proc-macro expansion (issues #10471, #10362). Specific concrete asks: smoke-test against `Result<_, _>` and `Option<_>` discrimination.
7. **Semgrep macro/false-positive rate on heavy macro code is not quantified.** No source surfaced gives an FP rate for axum handlers, `tracing::instrument`, tokio `select!`. The deepening pass left this as a gap to fill by inspecting `semgrep/semgrep` issues filtered by `[lang/rust]`.
8. **Trail of Bits AGPL implications for a re-implemented rule that mirrors the same `pattern-either` structure.** Pattern shapes are likely uncopyrightable (functional content), but a legal-comfort answer would justify whether the wedge can closely mirror the structure or must diverge intentionally.
9. **`semgrep --test` exit-code semantics in the latest 2026 CLI.** Issue #10319 (returns 0 on invalid rule) was open at deepening time; confirm whether it has been fixed before relying on `--validate` + `--test` as the v1 gate.
10. **`prek` license + maturity past 0.3.10 for CI fit.** MIT confirmed; pre-1.0 status and partial language-hook parity. CI adoption decision should pin a version and document fallback behaviour if the runner is missing.
11. **Whether `// ruleid:` annotation on `.rs` files works without quirks under `semgrep --test`.** Confirmed by upstream pack inspection (10/10 Rust fixtures use the convention), but not stress-tested against multi-line patterns or block-comment annotations.
12. **`kerberosmansour/SAST.GEN` 2024 prior art.** Specific implementation, gating logic, and what 2026 should improve on were not extracted in this run; the brief flagged it as a baseline.
13. **OSS LLM rule synthesisers on GitHub.** Searches did not surface any project that integrates as a Claude Code skill with corpus-gating; absence is not the same as "does not exist."
14. **`cargo-semgrep` subcommand wrapper existence.** Not surfaced by any search; treat as "not found", not "does not exist."

## References

- [RustSec Advisory Database — Categories](https://rustsec.org/categories/)
- [RustSec Advisory Database — Advisories](https://rustsec.org/advisories/)
- [RustSec Advisory Database — About](https://rustsec.org/)
- [RustSec Advisory Database — Reporting Vulnerabilities](https://rustsec.org/contributing.html)
- [rustsec/advisory-db GitHub repository](https://github.com/rustsec/advisory-db)
- [rustsec/advisory-db README (raw)](https://raw.githubusercontent.com/rustsec/advisory-db/main/README.md)
- [RUSTSEC-2024-0421 advisory page](https://rustsec.org/advisories/RUSTSEC-2024-0421.html)
- [RUSTSEC-2026-0070 advisory page](https://rustsec.org/advisories/RUSTSEC-2026-0070.html)
- [OSV — Data sources](https://google.github.io/osv.dev/data/)
- [google/osv.dev issue #3245 — connect OSV advisories to CWE/CVSS/EPSS metadata](https://github.com/google/osv.dev/issues/3245)
- [OSV vulnerability — RUSTSEC-2025-0046](https://osv.dev/vulnerability/RUSTSEC-2025-0046)
- [OSV vulnerability — RUSTSEC-2024-0421](https://osv.dev/vulnerability/RUSTSEC-2024-0421)
- [OSV API — RUSTSEC-2024-0421 record](https://api.osv.dev/v1/vulns/RUSTSEC-2024-0421)
- [OSV API — GHSA-h97m-ww89-6jmq record (idna)](https://api.osv.dev/v1/vulns/GHSA-h97m-ww89-6jmq)
- [OSV API — GHSA-x9xc-63hg-vcfq record](https://api.osv.dev/v1/vulns/GHSA-x9xc-63hg-vcfq)
- [Wiz — RUSTSEC-2025-0112](https://www.wiz.io/vulnerability-database/cve/rustsec-2025-0112)
- [Wiz — RUSTSEC-2024-0423](https://www.wiz.io/vulnerability-database/cve/rustsec-2024-0423)
- [Wiz — RUSTSEC-2025-0028](https://www.wiz.io/vulnerability-database/cve/rustsec-2025-0028)
- [cvedetails — Rust-lang Rust vulnerabilities](https://www.cvedetails.com/vulnerability-list/vendor_id-19029/product_id-48677/Rust-lang-Rust.html)
- [MITRE CVE search — keyword=rust](https://cve.mitre.org/cgi-bin/cvekey.cgi?keyword=rust)
- [RustXec: A Vulnerability Reproduction Dataset (2026 PDF)](https://people.cs.vt.edu/xinw/publications/RustXec26-B38KjKAe.pdf)
- [Penligent — CVE-2025-68260 (first Rust kernel vulnerability)](https://www.penligent.ai/hackinglabs/rusts-first-breach-cve-2025-68260-marks-the-first-rust-vulnerability-in-the-linux-kernel/)
- [CVE-2024-24576 — std::process::Command on Windows (BatBadBut)](https://blog.rust-lang.org/2024/04/09/cve-2024-24576.html)
- [CVE-2024-43402 — std::process::Command Windows hardening](https://blog.rust-lang.org/2024/09/04/cve-2024-43402.html)
- [CERT-EU — Critical Vulnerability in Rust on Windows (2024-035)](https://cert.europa.eu/publications/security-advisories/2024-035/)
- [2025 CWE Top 25 Most Dangerous Software Weaknesses (MITRE)](https://cwe.mitre.org/top25/archive/2025/2025_cwe_top25.html)
- [2025 CWE Top 25 (CISA alert)](https://www.cisa.gov/news-events/alerts/2025/12/11/2025-cwe-top-25-most-dangerous-software-weaknesses)
- [2025 CWE Top 10 KEV Weaknesses Insights](https://cwe.mitre.org/top25/archive/2025/2025_kev_insights.html)
- [CWE-1435: Weaknesses in the 2025 CWE Top 25](https://cwe.mitre.org/data/definitions/1435.html)
- [Common Weakness Enumeration (root)](https://cwe.mitre.org/)
- [Semgrep — Supported languages](https://semgrep.dev/docs/supported-languages)
- [Codebase-Aware Reachability Analysis Coverage for Rust (Semgrep blog, 2026)](https://semgrep.dev/blog/2026/semgrep-supply-chain-extends-reachability-coverage-to-rust/)
- [Semgrep — Taint analysis overview](https://semgrep.dev/docs/writing-rules/data-flow/taint-mode/overview)
- [Semgrep — Taint analysis (root)](https://semgrep.dev/docs/writing-rules/data-flow/taint-mode/)
- [Semgrep — Advanced techniques for taint analysis](https://semgrep.dev/docs/writing-rules/data-flow/taint-mode/advanced)
- [Demystifying Taint Mode (Semgrep blog, 2022)](https://semgrep.dev/blog/2022/demystifying-taint-mode/)
- [Taint mode is now in beta (Semgrep blog, 2021)](https://semgrep.dev/blog/2021/taint-mode-is-now-in-beta/)
- [Semgrep release notes — February 2023 (Rust beta)](https://semgrep.dev/docs/release-notes/february-2023)
- [Semgrep release notes — July 2023 (Rust macro taint propagation)](https://semgrep.dev/docs/release-notes/july-2023)
- [Semgrep v1.49.0 GitHub release](https://github.com/semgrep/semgrep/releases/tag/v1.49.0)
- [Kudelski Security — Advancing Rust Support in Semgrep (2021)](https://kudelskisecurity.com/research/advancing-rust-support-in-semgrep)
- [Semgrep — Test rules](https://semgrep.dev/docs/writing-rules/testing-rules)
- [Semgrep — Rule structure syntax](https://semgrep.dev/docs/writing-rules/rule-syntax)
- [Semgrep — Run rules](https://semgrep.dev/docs/running-rules)
- [Semgrep — Rules overview](https://semgrep.dev/docs/writing-rules/overview)
- [Semgrep — Pattern syntax (experimental)](https://semgrep.dev/docs/writing-rules/experiments/pattern-syntax)
- [Semgrep — metavariable-type docs](https://semgrep.dev/docs/writing-rules/experiments/metavariable-type)
- [Semgrep — Use the Semgrep rule schema in VS Code](https://semgrep.dev/docs/kb/rules/using-semgrep-rule-schema-in-vscode)
- [Semgrep — Contribute rules to the Semgrep Registry](https://semgrep.dev/docs/contributing/contributing-to-semgrep-rules-repository)
- [Writing Semgrep rules — methodology blog](https://semgrep.dev/blog/2020/writing-semgrep-rules-a-methodology/)
- [semgrep/semgrep-rules (GitHub)](https://github.com/semgrep/semgrep-rules)
- [semgrep/semgrep-rules — develop tree (recursive API)](https://api.github.com/repos/semgrep/semgrep-rules/git/trees/develop?recursive=1)
- [semgrep/semgrep-rules — rust/lang/security/unsafe-usage.yml](https://raw.githubusercontent.com/semgrep/semgrep-rules/develop/rust/lang/security/unsafe-usage.yml)
- [semgrep/semgrep-rules — rust/lang/security/rustls-dangerous.yml](https://raw.githubusercontent.com/semgrep/semgrep-rules/develop/rust/lang/security/rustls-dangerous.yml)
- [trailofbits/semgrep-rules (GitHub)](https://github.com/trailofbits/semgrep-rules)
- [trailofbits/semgrep-rules — /rs directory](https://github.com/trailofbits/semgrep-rules/tree/main/rs)
- [trailofbits/semgrep-rules — panic-in-function-returning-result.yaml (raw)](https://raw.githubusercontent.com/trailofbits/semgrep-rules/main/rs/panic-in-function-returning-result.yaml)
- [0xdea/semgrep-rules (GitHub)](https://github.com/0xdea/semgrep-rules)
- [Issue #2799 — `semgrep --test` a yaml file](https://github.com/returntocorp/semgrep/issues/2799)
- [Issue #1228 — Invalid schema on yaml/* unit test files](https://github.com/semgrep/semgrep-rules/issues/1228)
- [Semgrep Assistant — overview](https://semgrep.dev/docs/semgrep-assistant/overview)
- [Semgrep Assistant — AppSec Engineer Assistant product page](https://semgrep.dev/products/semgrep-code/assistant/)
- [Semgrep blog — The tech behind Semgrep Assistant (2024)](https://semgrep.dev/blog/2024/the-tech-behind-semgrep-assistant/)
- [Semgrep blog — Using AI to write secure code with Semgrep (2023)](https://semgrep.dev/blog/2023/using-ai-to-write-secure-code-with-semgrep/)
- [Semgrep blog — AI & Cybersecurity, three months of Semgrep Assistant (2023)](https://semgrep.dev/blog/2023/assistant-public-beta/)
- [Semgrep blog — 10x your AppSec program with Semgrep Assistant (2024)](https://semgrep.dev/blog/2024/assistant-ga-launch/)
- [Snyk vs Semgrep (Konvu)](https://konvu.com/compare/snyk-vs-semgrep)
- [Semgrep vs Snyk Code (AppSecSanta)](https://appsecsanta.com/sast-tools/semgrep-vs-snyk-code)
- [Snyk Code Review 2026 (AppSecSanta)](https://appsecsanta.com/snyk-code)
- [Best AI Code Security Tools 2025 (sanj.dev)](https://sanj.dev/post/ai-code-security-tools-comparison)
- [Snyk vs Semgrep: SCA vs Custom SAST Rules in 2026 (DEV)](https://dev.to/rahulxsingh/snyk-vs-semgrep-sca-platform-vs-custom-sast-rules-in-2026-3047)
- [7 Best Snyk Alternatives for 2026 (DeepSource)](https://deepsource.com/resources/snyk-alternatives)
- [Best SAST Tools for AI-Generated Code (Vibe-Eval)](https://vibe-eval.com/testing/sast-tools-ai-code)
- [Snyk vs Semgrep vs Corgea (Corgea)](https://corgea.com/blog/compare/snyk-vs-semgrep/)
- [Snyk vs Semgrep (Aikido)](https://www.aikido.dev/blog/snyk-vs-semgrep)
- [Semgrep Pricing in 2026 (DEV)](https://dev.to/rahulxsingh/semgrep-pricing-in-2026-open-source-vs-team-vs-enterprise-costs-3dic)
- [Clippy Lints index (master)](https://rust-lang.github.io/rust-clippy/master/index.html)
- [Clippy Lint Configuration](https://doc.rust-lang.org/clippy/lint_configuration.html)
- [rust-clippy — clippy_lints/src/indexing_slicing.rs](https://github.com/rust-lang/rust-clippy/blob/master/clippy_lints/src/indexing_slicing.rs)
- [rust-clippy issue #6636 — forbid all expect and unwrap use](https://github.com/rust-lang/rust-clippy/issues/6636)
- [rust-clippy CHANGELOG](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md)
- [Unleashing the Power of Clippy in Real-World Rust Projects (arxiv)](https://arxiv.org/pdf/2310.11738)
- [semgrep/pre-commit (GitHub)](https://github.com/semgrep/pre-commit)
- [Customize Semgrep in pre-commit](https://semgrep.dev/docs/kb/integrations/customize-semgrep-precommit)
- [j178/prek (GitHub)](https://github.com/j178/prek)
- [prek documentation](https://prek.j178.dev/)
- [HN — Better pre-commit, re-engineered in Rust](https://news.ycombinator.com/item?id=45931273)
- [HN — Prek: drop-in pre-commit replacement](https://news.ycombinator.com/item?id=46873138)
- [Comparing Code Quality Meta Tools (House Absolute)](https://blog.urth.org/2020/05/08/comparing-code-quality-meta-tools/)
- [Git hooks management with pre-commit and lefthook (0xDC.me)](https://0xdc.me/blog/git-hooks-management-with-pre-commit-and-lefthook/)
- [dotdc/test-lefthook-and-pre-commit (GitHub)](https://github.com/dotdc/test-lefthook-and-pre-commit)
- [matklad/cargo-xtask](https://github.com/matklad/cargo-xtask)
- [matklad/cargo-xtask README](https://github.com/matklad/cargo-xtask/blob/master/README.md)
- [matklad/cargo-xtask issue #8 — workspaces are recommended](https://github.com/matklad/cargo-xtask/issues/8)
- [matklad — Large Rust Workspaces (2021)](https://matklad.github.io/2021/08/22/large-rust-workspaces.html)
- [rust-analyzer — xtask docs](https://rust-lang.github.io/rust-analyzer/xtask/index.html)
- [OpenVMM Guide — cargo xtask](https://openvmm.dev/guide/dev_guide/dev_tools/xtask.html)
- [nickgerace/cargo-xtask-example](https://github.com/nickgerace/cargo-xtask-example)
- [jondot/xtaskops](https://github.com/jondot/xtaskops)
- [sebastienrousseau/xtasks](https://github.com/sebastienrousseau/xtasks)
- [tracel-xtask on crates.io](https://crates.io/crates/tracel-xtask)
- [Adding custom runnable tasks to a Rust project (bryantluk.com)](https://blog.bryantluk.com/cargo-xtask/)
- [lib.rs — cargo-xtask crate](https://lib.rs/crates/cargo-xtask)
- [docs.rs — xtasks](https://docs.rs/xtasks)
- [rust-secure-code/projects (security project index)](https://github.com/rust-secure-code/projects)
- [GitHub Acceptable Use Policy — Active Malware or Exploits](https://docs.github.com/en/site-policy/acceptable-use-policies/github-active-malware-or-exploits)
