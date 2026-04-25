# Business-Side Skill Pack, Runbook B1 — Discovery → Strategy → Definition Generators (AI-First Runbook v3)

> **Purpose**: Ship the four customer-facing generator skills (`/slo-talk-to-users`, `/slo-gtm`, `/slo-product`, `/slo-marketing`) that take a UK seed-stage founder from user discovery through GTM strategy and product definition into marketing tactics. Plus: integrate the deferred `/slo-verify` Pass 4 PII-pattern scan over `docs/biz-public/` (deferred from Runbook A M1 lessons file).
> **Audience**: AI coding agents first, humans second.
> **How to use**: Work through milestones sequentially. Generators are simpler than advisors — no four-mode contract, no hard-block gates, no per-skill regulator routing. The advisor pattern's `triage-gate.md` predicates are NOT applied to generators because no GENERATOR draft is professional-required-document territory.
> **Prerequisite reading**: [docs/RUNBOOK-BIZ-SKILL-PACK-A.md](RUNBOOK-BIZ-SKILL-PACK-A.md) (now merged), [docs/design/biz-skill-pack-overview.md](design/biz-skill-pack-overview.md), [docs/design/biz-skill-pack-interfaces.md](design/biz-skill-pack-interfaces.md), [docs/design/biz-skill-pack-threat-model.md](design/biz-skill-pack-threat-model.md), [SECURITY.md](../SECURITY.md) (root, biz section), [references/biz/artifact-schema.md](../references/biz/artifact-schema.md).

---

## Runbook Metadata

- **Runbook ID**: `biz-skill-pack-b1`
- **Prefix for test files and lessons files**: `biz-b1`
- **Primary stack**: Markdown `SKILL.md` prompt files under `skills/slo-<biz>/` + Markdown shared references already at `references/biz/`. Secondary: Rust 2021 workspace for structural-contract tests under `crates/sldo-install/tests/e2e_biz_b1_m<N>.rs`. Plus M1 only: edits to `skills/slo-verify/SKILL.md` to land the deferred PII-pattern scan.
- **Default test commands**: same as Runbook A.
  - Backend: `cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install`
  - E2E backend: `cargo test -p sldo-install --test e2e_biz_b1_m1 --test e2e_biz_b1_m2 --test e2e_biz_b1_m3 --test e2e_biz_b1_m4`
  - Build/boot: `cargo build -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install`
- **Allowed new dependencies by default**: `none`. Tests reuse stdlib + `tempfile`.
- **Schema/config migration allowed by default**: `no`.
- **Public interfaces that must remain stable**:
  - All existing Runbook A interfaces (4 advisor skill verbs, 4 triage-gate predicate IDs, 10 shared references, two-tier output convention, frontmatter contract, oneNDA placeholder mechanism).
  - NEW skill verbs added by this runbook: `/slo-talk-to-users`, `/slo-gtm`, `/slo-product`, `/slo-marketing`.
  - NEW `/slo-verify` Pass 4 PII-pattern scan rule for `docs/biz-public/`.
  - NEW artifact frontmatter additions for generators: `archetype: generator` (vs `advisor`); `mode_arg` field for skills with mode args (`/slo-product` modes `roadmap|metrics|okrs`; `/slo-marketing` modes `b2b|b2c`).

---

## Milestone Tracker

| # | Milestone | Status | Started | Completed | Lessons File | Completion Summary |
|---|---|---|---|---|---|---|
| 1 | `/slo-talk-to-users` + deferred `/slo-verify` Pass 4 PII-pattern scan integration | `done` | 2026-04-25 | 2026-04-25 | [docs/lessons/biz-b1-m1.md](lessons/biz-b1-m1.md) | [docs/completion/biz-b1-m1.md](completion/biz-b1-m1.md) |
| 2 | `/slo-gtm` (ICP / segmentation / motion choice / channel strategy) | `done` | 2026-04-25 | 2026-04-25 | [docs/lessons/biz-b1-m2.md](lessons/biz-b1-m2.md) | [docs/completion/biz-b1-m2.md](completion/biz-b1-m2.md) |
| 3 | `/slo-product` (mode arg: roadmap \| metrics \| okrs; north-star metric, activation funnel, retention, RICE/Kano prioritisation) | `done` | 2026-04-25 | 2026-04-25 | [docs/lessons/biz-b1-m3.md](lessons/biz-b1-m3.md) | [docs/completion/biz-b1-m3.md](completion/biz-b1-m3.md) |
| 4 | `/slo-marketing` (mode arg: b2b \| b2c; brand voice, content calendar, channel mix, demand gen) | `done` | 2026-04-25 | 2026-04-25 | [docs/lessons/biz-b1-m4.md](lessons/biz-b1-m4.md) | [docs/completion/biz-b1-m4.md](completion/biz-b1-m4.md) |

---

## End-to-End Architecture Diagram

See [docs/design/biz-skill-pack-overview.md](design/biz-skill-pack-overview.md) for the full diagram. Runbook B1 fills the "Discovery → Strategy → Definition" cluster of the pack diagram. Solid lines exist after Runbook A merge; dashed lines added by this runbook.

```
existing solid (post-Runbook-A):
  skills/slo-{legal,accounting,equity,fundraise}/   advisor cluster
  references/biz/                                    10 shared reference files
  skills/slo-{ideate,architect,plan,critique,...}/   14 pre-runbook skills

added by Runbook B1 (dashed):
  skills/slo-talk-to-users/         M1 — generator, output: docs/biz/users/<date>-<name>.md (confidential)
  skills/slo-gtm/                   M2 — generator, output: docs/biz-public/gtm/strategy.md (public)
  skills/slo-product/               M3 — generator, mode arg: roadmap|metrics|okrs
  skills/slo-marketing/             M4 — generator, mode arg: b2b|b2c
  skills/slo-verify/SKILL.md        M1 — Pass 4 sub-step: PII-pattern scan over docs/biz-public/
```

---

## High-Level Design for Formal Verification (TLA+ Section)

**N/A** — same justification as Runbook A. Markdown skill authoring + reference-doc reads + per-skill artifact writes. No concurrency, no consensus, no leases.

---

## Global Execution Rules

Same five rules as Runbook A's Global Execution Rules. Specific additions for B1:

- **Generators do NOT cite the four predicate IDs from `references/biz/triage-gate.md`.** That file is the advisor cluster's interface; generators are out of scope. Structural tests in `e2e_biz_b1_m<N>.rs` MUST NOT assert predicate-id citation in generator SKILL.mds (that test is advisor-only).
- **Generators DO cite `references/biz/artifact-schema.md` for frontmatter contract.** Output frontmatter MUST include `archetype: generator` to distinguish from advisor outputs.
- **`/slo-talk-to-users` is the first PII-shaped generator.** Its outputs land in `docs/biz/users/` (confidential, gitignored). The M1 PII-pattern scan in `/slo-verify` Pass 4 is the SECOND-line defense against accidental commits to `docs/biz-public/`.

---

## Background Context

### Current State (post-Runbook-A merge)

- 4 advisor skills shipped (`/slo-legal`, `/slo-accounting`, `/slo-equity`, `/slo-fundraise`).
- 10 shared reference files at `references/biz/` (triage-gate, cost-baseline-jpp-law-2026, artifact-schema, jurisdiction-uk, ico-duaa-index, ico-enforcement-reality, open-template-anchors, hmrc-vcm-index, ir35-cest-factors, plus templates/onenda-uk.md placeholder).
- 42 structural-contract tests green at `crates/sldo-install/tests/e2e_biz_a_m{1,2,3,4}.rs`.
- `/slo-verify` Pass 4 currently does supply-chain + variant-analysis + conditional DAST (per slo-security-embedding M4). It does NOT yet scan `docs/biz-public/` for PII patterns — deferred from Runbook A M1 to this milestone.

### Problem

1. **Customer-facing skill cluster is missing.** Founders running the advisor cluster have no scaffolding for the user discovery / GTM strategy / product definition / marketing tactics work that precedes legal + accounting + equity + fundraise concerns. Today, the founder context-switches out of SLO for these.
2. **`/slo-talk-to-users` is the FIRST PII-shaped generator.** Interview transcripts contain real persons' names + emails. The two-tier output convention (`docs/biz/` confidential vs `docs/biz-public/` public) is enforced by skill prose; runtime enforcement (the PII scan) was deferred from Runbook A M1.
3. **`/slo-product` and `/slo-marketing` carry a mode arg** that the current frontmatter contract doesn't formally declare. The artifact-schema reference needs a small addition (`mode_arg` optional field) so the schema is honest about how generator outputs are distinguished.

### Key Design Principles

- **Generators are one-shot.** No mode contract beyond optional `mode_arg`. No hard-block gates. No regulated-document territory.
- **PII scan is conservative.** Pattern set: email regex (RFC 5321 simplified), UK NI numbers (e.g., `AB123456C`), UK sort codes (`NN-NN-NN`), basic named-person heuristics (capitalised-bigram preceded by "name:"). False-positive tolerance is HIGH (better to flag than miss); founders can override per-artifact via a `pii_scan_override: true` frontmatter flag with a documented reason.
- **`/slo-talk-to-users` outputs default to `tier: confidential`.** Per artifact-schema; no override unless the founder explicitly anonymises and provides a `tier_override_reason` frontmatter line.
- **`/slo-product metrics` overlaps slightly with `/slo-metrics`** (shipped in Runbook B2). Disambiguation: `/slo-product metrics` covers PM-side north-star + activation + retention + feature-adoption KPIs; `/slo-metrics` (B2) covers financial / business KPIs (CAC, LTV, NDR, MoM growth, burn multiple). The split is per the design overview.

### Global Red Lines

Same as Runbook A. Plus:

- No advisor-pattern modes in generator SKILL.mds.
- No predicate-id citations in generator SKILL.mds (advisor-only).
- No PII-scan rule that's so noisy founders disable it; conservative regex + override mechanism is the right balance.

---

## Evidence Log Template

Same as Runbook A. Per-milestone Evidence Log copied into each milestone section.

---

## Self-Review Gate

Same questions as Runbook A.

---

## Milestone Plan

### Milestone 1 — `/slo-talk-to-users` + `/slo-verify` Pass 4 PII-pattern scan integration

**Goal**: Ship `/slo-talk-to-users` as the first generator skill in the biz pack + land the deferred PII-pattern scan in `/slo-verify` Pass 4 over `docs/biz-public/`.

**Important design rule**: M1 lands BOTH the generator AND the PII scan because the scan needs a generator to test against. Authoring order: scan first (modify `/slo-verify`), then `/slo-talk-to-users`, so the structural test for the scan runs against real fixture artifacts.

**Refactor budget**: Minimal local refactor permitted. The `/slo-verify` SKILL.md edit is additive (new sub-step in Pass 4); it does not change existing Pass 4 behaviour.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Locked Runbook A outputs (15 advisor + reference files); design overview + interfaces + artifact-schema. |
| Outputs | NEW: `skills/slo-talk-to-users/SKILL.md`, `crates/sldo-install/tests/e2e_biz_b1_m1.rs`, `docs/verify/biz-b1-m1-smoke.md`. EDITED: `skills/slo-verify/SKILL.md` (Pass 4 sub-step), `references/biz/artifact-schema.md` (add `archetype` and optional `mode_arg` keys). |
| Interfaces touched | NEW skill verb `/slo-talk-to-users`. NEW `/slo-verify` Pass 4 PII-scan sub-step (additive — no removal of existing Pass 4 behaviour). NEW frontmatter keys `archetype` (enum: `advisor`\|`generator`) and `mode_arg` (string, optional). |
| Files allowed to change | `skills/slo-talk-to-users/SKILL.md` (NEW); `skills/slo-verify/SKILL.md` (small additive edit); `references/biz/artifact-schema.md` (small additive edit); `crates/sldo-install/tests/e2e_biz_b1_m1.rs` (NEW); `docs/verify/biz-b1-m1-smoke.md` (NEW); `docs/ARCHITECTURE.md` (one-line). |
| Forbidden shortcuts | No removal or rename of existing `/slo-verify` Pass 4 sub-steps. No advisor-pattern modes in `/slo-talk-to-users` (it's a generator). No predicate-id citations in `/slo-talk-to-users` SKILL.md. No regex so loose it produces a flag-per-line on real fixtures. |
| **Data classification** | `Confidential` — `/slo-talk-to-users` outputs contain real persons' names + emails from interview transcripts. Founder's repo `.gitignore` MUST exclude `docs/biz/users/`; skill prose includes write-time warning. |
| **Proactive controls in play** | C1 (SECURITY.md + threat-model cited); C5 (PII regex pattern set is the validation surface); C8 (two-tier output convention enforced; PII scan is the runtime enforcement for tier discipline); C10 (regex match output is structured, not appended to user's terminal as raw matches — only line numbers + pattern type). |
| **Abuse acceptance scenarios** | Required — `/slo-talk-to-users` introduces a new PII-shaped output surface. BDD rows: `pii_scan_detects_email_regex_in_public_tier`, `pii_scan_detects_uk_ni_number_in_public_tier`, `pii_scan_override_documented_in_frontmatter`. Cite `tm-biz-abuse-1` (founder repo leak) and `tm-biz-abuse-6` (founder pastes PII into generator). |

#### Step-by-Step

1. Pre-flight: baseline tests green, M1-M4 of Runbook A still green (regression).
2. Author `crates/sldo-install/tests/e2e_biz_b1_m1.rs` with all stubs failing for the right reason.
3. Edit `references/biz/artifact-schema.md` — add `archetype` and `mode_arg` keys to the frontmatter table.
4. Edit `skills/slo-verify/SKILL.md` — add Pass 4 sub-step "Biz-pack PII-pattern scan over `docs/biz-public/`" with regex set documented.
5. Author `skills/slo-talk-to-users/SKILL.md` — generator, single-mode (no mode arg), output `docs/biz/users/<date>-<name>.md` confidential.
6. Author `docs/verify/biz-b1-m1-smoke.md` with 4 fixtures.
7. Run all tests, confirm 42 Runbook A tests still green + new M1 tests green.
8. Update Milestone Tracker + ARCHITECTURE.md row.

#### BDD Acceptance Scenarios

| Scenario | Category | Given | When | Then | Threat-model row | Control |
|---|---|---|---|---|---|---|
| `slo_talk_to_users_skill_md_is_generator_archetype` | happy path | new skills/slo-talk-to-users/SKILL.md | structural test parses frontmatter | `name: slo-talk-to-users`; description non-empty; SKILL.md does NOT cite the four predicate IDs (because generators are not advisors) | n/a | regex parse |
| `slo_verify_pass_4_documents_pii_scan_subset` | happy path | edited skills/slo-verify/SKILL.md | structural test searches Pass 4 section | section names "PII-pattern scan" or "biz-pack PII" + lists at least three regex types (email, UK NI, sort code) | n/a | regex parse |
| `pii_scan_detects_email_regex_in_public_tier` | abuse case | a fixture artifact at `docs/biz-public/users/test.md` containing `name: Alice Smith` and `email: alice@example.com` | Pass 4 PII scan runs over `docs/biz-public/` | scan flags the email pattern with line number; non-zero exit code from the scan step | tm-biz-abuse-6 | regex; the test creates the fixture, runs the scan logic in-process or via SKILL.md prose check, asserts detection |
| `pii_scan_detects_uk_ni_number_in_public_tier` | abuse case | a fixture artifact in `docs/biz-public/` containing `NI: AB123456C` | Pass 4 PII scan runs | scan flags the NI pattern | tm-biz-abuse-1 | regex |
| `pii_scan_override_documented_in_frontmatter` | happy path | a fixture artifact with `pii_scan_override: true` and `tier_override_reason: anonymised pseudonym for case study` | Pass 4 PII scan runs | scan reports override + reason; does NOT fail the milestone | n/a | regex parse of frontmatter override |
| `artifact_schema_documents_archetype_and_mode_arg` | happy path | edited references/biz/artifact-schema.md | structural test parses key list | both `archetype` and `mode_arg` are documented; `archetype` enum is `advisor` \| `generator` | n/a | regex |
| `slo_talk_to_users_outputs_to_confidential_tier_by_default` | happy path | slo-talk-to-users SKILL.md body | structural test parses output convention | body documents `docs/biz/users/<date>-<name>.md` as default + `tier: confidential` + the founder-repo `.gitignore` warning | tm-biz-abuse-1 | regex |
| `slo_verify_pass_4_existing_substeps_unchanged` | regression | skills/slo-verify/SKILL.md before+after | diff existing Pass 4 sub-steps | supply-chain + variant-analysis + DAST sub-steps unchanged; PII-scan added as a sibling sub-step | n/a | text comparison |

#### Definition of Done

- [ ] 4 new files created + 2 small additive edits to existing files.
- [ ] `cargo test -p sldo-install --test e2e_biz_b1_m1` returns 8/8 green.
- [ ] M1-M4 of Runbook A still 42/42 green.
- [ ] Full baseline green.
- [ ] `sldo-install --dry-run` shows 19 skills (18 post-A merge + `/slo-talk-to-users`).
- [ ] M1's smoke checklist ticked (4 fixtures).

---

### Milestone 2 — `/slo-gtm`

**Goal**: Ship `/slo-gtm` (generator) — produces `docs/biz-public/gtm/strategy.md` covering ICP definition, segmentation, GTM motion choice (PLG / sales-led / community-led / hybrid), channel strategy.

**Pattern**: Pure generator. No mode arg. Single output. Cites `references/biz/artifact-schema.md` for frontmatter; cites `references/biz/jurisdiction-uk.md` only if the strategy has UK-specific channel implications (typically not — GTM is largely jurisdiction-agnostic at strategy level).

**Files (NEW)**: `skills/slo-gtm/SKILL.md`, `crates/sldo-install/tests/e2e_biz_b1_m2.rs`, `docs/verify/biz-b1-m2-smoke.md`. ARCHITECTURE.md +1 row.

**BDD coverage**: skill structure (frontmatter, archetype: generator), output path + tier (`docs/biz-public/gtm/strategy.md`, `tier: public`), motion-choice keywords documented (`PLG`, `sales-led`, `community-led`, `hybrid`).

**Compatibility**: M1 outputs unchanged.

---

### Milestone 3 — `/slo-product` (mode arg: roadmap | metrics | okrs)

**Goal**: Ship `/slo-product` with a mode argument selecting between three artifact families.

**Pattern**: Generator with mode arg. Output paths differ by mode:
- `roadmap` → `docs/biz-public/product/roadmap.md` (RICE / Kano framework)
- `metrics` → `docs/biz-public/product/metrics.md` (north-star metric, activation funnel, retention curves, feature adoption)
- `okrs` → `docs/biz-public/product/okrs.md` (quarterly OKRs, ladders to north-star)

**Files (NEW)**: `skills/slo-product/SKILL.md`, `crates/sldo-install/tests/e2e_biz_b1_m3.rs`, `docs/verify/biz-b1-m3-smoke.md`. ARCHITECTURE.md +1 row.

**BDD coverage**: three mode-arg paths each produce the expected output path; mode-arg is a stable interface (test asserts the three values + rejects unknown modes).

**Disambiguation from `/slo-metrics` (Runbook B2)**: skill prose explicitly states "PM-side product metrics (DAU, activation, retention, feature adoption) NOT financial / business metrics — for CAC, LTV, NDR, MoM growth, burn multiple use `/slo-metrics`".

**Compatibility**: M1 + M2 unchanged.

---

### Milestone 4 — `/slo-marketing` (mode arg: b2b | b2c)

**Goal**: Ship `/slo-marketing` with B2B vs B2C variant.

**Pattern**: Generator with binary mode arg. Output paths:
- `b2b` → `docs/biz-public/marketing/b2b-plan.md`
- `b2c` → `docs/biz-public/marketing/b2c-plan.md`

**Content**: brand voice, content calendar, channel mix, demand gen, paid acquisition strategy. B2B variant emphasises content marketing + outbound + LinkedIn-organic + partner channels; B2C variant emphasises performance marketing + organic search + community + influencer + paid social — both call out direct-marketing PECR considerations and route ANY direct-marketing implementation question to `/slo-legal triage` (gate-4-gdpr-document fires for direct-marketing GDPR matters per DUAA Stage 3 + £17.5M PECR ceiling per `references/biz/ico-duaa-index.md`).

**Files (NEW)**: `skills/slo-marketing/SKILL.md`, `crates/sldo-install/tests/e2e_biz_b1_m4.rs`, `docs/verify/biz-b1-m4-smoke.md`. CLAUDE.md edit (single — extends biz catalog with B1 generators).

**BDD coverage**: two mode-arg paths each produce expected output paths; skill prose routes direct-marketing questions to `/slo-legal triage`; rejects unknown mode args.

**Compatibility**: M1 + M2 + M3 unchanged.

---

> **Status**: B1 runbook drafted; M1 execution starts immediately per user direction.
