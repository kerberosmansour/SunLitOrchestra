---
name: sast-rulegen-skill-pack
created: 2026-04-25
status: design-locked
tla_required: false
security_libs_required: false
ai_component: true
compliance: [soc2, asvs]
---

# Design overview — SAST rule-generation skill pack

Single source of truth for downstream skills (`/slo-tla` reads `tla_required`, `/slo-plan` reads scope, `/slo-critique` reads compliance and ai_component, `/slo-verify` cites the threat-model row IDs).

## What this is

Two new Claude Code skills + one xtask workspace crate + a shared `references/sast/` reference library that together let a Rust developer (a) bootstrap a Semgrep rule pack covering the top-10 CWE classes Rust is most susceptible to, and (b) extend that pack from a Claude-found bug + fix-diff into 3-5 variation rules with auto-derived corpus, gated by a deterministic `cargo xtask sast-verify` gate before any rule lands.

Wedge ships **Runbook A — Rust + Semgrep — 3 milestones**: M1 bootstrap pack + verifier gate, M2 extend-mode (per-bug variation rules), M3 CI + dev-env wiring (`cargo-audit`-driven trigger, `pre-commit` / `prek` local hook). TypeScript replication ships in **Runbook B** (separate, not in this overview).

## Frontmatter rationale

- `tla_required: false` — rule generation is offline batch in a single process. No concurrent shared state, no consensus, no leader election, no leases, no ordering guarantees across processes. The xtask runs `semgrep --validate` then `semgrep --test` deterministically; nothing to model-check.
- `security_libs_required: false` — no service surface, no auth, no crypto-confidentiality requirement. SunLitSecureLibraries / Hulumi components do not apply. Local CLI + skill pack only.
- `ai_component: true` — both skills drive Claude Code (`claude` CLI subprocess) and the extend-mode skill consumes founder-pasted bug summaries + fix diffs. MITRE ATLAS + OWASP LLM Top 10 + NIST AI RMF triad applies. Specific surfaces in the threat model.
- `compliance: [soc2, asvs]` — defaults. No GDPR (no personal data processing in the skill pack itself; the user's bug-summary text is treated as confidential per SECURITY.md two-tier convention but does not constitute personal data unless the user pastes some, in which case the same gitignore tier kicks in).

## Inputs

- `docs/idea/sast-rulegen-skill-pack.md` — pain, capabilities, top-3 risks, three approaches, recommendation (Approach C, Runbook A 3-milestone shape).
- `docs/research/sast-rulegen-skill-pack/dossier.md` — competitor table, adjacent-tools decisions, prior-art references.
- `docs/research/sast-rulegen-skill-pack/synthesis.md` — load-bearing design rules ("the design must handle X because <source>").
- This repo's HEAD state (`crates/sldo-*`, existing `skills/slo-*`, existing `references/biz/`, existing `SECURITY.md`).

## Outputs (this skill, this run)

- [docs/ARCHITECTURE.md](../ARCHITECTURE.md) — updated skill-pack table + new "SAST rule-gen skill pack" section with dashed components for the planned work.
- [docs/design/sast-rulegen-skill-pack-stack-decision.md](sast-rulegen-skill-pack-stack-decision.md) — chosen stack with rejected alternatives.
- [docs/design/sast-rulegen-skill-pack-interfaces.md](sast-rulegen-skill-pack-interfaces.md) — public-API stability contract for downstream milestones.
- [SECURITY.md](../../SECURITY.md) — merged additions under "SAST rule-gen skill pack — additional rules" (existing biz-pack and slo-* sections preserved).
- [docs/design/sast-rulegen-skill-pack-threat-model.md](sast-rulegen-skill-pack-threat-model.md) — STRIDE per component + abuse cases + compliance + AI/LLM threats.
- This file.

## Handoff

Next: `/slo-plan sast-rulegen-skill-pack` (no TLA — `tla_required: false`).
