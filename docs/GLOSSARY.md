# Glossary

Terms and acronyms used across the SunLitOrchestrate skill pack, runbook templates, and reference docs. Add an entry the first time you find yourself defining a term in a PR description.

## Core concepts

- **Skill** — a Markdown file (`SKILL.md`) loaded into a host AI agent (Claude Code, GitHub Copilot) that defines a slash-command and the procedure the agent should follow. Skills live under `skills/<name>/` and are installed by [`sldo-install`](crates/sldo-install/).
- **Host overlay** — the host-specific instruction file that points the agent at the skill pack. [CLAUDE.md](../CLAUDE.md) for Claude Code, [copilot-instructions.md](../copilot-instructions.md) for GitHub Copilot.
- **Runbook** — a per-feature plan authored against the v4 template at [docs/templates/runbook-template_v_4_template.md](templates/runbook-template_v_4_template.md). The output of `/slo-plan`. Lives at `docs/RUNBOOK-<FEATURE>.md`.
- **Milestone (M1, M2, …)** — one numbered phase inside a runbook. The unit of work for `/slo-execute`, `/slo-verify`, `/slo-retro`. Capped at five per runbook.
- **Sprint flow** — the seven-step pipeline a feature passes through: Think → Plan → Build → Review → Test → Ship → Reflect, mapped to `/slo-ideate` → `/slo-research` → `/slo-architect` → `/slo-plan` → `/slo-critique` → `/slo-execute` → `/slo-verify` → `/slo-retro` → `/slo-ship`.
- **Allow-list** — a per-milestone whitelist of file paths the milestone is permitted to touch. Enforced by `/slo-execute`.
- **Evidence Log** — the per-milestone table of test cases and actual results. `/slo-retro` refuses to close a milestone with a blank Evidence Log.
- **Carry-forward** — a lesson or finding from a prior runbook's retro that the current runbook must explicitly address. Surfaced by the v4 template.

## Engineering practices

- **BDD** — Behavior-Driven Development. Given/When/Then scenario format. The v4 runbook template requires BDD scenarios per milestone before code is written.
- **TLA+** — Temporal Logic of Actions; a formal specification language used by `/slo-tla` to model-check designs that involve concurrency, ordering, or distributed state. Set by `/slo-architect` via the `tla_required` flag.
- **TLC** — the model checker that executes TLA+ specs.
- **SAST** — Static Application Security Testing; in this repo, Semgrep rules that run pre-merge against Rust code. See `.semgrep/rust/`.
- **CWE** — [Common Weakness Enumeration](https://cwe.mitre.org/); the standard taxonomy for software weakness categories (e.g. CWE-89 SQL injection, CWE-787 out-of-bounds write).
- **STRIDE** — Microsoft's threat-modeling categorization (Spoofing, Tampering, Repudiation, Information disclosure, Denial of service, Elevation of privilege). Used in `docs/design/<slug>-threat-model.md` artifacts.
- **MSRV** — Minimum Supported Rust Version.
- **Carmack-style reliability controls** — the v4 template's debugger-first / mandatory static-analysis / assertion-driven invariants / bounded-resource / "make invalid states unrepresentable" practices. See [docs/templates/runbook-template_v_4_template.md](templates/runbook-template_v_4_template.md) and the [influence clip](https://youtu.be/tzr7hRXcwkw) acknowledged in the README.

## Biz pack — UK legal, tax, equity, fundraise

The biz pack is UK-only in v1. Non-UK use returns the canonical "v1 supports UK only" error.

- **CC BY-ND 4.0** — Creative Commons Attribution-NoDerivatives 4.0. The license under which the [oneNDA](https://www.onenda.org/) UK template is published. ND forbids derivative works — the skill never copies, renders, or redistributes the canonical text.
- **GDPR** — General Data Protection Regulation (UK GDPR + Data Protection Act 2018). All GDPR documents are hard-blocked from `draft` mode in `/slo-legal`; only `triage` and `translate` are permitted.
- **DUAA 2025** — Data (Use and Access) Act 2025. UK successor that raised PECR fines and added a complaints-procedure duty.
- **PECR** — Privacy and Electronic Communications Regulations. Governs cold email, SMS, and cookies. Fine ceiling raised to £17.5M / 4% global turnover under DUAA 2025.
- **ICO** — Information Commissioner's Office, the UK data-protection regulator.
- **DPA** — (a) Data Protection Act 2018 (statute); (b) Data Processing Agreement (contract). Context disambiguates.
- **DPO** — Data Protection Officer.
- **ROPA** — Record of Processing Activities. Required under UK GDPR Art. 30 for most controllers/processors.
- **HMRC** — His Majesty's Revenue and Customs, the UK tax authority.
- **VAT** — Value Added Tax. UK VAT registration threshold is £90,000 turnover (2024-25).
- **MTD** — Making Tax Digital, HMRC's digital filing programme.
- **R&D credit** — UK Research and Development tax relief / SME R&D scheme.
- **IR35** — UK off-payroll-working rules; whether a contractor is "really" an employee for tax purposes. CEST is HMRC's Check Employment Status for Tax tool. Seven factors enumerated in [references/biz/ir35-cest-factors.md](../references/biz/ir35-cest-factors.md).
- **SEIS / EIS** — Seed Enterprise Investment Scheme / Enterprise Investment Scheme. UK tax-advantaged investment schemes for early-stage companies. Governed by HMRC VCM index ([references/biz/hmrc-vcm-index.md](../references/biz/hmrc-vcm-index.md)).
- **Advance Assurance** — HMRC pre-clearance that a company's share issue will qualify for SEIS/EIS relief. `/slo-fundraise` refuses to draft term-sheet artifacts unless AA is at least 6 weeks ahead of the planned signature.
- **SAFE** — Simple Agreement for Future Equity. A convertible instrument used in early-stage rounds.
- **ASA / CAP Code** — Advertising Standards Authority and the UK Code of Non-broadcast Advertising. Constrains B2C marketing claims.
- **CRA 2015** — Consumer Rights Act 2015. Applies to consumer-facing T&Cs.

## Business / GTM metrics

- **GTM** — Go-To-Market.
- **PLG** — Product-Led Growth (acquisition driven by product use, not sales).
- **ICP** — Ideal Customer Profile. The narrow, well-specified target user.
- **CAC** — Customer Acquisition Cost.
- **LTV** — Lifetime Value (per customer).
- **NDR** — Net Dollar Retention; revenue from a cohort one year on, including expansion and churn.
- **ARR / MRR** — Annual / Monthly Recurring Revenue.
- **MoM** — Month-over-Month (growth rate).
- **Burn multiple** — net cash burned ÷ net new ARR added in the same period; lower is better.
- **Gross margin** — revenue minus cost of revenue, as a percentage of revenue.
- **Runway** — months of operation remaining at current burn rate.
- **DAU / MAU** — Daily / Monthly Active Users.
- **RICE** — Reach × Impact × Confidence ÷ Effort, a roadmap-prioritization scoring method.
- **Kano** — Kano model; categorizes features as basic / performance / delighter for prioritization.
- **OKR** — Objectives and Key Results.
- **North-star metric** — the single product metric a team optimizes; everything else is leading or lagging it.

## File-tier conventions

- **`docs/biz/`** — confidential drafts containing real PII (cofounder names, deal terms, interview transcripts). Gitignored.
- **`docs/biz-public/`** — placeholder / decision artifacts safe for git tracking. `/slo-verify` Pass 4 PII scan runs over this directory.
