# Manual smoke-test checklist — Runbook A M1 (`/slo-legal` v1)

> Created: 2026-04-25
> Runbook: [docs/RUNBOOK-BIZ-SKILL-PACK-A.md](../RUNBOOK-BIZ-SKILL-PACK-A.md) Milestone 1
> Skill under test: [skills/slo-legal/SKILL.md](../../skills/slo-legal/SKILL.md)
> Why manual: skill behavior is prompt-driven; structural-contract tests in `crates/sldo-install/tests/e2e_biz_a_m1.rs` cover the static contract, but runtime behavior (does the skill actually refuse the GDPR doc-type, route correctly, write the right artifact?) requires invoking the skill in a real Claude Code session.

## Prerequisites

- [ ] Baseline tests green: `cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install`.
- [ ] Structural tests green: `cargo test -p sldo-install --test e2e_biz_a_m1`.
- [ ] `sldo-install --dry-run` shows `slo-legal` discovered; no `references/biz/` entry.
- [ ] `sldo-install` ran (skill installed into `~/.claude/skills/slo-legal/`).
- [ ] `references/biz/cost-baseline-jpp-law-2026.md` placeholders replaced with current JPP Law GBP figures (manual fetch from https://www.jpplaw.co.uk/sectors/fixed-fee-startup/, retrieval date updated). See implementation note in that file.

## Smoke test 1 — Happy path: `/slo-legal draft contractor-sow`

Fixture: hiring `Acme Solo Eng` as contractor, daily rate £400 for 10 days = £4,000 total (under the £5k gate-2 threshold), IP scope "all deliverables and learnings."

Invocation: `/slo-legal draft contractor-sow` with the fixture context provided in the prompt.

Verify:

- [ ] Artifact landed at `docs/biz/legal/contractor-sow-acme-solo-eng-2026-04-25.md` (or similar kebab-slug + date).
- [ ] Founder's repo `.gitignore` excludes `docs/biz/` (skill should warn at write-time if not).
- [ ] Frontmatter:
  - [ ] `name:` is a kebab-slug.
  - [ ] `created:` is today's date.
  - [ ] `tier: confidential`.
  - [ ] `skill: slo-legal`.
  - [ ] `mode: draft`.
  - [ ] `jurisdiction: uk`.
  - [ ] `cost_baseline_ref: references/biz/cost-baseline-jpp-law-2026.md@<retrieval-date>`.
  - [ ] `triage_gate_passed: true`.
  - [ ] No `gates_fired:` list (because `triage_gate_passed: true`).
  - [ ] `lawyer_review_recommended: true`.
  - [ ] `expires_or_review_by:` is present and approximately 12 months out.
- [ ] Body opens with a "**LAWYER REVIEW RECOMMENDED**" header.
- [ ] Body contains an explicit IP-assignment clause (the load-bearing UK-law clause).
- [ ] Body footer contains the JPP Law cost-baseline ROI block citing `https://www.jpplaw.co.uk/sectors/fixed-fee-startup/` and the retrieval date.

## Smoke test 2 — Gate-4 GDPR hard-block: `/slo-legal draft privacy-notice`

Fixture: any context describing a B2C app processing personal data.

Invocation: `/slo-legal draft privacy-notice`.

Verify:

- [ ] Skill REFUSES to draft.
- [ ] Output landed at `docs/biz-public/legal/triage-privacy-notice-2026-04-25.md` (or similar).
- [ ] Frontmatter:
  - [ ] `tier: public`.
  - [ ] `mode: triage`.
  - [ ] `triage_gate_passed: false`.
  - [ ] `gates_fired: [gate-4-gdpr-document]`.
  - [ ] `lawyer_review_recommended: true`.
- [ ] Body explicitly cites `gate-4-gdpr-document` by id.
- [ ] Body recommends DPO routing (or lawyer + DPO if no DPO).
- [ ] Body cites `references/biz/ico-duaa-index.md` (M2 — if not yet shipped, body cites the canonical ICO DUAA URL https://ico.org.uk/about-the-ico/what-we-do/legislation-we-cover/data-use-and-access-act-2025/the-data-use-and-access-act-2025-duaa-summary-of-the-changes/ directly).
- [ ] Body includes a "what to bring to your DPO" briefing checklist.

## Smoke test 3 — Multi-gate triage: `/slo-legal triage "deal worth £20k where the other side has a lawyer"`

Fixture: deal value > £5,000 AND counterparty represented by a lawyer.

Invocation: `/slo-legal triage "deal worth £20k where the other side has a lawyer"`.

Verify:

- [ ] Output landed at `docs/biz-public/legal/triage-<slug>-2026-04-25.md`.
- [ ] Frontmatter:
  - [ ] `tier: public`.
  - [ ] `mode: triage`.
  - [ ] `triage_gate_passed: false`.
  - [ ] `gates_fired: [gate-2-deal-value-over-5k, gate-3-counterparty-has-lawyer-or-their-paper]` (or contains both predicates in any order).
  - [ ] `lawyer_review_recommended: true`.
- [ ] Body cites BOTH `gate-2-deal-value-over-5k` AND `gate-3-counterparty-has-lawyer-or-their-paper` by id.
- [ ] Body explains why each predicate fired in the founder's situation.
- [ ] Body includes a "what to bring to your lawyer" briefing checklist tailored to the £20k + counterparty-lawyer situation (negotiation leverage, redline rounds, indemnity caps, etc.).

## Smoke test 4 — Non-UK jurisdiction: `/slo-legal draft contractor-sow` for a US-based contractor

Fixture: founder describes hiring a contractor based in Delaware, US, for work performed in the US.

Invocation: `/slo-legal draft contractor-sow` with the fixture context.

Verify:

- [ ] Skill REFUSES to draft.
- [ ] Skill emits the canonical error "v1 supports UK only; US/EU is a v2 architectural pivot — see [docs/design/biz-skill-pack-overview.md](../design/biz-skill-pack-overview.md) ...".
- [ ] No artifact written — neither `docs/biz/legal/` nor `docs/biz-public/legal/`.
- [ ] No "for reference only" US output is produced.
- [ ] Skill suggests engaging a US-jurisdiction solicitor.

## Smoke test 5 — oneNDA placeholder still present: `/slo-legal draft nda`

Fixture: any context (the placeholder check is independent of the founder's situation).

Invocation: `/slo-legal draft nda`.

Verify (BEFORE the oneNDA canonical bytes have been fetched and replaced):

- [ ] Skill REFUSES to draft.
- [ ] Skill emits the error "oneNDA template not yet populated; see `references/biz/templates/onenda-uk.md` replacement instructions".
- [ ] Skill suggests `/slo-legal triage 'I need an NDA'` and `/slo-legal prepare 'NDA review with my solicitor'` as workarounds.

## Out-of-scope check

- [ ] `/slo-legal draft tos` for a B2C product → skill returns the "B2C T&Cs are out of scope in v1" error per the SKILL.md "Refusal patterns" section.
- [ ] Unknown mode — `/slo-legal foo` → skill returns the "Unknown mode" error listing the four valid modes.

## Notes captured during the smoke test

(Record any deviation from expected behavior, any new ambiguity in the SKILL.md prose that became visible during real invocation, and any improvements that should land in `/slo-execute` follow-up.)

- _<empty until smoke run>_
