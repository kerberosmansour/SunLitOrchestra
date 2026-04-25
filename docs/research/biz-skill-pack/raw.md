---
topic: # Research brief — biz-skill-pack  ## Wedge (one sentence)  Ship `/slo-legal` v1 — NDA + Contractor SOW + IP Assignment + Terms & Conditions templates plus an advisor-pattern triage gate that refuses …
generated_on: 2026-04-25 00:33:08 +0100
source_prompt_bytes: 5885
generator: sldo-research
---

# Research Dossier

This dossier is a structured research artifact produced by `sldo-research`. It is intended as the `prompt_file` input to `sldo-plan`.

## Repository Context

## Tech Stack

- **Primary language:** Rust 2021, multi-crate Cargo workspace (resolver "2"). No explicit MSRV pin, no `rust-toolchain.toml`.
- **Core crates (workspace deps):** `clap 4` (derive), `anyhow 1`, `thiserror 2`, `colored 2`, `regex 1`, `chrono 0.4`, `which 7`.
- **Per-crate deps of note:** `serde`/`serde_json`, `toml 0.8` (install + tla-sha), `reqwest 0.11` with `rustls-tls` (tla-sha), `sha2 0.10`, `url 2`, `tempfile 3` (tests), `dotenvy 0.15`, `base64 0.22`, `rig-core 0.33` with `audio` (Tauri voice).
- **Secondary stack (parked):** `crates/sldo-tauri/` — Tauri v2 + React/TypeScript UI. Requires Node.js ≥ 18 and `cargo install tauri-cli --version '^2'`. Parked since 2026-04 per `CLAUDE.md`; not in `cargo test` baseline.
- **External runtime dependencies:** the `claude` CLI (Claude Code) and `git` are shelled out to via `std::process::Command`. No managed async runtime outside of Tauri (which uses `tokio` "full"); the CLI binaries are synchronous with thread-per-pipe draining.
- **Skill-pack surface:** Markdown `SKILL.md` files under `skills/<name>/` installed into `~/.claude/skills/` by `sldo-install` via symlinks.

## Project Structure

One level deep from repo root (`/Users/sherifmansour/Documents/Dev/GitHub/SunLitOrchestrate`):

- `Cargo.toml`, `Cargo.lock`, `Makefile`, `README.md`, `CLAUDE.md`, `SECURITY.md` — workspace root + project docs.
- `crates/` — Rust workspace members:
  - `sldo-common/` — shared lib: `color`, `copilot` (Claude CLI runner), `detect`, `git`, `logging`, `preflight`, `runbook` parser, `toolflags`.
  - `sldo-plan/` — binary that drives Claude to produce a v3 runbook from a prompt file.
  - `sldo-run/` — binary that executes a runbook milestone-by-milestone with build/test verification.
  - `sldo-research/` — binary for the 5-phase research dossier pipeline (explore → web → deepen → synth).
  - `sldo-install/` — binary + manifest (`~/.sldo/install.toml`) that symlinks skills into `~/.claude/skills/`.
  - `sldo-tla-sha/` — bin+lib: fetches TLA+ tool binaries, enforces host allow-list, verifies SHA-256 before extraction.
  - `sldo-tauri/` — **parked** desktop app (Tauri + React).
- `src/` — legacy bash implementations: `plan-milestones.sh`, `run-milestones.sh` (superseded by Rust CLIs but retained).
- `skills/` — first-party `/slo-*` skill folders (ideate, research, architect, tla, plan, critique, execute, verify, retro, ship, second-opinion, freeze, resume) + vendored third-party `get-api-docs/`. Each has a `SKILL.md` with YAML frontmatter.
- `tests/` — workspace-level E2E tests registered in root `Cargo.toml` as `[[test]]` entries (`e2e_scaffold_m1`, `e2e_common_m2`, `e2e_plan_m3`, `e2e_run_m4`, `e2e_integration_m5`, seven `e2e_tauri_m*`, two `e2e_voice_tx_m*`, seven `e2e_research_m*`).
- `docs/` — `ARCHITECTURE.md`, `MIGRATION.md`, `skill-pack-catalog.md`, runbook template (`runbook-template_v_3_template.md`), per-feature `RUNBOOK-*.md`, and subtrees `idea/`, `research/`, `design/`, `critique/`, `completion/`, `lessons/`, `legal/`.
- `output/` — default dossier output dir (gitignored).
- `target/` — Cargo build artifacts (gitignored).
- Hidden: `.sldo-logs/`, `.copilot-logs/`, `.claude/`, `.sldo/` — runtime logs/state, all gitignored.

## Build & Test

Concrete commands, drawn from `Makefile`, `README.md`, and `CLAUDE.md`:

- **Baseline test command (project rule, CLAUDE.md):**
  ```bash
  cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install
  ```
  The `--workspace` baseline is **not** used because `sldo-tauri` (parked) leaves it red on macOS arm64 (missing esbuild arm64 binary in its `node_modules/`).
- **Full build (backend only, fast iteration):** `cargo build --workspace` (`make check`).
- **Build release binaries:** `cargo build --workspace --release` (artefacts at `target/release/sldo-{plan,run,research,install,tla-sha}`).
- **Install skill pack on this machine:**
  ```bash
  cargo build -p sldo-install --release
  ./target/release/sldo-install              # global: ~/.claude/skills/
  ./target/release/sldo-install --dry-run    # preview
  ./target/release/sldo-install uninstall    # reverse
  ```
- **Tauri (parked — do not touch per CLAUDE.md):** `make dev`, `make build`, `make test-frontend` (`npm test` in `crates/sldo-tauri/ui`); frontend setup is `cd crates/sldo-tauri/ui && npm install`.
- **Lint/format:** `cargo fmt` and `cargo clippy` are documented in `SECURITY.md` as ad-hoc/manual — no pre-commit hook is wired.
- **CI:** no `.github/workflows/` found at repo root via directory listing (not explicitly checked into the paths I walked).

## Existing Patterns

- **Error handling:** `anyhow::Result` on binary glue paths (`Context`/`bail!`), `thiserror 2` reserved for typed library errors. CLI error messages are concise; no stack traces in user-facing output.
- **CLI parsing:** every binary uses `clap 4` with `derive`, typed argument values (`PathBuf`, bounded integers, `String`). No manual `argv` parsing.
- **Subprocess invocation:** `std::process::Command::new("claude" | "git" | "cargo")` with explicit `.arg()` lists — never shell interpolation. `Stdio::piped()` with separate reader threads for stdout and stderr draining into a `std::sync::mpsc::channel` to avoid pipe-buffer deadlocks (see `sldo-common/src/copilot.rs`).
- **Module layout:** `lib.rs` re-exports submodules via `pub mod …`. Binaries mount local modules alongside `main.rs` (e.g. `sldo-research/src/{main,dossier,prompt,research}.rs`).
- **Logging:** append-only timestamped log files via `sldo_common::logging::{LogFile, ensure_log_dir}`; log dir is `.sldo-logs/` under the project (gitignored). No `tracing`/`log` crate in use.
- **Preflight gates:** every driver binary calls `sldo_common::preflight::check_git_safety` (refuses to run on `main`/`master`) and `check_claude_installed` (via `which`). Pattern is repeated across `sldo-plan`, `sldo-run`, `sldo-research`.
- **Tests:** Given/When/Then style comments inside unit tests (see `sldo-common/src/{git,preflight}.rs`). E2E tests per milestone are registered as separate `[[test]]` targets at the workspace root using `tempfile::TempDir` for isolation.
- **Async style:** synchronous `std::process` + threads in the CLIs; `tokio` "full" only inside parked `sldo-tauri`.
- **Skill pack convention:** each skill is `skills/<name>/SKILL.md` with YAML frontmatter (`name:`, `description:`) followed by body prose; optional `references/` or `examples/` subdir beside `SKILL.md`.
- **Runbook convention:** one file per feature at `docs/RUNBOOK-<KEBAB-SLUG>.md`, following `docs/runbook-template_v_3_template.md`. Milestone Tracker table parsed by `sldo_common::runbook::{MilestoneStatus, …}` (statuses: `not_started` / `in_progress` / `done`).
- **Reality-first architecture doc** (from MEMORY.md): `ARCHITECTURE.md` reflects code at HEAD; planned work lives in the runbook's "Target Architecture" section.

## Constraints

- **Test baseline restriction:** the `--workspace` cargo invocation is red on macOS arm64 due to the parked `sldo-tauri` crate (missing esbuild arm64 binary in its UI `node_modules/`). All test runs must use the explicit `-p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install` set until Tauri is un-parked.
- **Parked directory — no edits:** `crates/sldo-tauri/` is parked as of 2026-04. Do not modify it and do not merge its branches into skill-pack work (per `CLAUDE.md`).
- **Protected-branch admission gate:** `sldo-plan`, `sldo-run`, `sldo-research` refuse to execute on `main`/`master` via `preflight::check_git_safety`; development must happen on feature branches.
- **External binary required:** the `claude` CLI must be installed on `PATH` (`preflight::check_claude_installed`); each invocation consumes API credits (docs warn `--max-iterations` + `--max-searches` bound cost).
- **No license file:** no `LICENSE` file was observed at the repo root in the `ls` output (no LICENSE/COPYING — missing, stated explicitly). `SECURITY.md` notes the dep graph contains `MIT`, `Apache-2.0`, `BSD-3-Clause` licences but enforcement via `cargo deny` is not wired.
- **No rust-toolchain pin:** MSRV is implicit — "latest stable supporting `clap 4`, `thiserror 2`, `reqwest 0.11`" per `SECURITY.md`. No `rust-toolchain.toml` file observed.
- **Prompt-file trust boundary:** `sldo-research` invokes Claude Code with `WebFetch`/`WebSearch` enabled; prompt files are treated as trusted input. Passing hostile prompt files can cause data exfiltration (documented in `SECURITY.md` and README).
- **Gitignored output locations:** `output/`, `.sldo-logs/`, `.copilot-logs/`, `.claude/`, `.sldo/`, `target/`, `.env`, `docs/legal` are all gitignored — research scratch + logs may contain proprietary source excerpts.
- **TLA+ tool fetch constraints:** `sldo-tla-sha` enforces a host allow-list (pre- and post-redirect) and a 500 MiB streamed-response byte cap; every downloaded binary requires a pre-pinned SHA-256 in `tools.toml`.
- **Secrets handling:** `.env` (gitignored) supplies `OPENAI_API_KEY` read server-side by the Tauri backend only; never shipped to the frontend. No other project-level secrets.
- **Canonical planning artefact:** feature runbooks must live at `docs/RUNBOOK-<FEATURE>.md` following the v3 template; do not bypass `/slo-plan`'s interactive walk for batch CLI shortcuts (per `CLAUDE.md`).

## Executive Summary

`/slo-legal` v1 ships four UK templates (NDA, contractor SOW, IP assignment, T&Cs) plus a triage gate that hard-blocks `draft` for regulated matters, deal value > £5,000, counterparty-with-lawyer, and GDPR documents. Three findings reshape the wedge:

1. **The price wedge is not the wedge.** Cheapest credible substitute is **Sparqa Legal at £19.99/mo** or **Rocket Lawyer UK at £17.49/mo** — not the £5,700 Russell Cooke-equivalent ROI the idea doc anchored on. The defensible v1 positioning is *workflow integration inside the SLO loop + triage authority*, not raw price.
2. **The "hard-block ALL GDPR drafts" rule is conservative.** ICO enforcement against sub-£1M-turnover private companies in 2024–2026 is **PECR-direct-marketing-dominated**, not Article-13-privacy-notice-dominated. The hard-block is defensible on professional-negligence + upside-asymmetry grounds (one bad lawful-basis call can unwind a B2C launch; PECR ceiling rose to £17.5M under DUAA 2025), but the *enforcement-quantified* risk is narrower than the rule suggests.
3. **US/EU is a separate build per country.** SeedLegals took six months to localise France; Stripe Atlas and Clerky stayed Delaware-only by design. No surveyed prior art uses a "shared-prose + jurisdiction-flag" architecture. UK now, US/EU as a v2 architectural pivot — not a config matrix.

For SEIS/EIS (Q3) and IR35/employment (Q4), authoritative anchors exist (HMRC VCM34080 / VCM3000 / VCM31000 manual paths; ERA 1996 s86; CEST April-2025 refresh with documented MOO blind spot) and are sufficient for triage-gate wording. The DUAA 2025 commencement on 5 February 2026 (new 7th lawful basis, narrowed Article 22, PECR ceiling raised, complaints-procedure duty from 19 June 2026) is a moving baseline `/slo-legal` GDPR triage must reference.

Recommended structural pattern: shared `references/biz/` at repo root (outside `skills/` so `sldo-install` ignores it — confirmed by `crates/sldo-install/src/install.rs:44`), with a hybrid M1 footprint that ships only `triage-gate.md` + `cost-baseline-russell-cooke-2026.md` and defers the rest until `/slo-accounting` exists to cite them.

## Topic Decomposition

The brief's five questions decompose into the following sub-questions, in priority order for the wedge:

- **Q1 — Buy-vs-build for `/slo-legal`.**
  - Q1a: 2026 per-doc and per-month £ pricing for SeedLegals, Rocket Lawyer UK, Genie AI, Sparqa, Farillio, Lawpath UK.
  - Q1b: Which of NDA / contractor SOW / IP assignment / T&Cs each platform actually covers for UK-incorporated companies.
  - Q1c: Whether each platform bundles a qualified-solicitor review step or only template + optional escalation.
  - Q1d: US-side anchors (Stripe Atlas, Clerky) for completeness.
- **Q2 — UK GDPR enforcement reality for sub-seed companies.**
  - Q2a: ICO enforcement-action volume and £ split (PECR vs UK GDPR vs PECR-marketing) over the 2024-04 to 2026-04 window.
  - Q2b: Whether Article-13 / lawful-basis / privacy-notice failures specifically drive enforcement against private companies under £1M turnover or under 50 employees.
  - Q2c: Whether the proposed "hard-block draft for ALL GDPR documents" gate is justified by realistic risk or over-conservative.
  - Q2d: Moving-baseline factors (DUAA 2025, complaints-procedure duty, fine-ceiling changes).
- **Q3 — SEIS/EIS founder responsibility.**
  - Q3a: Advance Assurance process, lead time, cost.
  - Q3b: Top retroactive-disqualification triggers and HMRC manual citations (VCM31000 / VCM34000 / VCM3000).
  - Q3c: What `/slo-fundraise` triage gate must force *before* term-sheet drafting.
- **Q4 — UK employment minimums + IR35.**
  - Q4a: Statutory minimums for first hire (notice, holiday, SSP, NI/PAYE, right-to-work, pension auto-enrolment).
  - Q4b: IR35 / off-payroll-working status-determination factors at the level needed to triage contractor vs employee.
  - Q4c: CEST tool's authority and known limitations (post April 2025 refresh).
  - Q4d: What absolutely must hard-block to a lawyer + accountant.
- **Q5 — Jurisdiction expansion marginal cost.**
  - Q5a: Reusable share for US (Delaware C-corp, work-made-for-hire doctrine, consumer-rights regime).
  - Q5b: Reusable share for EU (UK GDPR derivative; per-state employment / consumer law).
  - Q5c: Prior art for multi-jurisdiction templating (Stripe Atlas, SeedLegals, Clerky).
  - Q5d: Architecture decision criterion — parallel `jurisdiction-<code>.md` matrix vs UK-now-v2-pivot.

Cross-cutting structural questions surfaced during research:
- **R1: How does `sldo-install` treat shared / underscore-prefixed directories under `skills/`?** Determines whether shared scaffolding can live inside `skills/` or must live outside it.
- **R8: Open-standard UK templates beyond oneNDA?** Affects which templates `/slo-legal` v1 cites versus drafts from scratch.

## Key Findings

### Q1 — UK legal-doc-as-a-service competitive landscape (2026 prices)

| Platform | UK plan | Price (2026) | v1 doc coverage | Lawyer review bundled? |
|---|---|---|---|---|
| **SeedLegals Access** | Pay-as-you-go templates | **£75/mo or £590/yr + VAT** | NDA, IP assignment, consultancy, employment, T&Cs via membership | "Unlimited help — no billable hours" reads as product support, not solicitor review |
| **SeedLegals Funding** | Start / Raise / Scale | **£1,490 / £2,790 / £4,990 per year + VAT** | Adds round tooling, EMI, R&D | Account manager + AI companion |
| **Rocket Lawyer UK Rocket Legal** | Monthly | **£34.99/mo** | NDA, independent-contractor, T&Cs, confidentiality | "Ask a lawyer" included; bespoke drafting from £90/hr member rate |
| **Rocket Lawyer UK Rocket Legal+** | Annual | **£17.49/mo billed £209.88/yr** (50% promo; £419.88 RRP) | Same as Rocket Legal | Lawyer Q&A included; document review from £120, drafting from £90/hr |
| **Genie AI Pro** | Monthly | **£30/mo, 10 docs/mo** (free tier 2 docs/mo) | 2,000+ templates incl. UK NDA, contractor (PSC), IP assignment | "Verified by 100+ UK lawyers" marketing; no per-doc solicitor review |
| **Sparqa Legal** | Monthly / annual | **£19.99 + VAT/mo or £199 + VAT/yr** for 370+ docs + 22+ toolkits | UK-jurisdiction business templates | No |
| **Farillio** | B2B2C only via Aviva, RSA, Direct Line, NatWest, Admiral | **No public direct price** | 2,000+ guides/templates incl. HR + contracts | No |
| **Lawpath UK** | Companies House 14400933 has active strike-off proposal late 2025; `lawpath.com` redirects to `lawpath.com.au` | **Effectively vapourware in UK** | — | — |

**Russell Cooke baseline** (idea doc, **not publicly retrievable** — see Risks): NDA £750, Standard Contractor Agreement £1,450, IP Assignment £750, T&Cs £2,750, GDPR basic £1,850, GDPR full £4,950, SHA cofounders £2,950, SHA SEIS £5,950, Articles £1,350. The v1 bundle ≈ £5,700 from a regulated solicitor.

**Wedge implication.** The cheapest credible template substitute is **£17.49–£19.99/mo**, not £5,700 once. `/slo-legal` cannot win on raw price; it must win on **integration with the SLO loop + triage authority** (the regulated/value/lawyer/GDPR hard-blocks).

### Q2 — ICO enforcement reality

- **2024 totals:** 62 enforcement actions against 47 organisations — 18 monetary fines, 29 reprimands, 15 enforcement notices. Total fines £2.7M; range £7,500–£750,000; average £153,722.
- **Sector skew:** of 32 UK GDPR cases, 27 went to the public sector and only 4 to private. Smallest 2024 fine (Central YMCA £7,500) was a national charity, not a sub-seed startup. Largest were public sector (PSNI £750K, MoD £350K).
- **PECR vs UK GDPR fine split (2024):** PECR ≈ £1.6M vs UK GDPR ≈ £1.1M.
- **2025 trajectory:** fewer fines, larger size — H1 2025 had 15 actions; total £ collected was 7× all of 2024; average fine rose to **>£2.8M**. Two-thirds were UK GDPR breaches (vs one-sixth in 2024).
- **Sub-£1M private-company exposure (Apr 2024 – Apr 2026):** monetary penalties on this segment are **PECR-direct-marketing-dominated** — AFK £90k, Skean Homes £100k, LADH £50k, Pinnacle Life £80k, Dr Telemarketing £100k, Poxell £150k. Standalone "bad privacy notice / Article 13" enforcement against small private companies in this window is **effectively zero**.
- **ICO audit themes (Aug 2023 – Jan 2024):** Article 13 transparency failures repeatedly flagged — fair processing notices judged insufficiently detailed on lawful bases, recipients, international transfers, retention. ICO state every business "however small" needs a privacy notice.
- **DUAA 2025 (commenced Stage 3 on 5 February 2026):** new 7th lawful basis ("Recognised Legitimate Interests"); legitimate-interest examples codified (direct marketing, intra-group admin, network security); Article 22 narrowed to special-category data; DSAR proportionality + "stop the clock" codified; **PECR fine ceiling raised £500k → £17.5M / 4% global turnover**; complaints-procedure duty for all controllers from **19 June 2026**.

**Q2c synthesis.** The "hard-block draft for ALL GDPR documents" rule is **defensible** on professional-negligence and upside-asymmetry grounds (max ceiling £17.5M, contractual/B2B-buyer DPA exposure, customer-trust risk) but is **conservative** versus the enforcement evidence, which shows PECR-direct-marketing as the real small-company enforcement pattern. A narrower posture (translate/triage for privacy notice; hard-block for direct-marketing/PECR contexts) is also defensible.

### Q3 — SEIS/EIS founder responsibility

**Top retroactive-disqualification triggers `/slo-fundraise` must surface:**

1. **Breach of control / independence.** Company must not be a 51% subsidiary nor under control of another company or connected persons. Authority: **HMRC VCM34080**.
2. **Disqualifying arrangements** (ITA07 s257HJ(1)) — any scheme/agreement that would breach independence at any point during period A. Authority: **VCM34080**.
3. **Preferential rights on share class.** Ordinary shares granted preferential rights (via articles or SHA drafting, even without reissue) lose SEIS/EIS. Anchor case law: *Abingdon Health Limited v HMRC* [2016] TC 05525; *Flix Innovations* line.
4. **Qualifying-trade drift into excluded activities.** Authority: **VCM3000 series**.
5. **Value extraction / non-independent transactions** to investors post-raise (loans, benefits, related-party).

**Advance Assurance lead time.** HMRC internal target ~15 working days; realistic 4–6 weeks; ~26% of applications exceeded 30 days in 2022-23. SeedLegals-prepared applications typically clear in 2–3 weeks after a 1–2 week pre-submission review. End-to-end planning figure: **3–5 weeks, 6+ if HMRC follow-up**. Practical floor: apply **6 weeks before term-sheet signature**. AA is bundled into SeedLegals **Plus plan £999/yr + VAT** (2026); £500 add-on if no prior AA, off-platform round, or nominee used.

**Triage actions before term-sheet drafting** (inferred from sources): "Have you applied for Advance Assurance?" (hard block); "Are you a subsidiary or controlled by another company per VCM34080?" (hard block to accountant if yes); "Have you audited qualifying-trade status against VCM3000?" (warn); "Are any share rights preferential vs ordinary?" (hard block to solicitor if yes).

### Q4 — UK employment first hire + IR35

**Statutory minimums:**

- **Notice (ERA 1996 s86):** employer gives 1 week if <2 yrs service, 1 week per completed year up to a max of **12 weeks** (at 12+ yrs); employee gives 1 week after 1 month continuous service; right triggers at 1-month qualifying service.
- **Holiday (WTR 1998):** 5.6 weeks paid leave / year = 28 days for full-time. From 2026, employers must keep records adequate to show compliance.
- **Pension auto-enrolment (Pensions Act 2008, 2026/27 thresholds):** earnings trigger £10,000/yr; lower qualifying earnings £6,240; upper £50,270 — held flat vs 2025/26.
- **Other (high confidence on requirement; current rates need annual refresh):** SSP £116.75/week (2024-25 baseline); NI/PAYE registration before first paycheque; right-to-work checks under IANA 2006 s15 (IDVT permitted for British/Irish passport holders).

**IR35 / CEST:**

- Three primary indicators: **right of substitution** (single strongest outside-IR35 factor), **mutuality of obligation (MOO)**, **control**.
- **April 2025 CEST refresh** (effective 30 April 2025): questionnaire restructured into ~6 sections, new gating question requires a contract to exist, MOO section added, substitution tightened — **the right must be unrestricted and genuinely exercisable** (recommending a replacement no longer counts). HMRC explicit: underlying technical principles unchanged.
- **CEST limitations:** ~20% of cases return "unable to determine" (34 of 72 outcome routes); the tool **does not weigh MOO** because HMRC's reading of *PGMOL v HMRC* [2024] UKSC 29 holds that MOO exists in any contract for services. CEST output is necessary but not sufficient. Bird & Bird, ContractorUK, ir35guide all warn against relying on CEST alone.
- **Small-company IR35 thresholds change 6 April 2026:** turnover £10.2M → **£15M**; balance sheet £5.1M → **£7.5M**; headcount 50 unchanged. Seed-stage clients remain "small" and continue to rely on contractor self-assessment, not engager-side determination.

**Hard-block-to-lawyer surface (Q4d, inference from above):** absent or sham substitution clause; contractor substantially full-time, exclusive, under direction; CEST "employed" or "unable to determine" result; engagement >6 months with rolling renewals; contractor uses engager equipment + attends engager premises.

### Q5 — Jurisdiction expansion marginal cost

- **Stripe Atlas** is single-jurisdiction by design — Delaware C-corps and Delaware LLCs only, even though founders from 140+ countries use it. Templates (bylaws, operating agreements, stock issuance, 83(b)) are Delaware-law products co-authored with Cooley LLP. Atlas's "international" story is *user nationality*, not *document jurisdiction*.
- **Clerky** is explicitly US-only — Delaware C-corp + CA/NY state registration; attorney-drafted (Orrick, Wilson Sonsini). Non-US users are accommodated only as non-resident owners of a US entity.
- **SeedLegals** runs true parallel per-jurisdiction stacks (UK, US, Ireland, France) with ~160 staff. The France build required **six months of prep** to internalise local rules (including BSPCE share-option regime). Governing-law default: English law unless company is in Ireland or France, in which case local law governs.
- **Anthony Rose (SeedLegals CEO):** "different jurisdictions require different legal infrastructure, but the principles remain the same" — i.e. triage logic and process scaffolding transfer; prose/clauses do not.
- **No surveyed prior art uses "shared-prose + jurisdiction-flag" architecture.** The market has converged on either (a) one-jurisdiction depth (Atlas, Clerky) or (b) parallel per-country modules (SeedLegals).

**US prose reuse:** US "work made for hire" doctrine (17 USC §101) covers nine specific commissioned-work categories plus employment; commissioned software is generally **not** a WMFH-eligible category unless the contractor is an employee. Best US practice mirrors UK: explicit present-tense assignment-of-rights clause. IP-assignment template prose is largely portable; what changes is governing-law/venue and the WMFH-recital paragraph. US T&Cs differ on consumer-rights regime (Magnuson-Moss vs CRA 2015), arbitration norms, choice-of-law/venue, DMCA-vs-EU-NTD safe harbour. Pure prose reuse: ~40–60%; structural reuse: high.

**EU prose reuse:** GDPR is shared (UK GDPR is a derivative); employment law is per-state with meaningful divergence on notice, probation, dismissal, working time. Templating "EU" as a single jurisdiction is a category error — natural seam is `eu-<de|fr|es|nl|...>.md`.

### R1 — `sldo-install` treatment of skills directories

From `crates/sldo-install/src/install.rs:44-71` (`discover_skills()`): iterates `<skills_dir>/*`, requires `<name>/SKILL.md` to exist, skips names starting with `.`. **Leading underscore is NOT filtered.** A `skills/_biz-shared/` directory will be discovered if and only if it contains a `SKILL.md`. Implication: shared-scaffold directories CAN live under `skills/` without being installed but only if they lack `SKILL.md` — putting shared refs at `references/biz/` (outside `skills/`) has zero installer interaction and matches the existing `skills/<name>/references/` pattern.

### R8 — Open-standard UK templates

- **oneNDA** (TLB consortium, 2021): UK-drafted, **CC BY-ND 4.0**, jurisdiction-neutral core with an England & Wales Country Schedule as canonical. Directly usable for UK startups.
- **oneSaaS** (UK SaaS T&Cs, 2023) and **oneDPA** (GDPR DPA, 2024) exist from the same consortium.
- **No CC-licensed UK-native template exists** for contractor SOW or IP assignment. Nearest adaptations: Simmonds Stewart / Kindrik Partners (NZ law) — ~80% structural overlap but governing-law and statutory references must be swapped for E&W.
- **CC BY-ND 4.0 forbids derivative works** — so a `/slo-legal draft nda` must render the unmodified template + separately-generated cover page / schedule, not edit oneNDA's text.

### Adjacent prior art

- **`ai-legal-claude`** ([GitHub](https://github.com/zubair-trabzada/ai-legal-claude)) — open-source Claude Code skill for contract review/NDA generation. Proves an in-Claude legal skill can be packaged; review-side rather than UK-triage-gate design.
- **Docassemble** — dominant open-source legal-doc automation (Python+YAML+Markdown+Docker); fits law firms / legal aid, not single-founder Claude skills. Useful only as inspiration for `draft`-mode question trees.
- **oneNDA, TechGeeta NDA generator, Sheetgo top-6 round-up, Spellbook, Sirion, Gavel, Concord, Lumin** — confirm the contract-automation category is crowded; no consolidation.

## Library & Tool Evaluations

| Candidate | Latest / state | License | Verdict for `/slo-legal` v1 |
|---|---|---|---|
| **oneNDA** | Jurisdiction-neutral template + UK Country Schedule (2021) | **CC BY-ND 4.0** | **Strong anchor** — cite as source-of-truth for the NDA template. ND clause means render the template unmodified + generate fields separately. |
| **oneSaaS, oneDPA** | Live 2025; TLB consortium | CC (same family) | Candidate anchors for M3+ T&Cs and GDPR DPA triage. Not needed for M1. |
| **Simmonds Stewart / Kindrik Partners** | CC-licensed NZ law | CC | Structural reference for contractor SOW + IP assignment; not canonical — E&W rewrite required. |
| **`python-docx`** | 1.2.0 (2025-06-16) | MIT | Reject for v1. Markdown-in-repo is the SLO convention; DOCX export is not on the critical path. |
| **`docxtpl`** | 0.20.2 (2025-11-13) | LGPL-3.0 | Reject for v1. Adds Python runtime + LGPL taint to a Rust workspace. |
| **Docassemble** | Active 2025; security checklist refreshed 2025-12 | MIT | Reject. Heavyweight Python+YAML+Docker; wrong fit for a Markdown-based Claude skill. Inspiration only. |
| **HMRC VCM Manual** (VCM34080, VCM3000, VCM31000 series) | Evolving Crown copyright; `updates` page tracked | Crown copyright (reusable under OGL where stated) | **Strong reference** — cite by URL with retrieval-date stamp. |
| **ICO public guidance + DUAA pages** | Active; DUAA Stage 3 commenced 5 Feb 2026 | OGL v3 | **Strong reference.** |
| **legislation.gov.uk** (ERA 1996 s86, WTR 1998, Pensions Act 2008, DUAA 2025) | Authoritative | OGL v3 | **Strong reference.** |
| **gov.uk CEST tool page** | April 2025 refresh | OGL v3 | **Strong reference** for IR35 triage. |
| **Apify UK-ICO-Enforcement-Actions dataset** | Paid actor (~$40/mo) | Proprietary | Reject for v1 — no turnover/headcount fields; not worth the cost vs periodic manual refresh. |
| **`ai-legal-claude`** (GitHub `zubair-trabzada/ai-legal-claude`) | Active community Claude skill | n/a | Inspiration / prior-art check; not a dependency. |

**Net.** `/slo-legal` v1 needs no runtime dependency. The library-shaped inputs are authoritative reference URLs (HMRC VCM, ICO, legislation.gov.uk, gov.uk CEST, oneNDA) snapshotted into `references/biz/` with explicit retrieval dates.

Skill-pack runtime infrastructure that already exists in this repo (`sldo-common`, `sldo-install`, `sldo-research`) is the substrate; no new crate is needed for the biz pack.

## Architecture Options

All assume the existing skill pattern: `skills/<name>/SKILL.md` + optional `references/` + optional `personas/` (precedent: `skills/slo-critique/`, `skills/slo-plan/`).

### Option A — `references/biz/` at repo root

```
skills/slo-legal/SKILL.md
skills/slo-legal/templates/{nda.md,contractor-sow.md,ip-assignment.md,terms-and-conditions.md}
skills/slo-accounting/SKILL.md
skills/slo-equity/SKILL.md
skills/slo-fundraise/SKILL.md
references/biz/
  triage-gate.md                 # single source of truth for the 4 gates
  jurisdiction-uk.md             # UK-only prose / anchors
  jurisdiction-us.md             # stub: "not supported" error surface
  jurisdiction-eu.md             # stub
  cost-baseline-russell-cooke-2026.md   # explicit provenance line required
  artifact-schema.md             # docs/biz/ frontmatter contract
  hmrc-vcm-index.md              # VCM34080, VCM3000, VCM31000 + retrieval date
  ico-duaa-index.md              # DUAA Stage 3 Feb 2026 anchor + complaints duty
  ir35-cest-factors.md           # MOO limitation + PGMOL note + 3-factor list
  ico-enforcement-reality.md     # PECR-not-Article-13 small-co pattern
  open-template-anchors.md       # oneNDA + oneSaaS + Kindrik notes
```

Trade-offs: lives outside `skills/` so installer never sees it (per `install.rs:44`). Matches the per-skill `references/` precedent at the package level. Forces advisor skills to cite one file per gate (discipline, not enforcement). Prose duplication risk if a skill author inlines instead of linking.

### Option B — `skills/_biz-shared/` as a sibling skill without `SKILL.md`

Because `install.rs` skips directories without `SKILL.md`, `skills/_biz-shared/` would be silently ignored by the installer. Content lives co-located with the skills that cite it.

Trade-offs: novel convention in this repo; leading-underscore convention is undocumented and untested. `sldo-install --dry-run` gives no signal that `_biz-shared/` is meant to be shared. Adding a `SKILL.md` later would accidentally publish it. Option A avoids this for free.

### Option C — Per-skill self-contained

`skills/slo-legal/SKILL.md` inlines triage + jurisdiction + cost table; each advisor skill owns its copy.

Trade-offs: fastest M1 ship. With four advisor skills sharing the same triage criteria (regulated / value / lawyer / GDPR), drift is the predictable failure mode — and the R3 finding (PECR vs Article 13) would have to be patched in four places. Best fit only if pack stays at 1–2 advisor skills.

### Option D — Hybrid: Option A skeleton, M1 ships only 2 shared refs

Ship M1 with only `references/biz/triage-gate.md` + `references/biz/cost-baseline-russell-cooke-2026.md`. Other shared files land in M2 when `/slo-accounting` exists to cite them.

Trade-offs: respects project CLAUDE.md preamble ("three similar lines is better than a premature abstraction"). Establishes one-source-of-truth discipline for the drift-prone surface (the triage gate). Defers schema-design work on the jurisdiction matrix and HMRC/ICO indexes until a second consumer exists.

## API & SDK Documentation

This research slice is mostly non-code: UK legal/tax rules, ICO enforcement, competitor SaaS platforms, and one Rust source-of-truth check (`crates/sldo-install/src/install.rs:44-71`, `discover_skills()`). No third-party API or SDK requires documentation here for `/slo-legal` v1 — the skill is prompt-shaped and consumes reference URLs rather than calling APIs.

The closest version-like facts surfaced for situational awareness:

| Item | Current state | Date | Note |
|---|---|---|---|
| UK Data (Use and Access) Act (DUAA) | Royal Assent | 19 June 2025 | Stage 3 commenced 5 Feb 2026; complaints-procedure duty from 19 June 2026; PECR ceiling £17.5M / 4% global turnover |
| HMRC CEST tool | Functional refresh | 30 April 2025 | Substitution tightened (must be unrestricted + genuinely exercisable); MOO section added but not weighted; engine unchanged |
| IR35 small-company thresholds | New thresholds | Effective 6 April 2026 | £15M turnover / £7.5M balance sheet / 50 headcount |
| HMRC Venture Capital Schemes Manual | Continuously updated | — | `updates` page is the change-log; cite anchors VCM34080, VCM3000, VCM31000 with retrieval dates |
| oneNDA | UK-drafted; jurisdiction-neutral + E&W Country Schedule | 2021 | CC BY-ND 4.0 — render unmodified |
| `python-docx` | 1.2.0 | 2025-06-16 | MIT; not adopted for v1 |
| `docxtpl` | 0.20.2 | 2025-11-13 | LGPL-3.0; not adopted for v1 |

## Design Recommendations

1. **Position `/slo-legal` v1 on workflow integration, not price (confidence: high).** Cheapest credible substitute is **£17.49–£19.99/mo** (Rocket Lawyer UK Rocket Legal+ / Sparqa Legal), not £5,700 once. Russell Cooke's £5,700 figure remains a valid *opportunity-cost* anchor for ROI claims provenance permitting (see Risks #2), but the wedge is "triage gate inside the SLO loop + repo-citable provenance," not "cheaper than templates."

2. **Use `references/biz/` at repo root (Option A) as the structural pattern (confidence: high).** Confirmed by `crates/sldo-install/src/install.rs:44-71` that shared scaffolding outside `skills/` has zero installer interaction. Matches the `skills/<name>/references/` precedent at the package level and keeps drift contained for the four advisor skills sharing identical triage criteria.

3. **For the M1 ship, follow the hybrid footprint (Option D) (confidence: medium).** Ship only `references/biz/triage-gate.md` + `references/biz/cost-baseline-russell-cooke-2026.md` initially; defer the rest until `/slo-accounting` exists to cite them. Respects CLAUDE.md's "three similar lines is better than a premature abstraction" rule. Promote to full Option A as soon as the second advisor skill arrives.

4. **Anchor the NDA template on oneNDA, render unmodified (confidence: high).** oneNDA is UK-drafted, lawyer-reviewed, CC BY-ND 4.0 with an England & Wales Country Schedule. The ND clause requires the template be rendered unmodified — `/slo-legal draft nda` must emit fields/cover page separately, not edit oneNDA prose. This both reduces liability and gives `/slo-legal` a citable source-of-truth.

5. **Draft contractor SOW, IP assignment, and T&Cs from scratch or from Kindrik/Simmonds Stewart adapted to E&W (confidence: medium).** No CC-licensed UK-native equivalent exists. Kindrik gives ~80% structural overlap but governing-law and statutory references must be swapped. For T&Cs in M3+, evaluate oneSaaS as a candidate anchor.

6. **Treat US/EU as v2 build, not v1 config-flip (confidence: high).** SeedLegals took 6 months to localise France; Stripe Atlas and Clerky stayed Delaware-only by design; no surveyed prior art uses shared-prose-with-jurisdiction-flag. Ship UK-only; if a `--jurisdiction` flag is added at all, surface a clear "UK only in v1" error for `us`/`eu` rather than implying false portability.

7. **`/slo-fundraise` triage must hard-gate on Advance Assurance, control/independence, share-class preferential rights, and qualifying-trade drift before any term-sheet drafting (confidence: high).** Authority anchors: HMRC VCM34080 (control/independence + disqualifying arrangements), VCM3000 (excluded activities), VCM31000 (SEIS income tax relief). Practical AA lead time: apply ≥6 weeks before term-sheet signature.

8. **`/slo-hire` + `/slo-legal` IR35 triage must hard-block to lawyer when CEST returns "unable to determine," substitution is conditional/restricted, or contractor is substantially full-time/exclusive/integrated (confidence: high).** CEST alone is insufficient — ~20% indeterminate rate + documented MOO blind spot from HMRC's reading of *PGMOL v HMRC* [2024] UKSC 29. Cite Bird & Bird "Spot the Difference" April 2025 review as evidence.

9. **GDPR posture: "translate / triage only, never draft" for privacy notice / ROPA / DPA / internal data-protection policies (confidence: medium).** Defensible on professional-negligence and upside-asymmetry grounds (DUAA raised PECR ceiling to £17.5M; complaints-procedure duty from 19 June 2026; B2B-buyer DPA exposure). The enforcement evidence shows PECR-direct-marketing rather than Article-13 as the small-company risk pattern, so a narrower rule is also defensible — surface as an explicit founder + Sherif decision before the SKILL.md text is frozen (Risks #1).

10. **For the cost-baseline reference file, record explicit provenance (confidence: high).** Russell Cooke 2026-27 price list is **not publicly retrievable** — no PDF or page on russell-cooke.co.uk matches. `references/biz/cost-baseline-russell-cooke-2026.md` must include "retrieved directly from firm 2026-MM-DD, PDF held at <path>" or swap to a publicly-itemised baseline (e.g. JPP Law fixed-fee startup page) so the ROI claim is auditable.

## Risks & Open Questions

Ordered by impact on the M1 ship.

1. **Is "hard-block draft for ALL GDPR documents" the right calibration?** Two defensible positions: (a) keep the broad rule on professional-negligence + upside-asymmetry grounds; (b) narrow it to PECR/direct-marketing contexts where the enforcement evidence actually clusters. Needs an explicit founder + Sherif decision before `/slo-legal` triage wording is frozen. **Highest pre-ship blocker.**

2. **Russell Cooke 2026-27 price list provenance.** The list is not publicly retrievable. Either provide the private PDF + retrieval date for `references/biz/cost-baseline-russell-cooke-2026.md`, or swap to a publicly-itemised baseline (JPP Law) so the ROI claim is auditable.

3. **oneNDA ND license implications for `draft` mode.** CC BY-ND 4.0 forbids derivative works; a `/slo-legal draft nda` that emits modified oneNDA text may breach the licence. Safe pattern: render unmodified template + separately-generated cover page / schedule. Needs an explicit SKILL.md rule and possibly a `/slo-verify` test.

4. **DUAA 2025 detail for `/slo-legal` GDPR triage prose.** Stage 3 commenced 5 February 2026; complaints-procedure duty from 19 June 2026 applies to all controllers regardless of size; new 7th lawful basis ("Recognised Legitimate Interests"); Article 22 narrowed to special-category data; PECR ceiling £17.5M. Triage wording must reflect these or warn that triage is operating against pre-DUAA assumptions.

5. **Equivalent of oneNDA for UK contractor SOW / IP assignment / T&Cs as of 2026?** No CC-licensed UK-native template surfaced for these three. Worth one targeted search against Practical Law free precedents, IPO model clauses, and Law Society / techUK model templates before drafting from scratch.

6. **Article-13 / lawful-basis enforcement against private companies under £1M turnover or under 50 employees: granular split.** ICO does not publish a size-segmented register; the Apify UK-ICO-Enforcement-Actions dataset has no turnover/headcount fields. The "effectively zero" finding is best-effort manual analysis of the public register, not a published statistic.

7. **Farillio and Sparqa £ pricing:** Sparqa pricing now resolved (£19.99 + VAT/mo or £199 + VAT/yr). Farillio has gone B2B2C-only via Aviva, RSA, Direct Line, NatWest, Admiral; no public direct price. Farillio should be removed from the competitive set unless a direct-to-startup channel resurfaces.

8. **Lawpath UK status.** Companies House 14400933 has an active proposal to strike off late 2025; `lawpath.com` redirects to `lawpath.com.au`. Treat Lawpath UK as vapourware in the 2026 competitive set.

9. **HMRC CEST April 2025 refresh's effect on triage-gate question wording.** Substitution is now stricter ("unrestricted and genuinely exercisable"). Needs one re-read of the Bird & Bird "Spot the Difference" analysis against the current CEST UI before the IR35 triage prose is frozen.

10. **SEIS/EIS Advance Assurance lead time signal.** Resolved: HMRC internal target ~15 working days; realistic 4–6 weeks; ~26% of applications exceeded 30 days in 2022-23; SeedLegals end-to-end ~3–5 weeks. `/slo-fundraise` should tell founders to apply ≥6 weeks before term-sheet signature.

11. **Open standards for shareholders agreement / EMI option scheme.** Out of scope for `/slo-legal` v1 but needed by `/slo-equity` and `/slo-fundraise`. oneNDA consortium has not published these; SeedLegals templates are proprietary. Needs a dedicated research slice before Runbook A M3.

12. **`--jurisdiction us` / `--jurisdiction eu` stubs vs no flag at all.** Q5 evidence says US/EU is a per-country build. An error surface that says "UK only in v1" is more honest than a `--jurisdiction` flag that implies false portability. Defer to v2 architectural pivot.

13. **SSP and pension-threshold annual refresh.** SSP rate £116.75/week (2024-25 baseline) and pension auto-enrolment thresholds are uprated annually each April. `/slo-hire` reference data needs a refresh cadence — per the project's reality-first convention, prefer to cite gov.uk URLs with retrieval-date stamps over copy-pasted figures.

## References

- [About us — our story and leaders | SeedLegals](https://seedlegals.com/about/)
- [AI Legal Document Generators Explained 2025 — Docupilot](https://www.docupilot.com/blog/ai-legal-document-generator)
- [Apify — UK ICO Enforcement Actions dataset](https://apify.com/spookyweb/uk-ico-enforcement-actions)
- [Artificial Lawyer — Lawpath Bags $10m, Advances Legal AI for SMBs](https://www.artificiallawyer.com/2025/02/04/lawpath-bags-10m-advances-legal-ai-solution-for-smbs/)
- [BDO — Information Commissioner's Office (ICO) Enforcement Trends 2025](https://www.bdo.co.uk/en-gb/insights/advisory/risk-and-advisory-services/trends-in-recent-ico-enforcement-action)
- [Bird & Bird — ICO enforcement consultation (2025)](https://www.twobirds.com/en/insights/2025/ico-enforcement-consultation-understand-the-guidance)
- [Bird & Bird — Spot the Difference: A Closer Look at HMRC's Updated CEST Tool (2025)](https://www.twobirds.com/en/insights/2025/uk/spot-the-difference-a-closer-look-at-hmrcs-updated-cest-tool)
- [Burges Salmon — ICO's draft enforcement guidance](https://www.burges-salmon.com/articles/102lwfq/icos-draft-enforcement-guidance-what-does-the-guidance-tell-us-about-the-icos/)
- [Capterra — SeedLegals Software Pricing 2026](https://www.capterra.com/p/172337/SeedLegals/)
- [Capterra UK — SeedLegals 2026](https://www.capterra.co.uk/software/172337/seedlegals)
- [Carta UK — SEIS/EIS Advance Assurance](https://carta.com/uk/en/learn/startups/fundraising/seis-eis-advance-assurance/)
- [CB Insights — SeedLegals company profile](https://www.cbinsights.com/company/seedlegals)
- [Clerky homepage](https://www.clerky.com/)
- [Clerky — Delaware Corporation Incorporation](https://www.clerky.com/startups/delaware-corporation-incorporation)
- [Clerky Help — What if a founder is located outside the US?](https://help.clerky.com/article/2819-founders-located-outside-us)
- [Clerky Legal Concepts for Founders — Core Concepts](https://handbooks.clerky.com/legal-concepts/core)
- [Clerky Pricing](https://www.clerky.com/pricing)
- [Clerky — Products for Startups](https://www.clerky.com/formation)
- [Clifford Chance — Key aspects of the Data (Use and Access) Act take effect (Feb 2026)](https://www.cliffordchance.com/insights/resources/blogs/talking-tech/en/articles/2026/02/key-aspects-of-the-data--use-and-access--act-take-effect.html)
- [Companies House — LAW PATH LTD 14400933](https://find-and-update.company-information.service.gov.uk/company/14400933)
- [Concord — Legal Contract AI](https://www.concord.app/legal-contract-ai/)
- [Contract Eye — IR35 / CEST limitations](https://www.contracteye.co.uk/ir35-what-is-hmrcs-cest-tool-what-are-its-limitations.shtml)
- [ContractorUK — IR35 mutuality of obligation 2026/27 explainer](https://www.contractoruk.com/ir35furtherreading/ir35-mutuality-obligation-202627-explainer-contractors)
- [Crabroom — Clerky vs Stripe Atlas](https://crabroom.com/blog/stripe/clerky-vs-stripe-atlas)
- [Data Protection Report — UK data protection reform (DUAA, July 2025)](https://www.dataprotectionreport.com/2025/07/uk-data-protection-reform-what-you-need-to-know-and-do/)
- [Docassemble (project home)](https://docassemble.org/)
- [Docassemble Alternatives — Knackly](https://knackly.io/docassemble-alternatives/)
- [Docassemble — Legaltech Hub vendor profile](https://www.legaltechnologyhub.com/vendors/docassemble/)
- [Docassemble — Open Source Legal](https://opensource.legal/projects/docassemble)
- [Docassemble Security Checklist for US Law Firms & Legal Aid (2025-12)](https://docassembledevelopment.com/2025/12/29/docassemble-security-checklist-us-law-firms/)
- [Docassemble vs Proprietary Legal Automation: Cost & Compliance (2025-12)](https://docassembledevelopment.com/2025/12/10/docassemble-vs-proprietary-legal-automation-tools-cost-control-compliance/)
- [docxtpl — docs](https://docxtpl.readthedocs.io/)
- [docxtpl on PyPI](https://pypi.org/project/docxtpl/)
- [docxtpl on Snyk Advisor](https://snyk.io/advisor/python/docxtpl)
- [Flowjam — Stripe Atlas vs Clerky](https://www.flowjam.com/blog/stripe-atlas-vs-clerky-which-is-better-for-your-startup)
- [Form a U.S. C Corporation or LLC with Stripe Atlas — Stripe Help & Support](https://support.stripe.com/questions/form-a-u-s-c-corporation-or-a-limited-liability-company-with-stripe-atlas)
- [Gavel — AI Contract Drafting & Automation](https://www.gavel.io/)
- [Gavel — Getting Started: Docassemble Developers](https://www.gavel.io/resources/docassemble)
- [Genie AI — homepage](https://www.genieai.co/en-us)
- [Genie AI — Legal AI for Contract Drafting](https://www.genieai.co/use-case/contract-drafting)
- [Genie AI — Legal AI for Startups](https://www.genieai.co/en-us/legal-ai-for/startup)
- [Genie AI — NDA For Consultant/Contractor Template UK](https://www.genieai.co/en-gb/templates/nda-for-consultant-contractor)
- [Genie AI — Pricing](https://www.genieai.co/pricing)
- [Genie AI — Product Development Agreement (England & Wales)](https://www.genieai.co/en-gb/template/product-development-agreement)
- [Genie AI — UK Contractor Agreement Templates](https://www.genieai.co/en-gb/template-type/contractor-agreement)
- [Genie AI — "Use Genie AI Legal Contract Templates"](https://www.genieai.co/en-us/company/genie-ai)
- [Genie AI vs Rocket Lawyer comparison](https://www.genieai.co/en-us/comparisons/genie-ai-vs-rocket-lawyer)
- [GetApp — Genie AI 2026 Pricing & Reviews](https://www.getapp.com/operations-management-software/a/genie-ai/)
- [GetApp — SeedLegals 2026 Pricing & Reviews](https://www.getapp.com/legal-law-software/a/seedlegals/)
- [GetApp UK 2026 — SeedLegals reviews](https://www.getapp.co.uk/software/2062471/seedlegals)
- [GitHub — ai-legal-claude (prior-art Claude skill)](https://github.com/zubair-trabzada/ai-legal-claude)
- [GOV.UK — Apply to use the Enterprise Investment Scheme](https://www.gov.uk/guidance/venture-capital-schemes-apply-for-the-enterprise-investment-scheme)
- [GOV.UK — Check employment status for tax (CEST)](https://www.gov.uk/guidance/check-employment-status-for-tax)
- [GOV.UK — Holiday pay and entitlement reforms (2024)](https://www.gov.uk/government/publications/simplifying-holiday-entitlement-and-holiday-pay-calculations/holiday-pay-and-entitlement-reforms-from-1-january-2024)
- [GOV.UK — Off-payroll working rules (IR35) Flowchart for contractors (PDF)](https://assets.publishing.service.gov.uk/media/608a6ee8d3bf7f01343a07cc/Off-payroll_working_rules__IR35__Flowchart_for_contractors__print_.pdf)
- [GOV.UK — Review of automatic enrolment thresholds 2026/27](https://www.gov.uk/government/publications/review-of-the-automatic-enrolment-earnings-trigger-and-qualifying-earnings-band-for-202627/review-of-the-automatic-enrolment-earnings-trigger-and-qualifying-earnings-band-for-202627)
- [GOV.UK — Understanding off-payroll working (IR35)](https://www.gov.uk/guidance/understanding-off-payroll-working-ir35)
- [GOV.UK — VCM3000: Excluded activities: contents](https://www.gov.uk/hmrc-internal-manuals/venture-capital-schemes-manual/vcm3000)
- [GOV.UK — VCM31000: SEIS income tax relief: contents](https://www.gov.uk/hmrc-internal-manuals/venture-capital-schemes-manual/vcm31000)
- [GOV.UK — VCM34080](https://www.gov.uk/hmrc-internal-manuals/venture-capital-schemes-manual/vcm34080)
- [GOV.UK — Venture Capital Schemes Manual (index)](https://www.gov.uk/hmrc-internal-manuals/venture-capital-schemes-manual)
- [GOV.UK — Venture Capital Schemes Manual: Updates](https://www.gov.uk/hmrc-internal-manuals/venture-capital-schemes-manual/updates)
- [GOV.UK — Venture-capital-schemes-apply-for-advance-assurance](https://www.gov.uk/guidance/venture-capital-schemes-apply-for-advance-assurance)
- [Greenberg Traurig — Threshold Changes to UK Off-Payroll Working Rules (IR35), March 2026](https://www.gtlaw.com/en/insights/2026/3/threshold-changes-to-uk-off-payroll-working-rules-ir35-end-user-and-contractor-considerations)
- [Headwest Guide — Stripe Atlas review (2025)](https://www.headwestguide.com/tools/stripe-atlas)
- [Hogan Lovells — UK Data (Use and Access) Act 2025: data protection provisions](https://www.hoganlovells.com/en/publications/uks-data-use-and-access-act-2025-data-protection-provisions-come-into-force)
- [ICAEW — Advising on EIS/VCT](https://www.icaew.com/technical/practice-resources/support-for-business-advisers/deliver-business-advice/advising-on-the-enterprise-investment-venture-capital-schemes)
- [ICO — Cookies and privacy notices in detail](https://ico.org.uk/for-organisations/advice-for-small-organisations/privacy-notices-and-cookies/cookies-and-privacy-notices-in-detail/)
- [ICO — DUAA 2025 summary of changes](https://ico.org.uk/about-the-ico/what-we-do/legislation-we-cover/data-use-and-access-act-2025/the-data-use-and-access-act-2025-duaa-summary-of-the-changes/)
- [ICO — Enforcement (direct marketing / PECR)](https://ico.org.uk/for-organisations/direct-marketing-and-privacy-and-electronic-communications/direct-marketing-guidance/enforcement/)
- [ICO — Enforcement action (hub)](https://ico.org.uk/action-weve-taken/enforcement/)
- [ICO — Enforcement of the data-sharing code](https://ico.org.uk/for-organisations/uk-gdpr-guidance-and-resources/data-sharing/data-sharing-a-code-of-practice/enforcement-of-this-code/)
- [ICO — Right to be informed](https://ico.org.uk/for-organisations/uk-gdpr-guidance-and-resources/individual-rights/individual-rights/right-to-be-informed/)
- [ICO — What privacy information should we provide?](https://ico.org.uk/for-organisations/uk-gdpr-guidance-and-resources/individual-rights/the-right-to-be-informed/what-privacy-information-should-we-provide/)
- [ICO homepage](https://ico.org.uk/)
- [Index Ventures — SeedLegals portfolio page](https://www.indexventures.com/companies/seedlegals/)
- [ir35guide.co.uk — HMRC CEST tool problems](https://ir35guide.co.uk/blog/hmrc-cest-tool-problems.html)
- [IT Contracting — Mutuality of obligation](https://www.itcontracting.com/ir35-status-what-is-the-mutuality-of-obligation-moo/)
- [JoinSecret — 18 best alternatives to Stripe Atlas (April 2026)](https://www.joinsecret.com/stripe-atlas/alternatives)
- [Jonathan Lea Network — Common SEIS and EIS Mistakes](https://www.jonathanlea.net/blog/common-seis-and-eis-mistakes/)
- [Jonathan Lea Network — How long does SEIS/EIS Advance Assurance take](https://www.jonathanlea.net/blog/how-long-does-it-take-to-get-seis-and-eis-advance-assurance/)
- [JPP Law — Fixed-fee startup services](https://www.jpplaw.co.uk/sectors/fixed-fee-startup/)
- [Kingsbridge — HMRC CEST update April 2025 review](https://www.kingsbridge.co.uk/blog/contractors/ir35/hmrc-cest-update-april-2025-review/)
- [LawNext Directory — Contract Automation & Drafting](https://directory.lawnext.com/categories/contract-automation-and-drafting-through-signature/)
- [Legalnodes — Delaware Incorporation founders guide (2025)](https://www.legalnodes.com/article/delaware-incorporation-founders-guide)
- [legislation.gov.uk — DUAA 2025](https://www.legislation.gov.uk/ukpga/2025/18)
- [legislation.gov.uk — Employment Rights Act 1996 s86](https://www.legislation.gov.uk/ukpga/1996/18/section/86)
- [Lewis Silkin — ICO fines 2024 round-up](https://www.lewissilkin.com/en/insights/2024/05/30/the-ico-fines-again-a-round-up-of-fines-issued-by-the-ico-in-2024)
- [LexisNexis — Employment Rights Act 1996 s86](https://www.lexisnexis.co.uk/legal/legislation/uk-parliament-acts/employment-rights-act-1996-c18/part-ix/section-86)
- [Lexology — ICO enforcement actions: trends and insights](https://www.lexology.com/library/detail.aspx?g=878e17b4-03b8-4d88-9791-47d1b87b421b)
- [LexRatio — Docassemble overview](https://lexratio.eu/2024/10/08/docassemble-legal-document-automation/)
- [Lumin — Free AI NDA Generator](https://www.luminpdf.com/generate/nda-generator)
- [Mayer Brown — UK GDPR-compliant privacy notice lessons from ICO](https://www.mayerbrown.com/en/insights/publications/2023/06/what-does-a-uk-gdpr-compliant-privacy-notice-look-like-lessons-learned-from-a-recent-ico-enforcement-decision)
- [Measured Collective — ICO Enforcement in 2025: Record Fines and What They Mean](https://measuredcollective.com/ico-enforcement-in-2025-record-fines-and-what-they-mean/)
- [ML Hive — Mastering Dynamic Word Document Generation with Python and docxtpl (2025)](https://mlhive.com/2025/12/mastering-dynamic-word-document-generation-python-docxtpl)
- [OffshoreCorpTalk — Stripe Atlas competitors & alternatives 2025](https://www.offshorecorptalk.com/threads/stripe-atlas-competitors-alternatives-in-2025-the-ultimate-guide.49263/)
- [oneNDA — open-standard NDA](https://www.onenda.org/)
- [oneNDA homepage (alt)](https://onenda.org)
- [Orrick Legal Guide for Stripe Atlas (PDF)](https://stripe.com/files/atlas/orrick-legal-guide.pdf)
- [Pinsent Masons — HMRC releases CEST 2.0 IR35 status determination tool](https://www.pinsentmasons.com/en-gb/out-law/news/hmrc-releases-cest-20ir35-status-determination-tool)
- [PublicTechnology — ICO to continue eschewing fines for public-sector entities](https://www.publictechnology.net/2024/12/09/education-and-skills/ico-to-continue-eschewing-fines-for-public-sector-entities/)
- [python-docx on PyPI](https://pypi.org/project/python-docx/)
- [Rocket Lawyer UK Pricing](https://www.rocketlawyer.com/gb/en/pricing)
- [RossMartin — EIS case: Rights changes disqualify shares](https://www.rossmartin.co.uk/companies/seis-eis/1881-eis-rights-changes-disqualify-shares)
- [RossMartin — EIS Preferential rights acquired](https://www.rossmartin.co.uk/companies/seis-eis/2501-eis-preferential-rights-acquired)
- [RossMartin — EIS: no relief for shares with preferential rights](https://www.rossmartin.co.uk/sme-tax-news/4400-eis-no-relief-for-shares-with-preferential-rights)
- [SeedLegals — 10 essential legal documents for UK startups](https://seedlegals.com/resources/legal-documents-for-uk-startups/)
- [SeedLegals — Consultancy Agreement](https://seedlegals.com/start/team-agreements/consultancy-agreement/)
- [SeedLegals — Employment Contracts UK](https://seedlegals.com/start/team-agreements/employment-agreement/)
- [SeedLegals Help — How long do HMRC take to approve SEIS/EIS applications](https://help.seedlegals.com/en/2137054-how-long-do-hmrc-take-to-approve-seis-eis-applications)
- [SeedLegals Help — Supported countries & jurisdictions](https://help.seedlegals.com/en/3556541-seedlegals-supported-countries-jurisdictions)
- [SeedLegals Help — Which contracts come with my Membership](https://help.seedlegals.com/en/which-contracts-policies-and-agreements-come-with-my-seedlegals-membership)
- [SeedLegals — IP Assignment](https://seedlegals.com/start/team-agreements/ip-assignment/)
- [SeedLegals — Legal Documents Hub](https://seedlegals.com/legal-documents/)
- [SeedLegals — Membership contracts, policies and agreements](https://seedlegals.com/resources/seedlegals-membership-contracts/)
- [SeedLegals — NDA](https://seedlegals.com/start/team-agreements/nda/)
- [SeedLegals — Pricing](https://seedlegals.com/pricing/)
- [SeedLegals — SEIS & EIS Tax Relief: 8 Things You Didn't Know](https://seedlegals.com/resources/seis-eis-tax-relief-facts/)
- [SeedLegals — SME Business Review profile](https://smebusinessreview.com/public/profiles/profile/seedlegals-simplifies-fundraising-and-legal-work-for-startups-across-the-u.k.-anthony-rose-founder-&-ceo-seedlegals)
- [SeedLegals — Disrupting the Legal Dimension… Ireland Expansion (Startup Network Europe)](https://startupnetwork.eu/startup-stories/seedlegals-disrupting-legal-dimension-of-startup-fundraising-ecosystem-ireland-expansion/)
- [SeedLegals US homepage](https://seedlegals.com/us/)
- [SeedLegals US pricing](https://seedlegals.com/us/pricing/)
- [Sheetgo — Top 6 NDA Generator Tools for 2025](https://www.sheetgo.com/blog/business-processes/top-6-nda-generator-tools/)
- [Sifted — UK's SeedLegals launches in France](https://sifted.eu/articles/seedlegals-france-launch)
- [Sirion — 8 Best Contract Drafting Software](https://www.sirion.ai/library/clm-platform/best-contract-drafting-software/)
- [Skala — Skala vs Stripe Atlas, Clerky, Doola](https://www.skala.io/blog/skala-vs-stripe-atlas-clerky-doola-etc)
- [Skillcast — 20 Biggest GDPR Fines 2018–2025](https://www.skillcast.com/blog/20-biggest-gdpr-fines)
- [Slashdot — Clerky vs StartGlobal vs Stripe Atlas (2026)](https://slashdot.org/software/comparison/Clerky-vs-StartGlobal-vs-Stripe-Atlas/)
- [SMB Guide — 7 best Clerky alternatives (2026)](https://www.smbguide.com/clerky-alternatives/)
- [Sparqa Legal homepage](https://www.sparqa.com/)
- [Spellbook — 6 Best Contract Automation Software for Law Firms](https://www.spellbook.legal/learn/law-firm-contract-automation-software)
- [Startup Savant — 10 best Stripe Atlas alternatives](https://startupsavant.com/service-reviews/stripe-atlas-alternatives)
- [Stripe Atlas — Incorporate your startup in Delaware](https://stripe.com/atlas)
- [Stripe Atlas — Stripe Documentation (root)](https://docs.stripe.com/atlas)
- [Stripe Atlas blog — Atlas for LLCs](https://stripe.com/blog/atlas-llc)
- [Stripe Atlas — Company types](https://docs.stripe.com/atlas/company-types)
- [Stripe Atlas — How to incorporate your company](https://docs.stripe.com/atlas/signup)
- [Stripe Resources — How to form a Delaware C corp](https://stripe.com/resources/more/what-is-a-delaware-c-corp)
- [TechGeeta NDA Generator](https://nda.techgeeta.com/)
- [Terms.Law Forum — Is Stripe Atlas worth $500?](https://terms.law/forum/thread/stripe-atlas-worth-it.html)
- [The Pensions Regulator — Earnings thresholds (ongoing employer duties)](https://www.thepensionsregulator.gov.uk/en/employers/new-employers/im-an-employer-who-has-to-provide-a-pension/declare-your-compliance/ongoing-duties-for-employers/earnings-thresholds)
- [URM Consulting — Analysis of Fines Imposed by the ICO in 2024](https://www.urmconsulting.com/blog/analysis-of-fines-imposed-by-the-information-commissioners-office-in-2024)
- [URM Consulting — Analysis of ICO Enforcement Action Jan–Jun 2025](https://www.urmconsulting.com/blog/analysis-of-ico-enforcement-action-january-june-2025)
