---
name: secure-value-loop
researched: 2026-06-02
---

# Sources — Secure Value Loop

All external URLs accessed 2026-06-02.

## OWASP

- OWASP Top 10 Proactive Controls 2024 — the Top 10 (exact C1–C10 names) — https://top10proactive.owasp.org/archive/2024/the-top-10/
- OWASP Top 10 Proactive Controls — project home — https://top10proactive.owasp.org/
- "What's new in the OWASP Proactive Controls for 2024" (Secure Ideas) — confirms 2018→2024 reorganisation — https://www.secureideas.com/blog/whats-new-in-the-owasp-proactive-controls-for-2024
- OWASP MASVS (mobile) — https://mas.owasp.org/MASVS/
- OWASP API Security Top 10 (2023) — https://owasp.org/API-Security/
- OWASP API Security Top 10: 2023 update explained — https://www.practical-devsecops.com/owasp-api-security-top-10/
- OWASP Top 10 for LLM Applications (2025), explained — agentic-aware update — https://securityboulevard.com/2026/03/the-owasp-top-10-for-llm-applications-2025-explained-simply/
- OWASP Top 10 for LLMs v0.5 milestone (2023 origin) — https://genai.owasp.org/2023/05/04/owasp-top-10-for-llms-hits-major-milestone-with-release-of-v0-5/

## NIST

- NIST SP 800-218 (SSDF) v1.1 — final — https://csrc.nist.gov/pubs/sp/800/218/final
- NIST SP 800-218 full PDF — https://nvlpubs.nist.gov/nistpubs/specialpublications/nist.sp.800-218.pdf
- NIST SP 800-218A — Secure Software Development Practices for Generative AI and Dual-Use Foundation Models — final — https://csrc.nist.gov/pubs/sp/800/218/a/final
- NIST SSDF project (CSRC) — https://csrc.nist.gov/projects/ssdf

## Supply chain (SLSA / SBOM)

- SLSA Framework Explained: build provenance, levels, attestation — https://secportal.io/blog/slsa-framework-explained
- From SBOM to SLSA: securing your software supply chain (SBOM vs SLSA, EO 14028 / EU CRA mandates) — https://petronellatech.com/blog/from-sbom-to-slsa-securing-your-software-supply-chain/
- Where the SLSA 1.0 Release Shines (and Its Limitations) — https://finitestate.io/blog/where-the-slsa-1.0-release-shines-and-its-limitations

## Cited operating models

- OpenAI Symphony — repo — https://github.com/openai/symphony
- OpenAI Symphony — SPEC.md (handoff states, isolated workspace, bounded concurrency/retry) — https://github.com/openai/symphony/blob/main/SPEC.md
- OpenAI announcement — open-source Codex orchestration: Symphony — https://openai.com/index/open-source-codex-orchestration-symphony/
- GStack ("Think → Plan → Build → Review → Test → Ship → Reflect") — **no canonical public source located**; cited only via the proposal `~/Downloads/sunlit_orchestra_secure_value_loop(1).md`. SLO already runs this chain (see CLAUDE.md sprint flow).

## Repo-local prior art (read 2026-06-02)

- `docs/slo/research/slo-security-embedding/{dossier,sources,synthesis}.md`
- `docs/slo/research/secure-execution-controls/synthesis.md`
- `docs/slo/research/slo-threat-model/synthesis.md`
- `docs/slo/templates/runbook-template_v_4_template.md` (Contract Block §17, Measurement Contract §5A, Carry-forward §10, status enum line 62)
- `skills/slo-architect/SKILL.md` (Step 3.5 threat-model emission), `skills/slo-critique/SKILL.md` (security persona), `skills/slo-verify/SKILL.md` (Pass 4/5), `skills/slo-retro/SKILL.md` + `references/issue-filing-discipline.md`, `skills/slo-plan/SKILL.md`, `skills/slo-execute/SKILL.md`
- `docs/LOOPS-ENGINEERING.md`, `docs/LOOPS-BUSINESS.md`
- Completion summaries: `docs/slo/completion/{slo-sec-m1..4, slo-threat-model-m1..2, slo-threat-model-producer-m1..2, sec-libs-m1..5, sast-rulegen-a-*, scanner-orch-m1..5, nettacker-hardening-m1}.md`
- The proposal under research: `~/Downloads/sunlit_orchestra_secure_value_loop(1).md`
