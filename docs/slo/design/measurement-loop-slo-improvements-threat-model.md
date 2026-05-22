---
name: measurement-loop-slo-improvements-threat-model
created: 2026-05-22
slug: measurement-loop-slo-improvements
compliance: [soc2, asvs, gdpr]
ai_component: false
status: planned
---

# Threat Model — measurement-loop-slo-improvements

> Top risks were not captured by a fresh `/slo-ideate` Q7 pass — the input is the deep-research report, not an
> idea doc. Conservative defaults used here, anchored to the report's own privacy section (ICO PECR /
> pseudonymisation / DPIA) and to SLO's existing template-injection and PII-leak controls.

## Scope and trust boundaries

This work changes **Markdown skill contracts, the v4 template, one frontmatter key, loop docs, and Rust
structural-test baselines**. It ships **no runtime telemetry collector** — it *generates measurement
contracts* that a downstream target product would implement. The trust-relevant surfaces are therefore:

1. **User-provided strings** (success-thesis text, feature names, metric labels, privacy notes) that flow
   from idea docs / product-metrics artifacts into generated runbook sections.
2. **Generated guidance that recommends telemetry** — the contract must not nudge teams toward collecting
   more personal data than necessary, or toward unmasked PII in analytics.
3. **Artifacts under `docs/biz-public/`** (git-tracked) that could carry PII if a feature spec quotes real
   users — already the remit of the `/slo-verify` Pass 4 PII scan, now extended to telemetry fields.

## STRIDE sweep (per component)

| Component | Spoofing | Tampering | Repudiation | Info disclosure | DoS | Elevation |
|---|---|---|---|---|---|---|
| Idea-doc `## Success thesis` (Markdown) | N/A — local authored doc | mitigated by — git review; no executable content | N/A | residual risk — author may paste a real customer name; routed to `/slo-verify` PII scan + privacy-controls row | N/A | N/A |
| `/slo-product metrics` feature spec + `feature_measurement_spec` flag | mitigated by — flag is author-set, not user-input-derived; `/slo-verify` cross-checks flag vs presence of spec section | mitigated by — git review | N/A | mitigated by — `tier: public` default + Pass 4 PII/telemetry-field scan | N/A | N/A — flag carries no privilege |
| v4 Measurement Contract section / Contract Block row | N/A | mitigated by — runbook is the execution contract; changes are reviewed in PR | mitigated by — Evidence Log records who/what/when | residual risk — telemetry deliverables list could name PII fields; **eliminated by class** via the mandatory Privacy controls sub-row (pseudonymise + mask + minimise + DPIA trigger) | N/A | N/A |
| `/slo-verify` measurement pass | N/A | mitigated by — read-only heuristic checks, no network, no writes | N/A | mitigated by — the pass *itself* flags unmasked PII + missing masking on telemetry fields | mitigated by — bounded heuristic scan over a bounded file set (same shape as Pass 4) | N/A |
| Template / SECURITY-adjacent string interpolation | mitigated by — user strings rendered into descriptive Markdown only | **eliminated by class** — user-provided strings wrapped in `~~~text` fences in any generated SECURITY/threat-model output (existing SLO rule, reused) | N/A | mitigated by — fenced literal rendering prevents metacharacter interpretation | N/A | **eliminated by class** — generated text cannot select control fields (ids/status/classification) |
| `xtasks/sast-verify` structural baselines | N/A | mitigated by — SHA baseline change is explicit and reviewed | N/A | N/A | N/A | N/A |

## Abuse cases

| ID | Attacker | Attack step | Desired outcome | Control |
|---|---|---|---|---|
| `tm-measurement-loop-abuse-1` | Unwary author / content-injector | Paste a crafted string into a success-thesis or feature-spec field containing Markdown/YAML/HTML metacharacters intended to break out of the generated section or smuggle instructions into a generated SECURITY/threat-model artifact | Template-placeholder injection: alter generated security defaults or inject agent-readable instructions | **Eliminated by class**: all user-provided strings rendered into generated security/threat artifacts are wrapped in `~~~text` fences (reuse of the load-bearing `/slo-architect` fence rule); measurement sections render user text as plain Markdown body only and never select control fields. |
| `tm-measurement-loop-abuse-2` | Privacy-careless team using the framework | Generated measurement contract drives collection of raw user identifiers / unmasked PII into analytics events, or a feature spec quotes a real customer, and the artifact lands under git-tracked `docs/biz-public/` | PII leak to a public remote; PECR/GDPR breach | **Mitigated**: Measurement Contract has a mandatory **Privacy controls** row (pseudonymised event identifiers by default, masking, data minimisation, consent for non-essential cookies, DPIA trigger for behaviour/location tracking). `/slo-verify` measurement pass extends the Pass 4 PII scan to telemetry field definitions; `pii_scan_override` + reason required for any intentional match. |
| `tm-measurement-loop-abuse-3` | Author gaming the gate | Set `feature_measurement_spec: true` (or check the Measurement Contract box) without an actual spec/telemetry, to pass `/slo-plan` / `/slo-verify` gating | False "measured" signal; ship-decision made on absent telemetry | **Mitigated**: `/slo-verify` measurement pass cross-checks the flag against the actual presence of the spec section and the failure-path emission check; `/slo-retro` "Results vs thesis" refuses on blank actuals (existing rule). Honest-failure surfacing, not silent pass. |

## Compliance mapping

| Concern | SOC 2 | ASVS | GDPR |
|---|---|---|---|
| Telemetry PII minimisation | CC6.1 (logical access — least data) | V8 (data protection) | Art. 5(1)(c) data minimisation; pseudonymisation as Art. 25 data-protection-by-design measure (ICO: pseudonymised data is still personal data) |
| Consent for non-essential tracking | CC2.3 (communication of objectives) | V3 (session/consent handling where applicable) | PECR reg. 6 — active consent for non-essential cookies/similar; strictly-necessary exemption only |
| High-risk processing assessment | CC3.2 (risk assessment) | — | DPIA required where processing is likely high risk (innovative tech, behaviour/geolocation tracking); DPIA is a living process feeding the project plan |
| Replay/session tooling masking | CC6.7 (transmission/handling) | V8.3 (sensitive data in client) | Default masking of input content, numbers, emails; unmasking only as an explicit, justified action |
| Audit/repudiation of ship decisions | CC7.2 / CC7.3 (monitoring) | V7 (logging) | Art. 5(2) accountability — Evidence Log + Results-vs-thesis record |

## Residual risks (accepted)

- **R1 — Author can still hand-author a thin or dishonest measurement contract.** Accepted: the framework
  forces the *shape* and the cross-checks, not the team's integrity; `/slo-verify` + `/slo-retro` raise the
  cost of faking but cannot fully prevent it. Tracked as `tm-measurement-loop-abuse-3` mitigation, not
  elimination.
- **R2 — This work generates guidance; it does not enforce the target product's runtime privacy.** Accepted
  and explicit: SLO ships no telemetry runtime. The contract makes the right defaults the path of least
  resistance and routes direct-marketing/cookie questions to `/slo-legal triage` (PECR) per existing biz-pack
  wiring.
