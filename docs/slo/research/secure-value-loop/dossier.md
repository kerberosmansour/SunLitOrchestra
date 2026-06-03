---
name: secure-value-loop
researched: 2026-06-02
incomplete: false
---

# Research Dossier — Secure Value Loop

The wedge: make security/value disciplines (operator readiness, detected-work
disposition, honest exit states) **typed and unavoidable** in the SLO runbook
contract, without rebuilding the security machinery the pack already ships.

Target user: Sherif — the engineer who drives AI agents through the SLO sprint
loop on identity/secrets/PII/cloud/AI/public-boundary work.

This dossier separates (A) what the proposal asks for that **already ships**,
(B) external standards the contract should cite correctly, and (C) the cited
external operating models. The "competitor" frame for a process-change proposal
is *comparable secure-SDLC / agentic-dev frameworks*.

## Market — who "pays" for this today

The proxy spend is Sherif's own rework: every security-adjacent runbook today
re-derives operator prerequisites by hand and disposes findings as lessons-file
prose. The proposal's own §11 acceptance criteria are the "willingness to pay"
signal — they are the exit bar this work is judged against. The broader market
(secure-SDLC tooling: GitHub Advanced Security, GitLab Ultimate, Snyk, Apiiro,
Jit, Arnica) sells the *scanners*; none of them sell the *runbook-contract
discipline* that forces a human-readable disposition on every finding inside an
agent-driven sprint. That gap is the wedge.

## Direct competitors — comparable frameworks/models

| Name | "Price" / cost model | Key feature | Gap vs our wedge |
|---|---|---|---|
| **NIST SSDF (SP 800-218 v1.1, + 800-218A GenAI profile)** | Free standard; cost is adoption effort | Four practice groups PO/PS/PW/RV; outcome-based, not prescriptive; PW group explicitly wants SAST + dynamic/fuzz testing | A *framework of outcomes*, not an executable runbook contract. SLO needs the SSDF outcomes *encoded as required milestone rows*; SSDF won't gate an agent. ([csrc.nist.gov](https://csrc.nist.gov/pubs/sp/800/218/final)) |
| **OWASP Top 10 Proactive Controls (2024 edition)** | Free | C1 Access Control … C10 Stop SSRF — developer-facing secure-build techniques | The proposal cites the **2018** list (define-requirements/leverage-libraries/…), which OWASP *reorganized* in 2024. Pure reference; needs version-pinning, not adoption. ([top10proactive.owasp.org](https://top10proactive.owasp.org/archive/2024/the-top-10/)) |
| **OpenAI Symphony** | Free open-source spec (`openai/symphony`) | Per-issue isolated workspace; repo-owned `WORKFLOW.md`; bounded concurrency (default 10) + exponential-backoff retry (10s→300s); runs end in a **workflow-defined handoff state, not only Done** | Symphony orchestrates *dispatch*; it does not carry a *security/value contract* per milestone. SLO borrows its handoff-state idea, not its runtime. ([github.com/openai/symphony](https://github.com/openai/symphony), [SPEC.md](https://github.com/openai/symphony/blob/main/SPEC.md)) |
| **GStack sprint chain** | Cited in proposal; no canonical public source located | Think → Plan → Build → Review → Test → Ship → Reflect, with reviews as *artefact-producing gates* | SLO **already runs this exact chain** (CLAUDE.md sprint flow). GStack's only net contribution — "reviews are gates, not commentary" — is already realised by `/slo-critique` + `/slo-verify`. Treat as shorthand, not a dependency. |
| **CISA Secure by Design** | Free guidance | Security as an early lifecycle responsibility; secure defaults; memory-safety | Principle-level; SLO's `secure-execution-controls` research already absorbed it ("secure-construction gate, not only secure verification"). |

## Adjacent tools — already inside the SLO pack (do not rebuild)

| Name | Why adjacent, not direct | Already covers? |
|---|---|---|
| `/slo-architect` Step 3.5 + threat-model producer runbook | Emits `SECURITY.md` + `threat-model.md` + `threat-model.slo.json` with STRIDE, frozen abuse IDs, compliance mapping | YES — proposal Stage 4 is **shipped** |
| `/slo-critique` security persona | Class-elimination + variant analysis, concrete-scenario-only gate | YES — proposal Stage 5 is **shipped** |
| `/slo-verify` Pass 4 (security) + Pass 5 (LLM) | SAST/SCA/secrets/IaC/DAST-via-`/slo-dast-tuner`/privacy-PII; LLM tests gated on `ai_component` | YES — proposal Stage 8 + Bundle E mostly **shipped** |
| `/slo-retro` issue-filing discipline | Lane-classified (product/upstream-OSS/slo-process), dedupe, per-session cap | PARTIAL — disposition vocabulary exists but is not the proposal's 5-state ledger |
| `slo-sast`, `slo-rulegen`, `slo-ruleverify`, `scanner-orch` runbook | Threat-model-driven Semgrep + GitHub Actions + audit-defense manifest | YES — proposal "SAST" lane **shipped** |
| `slo-dast-tuner` (zaprun), `slo-nettacker` | DAST/API + authorized active scanning | YES — proposal "DAST" lane **shipped** |
| `slo-cloud-threat-model` | Prebuilt-scenario AWS/GitHub/Cloudflare threat models, Hulumi policy IDs | YES — proposal Bundle D / Hulumi lane **shipped** |
| `slo-sec-libs` | CycloneDX 1.6 capability matching + upstream gap filing | YES — proposal Stage 7 "SecureLibraries" **shipped** |
| `slo-kani` / `slo-tla` | Bounded Rust proofs / protocol model-checking | YES — proposal "fuzz/property/formal" **shipped** |

## Technical prior art

- **OWASP Top 10 Proactive Controls 2024** — C1 Implement Access Control, C2 Use Cryptography to Protect Data, C3 Validate all Input & Handle Exceptions, C4 Address Security from the Start, C5 Secure-By-Default Configurations, C6 Keep Your Components Secure, C7 Secure Digital Identities, C8 Leverage Browser Security Features, C9 Implement Security Logging and Monitoring, C10 Stop Server-Side Request Forgery. **This is a renumber vs. the 2018 list the proposal embeds.** ([top10proactive.owasp.org](https://top10proactive.owasp.org/archive/2024/the-top-10/))
- **NIST SSDF SP 800-218 v1.1** + **SP 800-218A** (GenAI/dual-use-model profile, finalised) — the GenAI profile is directly relevant to SLO's `ai_component` lane. ([csrc.nist.gov SSDF](https://csrc.nist.gov/pubs/sp/800/218/final), [800-218A](https://csrc.nist.gov/pubs/sp/800/218/a/final))
- **SLSA + SBOM (CycloneDX / SPDX)** — SLSA Levels 0–3 prove *build integrity/provenance*; SBOM lists *components*. Tools: Syft/Trivy/cosign/Sigstore. SBOMs now mandated by EO 14028 + EU CRA. Relevant only to Ship-stage release artifacts (this repo: crates.io publishes + the release-zip workflow). ([SLSA explained](https://secportal.io/blog/slsa-framework-explained), [SBOM→SLSA](https://petronellatech.com/blog/from-sbom-to-slsa-securing-your-software-supply-chain/))
- **OWASP ASVS 5.0 / MASVS / API Security Top 10 (2023) / LLM Top 10 (2025)** — the verification-standard families the test bundles cite. ASVS for app/API verification, MASVS for Bundle F (mobile), API Top 10 for Bundle C, LLM Top 10 (2025, now agentic-aware) for Bundle E. Prior SLO research already set **SOC 2 + ASVS 5.0** as the default compliance columns. ([OWASP MASVS](https://mas.owasp.org/MASVS/), [API Security](https://owasp.org/API-Security/), [LLM Top 10 2025](https://securityboulevard.com/2026/03/the-owasp-top-10-for-llm-applications-2025-explained-simply/))
- **OpenAI Symphony SPEC** — handoff states, per-issue isolated workspace, bounded concurrency/retry, repo-owned workflow policy. ([SPEC.md](https://github.com/openai/symphony/blob/main/SPEC.md))
- **Internal prior art (the strongest)** — `docs/slo/research/slo-security-embedding/`, `secure-execution-controls/`, `slo-threat-model/` syntheses; completion summaries `slo-sec-m1..4`, `slo-threat-model{,-producer}-m1..2`, `sec-libs-m1..5`, `sast-rulegen-a-*`, `scanner-orch-m1..5`, `nettacker-hardening-m1`. These show the security envelope is ~80% built; this runbook closes the *contract-discipline* gap, not the *capability* gap.

## Regulatory / legal

- **No new regulatory obligation is created by this work** — it is an internal process/template change to a markdown skill pack, not a user-facing data system. SBOM/provenance mandates (EO 14028, EU CRA) apply only if/when SLO's *own* release artifacts ship to third parties; for the skill pack the honest answer for most milestones is `not_applicable`. ([SBOM mandates](https://petronellatech.com/blog/from-sbom-to-slsa-securing-your-software-supply-chain/))
- **GDPR posture is already locked** in the biz pack (broad hard-block on GDPR `draft`); the Secure Value Loop must not weaken it. Privacy/telemetry tests in the proposal align with the existing §5A Measurement Contract privacy-controls row and `/slo-verify` Pass 4 PII scan — reuse, don't duplicate.

## Open questions that research did not fully answer

- **GStack canonical source** — none located; treat as the proposal author's shorthand for the sprint chain SLO already runs. Cite the proposal, not an external GStack page.
- **Exact reconciliation of the two disposition taxonomies** (`/slo-retro` lanes vs. the proposal's 5-state ledger) — a *design* decision for `/slo-architect`, not answerable from sources. Recommended direction in synthesis.
- **Whether the expanded status enum breaks `/slo-resume` / Tracker parsing** — must be verified empirically during design/execute (read the parsers), not from external sources.
