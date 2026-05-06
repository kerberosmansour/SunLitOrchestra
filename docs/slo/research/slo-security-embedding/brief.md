# Research brief — slo-security-embedding

## Wedge (one sentence)

Update `/slo-architect` to produce `SECURITY.md` + `docs/slo/design/threat-model.md` as first-class artifacts alongside `ARCHITECTURE.md`, so every downstream SLO skill (`/slo-plan`, `/slo-critique`, `/slo-execute`, `/slo-verify`) has an upstream threat model to cite, generated — not prompted — per Google PSC's "remediate bug classes" framing and Jim Manico's `security.md` discipline.

## Target user (one sentence)

An engineer (Sherif today; other SLO users tomorrow) directing AI agents through the SLO skill pack, who today either re-injects security by hand after `/slo-plan` finishes, ships under-specified runbooks and hopes the executing agent reads the security libs, or avoids security-adjacent work inside SLO entirely because the current pack does not produce a defensible plan.

## Five specific research questions

### Q1 — Capability manifest format prior art

What structured formats exist for expressing "this library / component provides these security controls" that an LLM agent can parse and match against a runbook requirement?

- Look at: **CycloneDX** (SBOM + VDR + VEX), **OSCAL** (NIST SP 800-53 control catalogs, component definitions), **OpenSSF Scorecard** manifest, **agentskills.io SKILL.md** frontmatter, **MCP** server capability descriptors, **SPDX** (license + supply-chain), **SARIF** (static-analysis result interchange).
- For each: is it agent-parsable (JSON/YAML), does it express "control X implemented by component Y", and what's the adoption effort to add it to Hulumi + SunLitSecurityLibraries?
- Concrete output: a ranked shortlist with one concrete example of each format describing one security control (e.g. "Argon2id password hashing" or "envelope encryption").

### Q2 — Compliance framework default set for an OSS skill pack

For a skill pack whose users build web services + cloud infrastructure, which compliance frameworks should the threat-model compliance-mapping column default to?

- Baseline: SunLitSecurityLibraries cites **NIST 800-53** + **IEC 62443** + **SOC 2 Type II**.
- Candidates to add/drop: **GDPR** (EU audience), **HIPAA** (US healthcare), **PCI DSS 4.0** (payments), **FedRAMP** (US gov), **ISO 27001**, **CIS Controls v8**, **OWASP ASVS 4.0.3**.
- Question: what's the minimal set that covers ≥80% of real-world SLO users without making the threat-model file unwieldy? Evidence we can use: published developer-survey data (Stack Overflow, JetBrains, Snyk State of Open Source Security), compliance-framework adoption stats by company size, guidance from OWASP / NIST on "which framework for which project scale."

### Q3 — LLM-agent threat-modelling prior art

What prior art exists for LLM agents producing STRIDE threat models, abuse cases, or attack trees, that `/slo-threat-model` (Phase 2) can cite or borrow structure from rather than reinventing?

- Look at: **MITRE ATLAS** (adversarial threat landscape for AI), **MITRE ATT&CK mapping tools**, **Microsoft Threat Modeling Tool** export format, **pytm** (Python deterministic threat-model library), **IriusRisk** / **SecuriCAD** (commercial), **OpenThreatModel** (OTM) JSON schema, academic papers on "LLM-assisted threat modeling" (2023–2026).
- For each: what's the input format (DFD? narrative?), what's the output format (table? graph?), and is it composable with a Rust-toolchain + skill-pack flow?
- Required: at least one academic paper or industry post-2024 publication.

### Q4 — Variant analysis outside a monorepo

Google PSC's variant-analysis pattern ("when you find a bug, grep the codebase for the same pattern") assumes a monorepo. For typical ~10k–50k LOC polyglot OSS repos, what tooling is agent-friendly (CLI-invokable, structured output, runs in < 60s on commodity hardware)?

- Candidates: **Semgrep** (OSS + paid tiers; rules-as-code), **CodeQL** (free for OSS; GitHub Actions integrated), **Sourcegraph** (hosted + self-hosted; structural search), **tree-sitter** + custom queries, **ast-grep** (tree-sitter based, Rust, MIT), **grep.app** (hosted), **ripgrep** + hand-curated regexes.
- Question: cost/accuracy tradeoff per candidate, CLI ergonomics for agent invocation, output format (SARIF preferred), false-positive rate on a realistic SLO test repo. Which two are worth wiring into `/slo-security-test` (Phase 3)?

### Q5 — gh CLI agent-filing ergonomics

When an AI agent invokes `gh issue create` against a repo it does not own (e.g., `/slo-sec-libs` filing a capability-gap issue against `kerberosmansour/hulumi` from a user's project repo), what are the real operational limits?

- Concrete unknowns: OAuth scope requirements, GitHub REST API rate limits (5k/hr authed, 15k/hr via GHEC), GitHub spam-detection thresholds for cross-repo issue creation, enforced issue-template use (repos can mandate `.github/ISSUE_TEMPLATE/*.yml`), attribution (the issue will be authored by the user's `gh` identity — is that acceptable for Hulumi / SunLitSecurityLibraries?), alternatives (forks + PR, email intake, RFC repo).
- Required output: a decision table comparing ≥3 filing channels on auth surface, friction, and upstream-maintainer cost.

## Out of scope for this research

- Deep API reference for specific libraries (use `get-api-docs` / `chub` if needed).
- Restating SLO's own architecture (the idea doc already has it).
- Business model / pricing for SLO — SLO is an OSS skill pack, not a commercial product.
