---
name: slo-security-embedding
created: 2026-04-24
status: design-locked
tla_required: false
tla_reason: >
  Skill-pack edits and markdown authoring. No concurrent actors sharing state,
  no distributed consensus, no leader election, no cross-process ordering
  guarantees, no resource leases. Every phase is sequential file I/O plus
  subprocess invocation.
security_libs_required: true
security_libs_reason: >
  This feature exists to integrate with Hulumi and SunLitSecureLibraries, and
  to ship the reading side of CycloneDX 1.6+ declarations so /slo-sec-libs
  (Phase 4) can match runbook requirements to library capabilities.
ai_component: true
ai_component_reason: >
  SunLitOrchestrate orchestrates Claude Code (the `claude` CLI subprocess). The
  MITRE ATLAS + OWASP LLM Top 10 + NIST AI RMF triad applies — see the threat
  model file for citations.
compliance: [soc2, asvs]
compliance_reason: >
  Defaults — SLO is an OSS tool with no regulated data-processing of its own,
  so opt-in compliance frameworks (GDPR, HIPAA, PCI DSS, ISO 27001, FedRAMP)
  do not apply here. Downstream users pick frameworks per their project's
  constraints.
---

# Design — embed security across the SLO skill pack

## System goal

Thread security through every stage of the SLO skill pack so the AI agents executing SLO runbooks produce artifacts with threat models, abuse-case BDD scenarios, class-elimination critiques, proactive-control adoption nudges, and a feedback loop to the two upstream security libraries (Hulumi, SunLitSecureLibraries). Do this without turning the skill pack into a compiled codebase — prompts stay in Markdown, deterministic steps shell out.

## Planned architecture (this feature)

Solid lines exist today. Dashed lines are added by the Phase 1 runbook (M1–M4); dotted lines are added by follow-on runbooks (Phases 2–4).

```
┌───────────────────────────────────────────────────────────────────────────────┐
│                              USER (Sherif)                                    │
└──────────────────────────────────┬────────────────────────────────────────────┘
                                   │ /slo-* invocation
                                   ▼
┌───────────────────────────────────────────────────────────────────────────────┐
│                        Claude Code (skill loader)                             │
└──────────────────────────────────┬────────────────────────────────────────────┘
                                   │ reads SKILL.md + context
                                   ▼
┌───────────────────────────────────────────────────────────────────────────────┐
│   SLO Skill Pack  (skills/slo-*/SKILL.md — Markdown)                          │
│                                                                               │
│   slo-ideate ─────────► docs/slo/idea/<slug>.md    + Top risks block  (dashed)    │
│        │                                                                      │
│   slo-research  ────────► docs/slo/research/<slug>/{dossier,sources,synthesis}.md │
│        │                                                                      │
│   slo-architect ────────► docs/ARCHITECTURE.md  (solid)                       │
│                          docs/slo/design/<slug>-{overview,stack-decision,         │
│                                               interfaces}.md (solid)          │
│                          SECURITY.md           (dashed, M1)                   │
│                          docs/slo/design/<slug>-threat-model.md (dashed, M1)      │
│        │                                                                      │
│   slo-plan ─────────────► docs/RUNBOOK-<slug>.md                              │
│                            + Data classification     (dashed, M2)             │
│                            + Proactive controls row  (dashed, M2)             │
│                            + Abuse-case BDD scenarios(dashed, M2)             │
│        │                                                                      │
│   slo-critique ─────────► docs/slo/critique/<slug>.md                             │
│     (security persona rewritten for class elimination + variant analysis,     │
│      cites threat-model rows)                        (dashed, M3)             │
│        │                                                                      │
│   slo-execute ─► crate / skill edits  (respects allow-list)                   │
│        │                                                                      │
│   slo-verify ─► docs/slo/verify/<slug>-m<N>.md                                    │
│                 + Pass 4: supply-chain + DAST        (dashed, M4)             │
│        │                                                                      │
│   slo-retro ─► docs/slo/lessons/<slug>-m<N>.md                                    │
│        │                                                                      │
│   slo-ship ─► git push + gh pr create                                         │
└──────────────────────────────────┬────────────────────────────────────────────┘
                                   │
          ┌────────────────────────┼─────────────────────────────────┐
          │ (dashed, M4)           │ (dotted, Phase 2)               │ (dotted, Phase 4)
          ▼                        ▼                                 ▼
┌───────────────────────┐  ┌────────────────────────┐  ┌───────────────────────────────┐
│ cargo audit / deny    │  │ SecOpsTM v1.1.0        │  │ gh issue create               │
│ (local subprocess)    │  │ (Python subprocess,    │  │   ├─► SLO-owned intake repo   │
│                       │  │  local)                │  │   │   (default)              │
│ Semgrep CE            │  │                        │  │   └─► kerberosmansour/hulumi,  │
│ ast-grep 0.42         │  │ optional: Ollama       │  │       SunLitSecureLibraries   │
│ (local subprocess)    │  │ (local subprocess)     │  │       (gated, user-confirmed,  │
│                       │  │                        │  │        ≤40 issues/hr)         │
└───────────────────────┘  └────────────────────────┘  └───────────────────────────────┘
```

### Trust boundaries

- **User ↔ Claude Code.** Human intent crosses into agent interpretation. Standard rule applies: agents cannot be trusted with unbounded destructive actions without explicit authorization.
- **Claude Code ↔ Local filesystem.** Skills read/write only within paths allowed by the milestone allow-list (enforced by `/slo-execute`).
- **Skill ↔ Local subprocess.** Output from external tools (Semgrep, ast-grep, SecOpsTM, `cargo audit`) is untrusted data that must not be interpreted as prompt content. Output parsers must reject malformed SARIF/JSON rather than echo it into the conversation.
- **Skill ↔ GitHub API** (Phase 4). Authenticated as the local user's `gh` identity. Attribution leaks across repos; rate-limited by 80/min + 500/hr secondary limits. Content sent to third-party repos is visible to adversarial maintainers; SLO does not ship any secret there.
- **SLO skill pack ↔ SLO-owned intake repo** (Phase 4). Default filing destination. Decouples from third-party rate-limit pressure and attribution confusion.

### Data flow — new artifacts

Phase 1 introduces three new artifact shapes in the target repo SLO is directing. Each is generated by `/slo-architect` (and/or its sibling skills) and consumed by downstream skills:

1. **`SECURITY.md`** (repo root) — project-wide security rules: framework choices, crypto policy, auth model, input-handling discipline, allowed escape hatches. Every downstream agent reads this before generating any token. Jim Manico's `security.md` discipline, adapted to SLO's generator-first posture.
2. **`docs/slo/design/<slug>-threat-model.md`** — STRIDE per component, abuse cases, compliance mapping (SOC 2 + ASVS default, opt-in framework columns per runbook frontmatter), class-elimination claims. Generated alongside this overview; the runbook cites it.
3. **Runbook Contract Block additions** — `Data classification (Public / Internal / Confidential / Restricted)`, `Proactive controls in play`, and `Abuse acceptance scenarios` become required rows. These are filled by `/slo-plan` using the threat model as source.

## Non-negotiables (downstream cannot change these without migration)

- **Markdown-first skill contract.** Phase 1 adds zero Rust code. If future phases introduce a compiled `sldo-sec` crate, that is a separate runbook with its own migration plan.
- **Reality-first `docs/ARCHITECTURE.md`.** Any planned work produced by `/slo-architect` for this feature stays in `docs/slo/design/` or the runbook Target Architecture section — it does not enter ARCHITECTURE.md until shipped.
- **Threat model is generated, not prompted.** `/slo-architect` produces the threat-model file in one pass; the user reviews, does not author from scratch. 80/20 burden per Google PSC.
- **Escape hatches are documented residual risk.** `/slo-execute` may accept overrides of secure-default recommendations (e.g. using raw `String` instead of `SecureJson` type) only when the justification is written into the milestone's Evidence Log as a tracked residual risk — never a silent waiver.
- **Filing upstream is gated.** `/slo-sec-libs` (Phase 4) files into the SLO-owned intake repo by default. Filing against `kerberosmansour/hulumi` or `sunlitsecurelibraries/*` requires an explicit user-provided flag and is capped at 40 issues/hr per session.
- **No promised round-trip fidelity across formats.** OSCAL ↔ CycloneDX mapping is asserted by NIST CSWP 53 ipd but not published as a lossless table (synthesis). If SLO ever emits both, losses are documented rather than hidden.

## Residual risks carried into /slo-plan

Per the research synthesis, three findings must be explicit risks in the runbook:

1. **SecOpsTM cross-platform maturity** on macOS arm64 / Windows is not characterized. Phase 2 must begin with a spike install before milestone-2 commits.
2. **CycloneDX Property Taxonomy crypto-primitive namespaces** — whether `cdx:crypto:argon2id:iterations` (or equivalent) exists in the public Taxonomy is unresolved. SLO starts with a vendored namespace and migrates if upstream vocabulary covers it.
3. **GitHub content-creation per-endpoint point cost** is undocumented. Any `/slo-sec-libs` rate-guard is defensive, not guaranteed. SLO-intake fallback is the production path.
