# Interfaces — secure-value-loop

Every entry below is a contract downstream milestones must not rename or reshape
without explicit migration work. Stability levels: `stable` (frozen),
`evolving` (may change with migration), `internal` (fair game).

## 1. New optional v4-template section: "Secure Value & Security Contract"

Inserted **after** §5A Measurement Contract and **before** §6 Global Execution
Rules (matching the proposal's placement directive). Optional/additive — legacy
runbooks without it remain valid; `/slo-plan` requires it only for value-bearing
*or security-relevant* milestones.

Sub-blocks (each a named Markdown heading the structural test asserts):

| Sub-block | Required fields | Stability |
|---|---|---|
| Value Wedge | Value hypothesis; Smallest valuable wedge; User-visible proof of value; Security-visible proof of safety; "What would make this wedge too small to matter?" | `stable` |
| Security Definition of Ready (Operator Readiness) | Prerequisite; Owner (`human \| agent \| upstream`); Needed-by (M-N); Validation; Status (`ready \| partially_ready \| blocked`) + `safe_to_continue_without_blockers: true\|false` | `stable` |
| Threat Model Summary | Assets; Actors; Trust boundaries; Entry points; Abuse cases (cite `tm-<slug>-abuse-N`); Required controls; Residual risks — **populated from the existing `/slo-architect` threat model, not re-derived** | `stable` |
| Security Test Plan | one row per Bundle A–F test type: Required?; Command/tool; Evidence path; Waiver-if-not-applicable | `stable` |
| Detected Work Ledger | ID; Finding; Severity; Disposition; Owner; Evidence/link; Due | `stable` |

## 2. Milestone-status vocabulary (ADDITIVE — the highest-risk interface)

Current (frozen, must keep parsing): `not_started | in_progress | blocked | done`
(v4 template line 62 comment; read by `/slo-resume`, `/slo-execute` Step 1.5,
Milestone Tracker).

New additive values (`evolving` → becomes `stable` once shipped):

| Value | Meaning | Maps to legacy fallback for old parsers |
|---|---|---|
| `human_review_required` | run complete but a human must review before close | treated as `in_progress`/`blocked` semantics by legacy reads |
| `blocked_by_operator` | stalled on an operator prerequisite (account/cred/approval) | `blocked` |
| `blocked_by_upstream` | stalled on an upstream dependency issue/PR | `blocked` |
| `issue_filed` | work captured as a filed issue, milestone intentionally not completing it | `blocked`/`done`-adjacent; surfaced, not silent |
| `accepted_risk` | closed with a recorded residual-risk decision (owner + expiry) | `done` with annotation |

**Contract:** the template status comment lists all values; `/slo-resume` and
`/slo-execute` recognise the new ones explicitly; **any parser that does not
recognise a value treats it as `blocked` (safe default — never silently `done`).**
The structural test asserts the old four are still present in the comment.

**Deterministic consumer (F-ENG-1, resolved in M3):** there is a *published*
parser — `crates/sldo-common/src/runbook.rs` (`MilestoneStatus` enum,
`parse_tracker`, `all_done`, `next_incomplete`; `sldo-common` is on crates.io
v0.1.2). Today it knows only `{NotStarted, InProgress, Done}` (not even
`blocked`), its regex silently drops non-matching rows, and unknown→`NotStarted`
— so `all_done()` can report a runbook complete while a `blocked` row is
unfinished. **M3 extends this enum to be total over the documented set,
unknown→`Blocked`, with round-trip unit tests and a crates.io semver bump
(0.1.2→0.1.3).** This is the one place the additive-enum change touches Rust, not
just Markdown. Stability of the `sldo-common::runbook` public API: `evolving`
(extended additively, never reshaped).

## 3. Detected Work Ledger disposition vocabulary (RECONCILED — no third taxonomy)

The five ledger dispositions map onto existing `/slo-retro` + carry-forward lanes.
This mapping is the load-bearing anti-duplication contract:

| Ledger disposition (proposal §6) | Existing SLO lane it routes through | Where handled |
|---|---|---|
| `fix_now` | carry-forward `micro` | inside the current milestone (`/slo-execute`) |
| `file_github_issue` | `/slo-retro` lane `product` or `slo-process` + carry-forward `milestone`/`fresh-runbook` | `/slo-retro` issue-filing flow |
| `upstream_feedback` | `/slo-retro` lane `upstream-OSS` | `/slo-retro` issue-filing flow (existing per-session cap applies) |
| `operator_action` | Operator Readiness Gate (§Security Definition of Ready) | `/slo-plan` prereq row + `/slo-execute` Global Entry; status `blocked_by_operator` |
| `accepted_risk` | threat-model Residual-risks table convention (owner + `review_by`) | recorded in milestone; status `accepted_risk` |

Stability: `stable`. **No new lane verbs are introduced.** `/slo-retro`'s filing
discipline (dedupe, `~~~text` fence, per-session cap) is reused verbatim.

## 4. Security Test Bundles A–F (selection inputs, not a runner)

A referenceable table in `docs/SECURE-VALUE-LOOP.md` + the v4 Security Test Plan.
Resolved by `/slo-verify` Pass 4/5 surface detection — **not** a new test engine.

| Bundle | Trigger surface | Cites | Resolved by |
|---|---|---|---|
| A | docs/planning only | — | security assessment + secrets scan |
| B | application code | OWASP ASVS 5.0 | Pass 4 SAST/SCA/secrets + authz/abuse |
| C | backend/API | OWASP API Security Top 10 (2023) | Pass 4 + `/slo-dast-tuner` |
| D | cloud/IaC/K8s | CIS/CSA + Hulumi CrossGuard | `/slo-cloud-threat-model` + IaC scan |
| E | AI/LLM/agent | OWASP LLM Top 10 (2025), MITRE ATLAS, NIST AI RMF | Pass 5 (gated on `ai_component`) |
| F | mobile/native/client | OWASP MASVS | Pass 4 + platform checks |

Stability: `evolving` (bundle contents track the cited standards' editions).

## 5. `ship_state` vocabulary (NEW, `/slo-ship`)

`shipped | human_review_required | blocked | canary_only | docs_only`. Closed
enum, `stable`. With `reason`, `rollback`, `monitoring_links`. SBOM/provenance
rows are conditional (`not_applicable` for non-release-artifact milestones).

## 6. Proactive-controls citation format (TIGHTENED)

The existing Contract Block row "Proactive controls in play (optional)" changes
from "e.g., C1, C5, C9" to **"OWASP Proactive Controls 2024 by name, e.g.,
`C1 Implement Access Control`, `C4 Address Security from the Start`"**. Stability:
`stable`. This is a wording tightening, not a structural change — backward
compatible (old runbooks citing bare numbers still parse).

## 7. New canonical doc

`docs/SECURE-VALUE-LOOP.md` — referenced by LOOPS-ENGINEERING.md and the v4
template. Stability: `stable` path.

## Interfaces explicitly NOT changed (preserve)

- `/slo-product` / `/slo-metrics` verbs + output paths.
- `/slo-architect` Step 3.5 threat-model schema (`slo_schema_version: 0.1.0`,
  frozen `tm-<slug>-abuse-N` IDs) — the Secure Value Contract *cites* it.
- `/slo-verify` Pass 4/5 internals — bundles are *inputs* to existing detection.
- `/slo-retro` lane verbs + filing discipline — reused, not extended.
- The §5A Measurement Contract and §10 Carry-forward sections — untouched; the
  new section sits between §5A and §6 without renumbering them.
- Repo-root `SECURITY.md` and any root `ARCHITECTURE.md`.
