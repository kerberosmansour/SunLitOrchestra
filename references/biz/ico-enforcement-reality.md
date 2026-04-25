---
name: ico-enforcement-reality
created: 2026-04-25
retrieved: 2026-04-25
status: descriptive — NOT normative
audience: every advisor skill in the biz pack (citation source for the broad GDPR hard-block's provenance)
purpose: |
  Descriptive provenance for the broad GDPR hard-block on `draft` mode (locked 2026-04-25).
  Documents the ICO enforcement pattern for sub-£1M-turnover private companies, Apr 2024 – Apr 2026,
  showing PECR-direct-marketing is the dominant small-company enforcement surface (not Article 13 / privacy-notice).
sources:
  - https://www.bdo.co.uk/en-gb/insights/advisory/risk-and-advisory-services/trends-in-recent-ico-enforcement-action
  - https://www.urmconsulting.com/blog/analysis-of-fines-imposed-by-the-information-commissioners-office-in-2024
  - https://www.urmconsulting.com/blog/analysis-of-ico-enforcement-action-january-june-2025
  - https://www.lewissilkin.com/en/insights/2024/05/30/the-ico-fines-again-a-round-up-of-fines-issued-by-the-ico-in-2024
  - https://measuredcollective.com/ico-enforcement-in-2025-record-fines-and-what-they-mean/
---

# ICO enforcement reality for sub-£1M-turnover private companies

> **DESCRIPTIVE NOT NORMATIVE.** This file documents the realistic enforcement pattern as of the retrieval date. It is provenance for the broad GDPR hard-block on `draft` mode (locked 2026-04-25), NOT authorization to relax the gate. Reversal of the broad-block decision requires a fresh `/slo-architect` pass with new evidence. Skill prose drift testing in `crates/sldo-install/tests/e2e_biz_a_m2.rs` enforces the non-normative posture by asserting a list of forbidden phrases is NOT present in this file (the test source is the canonical list — keeping the list out of this doc avoids the circular self-trip).

## Headline

**ICO enforcement against sub-£1M-turnover private companies between April 2024 and April 2026 is PECR-direct-marketing-dominated, not UK-GDPR-Article-13-privacy-notice-dominated.** Standalone "bad privacy notice" enforcement against this segment in this window is effectively zero.

## 2024 totals (full year)

- 62 enforcement actions against 47 organisations.
- Composition: 18 monetary fines, 29 reprimands, 15 enforcement notices.
- Total fines: ~£2.7M.
- Range: £7,500 – £750,000.
- Average: ~£153,722.
- Sector skew: of 32 UK GDPR cases, 27 went to public sector, only 4 to private. Smallest 2024 fine (Central YMCA, £7,500) was a national charity, not a startup. Largest were public sector (PSNI £750k, MoD £350k).
- PECR vs UK GDPR fine split (2024): PECR ≈ £1.6M; UK GDPR ≈ £1.1M.

## 2025 trajectory (H1)

- 15 enforcement actions in H1.
- Total £ collected: 7× all of 2024.
- Average fine: > £2.8M.
- Two-thirds were UK GDPR breaches (vs one-sixth in 2024).
- Implication: ICO direction-of-travel is fewer fines, larger size, weighted toward public sector / large private (regulated industries).

## Sub-£1M-turnover private-company segment specifically

Monetary penalties on this segment in the Apr 2024 – Apr 2026 window cluster at PECR direct-marketing actions:

| Subject | Fine | Surface |
|---|---|---|
| AFK | £90k | PECR direct marketing |
| Skean Homes | £100k | PECR direct marketing |
| LADH | £50k | PECR direct marketing |
| Pinnacle Life | £80k | PECR direct marketing |
| Dr Telemarketing | £100k | PECR direct marketing |
| Poxell | £150k | PECR direct marketing |

**Standalone "bad privacy notice / Article 13 transparency" enforcement against small private companies in this window: effectively zero.**

ICO audit themes (Aug 2023 – Jan 2024) flagged Article 13 transparency failures repeatedly — fair processing notices judged insufficiently detailed on lawful bases, recipients, international transfers, retention. ICO state every business "however small" needs a privacy notice. But the audit-theme criticism has not translated into monetary penalties on the small-company segment.

## Why the broad GDPR hard-block (gate-4) is still the right call

The enforcement evidence above is **conservative versus what the gate enforces**. Two reasons the broad block stands:

1. **Professional-negligence framing.** A skill that drafts a privacy notice citing the wrong lawful basis transfers the negligence risk from the founder to the skill author / open-source project. Even with `lawyer_review_recommended: true`, founders ship without review more often than not. The downside is asymmetric — a founder who ships a defective notice and is later challenged loses something the skill never had visibility into. Refusing to draft is the only stance that preserves the skill's defensibility.

2. **DUAA 2025 PECR-ceiling raise to £17.5M / 4% global turnover** (commenced Stage 3 on 2026-02-05). Pre-DUAA, the PECR ceiling was £500k — a substantial fine but not catastrophic. Post-DUAA, the ceiling is unbounded for a small company in a way that prudent risk management cannot accept. The enforcement frequency is currently small-company-friendly; the magnitude per event is not.

## Counter-argument (documented for completeness, NOT endorsed)

A narrower posture — `draft` permitted for privacy notices but hard-blocked for direct-marketing-related templates (PECR consent forms, marketing communication scripts) — is also defensible on the enforcement evidence. **This counter-argument is documented here so a future `/slo-architect` re-pass has a starting point**; it is NOT a current configuration option, and skill prose MUST NOT implement the narrower posture.

## Refresh cadence

- **Annual**: re-retrieve and update `retrieved:` frontmatter.
- **Triggered**: re-retrieve when ICO publishes a major enforcement notice against a sub-£1M-turnover private company that is NOT PECR-direct-marketing — that would be a signal that the segment-skewed enforcement pattern is shifting.
- **Source-update mechanism**: the four enforcement-trend analyses cited in the frontmatter are each updated annually (URM Consulting, BDO, Lewis Silkin) or quarterly (Measured Collective). Refresh from any one of them; cross-check against the others.

## What this file is NOT

- This file is NOT a recommendation to weaken `gate-4-gdpr-document`.
- This file is NOT a substitute for ICO guidance.
- This file is NOT a complete enforcement register — it is a curated summary of the small-private-company segment.
