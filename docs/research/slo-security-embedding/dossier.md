---
name: slo-security-embedding
researched: 2026-04-24
incomplete: false
---

# Research Dossier — Embed security across the SLO skill pack

## Market

SLO is an open-source Claude Code skill pack; there is no paid market to size. Proxy demand exists in two adjacent commercial categories: threat-modeling platforms (IriusRisk, SecuriCAD, Microsoft Threat Modeling Tool) and compliance automation (Vanta, 15,000+ customers, 35+ frameworks; Drata, 8,000+ customers, 26+ frameworks). Neither category targets solo operators directing AI agents through structured runbooks — the niche SLO owns. Developer-facing OSS survey data (Tidelift 2024, Snyk 2024 State of Open Source Security) reports security-practice adoption (SBOM ~62%, Scorecard awareness ~40%, NIST SSDF ~39%, SLSA ~23%) but **does not report compliance-framework adoption** for OSS maintainers, which is a structural evidence gap the design must acknowledge rather than paper over.

## Direct competitors

| Name | Price | Key feature | Gap vs our wedge |
|---|---|---|---|
| **OWASP SecOpsTM v1.1.0** (2025-08-26) | Free / OWASP (OSS) | Python engine fusing pytm rule identification + component-level LLM (Ollama/Gemini/OpenAI/Mistral) + ChromaDB+HuggingFace RAG; emits mappings to MITRE ATT&CK, CAPEC, CVE, D3FEND, CIS, NIST 800-53; SVG/HTML reports | Python subprocess with heavy install footprint (ChromaDB, embeddings, optional Ollama); cross-platform maturity on macOS arm64 / Windows not characterized in any search round; no OTM output; not agent-composable out of the box |
| **STRIDE-GPT** (mrwadams, active OSS, no pinned version) | Free / MIT | Pure-LLM STRIDE generator with OWASP LLM Top 10 mode (LLM01–LLM10) | Documented **unstable categorization across runs** (Pure Storage 2025, FuzzingLabs 2025, ThreatCompute CCSW 2025) — unusable as a dependency without a deterministic validation gate; no schema emission |
| **OWASP Threat Dragon** (active OWASP Lab) | Free / Apache 2.0 | GUI + rule engine covering STRIDE, LINDDUN, CIA, DIE, PLOT4ai; JSON DFD format importable by third-party LLM extensions (`threat-dragon-ai-tool`, `threat-dragon-llm`) | GUI-first; no CLI automation path; no native OTM import; LLM extensions are third-party, neither official nor versioned |
| **pytm v1.3.1** (April 2024) | Free / MIT | Deterministic threat-model-as-Python-code; canonical for STRIDE in code | **No release in ~24 months**; no native NIST 800-53 mappings; SecOpsTM supersedes the entire output stack |
| **Microsoft Threat Modeling Tool** | Free (Windows only, closed-source) | Visual DFD editor with rule-driven threat generation; TMT7 XML export | Windows-only; no Mac/Linux agent path; closed proprietary format; no active CLI integration surface |

## Adjacent tools

| Name | Why adjacent, not direct | Can they pivot into us? |
|---|---|---|
| **IriusRisk / SecuriCAD** (commercial) | Enterprise threat-modeling platforms with libraries, integrations, and attack-path simulation | No — commercial platforms chase enterprise seats; SLO targets solo operators in the terminal |
| **OWASP Threat Model Library** | Curated corpus of reference threat models | Reference material only — citable source, not a runtime tool |
| **ThreatFinderAI, threat-dragon-llm, threat-dragon-ai-tool** | LLM wrappers around Threat Dragon | Candidate prior-art prompts; fragmented third-party projects; not a dependency SLO can pin |
| **Vanta / Drata** | Compliance-automation SaaS | Out of scope — SLO generates evidence artifacts; compliance platforms consume them |
| **MITRE ATLAS v5.1.0** (Nov 2025; Oct 2025 + Feb 2026 agentic updates) | Adversarial threat knowledge base for AI systems: 16 tactics, 84 techniques, 32 mitigations, 42 case studies; includes `AML.T0058 AI Agent Context Poisoning`, Memory Manipulation, Thread Injection | Citation target whenever the SLO target involves an AI/LLM component |
| **TMBOM** (emerging — CycloneDX × Threat Dragon × pytm) | Joint effort to standardize threat-model interchange | Watch, don't bet — no stable spec yet |

## Technical prior art

- **OTM 0.2.0 (Open Threat Model)** — IriusRisk-stewarded, Apache 2.0, frozen ~2 years; the only platform-independent threat-model interchange with a published JSON schema. Best fit as `/slo-threat-model` output schema with a strict validator gate. <https://github.com/iriusrisk/OpenThreatModel/blob/main/otm_schema.json>
- **CycloneDX 1.7 + Declarations** — spec 1.7 (October 2025), ECMA-424 2nd edition (December 2025). 1.6 (April 2024) introduced the Definitions/Declarations surface for standards/best-practices/maturity claims plus CBOM (Cryptographic Bill of Materials). OWASP ASVS, MASVS, SCVS, SAMM already published in CycloneDX format. <https://cyclonedx.org/specification/overview/>
- **OSCAL Component Definition v1.1.3** — NIST-backed XML/JSON/YAML schemas for publishing/implementing/assessing security controls. OSCAL Foundation formed January 2025. NIST CSWP 53 ipd (2025) explicitly names "Autonomous Risk Reasoning with Agentic AI" as a direction. <https://pages.nist.gov/OSCAL/>
- **SARIF 2.1.0** — OASIS standard interchange for static-analysis findings. Native emission from Semgrep (all versions), ast-grep 0.40.0+ (November 2025), CodeQL, and via `sarifw` wrapping ripgrep/ast-grep. <https://www.aptori.com/glossary/static-analysis-results-interchange-format-sarif>
- **Semgrep CE v1.160.0** (2026-04-16) — LGPL-2.1 core, proprietary pro rules since December 2024 relicense; median scan ~10 s, ~150 MB RAM, 30+ languages; native `--sarif` / `--sarif-output`; cross-file analysis **absent for Ruby/PHP/Swift/Rust**. <https://semgrep.dev/docs/cli-reference>
- **ast-grep 0.42.0** (2026-03-16) — Rust-native, MIT, tree-sitter based, `serde-sarif ^0.8.0` dep; SARIF output since 0.40.0 (November 2025); lighter-weight structural search without cloud tether; rule ecosystem smaller than Semgrep's; lacks taint and equivalences. <https://github.com/ast-grep/ast-grep/releases>
- **CodeQL** — free for public OSS via GitHub Advanced Security; ~12 languages with semantic data-flow/taint via relational DB build; scans minutes–30+ min, ~450 MB DB; **400+ CVEs identified in OSS via variant analysis**; incremental analysis all-languages since September 2025. <https://semgrep.dev/docs/faq/comparisons/codeql>
- **Trail of Bits `mrva`** (2025-12-11) — terminal-first multi-repo CodeQL variant analysis CLI; directly addresses "variant analysis outside a monorepo" (Google PSC framing). <https://blog.trailofbits.com/2025/12/11/introducing-mrva-a-terminal-first-approach-to-codeql-multi-repo-variant-analysis/>
- **Rust CycloneDX tooling gap (load-bearing)** — `cyclonedx-bom` 0.8.1 (2026-03-19) still targets CycloneDX spec 1.5; no Rust crate in the ecosystem emits 1.6+ `declarations`. Pure-Rust paths must hand-roll JSON, upstream 1.6 support, or shell out to Node (`@cyclonedx/cyclonedx-npm`) or Python (`cyclonedx-python-lib`). <https://github.com/CycloneDX/cyclonedx-rust-cargo/releases>
- **Academic prior art (2024–2025):**
  - Laponina (2025), *"Threat Modeling Software Development for LLM-Agent-Based Systems"*, Int. J. of Open Information Technologies. <https://www.injoit.ru/index.php/j1/article/view/2178/0>
  - arXiv 2411.17058 — *"ThreatModeling-LLM: Automating Threat Modeling using LLMs for Banking System"*. <https://arxiv.org/html/2411.17058v2>
  - arXiv 2504.19956 — *"Securing Agentic AI: A Comprehensive Threat Model and Mitigation Framework"* (SHIELD). <https://arxiv.org/html/2504.19956v2>
  - ACM CCSW 2025 — *ThreatCompute*, hybrid LLM + attack graphs for Kubernetes; finds hybrid reduces manual effort but struggles with stable categorization. <https://dl.acm.org/doi/10.1145/3733812.3765533>
  - USENIX Security 2025 (Kaur et al., prepub) — *"Investigating Threat Modeling Practices in Open-Source"*: OSS threat-modeling is "almost always ad hoc" because structured TM is perceived as high-cost / low-benefit by volunteer maintainers. <https://www.usenix.org/system/files/conference/usenixsecurity25/sec25cycle1-prepub-294-kaur.pdf>

## Regulatory / legal

- **No binding regulatory constraint on the skill pack itself** — SLO is OSS orchestration; it produces artifacts. The regulatory surface is what *consumers* of those artifacts must comply with.
- **Default compliance columns (evidence-backed):** SOC 2 and OWASP ASVS 5.0.0 (released May 2025, ~350 requirements across 17 chapters). Rationale: SOC 2 is the Vanta/Drata commercial-consensus entry framework for US SaaS → enterprise; ASVS is the OSS-native application-security benchmark, already published in CycloneDX format.
- **Opt-in compliance columns via runbook frontmatter:** GDPR, HIPAA, PCI DSS 4.0, NIST 800-53 moderate, ISO 27001. Rationale: each has concrete triggering conditions (EU PII handling, PHI, payment data, US federal systems, enterprise security management) that only the runbook author can assert.
- **GDPR design note (medium confidence):** GDPR encodes principles and data-processing obligations rather than discrete control IDs — better rendered as a **section** of the threat model than a column alone.
- **Cross-repo issue filing is rate-limit-bounded** — GitHub imposes a secondary limit of **80 content-creating requests/min, 500/hr**, uniformly across REST, GraphQL, and web UI. Exceeding returns HTTP 403/429 with `retry-after`. Per-endpoint point cost is deliberately undocumented. "A few capability-gap issues per week" sits ~3–4 orders of magnitude below the threshold — the binding constraints are **attribution semantics** (issues are authored by the user's `gh` identity, no service-account path without a separate GitHub App install) and **template-validation unreliability in org-owned repos** (community discussions #43859, #45084). <https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api>
- **Semgrep licensing shift:** December 2024 relicense moved several previously-OSS features behind commercial license; January 2025 ~10-vendor coalition forked Semgrep CE into **Opengrep** (LGPL-2.1). Core engine OSS, pro rules gated. Hedge, not default.

## Open questions that research did not answer

These remain unresolved after three deepening iterations and eight web searches. The `/slo-architect` step should name the assumption it makes for each, and `/slo-plan` should carry the assumption as a residual risk.

- **Per-framework compliance adoption among OSS-tool users** — no survey (Tidelift 2024, Snyk 2024, Stack Overflow, JetBrains, OpenSSF, Linux Foundation) reports framework-level adoption for OSS maintainers. Commercial-vendor catalog inference (Vanta/Drata) is the strongest available proxy. — *structural evidence gap; re-query annually*.
- **CycloneDX Property Taxonomy crypto-primitive namespaces** — whether `cdx:crypto:argon2id:iterations` (or equivalent) already exists in the public Taxonomy could not be confirmed. Either SLO contributes upstream, or SLO owns a vendored namespace. — *addressable by reading the Taxonomy repo directly during `/slo-architect`*.
- **SecOpsTM cross-platform maturity** — macOS arm64, Windows, ChromaDB footprint, Ollama fallback behaviour not characterized anywhere. Blocks making SecOpsTM a user-machine dependency with high confidence. — *addressable by spike install, out of research scope*.
- **OSCAL ↔ CycloneDX lossless crosswalk** — NIST CSWP 53 ipd asserts alignment; no published lossless mapping table exists. Sbomify publishes SPDX↔CycloneDX but not OSCAL↔CycloneDX. An SLO-authored crosswalk will be lossy at the `claims.evidence` ↔ `by-component.statements.remarks` boundary. — *design around lossiness rather than seek data*.
- **GitHub content-creation per-endpoint point cost** — deliberately undocumented. No way to know in advance how many capability-gap issues an agent session can file before throttle. Empirical probing against a disposable repo is the only path. — *mitigate via client-side cap ≤40/hr + SLO-owned intake repo fallback*.
- **MSRV for `ast-grep`** — not surfaced in release notes or `Cargo.toml` examination. Verify before committing if SLO's MSRV policy tightens.
- **Exact versions for Threat Dragon, STRIDE-GPT, Semgrep CLI, CodeQL CLI, and `gh` CLI** — not all pinned across rounds. Surface during `/slo-architect` stack-decision.
- **`compliance-trestle` (IBM OSCAL Python toolkit) maturity** — not evaluated; blocks Option B from being a concrete recommendation.
- **Node (`@cyclonedx/cyclonedx-npm`) / Python (`cyclonedx-python-lib`) 1.6 declarations support** — a one-shot check might unblock an Option A shortcut vs. hand-rolling JSON.
- **agentskills.io `SKILL.md` frontmatter + MCP server capability descriptors as capability-manifest formats** — listed in the brief but did not surface in any search round. Unclear whether they express security controls structurally.
