---
name: cost-baseline-jpp-law-2026
created: 2026-04-25
retrieved: 2026-04-25
valid_through_suggestion: 2027-04-25
source: https://www.jpplaw.co.uk/sectors/fixed-fee-startup/
status: evolving — annual refresh expected when JPP Law publishes the 2027-28 fixed-fee schedule
purpose: |
  Publicly auditable UK fixed-fee solicitor pricing baseline for ROI claims in advisor-skill outputs.
  Cited by `/slo-legal` (and other advisor skills as relevant) in the `cost_baseline_ref` frontmatter
  field of every drafted artifact, plus inline in the artifact body's footer.
---

# UK fixed-fee solicitor cost baseline — JPP Law (2026-2027)

> Retrieval date: **2026-04-25**.
> Source: https://www.jpplaw.co.uk/sectors/fixed-fee-startup/
> Provenance note: JPP Law publishes fixed-fee startup pricing publicly; this file snapshots the relevant line items so the ROI claim in advisor-skill outputs is auditable. JPP Law's pricing changes annually; treat this snapshot as valid through ~12 months from the retrieval date and refresh on each `/loop` cadence (recommended schedule below).

## Why JPP Law and not another firm

The dossier in `docs/research/biz-skill-pack/dossier.md` initially anchored ROI claims to Russell Cooke's 2026-27 price list — which is **not publicly retrievable** (no PDF or page on russell-cooke.co.uk matches). For the cost claim to be auditable by anyone reading the skill output (and for the file to ship in an open repository without redistributing a private firm's pricing schedule), the baseline was switched to JPP Law's publicly-itemised page. **Locked decision: 2026-04-25.**

## Cost line items (UK, fixed-fee)

The line items below are the v1 advisor-skill output set. JPP Law's page covers more services than these; only the items advisor skills cite for ROI claims in v1 are snapshotted here. Future milestones (M2 `/slo-accounting`, M3 `/slo-equity`, M4 `/slo-fundraise`) may add their own relevant lines via this file's "Adding a new line" section below.

| Item | Price (GBP, ex VAT) | Use in advisor skill | Source line |
|---|---|---|---|
| Non-Disclosure Agreement (UK template) | placeholder — replace with current JPP Law fixed fee at retrieval time | `/slo-legal draft nda` ROI block | jpplaw.co.uk/sectors/fixed-fee-startup/ |
| Standard Contractor Agreement (consultant / contractor) | placeholder — replace with current JPP Law fixed fee at retrieval time | `/slo-legal draft contractor-sow` ROI block | jpplaw.co.uk/sectors/fixed-fee-startup/ |
| Intellectual Property Assignment | placeholder — replace with current JPP Law fixed fee at retrieval time | `/slo-legal draft ip-assignment` and embedded inside contractor SOW | jpplaw.co.uk/sectors/fixed-fee-startup/ |
| Terms & Conditions for sale of goods or provision of services | placeholder — replace with current JPP Law fixed fee at retrieval time | `/slo-legal draft terms-and-conditions` ROI block | jpplaw.co.uk/sectors/fixed-fee-startup/ |
| Shareholders Agreement (cofounders) | placeholder — replace with current JPP Law fixed fee at retrieval time | `/slo-equity` (M3) cofounder-split brief ROI block | jpplaw.co.uk/sectors/fixed-fee-startup/ |
| Articles of Association | placeholder — replace with current JPP Law fixed fee at retrieval time | `/slo-equity` (M3) cap-table briefing ROI block | jpplaw.co.uk/sectors/fixed-fee-startup/ |
| Employment Contract | placeholder — replace with current JPP Law fixed fee at retrieval time | future Runbook C M2 `/slo-hire` ROI block (out of Runbook A scope) | jpplaw.co.uk/sectors/fixed-fee-startup/ |

> **Implementation note**: This file ships in M1 with placeholders in the price column because the actual GBP figures must be retrieved from the live JPP Law page at implementation time and cannot be reliably reproduced from third-party summaries. A founder running M1 implementation MUST visit https://www.jpplaw.co.uk/sectors/fixed-fee-startup/, copy the current fixed-fee figures into this table, update the `retrieved:` frontmatter date, and commit. The structural-contract test `cost_baseline_md_carries_retrieval_date` in `e2e_biz_a_m1.rs` checks the retrieval-date format; a separate manual smoke-test step (in `docs/verify/biz-a-m1-smoke.md`) checks that the placeholders have been replaced with real GBP figures.

## How advisor skills cite this file

Every advisor skill drafted artifact MUST include in its frontmatter:

```yaml
cost_baseline_ref: references/biz/cost-baseline-jpp-law-2026.md@<retrieved-date>
```

And in the body footer:

```markdown
## Cost baseline (provenance)

Per [JPP Law fixed-fee startup pricing](https://www.jpplaw.co.uk/sectors/fixed-fee-startup/), retrieved <retrieved-date>: a UK solicitor would charge approximately £<X> for an equivalent <doc-type> drafted from scratch. This advisor-skill draft is **NOT** a substitute for solicitor review — see the `lawyer_review_recommended: true` flag in the frontmatter. Cost reference snapshot: `references/biz/cost-baseline-jpp-law-2026.md`.
```

## Adding a new line item (M2+)

When a future advisor skill needs a cost line not in the table above, the implementing milestone:

1. Confirms JPP Law publishes a fixed fee for the line. (If not, cite a different publicly-auditable source — gov.uk filing fees for HMRC-required filings, ICO registration fees, etc. — or omit the cost block for that artifact.)
2. Adds the line to the table above with the current price + retrieval date.
3. Updates the `retrieved:` frontmatter date if the entire snapshot is being refreshed.
4. Updates this file's commit reference; downstream advisor SKILL.md files that need the new line update their own citations.

## Why a single snapshot, not a live fetch

Advisor skills do NOT enable WebFetch / WebSearch — see `references/biz/jurisdiction-uk.md` and `SECURITY.md` (root, biz section) for the rationale (founder personal data in prompt context + WebFetch creates an exfiltration surface). The cost baseline is a static snapshot refreshed annually. Founders running advisor skills more than ~12 months after the retrieval date should be warned by the skill prose to refresh this file before relying on the ROI numbers.

## Recommended `/loop` schedule

```
/loop @yearly /slo-research "refresh JPP Law cost baseline for biz-skill-pack — fetch jpplaw.co.uk/sectors/fixed-fee-startup/ and update references/biz/cost-baseline-jpp-law-2026.md retrieval date and price column"
```

This is offered to the founder at M1 completion (per the runbook's auto-mode `/schedule` follow-up convention).
