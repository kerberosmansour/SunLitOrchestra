# Business-Side Skill Pack, Runbook A — Advisor Cluster (AI-First Runbook v3)

> **Purpose**: Ship the four advisor-pattern biz skills (`/slo-legal`, `/slo-accounting`, `/slo-equity`, `/slo-fundraise`) with their `draft` + `translate` + `triage` + `prepare` modes, the four hard-block gates (regulated / >£5,000 / counterparty-has-lawyer / GDPR), and the shared `references/biz/` scaffolding that the four skills cite — UK only in v1, oneNDA verbatim with hash check, broad GDPR draft hard-block (locked 2026-04-25), JPP Law cost baseline (locked 2026-04-25).
> **Audience**: AI coding agents first, humans second. Written to reduce ambiguity, prevent scope drift, and ensure the advisor pattern lands once and is replicated cleanly across four skills.
> **How to use**: Work through milestones sequentially. Before starting any milestone, read its full section and the Global Execution Rules. After completing it, follow the Global Exit Rules. Never skip ahead. Never silently widen scope.
> **Prerequisite reading**: [docs/ARCHITECTURE.md](ARCHITECTURE.md), [docs/slo/idea/biz-skill-pack.md](idea/biz-skill-pack.md), [docs/slo/research/biz-skill-pack/synthesis.md](research/biz-skill-pack/synthesis.md), [docs/slo/design/biz-skill-pack-overview.md](design/biz-skill-pack-overview.md), [docs/slo/design/biz-skill-pack-stack-decision.md](design/biz-skill-pack-stack-decision.md), [docs/slo/design/biz-skill-pack-interfaces.md](design/biz-skill-pack-interfaces.md), [docs/slo/design/biz-skill-pack-threat-model.md](design/biz-skill-pack-threat-model.md), [SECURITY.md](../SECURITY.md) (root, biz section)

---

## Runbook Metadata

- **Runbook ID**: `biz-skill-pack-a`
- **Prefix for test files and lessons files**: `biz-a`
- **Primary stack**: Markdown `SKILL.md` prompt files under `skills/slo-<biz>/` (consumed by Claude Code) + Markdown shared references under `references/biz/`. Secondary: Rust 2021 workspace for structural-contract tests under `crates/sldo-install/tests/e2e_biz_a_m<N>.rs` using existing patterns from `e2e_slo_sec_m<N>`.
- **Primary package/app names**: `skills/slo-legal`, `skills/slo-accounting`, `skills/slo-equity`, `skills/slo-fundraise` (skill packages — Markdown directories, not crates); `references/biz/` (shared scaffolding directory at repo root, outside `skills/`); structural-contract tests inside `crates/sldo-install/tests/`.
- **Default test commands**:
  - Backend: `cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install` (canonical per CLAUDE.md; `sldo-tauri` is parked and breaks `--workspace` on macOS arm64)
  - Frontend: N/A (skill pack is markdown; no frontend in scope)
  - E2E backend: `cargo test -p sldo-install --test e2e_biz_a_m1 && cargo test -p sldo-install --test e2e_biz_a_m2 && cargo test -p sldo-install --test e2e_biz_a_m3 && cargo test -p sldo-install --test e2e_biz_a_m4` (cargo auto-discovers integration tests under `crates/sldo-install/tests/*.rs`; no root `[[test]]` registration needed — same convention as `e2e_slo_sec_m<N>`)
  - E2E frontend: N/A
  - Build/boot: `cargo build -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install`
- **Allowed new dependencies by default**: `none` (each milestone names any dependency it introduces; default is that Rust tests reuse `assert_cmd`, `tempfile`, `regex`, `anyhow`, `sha2` already in the workspace — `sha2` is already used by `sldo-tla-sha`)
- **Schema/config migration allowed by default**: `no`
- **Public interfaces that must remain stable unless explicitly listed otherwise**:
  - Existing skill invocation verbs: `/slo-ideate`, `/slo-research`, `/slo-architect`, `/slo-tla`, `/slo-plan`, `/slo-critique`, `/slo-execute`, `/slo-verify`, `/slo-retro`, `/slo-ship`, `/slo-resume`, `/slo-freeze`, `/slo-second-opinion`, `/get-api-docs`
  - New skill verbs added by this runbook: `/slo-legal`, `/slo-accounting`, `/slo-equity`, `/slo-fundraise` (four advisor skills; mode contract `draft|translate|triage|prepare`)
  - The four triage-gate predicate ids in `references/biz/triage-gate.md`: `gate-1-regulated`, `gate-2-deal-value-over-5k`, `gate-3-counterparty-has-lawyer-or-their-paper`, `gate-4-gdpr-document`
  - `SKILL.md` frontmatter keys consumed by the Claude Code skill loader: `name`, `description`
  - `docs/RUNBOOK-<slug>.md` v3 structure as defined in `docs/slo/templates/runbook-template_v_3_template.md` — existing runbooks must continue to parse
  - Canonical baseline test command above
  - oneNDA template body bytes — `references/biz/templates/onenda-uk.md` SHA-256 is pinned in `e2e_biz_a_m1`; license is CC BY-ND 4.0 verbatim render
  - The biz artifact frontmatter schema (`tier`, `skill`, `mode`, `jurisdiction`, `cost_baseline_ref`, `triage_gate_passed`, `lawyer_review_recommended`, `expires_or_review_by`) — defined in `docs/slo/design/biz-skill-pack-interfaces.md` and shipped in `references/biz/artifact-schema.md` in M2
  - Two-tier output convention: `docs/biz/` (gitignored, confidential) vs `docs/biz-public/` (git-tracked, placeholder-only) — the founder's local `.gitignore` template ships in M1

---

## Milestone Tracker

Update this table as each milestone is completed. This is the single source of truth for progress.

| # | Milestone | Status | Started | Completed | Lessons File | Completion Summary |
|---|---|---|---|---|---|---|
| 1 | `/slo-legal` v1 + shared `references/biz/` wedge (triage-gate, JPP Law cost baseline, oneNDA verbatim) | `done` | 2026-04-25 | 2026-04-25 | [docs/slo/lessons/biz-a-m1.md](lessons/biz-a-m1.md) | [docs/slo/completion/biz-a-m1.md](completion/biz-a-m1.md) |
| 2 | `/slo-accounting` + remainder of M2-tier shared refs (artifact-schema, jurisdiction-uk, ico-duaa-index, ico-enforcement-reality, open-template-anchors) | `done` | 2026-04-25 | 2026-04-25 | [docs/slo/lessons/biz-a-m2.md](lessons/biz-a-m2.md) | [docs/slo/completion/biz-a-m2.md](completion/biz-a-m2.md) |
| 3 | `/slo-equity` + `references/biz/hmrc-vcm-index.md` (VCM34080 / VCM3000 / VCM31000) | `done` | 2026-04-25 | 2026-04-25 | [docs/slo/lessons/biz-a-m3.md](lessons/biz-a-m3.md) | [docs/slo/completion/biz-a-m3.md](completion/biz-a-m3.md) |
| 4 | `/slo-fundraise` + `references/biz/ir35-cest-factors.md` + cross-skill citation test | `done` | 2026-04-25 | 2026-04-25 | [docs/slo/lessons/biz-a-m4.md](lessons/biz-a-m4.md) | [docs/slo/completion/biz-a-m4.md](completion/biz-a-m4.md) |

<!-- Status values: not_started | in_progress | blocked | done -->
<!-- Lessons files go in docs/slo/lessons/biz-a-m<N>.md -->
<!-- Completion summaries go in docs/slo/completion/biz-a-m<N>.md -->

---

## End-to-End Architecture Diagram

See [docs/slo/design/biz-skill-pack-overview.md](design/biz-skill-pack-overview.md) for the full diagram with legend. Summary view below; solid = exists today, dashed = added by this runbook (Runbook A); dotted = added by Runbooks B + C (out of this runbook's scope).

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        Founder (UK seed-stage)                              │
└──────────────────────────────────┬──────────────────────────────────────────┘
                                   │ /slo-* invocation
                                   ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                       Claude Code skill loader (solid)                      │
└──────────────────────────────────┬──────────────────────────────────────────┘
                                   │ reads SKILL.md + cited references
                                   ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│   Existing SLO skill pack (skills/slo-{ideate,research,...}/)               │
│                                                                             │
│   Biz skill pack — Runbook A (advisor cluster, dashed)                      │
│     skills/slo-legal/      [M1]  modes: draft|translate|triage|prepare      │
│     skills/slo-accounting/ [M2]  modes: draft|translate|triage|prepare      │
│     skills/slo-equity/     [M3]  modes: draft|translate|triage|prepare      │
│     skills/slo-fundraise/  [M4]  modes: draft|translate|triage|prepare      │
│                                                                             │
│   Biz skill pack — Runbook B1 (discovery → strategy → definition, dotted)   │
│     /slo-talk-to-users  /slo-gtm  /slo-product  /slo-marketing               │
│                                                                             │
│   Biz skill pack — Runbook B2 (execution → optimization, dotted)            │
│     /slo-launch  /slo-sales-funnel  /slo-pricing  /slo-metrics               │
│                                                                             │
│   Biz skill pack — Runbook C (team, dotted)                                 │
│     /slo-cofounder  /slo-hire  /slo-founder-check                            │
└────┬──────────────────────────────┬──────────────────────────────────────────┘
     │ reads (M1+)                  │ writes outputs (M1+)
     ▼                              ▼
┌──────────────────────────────┐  ┌──────────────────────────────────────────┐
│ Shared scaffolding (NEW)     │  │ Founder's repo (target of artifacts)     │
│ references/biz/  (dashed)    │  │ docs/biz/<area>/<artifact>.md            │
│   triage-gate.md      [M1]   │  │   (gitignored — confidential drafts)     │
│   cost-baseline-jpp-  [M1]   │  │ docs/biz-public/<area>/<artifact>.md     │
│     law-2026.md              │  │   (git-tracked — placeholder/decisions)  │
│   templates/                 │  │                                          │
│     onenda-uk.md      [M1]   │  │ Founder's .gitignore (M1 ships snippet)  │
│   artifact-schema.md  [M2]   │  │                                          │
│   jurisdiction-uk.md  [M2]   │  │                                          │
│   ico-duaa-index.md   [M2]   │  │                                          │
│   ico-enforcement-    [M2]   │  │                                          │
│     reality.md               │  │                                          │
│   open-template-      [M2]   │  │                                          │
│     anchors.md               │  │                                          │
│   hmrc-vcm-index.md   [M3]   │  │                                          │
│   ir35-cest-factors.md[M4]   │  │                                          │
└──────────────────────────────┘  └──────────────────────────────────────────┘
     │ cited (NOT fetched)
     ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│ External anchors (URL citations only — biz skills do NOT enable WebFetch)    │
│   onenda.org · gov.uk HMRC manual · gov.uk CEST · ico.org.uk DUAA            │
│   legislation.gov.uk · jpplaw.co.uk fixed-fee startup                        │
└──────────────────────────────────────────────────────────────────────────────┘

Legend:  ───  exists at HEAD       ─ ►  added by this runbook (Runbook A)
         · ·  added by Runbooks B/C (out of scope)
```

### Component Summary Table

| Component | Responsibility | Milestone Introduced/Changed | Key Interfaces |
|---|---|---|---|
| `skills/slo-legal/SKILL.md` | Advisor skill — UK legal templates + triage gate; `draft` modes for NDA, contractor SOW, IP assignment, T&Cs | M1 | Skill verb `/slo-legal`; mode contract; cites `references/biz/triage-gate.md` + `references/biz/templates/onenda-uk.md` + `references/biz/cost-baseline-jpp-law-2026.md` |
| `references/biz/triage-gate.md` | Single source of truth for the four hard-block predicates; cited by all four advisor skills | M1 | Predicate-id schema (`gate-1-regulated`, `gate-2-deal-value-over-5k`, `gate-3-counterparty-has-lawyer-or-their-paper`, `gate-4-gdpr-document`) |
| `references/biz/cost-baseline-jpp-law-2026.md` | UK fixed-fee solicitor pricing baseline with retrieval-date stamp; cited by advisor `draft` ROI block | M1 | Cost-line schema (item / price / source URL / retrieval date) |
| `references/biz/templates/onenda-uk.md` | Canonical oneNDA template body bytes (CC BY-ND 4.0 verbatim) | M1 | SHA-256 hash pinned in `e2e_biz_a_m1`; license obligation enforced by skill prose + verify regression |
| `crates/sldo-install/tests/e2e_biz_a_m1.rs` | Structural-contract tests for M1 — skill prose + frontmatter + reference paths + oneNDA hash + installer-ignores-references-biz | M1 | Cargo `[[test]]` (auto-discovered) |
| `skills/slo-accounting/SKILL.md` | Advisor skill — UK accounting triage (bookkeeping, VAT, R&D credits, MTD); `draft` mode limited to brief-the-accountant memos | M2 | Skill verb `/slo-accounting`; cites M1 + M2 references |
| `references/biz/artifact-schema.md` | Frontmatter contract for biz artifacts; declares which artifact categories live in `docs/biz/` (confidential) vs `docs/biz-public/` (placeholder) | M2 | Frontmatter keys read by `/slo-verify` PII-pattern scan |
| `references/biz/jurisdiction-uk.md` | UK-only jurisdiction prose anchors; explicit "UK only in v1" error pattern for non-UK requests | M2 | Cited by all four advisor skills |
| `references/biz/ico-duaa-index.md` | DUAA 2025 commencement dates + complaints-procedure duty + lawful-basis examples | M2 | Cited by `gate-4-gdpr-document` predicate body |
| `references/biz/ico-enforcement-reality.md` | PECR-vs-Article-13 enforcement pattern documentation (provenance for the broad GDPR hard-block) | M2 | Cited by `/slo-legal triage` GDPR responses |
| `references/biz/open-template-anchors.md` | oneNDA + oneSaaS + Kindrik notes; license obligations (CC BY-ND 4.0 verbatim rule) | M2 | Cited by `/slo-legal draft nda` + future `draft tos` |
| `crates/sldo-install/tests/e2e_biz_a_m2.rs` | Structural-contract tests for M2 — skill exists + new references parse + artifact-schema vocabulary | M2 | Cargo `[[test]]` |
| `skills/slo-equity/SKILL.md` | Advisor skill — cofounder split rationale, vesting schedule (4yr/1yr cliff), EMI option triage, dilution math | M3 | Skill verb `/slo-equity`; cites HMRC VCM index |
| `references/biz/hmrc-vcm-index.md` | VCM34080 (control/independence), VCM3000 (excluded activities), VCM31000 (SEIS income tax relief) — URLs + retrieval dates + plain-English summaries | M3 | Cited by `/slo-equity` and `/slo-fundraise` |
| `crates/sldo-install/tests/e2e_biz_a_m3.rs` | Structural-contract tests for M3 | M3 | Cargo `[[test]]` |
| `skills/slo-fundraise/SKILL.md` | Advisor skill — SAFE / cap-and-discount math, SEIS/EIS Advance Assurance triage, pitch narrative, term-sheet redline prep | M4 | Skill verb `/slo-fundraise`; cites HMRC VCM index + IR35 factors |
| `references/biz/ir35-cest-factors.md` | Three-factor list (substitution, MOO, control) + CEST April 2025 refresh + PGMOL v HMRC commentary; the contractor-vs-employee triage data | M4 | Cited by `/slo-legal triage`, `/slo-fundraise`, future `/slo-hire` (Runbook C) |
| `crates/sldo-install/tests/e2e_biz_a_m4.rs` | Structural-contract tests for M4 + cross-skill citation test (every advisor SKILL.md cites the four predicate ids) | M4 | Cargo `[[test]]` |
| `CLAUDE.md` | Project guide — adds biz pack section to skill catalog at end of M4 | M4 (single edit, after all four advisor skills exist) | Documentation only |

### Data Flow Summary

| Flow | From | To | Protocol/Mechanism | Milestone |
|---|---|---|---|---|
| Triage-gate predicate citation | All four advisor SKILL.md files | `references/biz/triage-gate.md` | Markdown link / prose citation (read-time) | M1 (skill #1) → M2/M3/M4 (replicated) |
| oneNDA verbatim body | `references/biz/templates/onenda-uk.md` | `/slo-legal draft nda` output (founder's repo) | Byte-for-byte copy at draft time; SHA-256 verified at install + verify time | M1 |
| Cost-baseline ROI line | `references/biz/cost-baseline-jpp-law-2026.md` | Advisor `draft` artifact frontmatter + body footer | Frontmatter `cost_baseline_ref` field + body block | M1 |
| Confidential draft write | Advisor `draft` mode | `docs/biz/<area>/<artifact>.md` (gitignored in founder's repo) | File write (Markdown); `tier: confidential` frontmatter | M1 (legal) → M2/M3/M4 |
| Triage / prepare / translate output | Advisor non-draft modes | `docs/biz-public/<area>/<artifact>.md` (founder's repo, tracked) | File write (Markdown); `tier: public` frontmatter | M1+ |
| GDPR doc hard-block routing | Advisor skill prose (gate-4-gdpr-document) | `/slo-legal triage` response with DPO routing + ICO references | Inline skill behavior (no file write) | M1 |
| HMRC manual citation | Advisor skill prose | `references/biz/hmrc-vcm-index.md` → URL citation in output | Markdown citation | M3 (equity) and M4 (fundraise) |
| IR35 / contractor-employee triage | `/slo-legal triage`, `/slo-fundraise` | `references/biz/ir35-cest-factors.md` → triage decision + lawyer routing | Read at decision time | M4 |
| Structural-contract test signal | `cargo test -p sldo-install --test e2e_biz_a_m<N>` | Milestone Evidence Log | Test exit code + assertion output | M1–M4 |

---

## High-Level Design for Formal Verification (TLA+ Section)

**N/A** — justification from [docs/slo/design/biz-skill-pack-overview.md](design/biz-skill-pack-overview.md) frontmatter: "Markdown skill authoring + reference-doc reads + per-skill artifact writes. No concurrent actors sharing state, no distributed consensus, no ordering guarantees across processes, no resource leases or locks. Every skill runs sequentially in one Claude Code session as file I/O."

The feature is sequential: founder invokes a skill, skill reads SKILL.md + references/biz/, skill writes outputs to founder's repo, exits. Structural-contract tests are synchronous and deterministic. Nothing in this design has the concurrency shape TLC models well.

---

## Global Execution Rules

These rules apply to every milestone in this runbook without exception.

### 1) Stay inside scope

- Only change files listed in the current milestone's allow-list.
- Do not refactor unrelated skill files or Rust crates.
- Do not rename existing skill invocation verbs, `SKILL.md` frontmatter keys, persona filenames, the canonical runbook baseline test command, or the four triage-gate predicate ids.
- Do not introduce a new dependency unless the milestone explicitly allows it.
- Do not modify `docs/slo/templates/runbook-template_v_3_template.md` — backward compatibility with existing runbooks is load-bearing.
- Do not modify `crates/sldo-tauri/` — parked per CLAUDE.md.
- **Do not place shared scaffolding under `skills/` or `skills/_biz-shared/`.** The location is `references/biz/` at repo root, full stop. `crates/sldo-install/src/install.rs:44-71` confirms that anything under `skills/<name>/` containing a `SKILL.md` is auto-discovered and symlinked into `~/.claude/skills/`; adding shared scaffolding there would either double-install or create a fragile undocumented convention.

### 2) Tests define the contract

- Write structural-contract tests under `crates/sldo-install/tests/e2e_biz_a_m<N>.rs` BEFORE editing `SKILL.md` / `references/biz/` files.
- Confirm tests fail for the right reason (file missing, frontmatter missing, predicate-id missing, hash mismatch) before implementing.
- A milestone is not done when Markdown parses. It is done when the declared structural contract is satisfied AND the manual smoke test (described per milestone) confirms the skill produces the promised artifacts when invoked against a fixture.
- The oneNDA hash test in M1 is non-negotiable — failing the hash check is a CC BY-ND 4.0 license risk, not a developer-quality issue.

### 3) No placeholders in production paths

- No `TODO` lines or placeholder prose inside shipped `SKILL.md` files.
- No `[FIXME]` in reference docs.
- No half-removed old sections; replace completely or leave untouched.
- No invented downstream skill names in skill prose (`/slo-talk-to-users`, `/slo-launch`, `/slo-cofounder`, etc. are explicitly placeholders for Runbooks B + C; they may be *mentioned* as "see Runbook B/C" but not *invoked* by M1–M4).

### 4) Preserve backwards compatibility

- Every milestone must verify existing runbooks (`docs/RUNBOOK-*.md` pre-dating this work) parse cleanly.
- Existing skills (`skills/slo-{ideate,research,architect,...}/`) must continue to function unchanged — this runbook adds 4 new skill directories, never edits the existing 14.
- `sldo-install` must still install the existing skill pack without schema errors AND must NOT install `references/biz/` as if it were a skill.

### 5) Prefer smallest safe change

- Each new SKILL.md should be authored as one cohesive document; the four advisor skills share structure, so the second-fourth can copy M1's structure and substitute domain-specific content.
- Prefer adding a new `references/biz/` file over inflating SKILL.md prose beyond ~250 lines.
- If a refactor is required, keep it minimal and directly justified by the milestone goal.

### 6) Record evidence, not claims

Evidence Log rows required per milestone: baseline tests; structural-contract tests created; manual smoke test stub created; implementation summary; full tests; build/boot; smoke tests executed; test-artifact cleanup; `.gitignore` review; compatibility checks (existing runbooks parse, existing skills load, `sldo-install` ignores `references/biz/`). See Evidence Log Template below.

### 7) Keep .gitignore current and clean up test artifacts

- Structural-contract tests read from in-tree `skills/*` and `references/biz/*` files. They do not write to the working tree; if any milestone introduces generated output (it should not — this runbook is Markdown), add patterns before committing.
- Review `.gitignore` at the end of every milestone for staleness.
- Founder-repo `.gitignore` snippet (the one M1 ships) is a TEMPLATE for downstream founders — it is NOT applied to this SLO repo's `.gitignore`. The SLO repo continues to ignore only what `crates/sldo-install/` and `output/` need.
- Record the `.gitignore` review in the Evidence Log.

---

## Global Entry Rules (Pre-Milestone Protocol)

1. Read the lessons file from the previous milestone (`docs/slo/lessons/biz-a-m<N-1>.md`), if one exists.
2. Read the current milestone fully.
3. Read the relevant existing reference files (`docs/slo/design/biz-skill-pack-{overview,interfaces,threat-model}.md`, `references/biz/<files-from-prior-milestones>`).
4. Run `cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install` and confirm green. Record baseline in Evidence Log.
5. Read the files in "Files Allowed To Change" and "Files To Read Before Changing Anything".
6. Update the Milestone Tracker in this file: status `in_progress`, record Started date.
7. Create structural-contract test file FIRST (`crates/sldo-install/tests/e2e_biz_a_m<N>.rs`) and confirm it fails for the right reasons.
8. Create the manual smoke-test stub file (`docs/slo/verify/biz-a-m<N>-smoke.md`) with the empty checklist.
9. Copy the milestone's Evidence Log template into working notes.
10. Re-state the milestone constraints in your own words before editing any SKILL.md or reference file.

---

## Global Exit Rules (Post-Milestone Protocol)

1. Run full test suite — every pre-existing test green, every new BDD scenario green.
2. Run the milestone-specific E2E test: `cargo test -p sldo-install --test e2e_biz_a_m<N>` — green.
3. Verify `cargo build -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install` builds cleanly.
4. Run `./target/release/sldo-install --dry-run` and confirm: (a) the new skill (M<N>'s `slo-<biz>`) is discovered; (b) `references/biz/` is NOT in the discovery list.
5. Run smoke tests listed in the milestone (manual fixture invocation of the new skill).
6. Verify backward compatibility per the milestone's Compatibility Checklist (existing runbooks parse, existing skills still load).
7. Complete the Self-Review Gate.
8. `git status` — no untracked test artifacts.
9. Review `.gitignore`.
10. Update `docs/ARCHITECTURE.md` only if the milestone shipped something observable at HEAD (per the reality-first rule). Most M1–M4 milestones add new skill directories under `skills/` and reference files under `references/biz/`; ARCHITECTURE.md gets a one-line addition each milestone naming the new skill verb + its mode contract, plus a one-time `references/biz/` line in M1.
11. Update `CLAUDE.md` — the biz pack catalog table is a single CLAUDE.md edit landed in M4 once all four advisor skills exist; M1–M3 do NOT edit CLAUDE.md.
12. Write `docs/slo/lessons/biz-a-m<N>.md`.
13. Write `docs/slo/completion/biz-a-m<N>.md`.
14. Update Milestone Tracker: status `done`, record Completed date.
15. Re-read the next milestone with fresh eyes.

---

## Background Context

### Current State

At HEAD, the SLO skill pack provides 14 skills covering Ideate → Ship for engineering work plus 1 third-party (`get-api-docs`). All 14 are engineering-shaped: ideate, research, architect, TLA, plan, critique, execute, verify, retro, ship, plus tooling skills (resume, freeze, second-opinion). The shared infrastructure for those skills (`crates/sldo-{plan,run,research,install,common,tla-sha}`) is Rust + Markdown. There is no precedent in the pack for "advisor pattern" skills (multi-mode skills with a hard-block triage gate), no `references/biz/` directory, no biz artifact convention (`docs/biz/` vs `docs/biz-public/`), and no cost-baseline / jurisdiction reference data. The slo-security-embedding runbook (`docs/slo/completed/RUNBOOK-SLO-SECURITY-EMBEDDING.md`) is the closest precedent for a Markdown-only feature with structural-contract tests under `crates/sldo-install/tests/e2e_slo_sec_m<N>.rs`.

### Problem

1. **Founder hits a legal/business-ops wall on a Tuesday and SLO has nothing to offer.** The pack covers engineering ideate-through-ship; the company-around-the-product (legal, accounting, equity, fundraising, GTM, hiring) is invisible. Today, the founder context-switches to a different tool (or stalls), breaking the SLO loop.
2. **No advisor pattern exists in the pack.** All 14 existing skills are one-shot generators. Regulated/professional domains (legal, accounting) need a skill that *refuses to draft* under specific conditions and produces a "see a lawyer / accountant / DPO + here's what to brief them on" artifact instead. The pack has no template for this.
3. **No shared scaffolding location.** Cross-skill predicates (the four hard-block gates) and reference data (cost baselines, jurisdiction matrix, regulatory anchors) need a single source of truth or four advisor skills will drift. Today there is no `references/biz/` and no convention saying it must live outside `skills/` (which is enforced by `crates/sldo-install/src/install.rs:44-71`'s skill discovery walk).
4. **No biz artifact convention.** Founder-supplied personal data (real names, deal values, IP scope) lands in skill outputs. The pack has no two-tier output convention (`docs/biz/` confidential vs `docs/biz-public/` tracked) and no PII-pattern scan in `/slo-verify`.
5. **No template-source-of-truth for legal docs.** UK template work needs a CC-licensed open-standard anchor (oneNDA, CC BY-ND 4.0) for the NDA, which forbids derivative works. Without a verbatim render policy + hash check, future "improvements" silently break the licence.
6. **No cost-baseline anchoring.** ROI claims in `/slo-legal` outputs need an auditable, publicly-retrievable baseline (JPP Law fixed-fee public pricing, locked 2026-04-25), not a private firm's price list.

### Target Architecture

See the End-to-End Architecture Diagram above and [docs/slo/design/biz-skill-pack-overview.md](design/biz-skill-pack-overview.md).

### Key Design Principles

1. **Advisor pattern is a contract, not a suggestion.** All four advisor skills (legal, accounting, equity, fundraise) MUST accept exactly four modes (`draft|translate|triage|prepare`) and MUST cite `references/biz/triage-gate.md` for hard-block predicates. The structural-contract tests enforce this; SKILL.md prose documents it.
2. **Hard-block gates have stable IDs.** The four predicate IDs (`gate-1-regulated`, `gate-2-deal-value-over-5k`, `gate-3-counterparty-has-lawyer-or-their-paper`, `gate-4-gdpr-document`) are runbook-scope public interface. Adding a fifth gate is a `/slo-architect` decision, not a per-milestone option.
3. **GDPR broad hard-block on draft (locked 2026-04-25).** Privacy notice / ROPA / DPA / internal data-protection policies → translate or triage only, never draft. Defensible on professional-negligence + upside-asymmetry grounds; reversible only via fresh `/slo-architect` pass with new ICO enforcement evidence.
4. **JPP Law cost baseline (locked 2026-04-25).** ROI claims anchor to https://www.jpplaw.co.uk/sectors/fixed-fee-startup/ (publicly retrievable, auditable). Russell Cooke 2026-27 list is not publicly retrievable and was rejected as a citable baseline.
5. **UK only in v1.** No `--jurisdiction us` / `--jurisdiction eu` flag in advisor skills. Non-UK requests emit a clear "v1 supports UK only; US/EU is a v2 architectural pivot" error. Pre-emptive jurisdiction stubs are forbidden.
6. **oneNDA verbatim render under CC BY-ND 4.0.** `/slo-legal draft nda` MUST emit the canonical oneNDA template body byte-for-byte unmodified. Company / counterparty / cover fields render as a separately-rendered artifact wrapping but not editing the canonical text. SHA-256 hash check at install time + `/slo-verify` regression test on the rendered NDA body bytes.
7. **No WebFetch / WebSearch in biz skills.** Founder personal data lives in the prompt context; enabling model-driven web fetching would create an exfiltration surface. Citations only; the founder follows links manually.
8. **Two-tier output, default to confidential.** `docs/biz/` (gitignored) for any artifact carrying real names / values / IP scope; `docs/biz-public/` (git-tracked) for placeholder templates and decision/triage memos that contain no founder PII. The artifact-schema reference (M2) declares each artifact category's tier; `/slo-verify` PII-pattern scan enforces.

### What to Keep

- Existing skill invocation verbs and file paths (see Runbook Metadata).
- `docs/slo/templates/runbook-template_v_3_template.md` — do not modify.
- `skills/slo-{ideate,research,architect,tla,plan,critique,execute,verify,retro,ship,resume,freeze,second-opinion}/` and `skills/get-api-docs/` — none touched in this runbook.
- `crates/sldo-install/src/install.rs` skill-discovery walk — do not change; the design relies on its current behavior of requiring `<skills_dir>/<name>/SKILL.md` to discover.
- Baseline test command: `cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install`.
- Parked `sldo-tauri` crate: do not touch.
- Existing `SECURITY.md` (root) — M1 has already been merged with biz-pack-specific section by `/slo-architect`; do not re-edit in milestones.
- The four locked decisions from `/slo-architect`: GDPR broad hard-block, JPP Law cost baseline, UK only v1, oneNDA verbatim.

### What to Change

- **NEW: `skills/slo-legal/SKILL.md`** (M1) — advisor skill with `draft|translate|triage|prepare` modes; cites `references/biz/triage-gate.md`, `references/biz/cost-baseline-jpp-law-2026.md`, `references/biz/templates/onenda-uk.md`.
- **NEW: `references/biz/triage-gate.md`** (M1) — single source of truth for the four hard-block predicates.
- **NEW: `references/biz/cost-baseline-jpp-law-2026.md`** (M1) — JPP Law fixed-fee public pricing with retrieval date.
- **NEW: `references/biz/templates/onenda-uk.md`** (M1) — canonical oneNDA template body, CC BY-ND 4.0 verbatim.
- **NEW: `crates/sldo-install/tests/e2e_biz_a_m1.rs`** (M1) — structural-contract tests for M1.
- **NEW: `docs/slo/verify/biz-a-m1-smoke.md`** (M1) — manual smoke-test checklist for M1.
- **NEW: `skills/slo-accounting/SKILL.md`** (M2) + the M2-tier shared references (artifact-schema, jurisdiction-uk, ico-duaa-index, ico-enforcement-reality, open-template-anchors).
- **NEW: `skills/slo-equity/SKILL.md`** (M3) + `references/biz/hmrc-vcm-index.md`.
- **NEW: `skills/slo-fundraise/SKILL.md`** (M4) + `references/biz/ir35-cest-factors.md`.
- **`CLAUDE.md`** (M4 only — single edit) — adds biz pack section to the skill catalog table.
- **`docs/ARCHITECTURE.md`** (incremental, per milestone) — one-line addition per new skill verb + mode contract; one-time `references/biz/` line in M1.

### Global Red Lines

- No unrelated refactors of existing skills or crates.
- No new dependencies beyond what each milestone explicitly lists (M1–M4 add no new crates; tests reuse `assert_cmd`, `regex`, `tempfile`, `anyhow`, `sha2` already in the workspace).
- No schema migrations to the v3 runbook template.
- No config key renames in existing skill frontmatter.
- No public skill-verb renames.
- No production placeholders (no `[TODO]` in shipped SKILL.md).
- No silent error swallowing in structural-contract tests.
- No secrets in source control.
- No test output data committed to source control.
- **No modifications to `crates/sldo-tauri/`** — parked per CLAUDE.md.
- **No bypassing `docs/slo/templates/runbook-template_v_3_template.md` backward compat** — every runbook predating this work must continue to parse.
- **No advisor mode inventions.** The four modes are fixed: `draft`, `translate`, `triage`, `prepare`. Adding a fifth mode is a `/slo-architect` decision.
- **No edits to oneNDA template body bytes after M1 ships.** The hash is pinned. License-required.
- **No enabling WebFetch / WebSearch in any biz SKILL.md.** Citations only.

---

## BDD and Runtime Validation Rules

Every milestone follows these rules.

### Write Tests Before Production Code

For each milestone:
1. Read the BDD acceptance table.
2. Create `crates/sldo-install/tests/e2e_biz_a_m<N>.rs` with test stubs for each scenario; each stub fails with the expected shape ("file missing", "frontmatter key absent", "predicate-id not cited", "hash mismatch").
3. No root `Cargo.toml` registration needed — cargo auto-discovers integration tests under `crates/sldo-install/tests/*.rs`. Confirm with `cargo test -p sldo-install --test e2e_biz_a_m<N>`.
4. Confirm tests fail for the right reason.
5. Edit/add the SKILL.md / reference / template files to make tests pass.
6. Re-run tests after any textual refactor.

### Required Test Coverage Categories

Every milestone covers the categories that apply. For M1–M4 the relevant categories are:

- **Happy path** — required SKILL.md + frontmatter + reference files exist and parse.
- **Invalid input** — malformed frontmatter, missing predicate-id citation, oneNDA hash mismatch — caught by structural-contract tests.
- **Empty state** — N/A (skills are not stateful; no first-run state).
- **Dependency failure** — N/A (no runtime dependency in structural tests).
- **Retry** — N/A (single-pass file reads).
- **Concurrency** — N/A per TLA section.
- **Persistence** — N/A (Markdown-only).
- **Backward compatibility** — existing runbooks still parse; existing skills still load (`sldo-install --dry-run` shows them); `references/biz/` is NOT discovered as a skill.
- **Abuse case** — REQUIRED in every milestone introducing a new surface (M1 introduces three: `/slo-legal` skill, `references/biz/` directory, biz artifact write to `docs/biz/`). See the milestone-specific abuse-case rows; each cites `tm-biz-abuse-N` from `docs/slo/design/biz-skill-pack-threat-model.md`.

### Scenario Structure

Every BDD scenario uses Given/When/Then:

```rust
#[test]
fn descriptive_test_name() {
    // Given: [precondition — e.g. the new skills/slo-legal/SKILL.md file exists]
    // When:  [action — e.g. parsing it for the four mode keywords]
    // Then:  [expected — e.g. all four mode keywords appear at least once and in the modes table]
}
```

### Test File Naming

| Layer | Convention | Location |
|---|---|---|
| Structural-contract tests (per milestone) | `e2e_biz_a_m<N>.rs` | `crates/sldo-install/tests/` |
| Reusable parsers / helpers | `crates/sldo-install/tests/common/biz_a_fixtures.rs` — only if shared across multiple milestones | `crates/sldo-install/tests/common/` |
| Manual smoke-test checklist | `biz-a-m<N>-smoke.md` | `docs/slo/verify/` |

### Test Artifact Cleanup Rules

M1–M4 structural-contract tests read from in-tree `skills/*` and `references/biz/*` directories. Tests that need a temp working tree use `tempfile::TempDir` (already a workspace dep via `sldo-install`). Tests must not modify `skills/*`, `references/biz/*`, or `docs/*` files.

### End-to-End Runtime Validation

Each milestone's E2E runtime validation step invokes the new skill against a real fixture (e.g., a deal-shaped pitch with realistic UK seed-stage details — counterparty name, payment terms, IP scope; or a borderline IR35 fact pattern) and asserts the artifact produced has the promised shape and lands in the correct tier directory. Because skill behavior is prompt-driven, runtime validation cannot be fully automated in Rust; it is executed as a **manual smoke test** against the fixture, with output checked into the verification report (`docs/slo/verify/biz-a-m<N>-smoke.md`), plus an automated structural-contract test that validates the outputs.

### E2E Test Design Rules

1. Test runtime behavior (does `/slo-legal draft contractor-sow` produce the right artifact in the right tier?), not just SKILL.md prose shape.
2. Test the full skill invocation when possible (the fixture in the manual smoke test is a real-shape input).
3. Test degraded states (deal value > £5k → triage; counterparty has lawyer → triage; GDPR doc → triage; non-UK jurisdiction → error).
4. Assert against observable artifacts (files on disk, frontmatter parsed, body bytes hashed).
5. Prefer at least one test that exercises the cross-skill citation (e.g. M4's test asserts every advisor SKILL.md cites the four predicate ids).

---

## Dependency, Migration, and Refactor Policy

### Dependency policy

M1–M4 add no new dependencies. Tests reuse: `assert_cmd` (binary invocation, in `sldo-install` dev-deps), `tempfile` (temp dirs), `regex` (markdown section parsing), `anyhow` (error handling), `sha2` (already used by `sldo-tla-sha` for hash verification — pulled into `sldo-install` dev-deps in M1 if not already present), `std::fs` (file reads). All already in the workspace.

### Migration policy

No schema migration in M1–M4. No new runbook template fields. No changes to `docs/slo/templates/runbook-template_v_3_template.md`. New artifact frontmatter schema is additive (M2 ships `references/biz/artifact-schema.md` describing it; consumers — the founders' downstream tooling — are not yet wired to read it, so the schema is a forward contract, not a migration).

### Refactor budget

- **M1**: Minimal local refactor permitted in listed files only. (M1 introduces `/slo-legal` and three new reference files; no existing files are edited except possibly `crates/sldo-install/Cargo.toml` to add `sha2` as a dev-dependency if not already present.)
- **M2**: Minimal local refactor permitted in listed files only.
- **M3**: Minimal local refactor permitted in listed files only.
- **M4**: Minimal local refactor permitted in listed files only PLUS a single CLAUDE.md edit to add the biz-pack catalog table.

---

## Evidence Log Template

Copy this table into each milestone section and fill it in during execution.

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install` | all pre-existing tests green | | | |
| Structural-contract tests created | `crates/sldo-install/tests/e2e_biz_a_m<N>.rs` | compiles, fails for expected reason | | | |
| Smoke-test checklist created | `docs/slo/verify/biz-a-m<N>-smoke.md` | exists with empty checkboxes | | | |
| Implementation | SKILL.md + references + templates edits | structural contract satisfied | | | |
| Full tests | baseline command | green | | | |
| Milestone E2E test | `cargo test -p sldo-install --test e2e_biz_a_m<N>` | green | | | |
| Build/boot | `cargo build -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install` | builds cleanly | | | |
| Installer dry-run | `./target/release/sldo-install --dry-run` | new skill discovered; `references/biz/` NOT discovered | | | |
| Smoke tests | manual fixture invocation per `docs/slo/verify/biz-a-m<N>-smoke.md` | all checked | | | |
| Test artifact cleanup | `git status` | no untracked test artifacts | | | |
| .gitignore review | review `.gitignore` | patterns current | | | |
| Compatibility checks | parse existing runbooks; existing skills load; advisor SKILL.md cites all four predicate ids (M2+ check) | no regressions | | | |

---

## Self-Review Gate

Before marking a milestone done, answer every question.

- Did I change only allowed files?
- Did I avoid unrelated refactors?
- Did I preserve all listed public interfaces (existing skill verbs, `SKILL.md` frontmatter keys, the four triage-gate predicate IDs, the v3 template, the canonical baseline test command)?
- Did I add structural-contract tests for failure modes (missing frontmatter, oneNDA hash mismatch, predicate-id missing, `references/biz/` accidentally discovered as a skill) not just happy paths?
- Did I remove temporary debug code, mocks, placeholders, and commented-out dead code from edited files?
- Did I update `docs/ARCHITECTURE.md` only for reality at HEAD, never for planned work?
- Is every assumption either verified or explicitly documented as unresolved?
- Do all tests clean up their output artifacts? Does `git status` show a clean working tree?
- Is `.gitignore` up to date?
- Is the milestone truly done according to its Definition of Done?

If any answer is "no", the milestone is not complete.

---

## Lessons-Learned File Template

Path: `docs/slo/lessons/biz-a-m<N>.md`. Use the shape from `docs/slo/templates/runbook-template_v_3_template.md`.

## Completion Summary Template

Path: `docs/slo/completion/biz-a-m<N>.md`. Use the shape from `docs/slo/templates/runbook-template_v_3_template.md`.

---

## Milestone Plan

### Milestone 1 — `/slo-legal` v1 + shared `references/biz/` wedge

**Goal**: Ship `/slo-legal` (the first advisor skill) end-to-end with `draft|translate|triage|prepare` modes, the four hard-block gates wired in, oneNDA verbatim render under CC BY-ND 4.0 with SHA-256 hash check, JPP Law cost-baseline ROI block, UK-only jurisdiction handling — alongside the two M1-tier shared references (`references/biz/triage-gate.md` + `references/biz/cost-baseline-jpp-law-2026.md`) and the canonical oneNDA template (`references/biz/templates/onenda-uk.md`). The wedge proves the advisor pattern in one skill before M2–M4 replicate it.

**Context**: Today, the SLO skill pack has no advisor-pattern skill. `crates/sldo-install/src/install.rs:44-71` (`discover_skills()`) iterates `<skills_dir>/*`, requires `<name>/SKILL.md` to exist, and skips `.`-prefixed names — but does NOT filter leading underscore. This is why shared scaffolding lives at `references/biz/` outside `skills/` (verified in `docs/slo/design/biz-skill-pack-overview.md` and `docs/slo/design/biz-skill-pack-stack-decision.md`). The four hard-block predicates and the oneNDA-verbatim rule are the design's most drift-prone surfaces; landing them once in M1 and citing them from M2/M3/M4 prevents the four-skill drift that the dossier identified as the predictable failure mode of Option A (per-skill self-contained).

**Important design rule**: M1 ships ONLY the two shared reference files + the oneNDA template + `/slo-legal`. The full `references/biz/` skeleton (artifact-schema, jurisdiction-uk, ico-duaa-index, ico-enforcement-reality, open-template-anchors, hmrc-vcm-index, ir35-cest-factors) is deferred to M2–M4 — per CLAUDE.md's "three similar lines is better than a premature abstraction" rule. Each shared reference lands in the milestone where its second consumer arrives.

**Refactor budget**: `Minimal local refactor permitted in listed files only`. The only existing-file edit is a possible `crates/sldo-install/Cargo.toml` dev-dependencies addition (to add `sha2` if not already present in `sldo-install` dev-deps).

#### Contract Block

| Field | Value |
|---|---|
| Inputs | The locked design decisions in `docs/slo/design/biz-skill-pack-{overview,stack-decision,interfaces,threat-model}.md`; the canonical oneNDA template at https://www.onenda.org/ (downloaded once at M1 implementation time and saved as `references/biz/templates/onenda-uk.md`); JPP Law fixed-fee public pricing at https://www.jpplaw.co.uk/sectors/fixed-fee-startup/ (retrieved 2026-04-25). |
| Outputs | New skill: `skills/slo-legal/SKILL.md`. New shared references: `references/biz/triage-gate.md`, `references/biz/cost-baseline-jpp-law-2026.md`, `references/biz/templates/onenda-uk.md`. New structural-contract test: `crates/sldo-install/tests/e2e_biz_a_m1.rs`. New manual smoke-test checklist: `docs/slo/verify/biz-a-m1-smoke.md`. Optional: `crates/sldo-install/Cargo.toml` dev-dependencies addition for `sha2` if not present. |
| Interfaces touched | NEW skill verb `/slo-legal` (advisor pattern: `draft <doc-type>`, `translate <file>`, `triage <situation>`, `prepare <situation>`). NEW shared-scaffolding location: `references/biz/` at repo root. NEW four predicate IDs: `gate-1-regulated`, `gate-2-deal-value-over-5k`, `gate-3-counterparty-has-lawyer-or-their-paper`, `gate-4-gdpr-document`. NEW biz artifact convention: `docs/biz/<area>/<artifact>.md` (gitignored confidential) and `docs/biz-public/<area>/<artifact>.md` (tracked) — the conventions are documented in `/slo-legal` SKILL.md and the founder-repo `.gitignore` snippet ships in M1. NEW oneNDA license obligation: SHA-256 hash check at install + verify time. None of the existing 14 skills' interfaces change. |
| Files allowed to change | `skills/slo-legal/SKILL.md` (NEW); `references/biz/triage-gate.md` (NEW); `references/biz/cost-baseline-jpp-law-2026.md` (NEW); `references/biz/templates/onenda-uk.md` (NEW); `crates/sldo-install/tests/e2e_biz_a_m1.rs` (NEW); `crates/sldo-install/Cargo.toml` (only the `[dev-dependencies]` table, only to add `sha2` if not present); `docs/slo/verify/biz-a-m1-smoke.md` (NEW); `docs/ARCHITECTURE.md` (one-line addition naming `/slo-legal` + `references/biz/` location, after M1 ships); this runbook's Milestone Tracker + Evidence Log rows. |
| Files to read before changing anything | `docs/slo/design/biz-skill-pack-overview.md` (locked decisions, planned architecture, non-negotiables); `docs/slo/design/biz-skill-pack-interfaces.md` (advisor mode contract, predicate-id schema, frontmatter contract); `docs/slo/design/biz-skill-pack-threat-model.md` (abuse cases tm-biz-abuse-1, tm-biz-abuse-2, tm-biz-abuse-3, tm-biz-abuse-7); `docs/slo/research/biz-skill-pack/dossier.md` + `synthesis.md`; `crates/sldo-install/src/install.rs` (lines 44-71 — confirm the skill-discovery behavior the design relies on); `crates/sldo-install/Cargo.toml` (check whether `sha2` is already a dev-dependency); `docs/slo/completed/RUNBOOK-SLO-SECURITY-EMBEDDING.md` (precedent for Markdown-only milestones with `e2e_slo_sec_m<N>.rs` tests under `crates/sldo-install/tests/`); `skills/slo-architect/SKILL.md` (existing skill structure precedent); `SECURITY.md` (root, biz-pack section already merged by `/slo-architect`); `https://www.onenda.org/` (canonical oneNDA template — download once); `https://www.jpplaw.co.uk/sectors/fixed-fee-startup/` (cost-baseline page, retrieve at impl time and snapshot prices into `cost-baseline-jpp-law-2026.md` with retrieval date). |
| New files allowed | `skills/slo-legal/SKILL.md`; `references/biz/triage-gate.md`; `references/biz/cost-baseline-jpp-law-2026.md`; `references/biz/templates/onenda-uk.md`; `crates/sldo-install/tests/e2e_biz_a_m1.rs`; `docs/slo/verify/biz-a-m1-smoke.md`. The `references/biz/` and `references/biz/templates/` directories do not exist today; creating them is part of this milestone. |
| New dependencies allowed | `none` runtime; `sha2` as a `crates/sldo-install/Cargo.toml` `[dev-dependencies]` entry IF NOT ALREADY PRESENT (it is already a workspace member dependency in `sldo-tla-sha`, so adding it as a dev-dep in `sldo-install` is a single line; check first). |
| Migration allowed | `no` |
| Compatibility commitments | All existing 14 skills (`/slo-ideate`, `/slo-research`, `/slo-architect`, `/slo-tla`, `/slo-plan`, `/slo-critique`, `/slo-execute`, `/slo-verify`, `/slo-retro`, `/slo-ship`, `/slo-resume`, `/slo-freeze`, `/slo-second-opinion`, `/get-api-docs`) continue to load and run unchanged. `sldo-install` continues to install all 14 existing skills AND now installs `/slo-legal` (15 total). `sldo-install` MUST NOT discover `references/biz/` as a skill (the directory has no `SKILL.md`, but the test asserts the dry-run output explicitly). All existing runbooks (`docs/RUNBOOK-API-FACADE.md`, `RUNBOOK-AWS-ORG-SETUP.md`, `RUNBOOK-RESEARCH-GENERATED.md`, `RUNBOOK-RESEARCH.md`, `RUNBOOK-RUST-REWRITE.md`, `RUNBOOK-SKILL-PACK.md`, `RUNBOOK-SLO-SECURITY-EMBEDDING.md`, `RUNBOOK-TAURI-DESKTOP.md`, `RUNBOOK-TLA-SHA-AUTOPOP.md`, `RUNBOOK-VOICE-FIX-HOME.md`) continue to parse against the v3 template. |
| Forbidden shortcuts | No inlining of `triage-gate.md` predicates into `slo-legal/SKILL.md` — predicates live in the shared reference, skill prose cites them with explicit predicate-id mentions. No partial oneNDA template (truncated for brevity) — the canonical bytes are full or the hash check fails. No "TODO: add cost lines later" in `cost-baseline-jpp-law-2026.md` — every line item ships with a price + source URL + retrieval date or the file isn't in M1. No silent edits to oneNDA prose ("just fixing a typo") — license-required verbatim. No premature stubs for `--jurisdiction us` / `--jurisdiction eu` — UK only in v1. No enabling WebFetch / WebSearch in `slo-legal/SKILL.md`. No moving `references/biz/` under `skills/`. No skipping the SHA-256 hash test "because the file is right there in git" — the hash check is the license-compliance enforcement mechanism, not a developer-quality check. |
| **Data classification** | `Confidential` — `/slo-legal draft` outputs land in `docs/biz/legal/<doc>-<counterparty>.md` (founder's repo, gitignored) and contain real counterparty names, deal values, IP scope. The skill prose itself (in `skills/slo-legal/SKILL.md`) is `Public`, but the artifact-write surface this milestone introduces is Confidential. Per `references/proactive-controls-vocabulary.md`, this milestone MUST cite at least one `secure_data`-equivalent control in the next row and include at least one abuse-case scenario covering the disclosure surface. |
| **Proactive controls in play** | This is a Markdown-only skill with `security_libs_required: false`. OWASP Proactive Controls v3 vocabulary applies. Cited: **C1 Define Security Requirements** — `SECURITY.md` (root, biz-pack section) + `docs/slo/design/biz-skill-pack-threat-model.md` (rows tm-biz-abuse-1, -2, -3, -7) are the project-wide security defaults consumed before the skill drafts anything. **C5 Validate All Inputs** — the four hard-block predicates in `references/biz/triage-gate.md` are validation predicates run BEFORE any draft logic; `gate-2-deal-value-over-5k` and `gate-4-gdpr-document` reject input categorically. **C8 Protect Data Everywhere** — two-tier output convention places confidential drafts in `.gitignore`'d `docs/biz/`; founder-repo `.gitignore` snippet ships in M1; skill prose includes a write-time warning when the target dir is git-tracked AND a remote exists AND `tier: confidential`. **C10 Handle All Errors and Exceptions** — non-UK jurisdiction emits a clean "v1 supports UK only" error; oneNDA hash mismatch fails install with a specific license-violation error message, not a panic. |
| **Abuse acceptance scenarios** | Required — this milestone introduces three new surfaces: `/slo-legal` skill (draft writes), `references/biz/templates/onenda-uk.md` (license-protected canonical bytes), founder's-repo `docs/biz/` artifact write. Three abuse-case BDD rows below: `gdpr_doc_draft_routes_to_triage` (cites tm-biz-abuse-3), `onenda_template_tampering_blocked` (cites tm-biz-abuse-2), `confidential_draft_to_public_tier_rejected` (cites tm-biz-abuse-1). Plus one Q4-shaped: `non_uk_jurisdiction_arg_rejected` (operational guard, not a tm-row). See BDD Acceptance Scenarios table below. |

#### Out of Scope / Must Not Do

- No changes to `skills/slo-{ideate,research,architect,tla,plan,critique,execute,verify,retro,ship,resume,freeze,second-opinion}/`, `skills/get-api-docs/` — those are out of this runbook entirely.
- No work on `/slo-accounting`, `/slo-equity`, `/slo-fundraise` — those are M2/M3/M4.
- No `references/biz/artifact-schema.md`, `references/biz/jurisdiction-uk.md`, `references/biz/ico-duaa-index.md`, `references/biz/ico-enforcement-reality.md`, `references/biz/open-template-anchors.md`, `references/biz/hmrc-vcm-index.md`, or `references/biz/ir35-cest-factors.md` — those are M2/M3/M4.
- No customer-generator skills (`/slo-talk-to-users`, etc.) — Runbook B.
- No team skills (`/slo-cofounder`, etc.) — Runbook C.
- No DOCX export — out of scope; Markdown only.
- No edits to `docs/slo/templates/runbook-template_v_3_template.md`.
- No edits to `crates/sldo-tauri/`.
- No PII-pattern scan implementation in `/slo-verify` — that lands in M2 (when the artifact-schema reference exists to define what categories are confidential vs public). M1's `docs/biz/` placement discipline is enforced by skill prose + manual smoke test only.
- No `--jurisdiction` flag implementation in M1; non-UK requests emit an error message, not a stub for future jurisdictions.

#### Pre-Flight

1. Read `docs/slo/design/biz-skill-pack-overview.md` end-to-end. Confirm understanding of the four locked decisions: GDPR broad hard-block, JPP Law cost baseline, UK only v1, oneNDA verbatim.
2. Read `docs/slo/design/biz-skill-pack-interfaces.md` — internalise the advisor mode contract, the predicate-id schema, and the frontmatter contract.
3. Read `docs/slo/design/biz-skill-pack-threat-model.md` rows tm-biz-abuse-1 through tm-biz-abuse-3 and tm-biz-abuse-7 (these are the M1 abuse cases).
4. Read `crates/sldo-install/src/install.rs` lines 44-71 (`discover_skills()`). Confirm the function requires `<skills_dir>/<name>/SKILL.md` and that `references/biz/` (a sibling of `skills/`) cannot be discovered.
5. Read `docs/slo/completed/RUNBOOK-SLO-SECURITY-EMBEDDING.md` Milestone 1 — copy the `e2e_slo_sec_m1.rs` test structure as the precedent for `e2e_biz_a_m1.rs`.
6. Download oneNDA canonical template from https://www.onenda.org/ (latest UK version; verify via the consortium's published PDF + Markdown copy). Save to `references/biz/templates/onenda-uk.md` as the canonical body. Compute SHA-256; pin into `e2e_biz_a_m1.rs` as a `const`.
7. Retrieve JPP Law fixed-fee pricing from https://www.jpplaw.co.uk/sectors/fixed-fee-startup/ (today, 2026-04-25). Snapshot the relevant line items (NDA, contractor agreement, IP assignment, T&Cs equivalents) with the retrieval date.
8. Run the baseline test command and confirm green: `cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install`.
9. Update Milestone Tracker: M1 status `in_progress`, Started date today.
10. Create the structural-contract test file `crates/sldo-install/tests/e2e_biz_a_m1.rs` with all test stubs failing for the right reason.

#### Files Allowed to Change

| File | Purpose | Scope |
|---|---|---|
| `skills/slo-legal/SKILL.md` (NEW) | Advisor skill prose: name, description, four-mode contract, hard-block gate citations, oneNDA verbatim rule, JPP Law ROI block reference, UK-only jurisdiction error | Full new file |
| `references/biz/triage-gate.md` (NEW) | Single source of truth for the four hard-block predicates; predicate-id schema, predicate body prose, route-to mapping | Full new file |
| `references/biz/cost-baseline-jpp-law-2026.md` (NEW) | JPP Law fixed-fee public pricing snapshot with retrieval date and explicit line-item table | Full new file |
| `references/biz/templates/onenda-uk.md` (NEW) | Canonical oneNDA UK template body bytes, CC BY-ND 4.0 verbatim | Full new file |
| `crates/sldo-install/tests/e2e_biz_a_m1.rs` (NEW) | Structural-contract tests: SKILL.md exists + frontmatter + four mode keywords + four predicate-id citations; triage-gate.md predicate-id format; cost-baseline file format; oneNDA SHA-256 hash check; `sldo-install --dry-run` does NOT include `references/biz/` | Full new file |
| `crates/sldo-install/Cargo.toml` | Add `sha2` to `[dev-dependencies]` only if not already present | Single-line addition |
| `docs/slo/verify/biz-a-m1-smoke.md` (NEW) | Manual smoke-test checklist: invoke `/slo-legal draft contractor-sow` against a fixture deal; verify output landed in `docs/biz/`; verify ROI block cites JPP Law; invoke `/slo-legal draft privacy-notice` and verify it routes to triage (gate-4); invoke `/slo-legal triage "deal worth £20k with counterparty's lawyer present"` and verify both gate-2 and gate-3 fire | Full new file |
| `docs/ARCHITECTURE.md` | One-line addition naming `/slo-legal` + `references/biz/` location; only after M1 ships | Single-section additive edit |
| This runbook (Milestone Tracker + Evidence Log) | Track progress | Two row updates |

#### Step-by-Step

1. **Pre-flight** (Pre-Flight section above) — read all listed files, download oneNDA template, retrieve JPP Law pricing, baseline tests green.
2. **Write structural-contract tests first.** Create `crates/sldo-install/tests/e2e_biz_a_m1.rs` with every test failing for the expected reason (file not found, frontmatter key absent, predicate-id missing, hash mismatch). Run `cargo test -p sldo-install --test e2e_biz_a_m1` — confirm all-fail with the right reasons.
3. **Author `references/biz/triage-gate.md`.** Document the four predicate IDs in a table with `id` / `name` / `predicate` / `if_true` / `route_to` / `rationale_doc` columns per the interfaces doc. Pin the £5,000 threshold for `gate-2`. For `gate-4`, document the broad-block rule explicitly with provenance pointer (synthesis paragraph 2 + the locked 2026-04-25 decision in the overview).
4. **Author `references/biz/cost-baseline-jpp-law-2026.md`.** Snapshot the JPP Law fixed-fee table with retrieval date `2026-04-25`. Each line carries `item` / `price (£)` / `source URL` / `retrieved`. Add a "valid through" suggestion of one year (annual refresh expected).
5. **Land oneNDA verbatim at `references/biz/templates/onenda-uk.md`.** Save the canonical UK template body bytes exactly as published by the oneNDA consortium. Compute SHA-256. Pin the hash as a `const` in `e2e_biz_a_m1.rs`. Confirm hash test passes.
6. **Author `skills/slo-legal/SKILL.md`.** Frontmatter: `name: slo-legal`, `description: <advisor skill summary>`. Body: four-mode contract; hard-block gate citations (every gate cited by id + the predicate body's one-line description); oneNDA verbatim rule with explicit "do not edit the body" language; JPP Law cost-baseline reference; UK-only jurisdiction error pattern; output-tier convention; no-WebFetch reminder; output-frontmatter contract; lawyer-review-recommended header. Cite `references/biz/triage-gate.md`, `references/biz/cost-baseline-jpp-law-2026.md`, `references/biz/templates/onenda-uk.md`. End with a handoff suggestion to `/slo-execute` if the founder is producing a draft for the first time.
7. **Author `docs/slo/verify/biz-a-m1-smoke.md`.** Empty checklist with the four manual smoke-test items: (a) `draft contractor-sow` happy path → check `docs/biz/legal/<contractor>-sow.md` exists with right frontmatter + ROI block; (b) `draft privacy-notice` → routes to triage with DPO-required language; (c) `triage "deal worth £20k with counterparty's lawyer"` → routes to lawyer with both gate-2 and gate-3 cited; (d) `draft contractor-sow --jurisdiction us` → emits "v1 supports UK only" error.
8. **Re-run structural-contract tests** — `cargo test -p sldo-install --test e2e_biz_a_m1` — all green.
9. **Run installer dry-run** — `cargo build -p sldo-install --release && ./target/release/sldo-install --dry-run` — confirm `slo-legal` appears in the discovered set, confirm no entry under `references/biz/`.
10. **Manual smoke test** — invoke `/slo-legal` against the four fixtures from step 7 in a real Claude Code session; tick checkboxes in `docs/slo/verify/biz-a-m1-smoke.md`; record any deviations as Evidence Log notes.

#### BDD Acceptance Scenarios

| Scenario | Category | Given | When | Then | Threat-model row | Control |
|---|---|---|---|---|---|---|
| `slo_legal_skill_md_has_required_frontmatter` | happy path | a freshly-authored `skills/slo-legal/SKILL.md` | the structural-contract test parses YAML frontmatter | the `name` field equals `slo-legal` and the `description` field is non-empty | n/a (frontmatter contract, not abuse) | static parse via `regex` on frontmatter delimiters |
| `slo_legal_skill_md_documents_four_modes` | happy path | the `skills/slo-legal/SKILL.md` body | the structural-contract test searches for the four mode keywords | each of `draft`, `translate`, `triage`, `prepare` appears at least once in a mode-defining heading or table | n/a | regex search for `\bdraft\b`, `\btranslate\b`, `\btriage\b`, `\bprepare\b` in the modes section |
| `triage_gate_md_defines_four_predicate_ids` | happy path | `references/biz/triage-gate.md` | the structural-contract test parses the predicate-id table | exactly four predicate IDs are present: `gate-1-regulated`, `gate-2-deal-value-over-5k`, `gate-3-counterparty-has-lawyer-or-their-paper`, `gate-4-gdpr-document`; each has all six required columns (`id`, `name`, `predicate`, `if_true`, `route_to`, `rationale_doc`) | n/a | regex match on the four exact predicate-id strings |
| `slo_legal_skill_md_cites_all_four_predicate_ids` | happy path | the `skills/slo-legal/SKILL.md` body | the structural-contract test searches the SKILL.md body for predicate-id mentions | each of the four predicate IDs appears at least once in the SKILL.md body, anchored to a hard-block discussion (not in a header / TOC only) | n/a | regex search; the test rejects if a predicate id appears only inside a single heading line |
| `cost_baseline_md_carries_retrieval_date` | happy path | `references/biz/cost-baseline-jpp-law-2026.md` | the structural-contract test parses the file | a `retrieved:` frontmatter field is present and matches `\d{4}-\d{2}-\d{2}`; at least one line item with a `£` price and a JPP Law source URL is present | n/a | regex parse |
| `onenda_template_sha256_matches_pinned_hash` | abuse case | the canonical oneNDA UK template at `references/biz/templates/onenda-uk.md` | the structural-contract test computes SHA-256 of the file bytes | the hash equals the pinned `const ONENDA_UK_SHA256` in `e2e_biz_a_m1.rs`; mismatch fails the test with the message `oneNDA template tampering — license-required byte verbatim` | `tm-biz-abuse-2` | `sha2::Sha256::digest` over the file bytes; pin the hash as a `const` in the test |
| `references_biz_dir_not_discovered_as_skill` | abuse case (compatibility) | `references/biz/` exists at repo root with files but no `SKILL.md` | the test invokes `sldo-install --dry-run` and parses the discovered-skill list | `references/biz/` does NOT appear; the four existing skill verbs that match `slo-` are present plus the new `slo-legal`; total count increases by exactly 1 vs. baseline | tied to `crates/sldo-install/src/install.rs:44-71` invariant; not a new abuse row | use `assert_cmd::Command` to invoke; parse stdout for the discovered-set listing |
| `gdpr_doc_draft_routes_to_triage` | abuse case | a fixture invocation `/slo-legal draft privacy-notice` | the SKILL.md body is parsed for the gate-4 routing rule | the SKILL.md body contains an unambiguous rule that GDPR doc-types (privacy-notice, ropa, dpa, internal-data-protection-policy) trigger gate-4 and route to triage with DPO-required language and an ICO-DUAA reference; structural test asserts presence of the rule + the doc-type list | `tm-biz-abuse-3` | regex parse on the SKILL.md body for the rule + the four GDPR doc-types |
| `confidential_draft_to_public_tier_rejected` | abuse case | a manual-smoke fixture writing `/slo-legal draft contractor-sow` output | the test asserts the SKILL.md body documents the two-tier convention | the SKILL.md body explicitly mentions `docs/biz/` as the default for `tier: confidential` AND mentions the founder-repo `.gitignore` requirement for `docs/biz/`; the body MUST NOT include guidance to place confidential drafts in `docs/biz-public/` | `tm-biz-abuse-1` | regex parse for the convention prose + the explicit forbidden-pattern absence |
| `non_uk_jurisdiction_arg_rejected` | invalid input | a fixture invocation pattern documented in SKILL.md | the SKILL.md body documents the non-UK error path | the body contains an unambiguous "v1 supports UK only" error string + an explicit "see Runbook v2 architectural pivot" reference; structural test asserts presence | n/a (operational guard) | regex parse |

#### Regression Tests

The following pre-existing tests must still pass after M1:

- `cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install` (full baseline)
- `cargo test -p sldo-install --test e2e_slo_sec_m1` through `e2e_slo_sec_m4` (slo-security-embedding regression)
- `cargo test -p sldo-install` integration-test suite (existing skill-discovery tests)

#### Compatibility Checklist

- [ ] All 14 existing skills load via `sldo-install --dry-run` without error.
- [ ] All existing runbooks (`docs/RUNBOOK-API-FACADE.md`, `RUNBOOK-AWS-ORG-SETUP.md`, `RUNBOOK-RESEARCH-GENERATED.md`, `RUNBOOK-RESEARCH.md`, `RUNBOOK-RUST-REWRITE.md`, `RUNBOOK-SKILL-PACK.md`, `RUNBOOK-SLO-SECURITY-EMBEDDING.md`, `RUNBOOK-TAURI-DESKTOP.md`, `RUNBOOK-TLA-SHA-AUTOPOP.md`, `RUNBOOK-VOICE-FIX-HOME.md`) continue to parse against `docs/slo/templates/runbook-template_v_3_template.md`.
- [ ] `crates/sldo-install/src/install.rs` `discover_skills()` is unchanged.
- [ ] `docs/slo/templates/runbook-template_v_3_template.md` is unchanged.
- [ ] `SECURITY.md` (root) — only the existing biz-pack section (added by `/slo-architect`) is present; no further edits in M1.
- [ ] `crates/sldo-tauri/` is unchanged.

#### E2E Runtime Validation

- Test functions in `crates/sldo-install/tests/e2e_biz_a_m1.rs`:
  - `slo_legal_skill_md_has_required_frontmatter`
  - `slo_legal_skill_md_documents_four_modes`
  - `triage_gate_md_defines_four_predicate_ids`
  - `slo_legal_skill_md_cites_all_four_predicate_ids`
  - `cost_baseline_md_carries_retrieval_date`
  - `onenda_template_sha256_matches_pinned_hash`
  - `references_biz_dir_not_discovered_as_skill`
  - `gdpr_doc_draft_routes_to_triage`
  - `confidential_draft_to_public_tier_rejected`
  - `non_uk_jurisdiction_arg_rejected`
- Pass criteria: all ten green; baseline test command unchanged; `sldo-install --dry-run` shows 15 skills (14 existing + `slo-legal`) and no `references/biz/` entry.

#### Smoke Tests

Documented in `docs/slo/verify/biz-a-m1-smoke.md`. Manual invocation in a real Claude Code session against four fixtures:

1. **Happy path — `/slo-legal draft contractor-sow`** with a fixture deal (counterparty `Acme Solo Eng`, daily rate £400 for 10 days, IP scope = "all deliverables and learnings"). Verify: artifact lands at `docs/biz/legal/contractor-sow-acme-solo-eng-2026-04-25.md` (gitignored); frontmatter has `tier: confidential`, `mode: draft`, `triage_gate_passed: true`, `lawyer_review_recommended: true`, `cost_baseline_ref: references/biz/cost-baseline-jpp-law-2026.md@2026-04-25`; body contains a "LAWYER REVIEW RECOMMENDED" header; ROI block cites JPP Law.
2. **GDPR hard-block — `/slo-legal draft privacy-notice`** with any fixture context. Verify: skill refuses to draft; routes to triage; output lands at `docs/biz-public/legal/triage-privacy-notice-2026-04-25.md`; body explicitly cites `gate-4-gdpr-document`; body recommends DPO + ICO DUAA reference.
3. **Multi-gate triage — `/slo-legal triage "deal worth £20k where the other side has a lawyer"`**. Verify: triage output cites BOTH `gate-2-deal-value-over-5k` AND `gate-3-counterparty-has-lawyer-or-their-paper`; routes to lawyer; landed in `docs/biz-public/legal/triage-...`.
4. **Non-UK jurisdiction — `/slo-legal draft contractor-sow --jurisdiction us`** (or any non-UK arg surface the SKILL.md documents). Verify: skill emits "v1 supports UK only; US/EU is a v2 architectural pivot — see [link]" error; no artifact written.

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install` | all pre-existing tests green | | | |
| Structural-contract tests created | `crates/sldo-install/tests/e2e_biz_a_m1.rs` | compiles, 10 tests fail for expected reasons | | | |
| Smoke-test checklist created | `docs/slo/verify/biz-a-m1-smoke.md` | exists with 4 unchecked checkboxes | | | |
| oneNDA template downloaded | `references/biz/templates/onenda-uk.md` | file exists, SHA-256 captured as a const in test | | | |
| JPP Law pricing retrieved | `references/biz/cost-baseline-jpp-law-2026.md` | file exists, retrieval-date `2026-04-25`, line items present | | | |
| `triage-gate.md` authored | `references/biz/triage-gate.md` | four predicate-id rows, six columns each | | | |
| `slo-legal/SKILL.md` authored | `skills/slo-legal/SKILL.md` | frontmatter + four-mode contract + four-predicate citations + oneNDA-verbatim rule + UK-only error + no-WebFetch | | | |
| Full tests | baseline command | green | | | |
| Milestone E2E test | `cargo test -p sldo-install --test e2e_biz_a_m1` | 10/10 green | | | |
| Build/boot | `cargo build -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install` | builds cleanly | | | |
| Installer dry-run | `./target/release/sldo-install --dry-run` | 15 skills discovered (14 existing + `slo-legal`); no `references/biz/` entry | | | |
| Smoke tests | manual fixture invocations 1–4 from `biz-a-m1-smoke.md` | all checked | | | |
| Test artifact cleanup | `git status` | no untracked test artifacts | | | |
| .gitignore review | review `.gitignore` | unchanged (no new generated files) | | | |
| Compatibility checks | parse existing runbooks; existing skills load; `sldo-install` ignores `references/biz/` | no regressions | | | |

#### Definition of Done

- [ ] Four files created: `skills/slo-legal/SKILL.md`, `references/biz/triage-gate.md`, `references/biz/cost-baseline-jpp-law-2026.md`, `references/biz/templates/onenda-uk.md`.
- [ ] Two test/verification files created: `crates/sldo-install/tests/e2e_biz_a_m1.rs`, `docs/slo/verify/biz-a-m1-smoke.md`.
- [ ] `cargo test -p sldo-install --test e2e_biz_a_m1` returns 10/10 green.
- [ ] `cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install` (full baseline) green.
- [ ] `sldo-install --dry-run` shows 15 skills (14 existing + `slo-legal`); no `references/biz/` entry.
- [ ] All four manual smoke-test scenarios in `docs/slo/verify/biz-a-m1-smoke.md` are ticked.
- [ ] oneNDA SHA-256 hash test passing — license-required.
- [ ] Compatibility Checklist all checked.
- [ ] Self-Review Gate all "yes".
- [ ] `docs/ARCHITECTURE.md` updated with one-line addition for `/slo-legal` + `references/biz/` location.
- [ ] `docs/slo/lessons/biz-a-m1.md` and `docs/slo/completion/biz-a-m1.md` written.
- [ ] Milestone Tracker: M1 status `done`, Completed date recorded.

---

## Documentation Update Table

(To be completed after each milestone — what gets written / appended / linked.)

| File | M1 | M2 | M3 | M4 | Notes |
|---|---|---|---|---|---|
| `docs/ARCHITECTURE.md` | one-line: `/slo-legal` + `references/biz/` | one-line: `/slo-accounting` + 5 new refs | one-line: `/slo-equity` + `hmrc-vcm-index.md` | one-line: `/slo-fundraise` + `ir35-cest-factors.md` | reality at HEAD; planned work stays in design overview |
| `CLAUDE.md` | no edit | no edit | no edit | adds biz pack catalog table | single edit at end of Runbook A |
| `SECURITY.md` (root) | no edit (already merged by `/slo-architect`) | no edit | no edit | no edit | biz-pack section is stable; gate IDs documented in interfaces doc |
| `docs/slo/lessons/biz-a-m<N>.md` | NEW | NEW | NEW | NEW | one per milestone |
| `docs/slo/completion/biz-a-m<N>.md` | NEW | NEW | NEW | NEW | one per milestone |
| `docs/slo/design/biz-skill-pack-overview.md` | no edit | no edit | no edit | no edit | already locked by `/slo-architect` |

---

### Milestone 2 — `/slo-accounting` + M2-tier shared references

**Goal**: Ship `/slo-accounting` (the second advisor skill) end-to-end with the four-mode contract, proving the advisor pattern replicates cleanly across skills by **citing — not redefining — the four predicate IDs from M1's `references/biz/triage-gate.md`**. Land the five M2-tier shared references the design overview names: `references/biz/artifact-schema.md` (frontmatter contract for biz artifacts), `references/biz/jurisdiction-uk.md` (UK-only prose anchors plus the "v1 UK only" error pattern), `references/biz/ico-duaa-index.md` (DUAA 2025 dates + lawful-basis examples), `references/biz/ico-enforcement-reality.md` (PECR-vs-Article-13 pattern, descriptive provenance for the broad GDPR hard-block locked 2026-04-25), `references/biz/open-template-anchors.md` (oneNDA + oneSaaS + Kindrik notes including the CC BY-ND 4.0 verbatim rule).

**Context**: M1 shipped `/slo-legal`, the four hard-block predicates, the JPP Law cost baseline, and the oneNDA template. M2's value is twofold: (1) prove the advisor pattern is reusable — the second skill should be authorable in roughly the same effort as the first because the shared scaffolding does the work; (2) ship the rest of the M2-tier shared references now that a second consumer (`/slo-accounting`) exists to cite them, per CLAUDE.md's "three similar lines is better than a premature abstraction" rule. The five M2-tier references all have a clear M2 consumer: artifact-schema is cited by every advisor skill's frontmatter contract; jurisdiction-uk is cited by the UK-only error path in `/slo-legal` and `/slo-accounting`; ico-duaa-index and ico-enforcement-reality are cited by the gate-4 (GDPR) routing prose; open-template-anchors is cited by `/slo-legal draft nda` (oneNDA verbatim) and seeds the M3+ work where T&Cs and oneSaaS land.

**Important design rule**: M2 MUST NOT modify `references/biz/triage-gate.md`. The four predicate IDs (`gate-1-regulated`, `gate-2-deal-value-over-5k`, `gate-3-counterparty-has-lawyer-or-their-paper`, `gate-4-gdpr-document`) are stable interface. `/slo-accounting`'s accounting-specific routing happens via the existing `route_to: accountant` value (already declared in M1's gate set) and via the SKILL.md body prose in `skills/slo-accounting/SKILL.md`. The cross-skill citation structural test in `e2e_biz_a_m2.rs` enforces this — the test asserts that `/slo-legal` AND `/slo-accounting` both cite all four predicate IDs without inlining or paraphrasing the predicate bodies.

**Refactor budget**: `Minimal local refactor permitted in listed files only`. No edits to `skills/slo-legal/SKILL.md`, no edits to `references/biz/triage-gate.md`, no edits to `references/biz/cost-baseline-jpp-law-2026.md`, no edits to `references/biz/templates/onenda-uk.md` — those are M1 outputs and frozen.

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Locked design decisions in `docs/slo/design/biz-skill-pack-{overview,interfaces,threat-model}.md`. M1's outputs (`skills/slo-legal/SKILL.md`, `references/biz/triage-gate.md`, etc.) as the canonical advisor-pattern reference. ICO DUAA pages (https://ico.org.uk/about-the-ico/what-we-do/legislation-we-cover/data-use-and-access-act-2025/the-data-use-and-access-act-2025-duaa-summary-of-the-changes/), Clifford Chance DUAA Feb 2026 commencement note, BDO 2025 ICO enforcement trends, URM Consulting 2024 + H1 2025 enforcement analyses (sources cited in `docs/slo/research/biz-skill-pack/sources.md`). |
| Outputs | New skill: `skills/slo-accounting/SKILL.md`. Five new shared references: `references/biz/artifact-schema.md`, `references/biz/jurisdiction-uk.md`, `references/biz/ico-duaa-index.md`, `references/biz/ico-enforcement-reality.md`, `references/biz/open-template-anchors.md`. New structural-contract test: `crates/sldo-install/tests/e2e_biz_a_m2.rs`. New manual smoke-test checklist: `docs/slo/verify/biz-a-m2-smoke.md`. |
| Interfaces touched | NEW skill verb `/slo-accounting` (advisor pattern, four modes, `route_to: accountant` for gate firings). NEW frontmatter contract documented in `references/biz/artifact-schema.md` — formal schema for biz artifacts; the schema itself is `stable` interface for downstream `/slo-verify` PII scans (deferred from M1 to Runbook B1 M1 — see "Out of Scope" below). NEW jurisdiction-uk prose anchor and "v1 UK only" error pattern formalised in `references/biz/jurisdiction-uk.md`. NEW DUAA dates / lawful-basis examples in `references/biz/ico-duaa-index.md` are cited by `gate-4-gdpr-document`'s rationale prose (rationale is cited, predicate-id is unchanged). None of M1's outputs change. |
| Files allowed to change | `skills/slo-accounting/SKILL.md` (NEW); `references/biz/artifact-schema.md` (NEW); `references/biz/jurisdiction-uk.md` (NEW); `references/biz/ico-duaa-index.md` (NEW); `references/biz/ico-enforcement-reality.md` (NEW); `references/biz/open-template-anchors.md` (NEW); `crates/sldo-install/tests/e2e_biz_a_m2.rs` (NEW); `docs/slo/verify/biz-a-m2-smoke.md` (NEW); `docs/ARCHITECTURE.md` (one-line addition naming `/slo-accounting` + the five new references); this runbook's Milestone Tracker + Evidence Log rows. |
| Files to read before changing anything | `skills/slo-legal/SKILL.md` (M1 advisor-pattern precedent — copy structure, swap domain); `references/biz/triage-gate.md` (predicate-id schema; do NOT edit, only cite); `references/biz/cost-baseline-jpp-law-2026.md` (cost-line schema; M2 may reference accounting-relevant lines from JPP Law's pricing if available); `docs/slo/design/biz-skill-pack-{overview,interfaces,threat-model}.md` (locked design); `docs/slo/research/biz-skill-pack/dossier.md` Q2 (ICO enforcement reality data — feeds `ico-enforcement-reality.md`); `docs/slo/research/biz-skill-pack/sources.md` (cite source URLs with retrieval dates); `crates/sldo-install/tests/e2e_biz_a_m1.rs` (M1 test precedent — copy structure for structural-contract tests); `docs/slo/verify/biz-a-m1-smoke.md` (smoke-test format precedent). |
| New files allowed | `skills/slo-accounting/SKILL.md`; `references/biz/artifact-schema.md`; `references/biz/jurisdiction-uk.md`; `references/biz/ico-duaa-index.md`; `references/biz/ico-enforcement-reality.md`; `references/biz/open-template-anchors.md`; `crates/sldo-install/tests/e2e_biz_a_m2.rs`; `docs/slo/verify/biz-a-m2-smoke.md`. No new directories — `references/biz/` already exists from M1. |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | All 15 existing skills (14 pre-runbook + `/slo-legal` from M1) continue to load via `sldo-install --dry-run`. M1 outputs are unchanged. The four predicate IDs in `references/biz/triage-gate.md` are byte-identical to M1's commit (structural test asserts unchanged hash or unchanged textual predicate-id table). All existing runbooks continue to parse. The oneNDA template hash from M1 is unchanged — `e2e_biz_a_m1.rs` still passes. `e2e_biz_a_m2.rs` adds new tests; it does not touch M1's tests. |
| Forbidden shortcuts | No editing `references/biz/triage-gate.md` for any reason — predicate IDs frozen. No paraphrasing predicate bodies into `/slo-accounting/SKILL.md` — cite, don't inline. No bypassing the cross-skill citation test in `e2e_biz_a_m2.rs` — every advisor SKILL.md (`/slo-legal` AND `/slo-accounting`) MUST cite all four predicate IDs. No editing `references/biz/cost-baseline-jpp-law-2026.md` to add accounting-specific lines if JPP Law doesn't publish them (cite gov.uk for accountant-required filings instead, with retrieval-date stamps). No `--jurisdiction us` / `--jurisdiction eu` stubs — UK only. No edits to `/slo-verify` SKILL.md (PII scan integration deferred — see Out of Scope). No relaxing the broad GDPR hard-block locked 2026-04-25 — `ico-enforcement-reality.md` is descriptive, not normative; gate-4 stays unconditional refusal of `draft`. |
| **Data classification** | `Confidential` — `/slo-accounting draft` outputs (e.g., a brief-the-accountant memo for a specific founder + accountant pairing) land in `docs/biz/accounting/<artifact>.md` (gitignored) and may contain real founder + counterparty + financial details. The five new reference files themselves are `Public` (descriptive UK regulatory and license content). Per `references/proactive-controls-vocabulary.md`, this milestone handles `Confidential` and so cites `secure_data`-equivalent controls (C8) below and includes abuse-case rows. |
| **Proactive controls in play** | This is a Markdown-only skill; OWASP Proactive Controls v3 vocabulary applies. Cited: **C1 Define Security Requirements** — `SECURITY.md` (root, biz-pack section already merged) + `docs/slo/design/biz-skill-pack-threat-model.md` rows tm-biz-abuse-1 / -3 / -5 / -6 govern this milestone's surfaces. **C5 Validate All Inputs** — the four hard-block predicates (cited from M1's `triage-gate.md`, NOT redefined) plus the formalised UK-jurisdiction validator in `jurisdiction-uk.md`; the artifact-schema enum on `tier` (`confidential` \| `public`) prevents free-form values. **C8 Protect Data Everywhere** — two-tier output convention now formally documented in `artifact-schema.md`; the schema declares which artifact categories are confidential vs public for downstream tooling (deferred PII-pattern scan implementation). **C10 Handle All Errors and Exceptions** — non-UK jurisdiction returns the formalised "v1 supports UK only" error string from `jurisdiction-uk.md`. |
| **Abuse acceptance scenarios** | Required — this milestone introduces three new surfaces: `/slo-accounting` skill, the artifact-schema authority over which output categories carry which `tier`, and the cross-skill citation surface that the predicate-id set must not drift. Two abuse-case BDD rows below: `triage_gate_predicate_set_unchanged_from_m1` (cites tm-biz-abuse-5), `artifact_schema_tier_value_constrained_to_enum` (operational guard — cites tm-biz-abuse-1 indirectly). One descriptive-not-normative scenario: `ico_enforcement_reality_doc_does_not_contradict_gate_4` — the new ICO-enforcement-reality reference must not be authored as an argument for relaxing gate-4. |

#### Out of Scope / Must Not Do

- No changes to `/slo-legal` SKILL.md or to any of M1's reference files. M1 outputs are frozen.
- No changes to `/slo-equity`, `/slo-fundraise`, or any Runbook B1/B2/C skill.
- No changes to `skills/slo-verify/SKILL.md` — the PII-pattern scan integration into `/slo-verify` Pass 4 is deferred from M1's earlier promise to **Runbook B1 M1** (`/slo-talk-to-users`), where the first PII-shaped generator artifacts land. M2 ships the schema (`artifact-schema.md`) that defines what to scan; the runtime integration follows when there's a real artifact to scan against. Document this deferral in `docs/slo/lessons/biz-a-m2.md`.
- No edits to `crates/sldo-install/src/install.rs`, `crates/sldo-install/src/main.rs`, or any other production Rust source — only `tests/e2e_biz_a_m2.rs` is added.
- No new dependencies. All structural-contract tests use crates already pulled in by M1's `e2e_biz_a_m1.rs` (which added `sha2` if not present).
- No template-render functionality for `/slo-accounting` — accounting docs (e.g., a one-page brief-the-accountant memo) are prose-shaped and do not need a verbatim template like oneNDA. If a consumer wants templated output for, say, an R&D claim cover letter, that ships in a future runbook.
- No `--jurisdiction` flag implementation for non-UK jurisdictions; the formalised error in `jurisdiction-uk.md` is the v1 surface.
- No `docs/slo/templates/runbook-template_v_3_template.md` edits.
- No `crates/sldo-tauri/` edits.
- No editing the oneNDA bytes — license-required.

#### Pre-Flight

1. Run baseline tests green: `cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install` plus `cargo test -p sldo-install --test e2e_biz_a_m1` (M1 must remain green throughout M2).
2. Read `docs/slo/lessons/biz-a-m1.md` (M1's lessons file). Note any structural deviations or surprises from M1 that affect M2's authoring.
3. Read `skills/slo-legal/SKILL.md` end-to-end. The `/slo-accounting` SKILL.md follows the same five-section structure (frontmatter / intro / mode contract / triage-gate citations / output convention) — copy structure, swap domain content.
4. Read `references/biz/triage-gate.md` and confirm it is unchanged from M1's commit hash. Treat the file as read-only for M2.
5. Read `docs/slo/research/biz-skill-pack/dossier.md` Q2 (ICO enforcement reality, DUAA 2025) — this is the source for `ico-duaa-index.md` and `ico-enforcement-reality.md`. Cite source URLs with retrieval-date stamps.
6. Open https://ico.org.uk/.../duaa-summary-of-the-changes/ and confirm DUAA Stage 3 commenced 5 February 2026; complaints-procedure duty effective 19 June 2026. If the page has updated since 2026-04-25, snapshot the current state with the new retrieval date.
7. Update Milestone Tracker: M2 status `in_progress`, Started date today.
8. Create `crates/sldo-install/tests/e2e_biz_a_m2.rs` with all stubs failing for the right reasons.

#### Files Allowed to Change

| File | Purpose | Scope |
|---|---|---|
| `skills/slo-accounting/SKILL.md` (NEW) | Advisor skill prose: name, description, four-mode contract, hard-block gate citations (referencing M1's triage-gate.md), JPP Law / gov.uk cost references, UK-only jurisdiction error (citing jurisdiction-uk.md), output convention, lawyer-vs-accountant routing | Full new file |
| `references/biz/artifact-schema.md` (NEW) | Frontmatter contract for biz artifacts: `tier` enum (`confidential` \| `public`), `skill`, `mode`, `jurisdiction`, `cost_baseline_ref`, `triage_gate_passed`, `lawyer_review_recommended`, `expires_or_review_by`. Per-artifact-category default tier table. | Full new file |
| `references/biz/jurisdiction-uk.md` (NEW) | UK-only prose anchors (legal + accounting), "v1 UK only" canonical error string, sources + retrieval dates for the UK-specific regulatory anchors that the four advisor skills cite. | Full new file |
| `references/biz/ico-duaa-index.md` (NEW) | DUAA 2025 commencement timeline (Royal Assent 19 June 2025, Stage 3 commenced 5 February 2026, complaints-procedure duty 19 June 2026), new 7th lawful basis, Article 22 narrowing, PECR ceiling £17.5M / 4% global turnover. URLs + retrieval-date stamps. | Full new file |
| `references/biz/ico-enforcement-reality.md` (NEW) | **Descriptive** documentation of the PECR-vs-Article-13 enforcement pattern for sub-£1M-turnover private companies, Apr 2024 – Apr 2026; cites BDO, URM Consulting, Lewis Silkin, Lexology, Mayer Brown sources. **Explicit non-normative disclaimer**: this doc explains why the broad GDPR hard-block (locked 2026-04-25) is conservative versus the enforcement evidence; it does NOT authorize relaxing the gate. | Full new file |
| `references/biz/open-template-anchors.md` (NEW) | oneNDA (CC BY-ND 4.0, verbatim render rule) + oneSaaS (M3+ candidate for T&Cs) + oneDPA (out of scope due to broad GDPR block) + Kindrik Partners / Simmonds Stewart NZ-law (structural reference, E&W rewrite required for non-NDA templates). | Full new file |
| `crates/sldo-install/tests/e2e_biz_a_m2.rs` (NEW) | Structural-contract tests: `/slo-accounting` SKILL.md frontmatter + four-mode keywords + four-predicate-id citations; cross-skill citation test (M1 + M2 advisor SKILL.mds both cite all four); triage-gate.md predicate-id set unchanged from M1; artifact-schema.md tier-enum constraint; jurisdiction-uk.md error-string presence; ico-duaa-index.md DUAA dates; ico-enforcement-reality.md non-normative disclaimer; open-template-anchors.md oneNDA license rule. | Full new file |
| `docs/slo/verify/biz-a-m2-smoke.md` (NEW) | Manual smoke-test checklist: invoke `/slo-accounting draft brief-the-accountant` against an R&D-claim fixture; verify output landed at `docs/biz/accounting/...` with frontmatter; invoke `/slo-accounting triage "registering for VAT"` and verify it routes to accountant; invoke `/slo-accounting draft tax-return` (or any HMRC-filing surface) and verify it routes to accountant via gate-1 (regulated); invoke `/slo-accounting --jurisdiction us` (or any non-UK arg) and verify the v1 UK-only error fires from jurisdiction-uk.md. | Full new file |
| `docs/ARCHITECTURE.md` | One-line addition: `/slo-accounting` skill verb + the five new reference files. Only after M2 ships. | Single-section additive edit |
| This runbook | Milestone Tracker + Evidence Log row updates | Two row updates |

#### Step-by-Step

1. **Pre-flight** (Pre-Flight section above) — baselines green, M1 lessons read, ICO DUAA URL retrieved.
2. **Write structural-contract tests first.** Create `crates/sldo-install/tests/e2e_biz_a_m2.rs` with every test failing for the expected reason. Run `cargo test -p sldo-install --test e2e_biz_a_m2` — confirm all-fail with the right reasons.
3. **Author `references/biz/artifact-schema.md`.** Document the frontmatter schema as a table; per-category default-tier mapping (e.g., `legal/draft/*` → `confidential`; `legal/translate/*` → `public`; `legal/triage/*` → `public`; `accounting/draft/*` → `confidential`; etc.). Include the canonical `tier` enum (`confidential` | `public`) and forbid free-form values.
4. **Author `references/biz/jurisdiction-uk.md`.** Document the UK-only stance, the canonical error string ("v1 supports UK only; US/EU is a v2 architectural pivot — see [link]"), and the regulatory-anchor index (gov.uk, ico.org.uk, legislation.gov.uk roots) so all four advisor skills cite from one place.
5. **Author `references/biz/ico-duaa-index.md`.** Snapshot DUAA 2025 dates, lawful-basis examples, Article 22 narrowing, PECR ceiling — every fact carries a source URL + retrieval-date stamp.
6. **Author `references/biz/ico-enforcement-reality.md`.** Document the sub-£1M-turnover private-company enforcement pattern (PECR direct-marketing-dominated; Article 13 enforcement effectively zero in this segment). **Lead with the non-normative disclaimer**: "this document is descriptive provenance for the locked broad GDPR hard-block; it is NOT authorization to relax gate-4. Reversal of the broad-block decision requires a fresh `/slo-architect` pass with new evidence."
7. **Author `references/biz/open-template-anchors.md`.** Document oneNDA + oneSaaS + oneDPA + Kindrik anchors with license obligations (CC BY-ND 4.0 verbatim for oneNDA; oneSaaS license to be confirmed at M3 if used).
8. **Author `skills/slo-accounting/SKILL.md`.** Frontmatter (`name: slo-accounting`, `description: <advisor skill summary>`). Body follows the M1 `slo-legal` SKILL.md structure: intro, four-mode contract, hard-block gate citations (cite all four predicate IDs from `references/biz/triage-gate.md`), output convention (cite `artifact-schema.md`), JPP Law cost references where accounting work has fixed-fee equivalents, gov.uk references for HMRC-required filings, UK-only jurisdiction error (cite `jurisdiction-uk.md`), no-WebFetch reminder, lawyer-vs-accountant routing prose (gate firings route to accountant unless gate-3 with counterparty-has-lawyer fires, in which case route to lawyer + accountant). End with handoff suggestion.
9. **Author `docs/slo/verify/biz-a-m2-smoke.md`.** Empty checklist with four manual smoke-test items per the description in the Files table above.
10. **Re-run structural-contract tests** — `cargo test -p sldo-install --test e2e_biz_a_m2` — all green. Re-run M1 — `cargo test -p sldo-install --test e2e_biz_a_m1` — must also remain all green (M1 untouched).
11. **Run installer dry-run** — `./target/release/sldo-install --dry-run` — confirm 16 skills (14 pre-runbook + `slo-legal` + `slo-accounting`) and no `references/biz/` entry.
12. **Manual smoke test** — invoke `/slo-accounting` against the four fixtures from step 9 in a real Claude Code session; tick checkboxes in `docs/slo/verify/biz-a-m2-smoke.md`; record any deviations as Evidence Log notes.

#### BDD Acceptance Scenarios

| Scenario | Category | Given | When | Then | Threat-model row | Control |
|---|---|---|---|---|---|---|
| `slo_accounting_skill_md_has_required_frontmatter` | happy path | a freshly-authored `skills/slo-accounting/SKILL.md` | the test parses YAML frontmatter | the `name` field equals `slo-accounting` and `description` is non-empty | n/a | regex parse |
| `slo_accounting_skill_md_documents_four_modes` | happy path | the `skills/slo-accounting/SKILL.md` body | the test searches for the four mode keywords | each of `draft`, `translate`, `triage`, `prepare` appears in a mode-defining heading or table | n/a | regex search |
| `slo_accounting_skill_md_cites_all_four_predicate_ids` | happy path | the `skills/slo-accounting/SKILL.md` body | the test searches for predicate-id mentions | each of the four predicate IDs appears at least once anchored to a hard-block discussion | n/a | regex search |
| `cross_skill_advisor_pattern_replicated` | abuse case (drift) | both `skills/slo-legal/SKILL.md` and `skills/slo-accounting/SKILL.md` exist | the test enumerates advisor SKILL.mds (M1 + M2) and counts predicate-id citations per file | every advisor SKILL.md cites all four predicate IDs (set equality, no missing or extra) | tied to `tm-biz-abuse-5` (gate predicate addition / drift) | regex over each SKILL.md body |
| `triage_gate_predicate_set_unchanged_from_m1` | abuse case (immutability) | `references/biz/triage-gate.md` at M1's commit hash and at HEAD | the test computes a fingerprint over the predicate-id table | the predicate-id set is `{gate-1-regulated, gate-2-deal-value-over-5k, gate-3-counterparty-has-lawyer-or-their-paper, gate-4-gdpr-document}` exactly; no additions, no removals, no renames; six required columns per row | `tm-biz-abuse-5` | regex parse of the table; assert exact predicate-id set |
| `artifact_schema_tier_value_constrained_to_enum` | abuse case (operational) | `references/biz/artifact-schema.md` | the test parses the `tier` enum specification | the schema declares exactly two values: `confidential` and `public`; no free-form, no additions; the per-artifact-category mapping table cites only these two values | tied to `tm-biz-abuse-1` indirectly (default-confidential discipline) | regex parse |
| `jurisdiction_uk_md_has_canonical_error_string` | happy path | `references/biz/jurisdiction-uk.md` | the test searches for the canonical error string | the string `v1 supports UK only` appears at least once and is followed by a "v2 architectural pivot" reference | n/a | regex search |
| `ico_duaa_index_carries_2026_dates` | happy path | `references/biz/ico-duaa-index.md` | the test parses the DUAA timeline | the dates `2025-06-19` (Royal Assent), `2026-02-05` (Stage 3 commencement), `2026-06-19` (complaints-procedure duty) all appear with source URLs | n/a | regex search |
| `ico_enforcement_reality_doc_does_not_contradict_gate_4` | abuse case (descriptive-not-normative) | `references/biz/ico-enforcement-reality.md` | the test searches for the non-normative disclaimer | a disclaimer asserting the doc is descriptive provenance, not authorization to relax gate-4, appears in the first 30 lines; the doc does NOT contain prose like "narrow gate-4 to PECR direct-marketing only" or "draft mode is safe for privacy notices" | tied to `tm-biz-abuse-3` (GDPR-block circumvention) | regex search for the disclaimer + regex assert-absence for forbidden phrases |
| `open_template_anchors_documents_onenda_license` | happy path | `references/biz/open-template-anchors.md` | the test parses the oneNDA section | the section explicitly cites CC BY-ND 4.0 and the verbatim-render rule, plus a pointer to `references/biz/templates/onenda-uk.md` | tied to `tm-biz-abuse-7` indirectly (license-violation defense) | regex search |
| `references_biz_dir_still_not_discovered_as_skill` | abuse case (regression) | `references/biz/` now contains seven files (M1's three + M2's five — note: `templates/` is a subdirectory) | the test invokes `sldo-install --dry-run` | `references/biz/` does NOT appear in the discovered-skill list; total skill count is 16 (14 pre-runbook + `slo-legal` + `slo-accounting`) | regression on `crates/sldo-install/src/install.rs:44-71` invariant | `assert_cmd::Command` invocation; parse stdout |

#### Regression Tests

The following pre-existing tests must still pass after M2:

- `cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install` (full baseline)
- `cargo test -p sldo-install --test e2e_biz_a_m1` (all 10 M1 tests)
- `cargo test -p sldo-install --test e2e_slo_sec_m1` through `e2e_slo_sec_m4` (slo-security-embedding regression)

#### Compatibility Checklist

- [ ] All 14 pre-runbook skills still load via `sldo-install --dry-run`.
- [ ] `/slo-legal` (M1) still loads.
- [ ] `references/biz/triage-gate.md` is byte-identical to its M1 commit (or the predicate-id table textually unchanged).
- [ ] `references/biz/cost-baseline-jpp-law-2026.md` is unchanged from M1.
- [ ] `references/biz/templates/onenda-uk.md` SHA-256 matches the pinned hash in `e2e_biz_a_m1.rs`.
- [ ] All existing runbooks parse against `docs/slo/templates/runbook-template_v_3_template.md`.
- [ ] `docs/slo/templates/runbook-template_v_3_template.md` unchanged.
- [ ] `crates/sldo-install/src/install.rs` unchanged.
- [ ] `crates/sldo-tauri/` unchanged.
- [ ] `SECURITY.md` (root) — unchanged in M2 (the count update from 12 → 15 already landed alongside this M2 draft as part of the design propagation).

#### E2E Runtime Validation

- Test functions in `crates/sldo-install/tests/e2e_biz_a_m2.rs`:
  - `slo_accounting_skill_md_has_required_frontmatter`
  - `slo_accounting_skill_md_documents_four_modes`
  - `slo_accounting_skill_md_cites_all_four_predicate_ids`
  - `cross_skill_advisor_pattern_replicated`
  - `triage_gate_predicate_set_unchanged_from_m1`
  - `artifact_schema_tier_value_constrained_to_enum`
  - `jurisdiction_uk_md_has_canonical_error_string`
  - `ico_duaa_index_carries_2026_dates`
  - `ico_enforcement_reality_doc_does_not_contradict_gate_4`
  - `open_template_anchors_documents_onenda_license`
  - `references_biz_dir_still_not_discovered_as_skill`
- Pass criteria: all 11 green; M1's 10 tests still green; baseline test command still green; `sldo-install --dry-run` shows 16 skills and no `references/biz/` entry.

#### Smoke Tests

Documented in `docs/slo/verify/biz-a-m2-smoke.md`. Manual invocation in a real Claude Code session against four fixtures:

1. **Happy path — `/slo-accounting draft brief-the-accountant`** for an R&D tax credit claim (fixture: 6 months of FTE eng spend, plausible R&D narrative, claim-period dates). Verify: artifact lands at `docs/biz/accounting/brief-the-accountant-rd-claim-2026-04-25.md` (gitignored); frontmatter matches `artifact-schema.md` (`tier: confidential`, `skill: slo-accounting`, `mode: draft`, `triage_gate_passed: true`, `lawyer_review_recommended: true`); body is a brief structured for the accountant call (numbers + qualifying-activity argument + open questions).
2. **Routing — `/slo-accounting triage "registering for VAT"`**. Verify: routes to accountant (not lawyer); output lands at `docs/biz-public/accounting/triage-vat-registration-2026-04-25.md`; cites the £85k VAT-registration threshold (gov.uk, retrieval-date stamped).
3. **Hard-block — `/slo-accounting draft tax-return`** (or any HMRC-filing-shaped surface). Verify: skill refuses to draft; routes to triage citing `gate-1-regulated` (HMRC is a regulator); output recommends accountant with a brief of what to bring.
4. **Non-UK — `/slo-accounting --jurisdiction us`** or any non-UK arg surface. Verify: emits the canonical error from `jurisdiction-uk.md`: "v1 supports UK only; US/EU is a v2 architectural pivot — see [link]". No artifact written.

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install` | green | | | |
| M1 regression | `cargo test -p sldo-install --test e2e_biz_a_m1` | 10/10 green | | | |
| Structural-contract tests created | `crates/sldo-install/tests/e2e_biz_a_m2.rs` | compiles, 11 tests fail for expected reasons | | | |
| Smoke-test checklist created | `docs/slo/verify/biz-a-m2-smoke.md` | exists with 4 unchecked checkboxes | | | |
| `artifact-schema.md` authored | `references/biz/artifact-schema.md` | tier enum + per-category mapping table | | | |
| `jurisdiction-uk.md` authored | `references/biz/jurisdiction-uk.md` | canonical error string + UK regulatory-anchor index | | | |
| `ico-duaa-index.md` authored | `references/biz/ico-duaa-index.md` | three DUAA dates with source URLs + retrieval-date stamps | | | |
| `ico-enforcement-reality.md` authored | `references/biz/ico-enforcement-reality.md` | non-normative disclaimer first; PECR-vs-Article-13 pattern with sources | | | |
| `open-template-anchors.md` authored | `references/biz/open-template-anchors.md` | oneNDA CC BY-ND 4.0 verbatim rule documented | | | |
| `slo-accounting/SKILL.md` authored | `skills/slo-accounting/SKILL.md` | frontmatter + four-mode contract + four-predicate citations + jurisdiction-uk citation + accountant-vs-lawyer routing | | | |
| Full tests | baseline command | green | | | |
| Milestone E2E test | `cargo test -p sldo-install --test e2e_biz_a_m2` | 11/11 green | | | |
| M1 regression after M2 | `cargo test -p sldo-install --test e2e_biz_a_m1` | 10/10 green (still) | | | |
| Build/boot | `cargo build -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install` | builds cleanly | | | |
| Installer dry-run | `./target/release/sldo-install --dry-run` | 16 skills discovered; no `references/biz/` entry | | | |
| Smoke tests | manual fixture invocations 1–4 from `biz-a-m2-smoke.md` | all checked | | | |
| Test artifact cleanup | `git status` | no untracked test artifacts | | | |
| .gitignore review | review `.gitignore` | unchanged | | | |
| Compatibility checks | M1 outputs unchanged; predicate IDs unchanged; existing runbooks parse | no regressions | | | |

#### Definition of Done

- [ ] Six new files created: `skills/slo-accounting/SKILL.md`, `references/biz/artifact-schema.md`, `references/biz/jurisdiction-uk.md`, `references/biz/ico-duaa-index.md`, `references/biz/ico-enforcement-reality.md`, `references/biz/open-template-anchors.md`.
- [ ] Two test/verification files created: `crates/sldo-install/tests/e2e_biz_a_m2.rs`, `docs/slo/verify/biz-a-m2-smoke.md`.
- [ ] `cargo test -p sldo-install --test e2e_biz_a_m2` returns 11/11 green.
- [ ] `cargo test -p sldo-install --test e2e_biz_a_m1` returns 10/10 green (regression preserved).
- [ ] `cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install` (full baseline) green.
- [ ] `sldo-install --dry-run` shows 16 skills (14 pre-runbook + `slo-legal` + `slo-accounting`); no `references/biz/` entry.
- [ ] All four manual smoke-test scenarios in `docs/slo/verify/biz-a-m2-smoke.md` are ticked.
- [ ] M1 outputs (`/slo-legal`, `triage-gate.md`, `cost-baseline-jpp-law-2026.md`, `templates/onenda-uk.md`) byte-identical to M1's commit (or text-identical for files where formatting tools may have touched whitespace — check via `git diff <m1-commit>..HEAD -- <files>`).
- [ ] Cross-skill citation test green: every advisor SKILL.md (`/slo-legal` and `/slo-accounting`) cites all four predicate IDs without inlining the predicate bodies.
- [ ] Compatibility Checklist all checked.
- [ ] Self-Review Gate all "yes".
- [ ] `docs/ARCHITECTURE.md` updated with one-line addition naming `/slo-accounting` + the five new reference files.
- [ ] `docs/slo/lessons/biz-a-m2.md` written, including explicit note on the deferred `/slo-verify` PII-pattern scan integration to Runbook B1 M1.
- [ ] `docs/slo/completion/biz-a-m2.md` written.
- [ ] Milestone Tracker: M2 status `done`, Completed date recorded.

---

### Milestone 3 — `/slo-equity` + `references/biz/hmrc-vcm-index.md`

**Goal**: Ship `/slo-equity` (third advisor skill) and the HMRC Venture Capital Schemes Manual index. v1 surfaces: cofounder split rationale, 4yr/1yr-cliff vesting schedule, EMI option triage, dilution math, cap-table snapshot.

**Pattern**: M3 follows M2's structure exactly — replicates the advisor pattern, cites the four predicate IDs from M1 without modification. New shared reference `references/biz/hmrc-vcm-index.md` carries VCM34080 (control / disqualifying arrangements), VCM3000 (excluded activities), VCM31000 (SEIS income tax relief) with retrieval-date stamps. The cross-skill citation test in `e2e_biz_a_m3.rs` extends to assert all three advisor SKILL.mds (`slo-legal`, `slo-accounting`, `slo-equity`) cite all four predicates.

**Files (NEW)**: `skills/slo-equity/SKILL.md`, `references/biz/hmrc-vcm-index.md`, `crates/sldo-install/tests/e2e_biz_a_m3.rs`, `docs/slo/verify/biz-a-m3-smoke.md`.

**Compatibility**: M1 + M2 outputs unchanged (all reference files, all SKILL.mds, all tests).

**Contract Block**: data classification `Confidential` (cofounder splits + cap tables contain founder PII); proactive controls per OWASP v3 (C1, C5, C8, C10) following M1/M2 pattern; abuse cases follow M2 cross-skill drift + immutability tests, plus a new HMRC-VCM-citation accuracy test (every cited paragraph in the index file has a source URL + retrieval date).

**Smoke fixtures**: (1) cofounder split for two founders contributing differently; (2) "should I file SEIS Advance Assurance now?" triage; (3) employee EMI option triage; (4) non-UK jurisdiction → canonical UK-only error.

**Blockers**: requires M2's `references/biz/jurisdiction-uk.md` (cited for the UK-only error path) and M2's `references/biz/artifact-schema.md` (cited for frontmatter contract). M3 will not start until M2 ships.

---

### Milestone 4 — `/slo-fundraise` + `references/biz/ir35-cest-factors.md`

**Goal**: Ship `/slo-fundraise` (fourth and final advisor skill in Runbook A) and the IR35 / CEST factors reference. v1 surfaces: SAFE / cap-and-discount math, SEIS/EIS Advance Assurance triage gate (cite VCM34080 / VCM3000 / VCM31000 from M3's index), pitch narrative composer, term-sheet redline preparation. The IR35 factors reference is shared across `/slo-legal triage` (contractor-vs-employee), `/slo-fundraise` (SEIS qualifying-employee context), and future Runbook C M2 `/slo-hire`.

**Pattern**: M4 follows M2/M3's structure exactly. Cross-skill citation test in `e2e_biz_a_m4.rs` extends to assert all FOUR advisor SKILL.mds cite all four predicates — final replication proof for the advisor pattern.

**Files (NEW)**: `skills/slo-fundraise/SKILL.md`, `references/biz/ir35-cest-factors.md`, `crates/sldo-install/tests/e2e_biz_a_m4.rs`, `docs/slo/verify/biz-a-m4-smoke.md`.

**Single CLAUDE.md edit (M4 only)**: Add a "Biz skill pack — first-party `/slo-*` skills" section to CLAUDE.md cataloging all four advisor skills with their mode contracts and the `references/biz/` location. This is the only CLAUDE.md edit in Runbook A — bundled at M4 because that's when all four advisor skills exist to be cataloged together.

**Contract Block**: data classification `Confidential` (SAFE math + cap-table snapshots + investor lists are highly confidential); proactive controls per M1/M2/M3 pattern; abuse cases include a SEIS/EIS Advance Assurance triage timing test (Advance Assurance must be flagged as ≥6-week lead-time before any term-sheet drafting per the dossier's Q3 finding); IR35 factors test (substitution / MOO / control three-factor structure asserted in `e2e_biz_a_m4.rs`).

**Smoke fixtures**: (1) SAFE drafting for £200k seed at £2M cap → routes to triage via gate-2 (>£5k); (2) "have you applied for Advance Assurance?" triage as the first question of any fundraise interaction; (3) IR35 contractor-vs-employee determination on a marginal fact pattern; (4) non-UK jurisdiction → canonical UK-only error.

**Blockers**: requires M2's shared scaffolding (artifact-schema, jurisdiction-uk, ico-duaa-index for the EIS / ICO interaction edge case where some funded companies process personal data) and M3's `references/biz/hmrc-vcm-index.md` (cited as the SEIS/EIS authority anchor). M4 will not start until M3 ships.

---

> **Status**: M1 `done` (2026-04-25). M2 drafted and ready for execution. M3 + M4 compact stubs above.
> Per `/slo-plan`'s discipline, M2 will be executed before M3 + M4 are expanded to full Contract-Block detail. The compact stubs are sufficient scope-bounds for `/slo-critique` to review the runbook end-to-end before further milestone work.
