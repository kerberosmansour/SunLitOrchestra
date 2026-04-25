---
name: open-template-anchors
created: 2026-04-25
status: stable-interface for the documented license obligations; evolving for the consortium template list
audience: every advisor skill in the biz pack that uses or considers using an open-licensed template
purpose: |
  Index of open-licensed template anchors (oneNDA, oneSaaS, oneDPA, Kindrik Partners / Simmonds Stewart) plus the license obligations attached to each. The CC BY-ND 4.0 verbatim-render rule for oneNDA is the load-bearing rule; everything else is candidate references for M3+ work or future runbooks.
---

# Open-template anchors and license obligations

This file is the citation index for open-licensed legal templates the biz pack uses or considers. The most important rule is the **CC BY-ND 4.0 verbatim render obligation for oneNDA** — it is enforced by the `/slo-legal draft nda` placeholder mechanism and the structural-contract test `onenda_template_placeholder_or_pinned_hash` in `crates/sldo-install/tests/e2e_biz_a_m1.rs`.

## TLB consortium (TheLawBoutique) — oneNDA family

| Template | Status | License | Used by |
|---|---|---|---|
| **oneNDA** | Live (2021) | **CC BY-ND 4.0** | `/slo-legal draft nda` — M1 ships placeholder at `references/biz/templates/onenda-uk.md`; canonical bytes deferred |
| **oneSaaS** | Live (2023) | CC (same family — verify SPDX at retrieval time) | Candidate anchor for `/slo-legal draft terms-and-conditions` SaaS variant in M3+ work |
| **oneDPA** | Live (2024) | CC (same family — verify SPDX at retrieval time) | OUT OF SCOPE for biz pack v1 — broad GDPR hard-block on `draft` (gate-4) refuses DPA generation. Documented here for future-runbook reference only. |

### CC BY-ND 4.0 verbatim render obligation (oneNDA)

The oneNDA license is **CC BY-ND 4.0** — Attribution + No Derivatives. Key clauses:

- **Attribution**: cite the consortium and the template URL.
- **No Derivatives**: do NOT modify the template body. "Derivative" in this license means any altered version, including:
  - Editing prose ("fixing typos", "modernizing the language")
  - Removing clauses
  - Adding clauses inline
  - Translating to another language without consortium permission

**Implementation in this skill pack**:

1. The canonical oneNDA UK Country Schedule body is rendered **byte-for-byte unmodified** as the body of `docs/biz/legal/nda-<counterparty>-<date>.md`.
2. Company-specific fields (parties, effective date, governing law jurisdiction selection where the template offers a choice, return-of-materials timeline) are emitted in a SEPARATE artifact at `docs/biz/legal/nda-cover-<counterparty>-<date>.md` that wraps but does not edit the canonical body.
3. The cover artifact's frontmatter carries `template_source: https://www.onenda.org/` and `template_license: CC-BY-ND-4.0` per the artifact-schema.
4. The structural test `onenda_template_placeholder_or_pinned_hash` in `e2e_biz_a_m1.rs` enforces the placeholder marker OR a future-pinned canonical-bytes SHA-256 — the non-modification invariant.
5. `/slo-verify` regression test (deferred to a follow-up runbook) will assert the rendered NDA body bytes match the pinned hash on every fresh draft.

### oneSaaS — candidate for M3+ T&Cs work

oneSaaS is the consortium's UK SaaS T&Cs template. License is the same family as oneNDA (verify SPDX at retrieval time before adopting). For `/slo-legal draft terms-and-conditions` (currently doc-type B2B-only in v1), oneSaaS is a candidate anchor in a future milestone where T&Cs work expands beyond v1's narrow B2B scope. Until then, `/slo-legal draft terms-and-conditions` produces a generic UK B2B template authored from skill prose, NOT a oneSaaS-derivative.

### oneDPA — out of scope (broad GDPR hard-block)

oneDPA is the consortium's GDPR DPA template. Because gate-4-gdpr-document hard-blocks `draft` for ALL GDPR-related documents (locked 2026-04-25), the biz pack does NOT use oneDPA as a draft source. Documented here so a future runbook performing a GDPR-rule reversal has the anchor in hand.

## Kindrik Partners / Simmonds Stewart — NZ-law open-licensed startup templates

| Template family | License | Used by |
|---|---|---|
| Contractor / consultant agreements | CC (verify SPDX) | Structural reference for `/slo-legal draft contractor-sow` UK-rewrite — ~80% structural overlap; UK governing-law and statutory-citation rewrites required |
| IP assignment agreements | CC (verify SPDX) | Structural reference for `/slo-legal draft ip-assignment` UK-rewrite |
| Cofounder agreements / shareholders agreements | CC (verify SPDX) | Structural reference for `/slo-equity` (M3) cofounder split — same UK-rewrite caveat |

**Important caveat**: Kindrik / Simmonds Stewart templates are drafted under New Zealand law. Adapting them to England & Wales requires:

1. Replace governing-law clauses (NZ → E&W).
2. Replace statutory citations (e.g., NZ Contract and Commercial Law Act 2017 → English common law of contract; NZ Privacy Act 2020 → UK GDPR + DPA 2018; NZ Holidays Act 2003 → WTR 1998).
3. Replace dispute-resolution venue (NZ courts → E&W courts).
4. Verify case-law references — UK case law (Carlill v Carbolic Smoke Ball, Hadley v Baxendale) is not the same as NZ case law.

The biz pack does NOT auto-render Kindrik / Simmonds Stewart templates. The skill prose may CITE them as structural references for the founder to compare against, but `/slo-legal draft contractor-sow` produces a UK-original template authored from skill prose.

## ai-legal-claude (community Claude skill — prior art reference)

| Project | URL | License | Used by |
|---|---|---|---|
| ai-legal-claude (zubair-trabzada) | https://github.com/zubair-trabzada/ai-legal-claude | (community — verify) | Inspiration / prior-art reference; not a dependency |

The biz pack's `/slo-legal` differs from ai-legal-claude:

- UK-jurisdiction-only (ai-legal-claude is jurisdiction-agnostic / US-leaning).
- Advisor pattern with hard-block triage gate (ai-legal-claude is generator-only).
- Integrated with the SLO loop (ai-legal-claude is a standalone skill).

Documented here to acknowledge the prior art and to make the differentiation explicit; no shared code or prose.

## Refresh cadence

- **Annual**: re-retrieve oneNDA / oneSaaS / oneDPA license terms in case the consortium revises the SPDX or template content.
- **Triggered**: re-retrieve immediately when the consortium publishes a new version of any cited template (the canonical-bytes hash for oneNDA would shift).
- **License changes**: any change in oneNDA's license away from CC BY-ND 4.0 invalidates the verbatim-render mechanism in `/slo-legal draft nda`; would require a fresh `/slo-architect` pass.
