---
name: slo-security-embedding
created: 2026-04-24
status: ideation
tla_required: false
---

# Embed security across the SLO skill pack

## The pain

Sherif is the engineer who tells AI agents what to build. He ships runbooks like `docs/RUNBOOK-AWS-ORG-SETUP.md` knowing they are under-specified for security — if `/slo-critique` ran on it today it would call the design flimsy and open to the internet, and there is no upstream threat model any downstream skill can cite. Today he pays for this three ways: (1) he re-injects security by hand after `/slo-plan` finishes, adding rework per runbook; (2) he ships as-is and hopes the executing agent has read Hulumi or SunLitSecurityLibraries on its own initiative; (3) he avoids security-adjacent work (auth, multi-tenant, cloud identity) inside SLO entirely because he does not trust the current pack to produce a defensible plan. The delta is measurable — Sherif's own apps and his SunLitSecurityLibraries workspace only got strong because threat modelling happened first; the SLO pack skips that step entirely, so its output is a weaker version of the discipline its author already knows works.

## Five capabilities the user described without realising

- **Threat model as a first-class generated artifact** — produced by `/slo-architect` alongside `ARCHITECTURE.md`, not prompted from the user.
- **Bug-class elimination framing** (Google PSC) — `/slo-critique`'s security persona asks "does this plan make class X impossible?" instead of "is there an instance of X here?"
- **Stack-aware secure-default recommender with upstream feedback loop** — `/slo-sec-libs` matches runbook requirements to Hulumi or SunLitSecurityLibraries components and, when no component fits, files an issue upstream so the owner knows the gap.
- **Variant analysis on every finding** — when a concrete exploit is identified, the codebase is grepped for the same pattern elsewhere before the finding closes.
- **Escape hatches with documented residual risk** — `/slo-execute` allows overrides of secure-default recommendations but forces the justification into the Evidence Log as tracked residual risk, not a silent waiver.

## Approach A — prompts-only

- **Effort**: ~2 weeks.
- **Wedge**: ship Phase 1 edits to the five `SKILL.md` files in 3 days; every security addition is a markdown edit; `/slo-sec-libs` reasons entirely in the LLM context and drafts `gh issue create` commands for the user to run manually.
- **Risks**: reasoning drift — without deterministic backing, `/slo-sec-libs` hallucinates library capabilities as the libraries evolve; typo-prone manual issue filing; no CI-gated security test runs; threat model quality varies by session.

## Approach B — prompts + shell-outs (hybrid)

- **Effort**: ~4 weeks total (Phase 1 ~3 weeks as a 4-milestone runbook; Phases 2–4 as separate follow-up runbooks).
- **Wedge**: `/slo-architect` produces `SECURITY.md` + `docs/slo/design/threat-model.md` as the first new artifacts, because they are the upstream objects every other skill cites. Reasoning stays in skill prompts; deterministic pieces shell out — `gh issue create` for upstream feedback, `cargo audit` / `cargo deny` / ZAP / Dastardly for `/slo-security-test`.
- **Risks**: dependency surface grows (Docker for ZAP, `gh` CLI, `cargo-*` tools); cross-platform friction on Windows contributors; failure modes heterogeneous (network, auth, container runtime); still no structured capability manifest in the two security repos, so `/slo-sec-libs` matching leans on LLM reading READMEs.

## Approach C — Rust backend crate (`sldo-sec`)

- **Effort**: ~5–6 weeks.
- **Wedge**: a new crate under `crates/` mirrors the `sldo-research` precedent — parses `ARCHITECTURE.md`, indexes the two library repos' capability manifests, matches requirements to components deterministically, emits issue drafts structured for `gh issue create`. Skills call the binary.
- **Risks**: biggest scope; requires designing structured capability manifests in Hulumi + SunLitSecurityLibraries that do not exist today (co-design slip); locks the project into maintaining the indexer as libs evolve; reasoning tasks the LLM does well (STRIDE, abuse cases, "does this requirement map to `secure_authz` or `secure_data`?") get forced into Rust code where they are worse.

## Recommendation

Approach B. The reasoning work — STRIDE sweep, abuse case generation, mapping a requirement to `secure_authz` versus `secure_data` — is LLM-native and brittle in Rust. The deterministic work — running `cargo audit`, filing upstream issues, invoking ZAP — is shell-native and dangerous in the LLM. B splits them correctly. The wedge is Phase 1 shipped as a 4-milestone runbook: (M1) `/slo-ideate` 7th risk question + `/slo-architect` emits `SECURITY.md` + `docs/slo/design/threat-model.md` with STRIDE table, compliance mapping stub, `security_libs_required` flag, and a defense-in-depth section covering presubmit / admission / dark-launch plans; (M2) `/slo-plan` Contract Block gains `Data classification`, `Proactive controls in play`, and `Abuse acceptance scenarios` required rows, generated from the threat model; (M3) `/slo-critique` security persona rewritten around class elimination and variant analysis, rejecting findings that do not cite a threat-model row; (M4) `/slo-verify` Pass 4 security — supply-chain checks, DAST if a smoke service exists, variant-analysis spot-check. Phases 2 (`/slo-threat-model` dedicated skill), 3 (`/slo-security-test` harness), and 4 (`/slo-sec-libs` stack-aware recommender with upstream issue filing, promoted from stretch to core because SLO is the feedback loop for the two downstream libraries) are separate follow-up runbooks.

## Open questions for /slo-research

1. Do Hulumi and SunLitSecurityLibraries ship machine-readable capability manifests today, or will `/slo-sec-libs` need to parse READMEs? If the latter, what structured index format would the two repo owners accept upstream?
2. Which compliance frameworks drive the threat-model compliance-mapping column for SLO's target audience (SOC 2, NIST 800-53, HIPAA, GDPR, FedRAMP, IEC 62443)? The SunLitSecurityLibraries threat model cites 800-53 + IEC 62443 + SOC 2 — is that the right default set for SLO, or broader?
3. What prior art exists for "LLM agent + threat modelling" (MITRE ATLAS, academic work, Microsoft Threat Modelling Tool exports) that `/slo-threat-model` can cite or borrow structure from, rather than reinventing STRIDE?
4. How does Google's variant-analysis pattern translate from a monorepo to typical ~10k–50k LOC open-source repos? Is `ripgrep`-driven pattern search enough, or does it need AST-level matching (tree-sitter) to avoid false positives?
5. What is the current state of upstream-issue-filing ergonomics via `gh` CLI when the caller is an AI agent — rate limits, auth scoping, template enforcement — so `/slo-sec-libs` filing a gap issue does not become a permission rabbit hole?
