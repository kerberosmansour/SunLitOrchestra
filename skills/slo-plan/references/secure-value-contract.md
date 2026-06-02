---
name: slo-plan-secure-value-contract
source_skill: skills/slo-plan/SKILL.md
---

# Secure Value & Security Contract (§5B) authoring detail

`/slo-plan` requires the §5B Secure Value and Security Contract for any
**value-bearing OR security-relevant** milestone. "Security-relevant" is
deterministic: the work touches identity, secrets, PII, payment, cloud accounts,
AI agents, public/network boundaries, CI/CD, or infrastructure. Canonical
definition: [docs/SECURE-VALUE-LOOP.md](../../../docs/SECURE-VALUE-LOOP.md).

Populate the five §5B sub-blocks:

- **Value Wedge** — value hypothesis, smallest valuable wedge, user-visible proof
  of value, security-visible proof of safety, "what would make this too small".
- **Security Definition of Ready (Operator Readiness)** — each prerequisite
  (cloud account / OAuth app / API key / test device / DNS / cert / approval)
  with owner (`human | agent | upstream`), needed-by milestone, an **executable**
  validation, and status; plus `safe_to_continue_without_blockers: true | false`.
  **Inert-window note (F-ENG-3):** the Operator Readiness Gate is *enforced* by
  `/slo-execute`'s Global Entry from the **M3** release of the Secure Value Loop
  onward — the generated §5B MUST state this so an author does not assume earlier
  enforcement.
- **Threat Model Summary** — cite the existing `/slo-architect` threat model
  (`docs/slo/design/<slug>-threat-model.md` + `.slo.json`) and its frozen
  `tm-<slug>-abuse-N` IDs; do not re-derive.
- **Security Test Plan** — reference the security-test Bundle(s) (A docs / B app /
  C backend-API / D cloud-IaC / E AI-LLM / F mobile) the surface triggers;
  SBOM/provenance stays conditional (`not_applicable` for non-release-artifact
  work).
- **Detected Work Ledger** — initialize the ledger; the five dispositions
  (`fix_now | file_github_issue | operator_action | upstream_feedback |
  accepted_risk`) route to existing `/slo-retro` lanes — introduce **no new lane
  verb**.

Cite proactive controls **by OWASP Proactive Controls 2024 name** (e.g.
`C1 Implement Access Control`), never a bare number (OWASP renumbered C1–C10
between 2018 and 2024).

**Forward-looking, not retroactive.** If a milestone reaches completion without a
required §5B, **flag the gap** and require it before handoff — but do **not**
retroactively invalidate legacy runbooks authored before §5B existed (they remain
valid; mirrors the §5A Measurement Contract and §10 Carry-forward backward-compat
posture). For pure refactor / docs / tooling with no security-relevant surface,
mark §5B `N/A — not value-bearing or security-relevant, see <reason>`.
