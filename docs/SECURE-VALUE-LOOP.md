# SunLit Orchestra — Secure Value Loop

**Status:** Canonical operating-model overlay
**Audience:** SLO agents, runbook authors, reviewers, human operators
**Purpose:** Wrap the existing SLO sprint loop in a security/value envelope so every value-bearing runbook carries three typed, unavoidable disciplines — an Operator Readiness Gate, a Detected Work Ledger, and honest exit states — while **reusing** the security machinery the pack already ships.

This document is the single source of truth for the envelope. The v4 runbook template's [§5B Secure Value and Security Contract](slo/templates/runbook-template_v_4_template.md) section is the per-runbook surface; this document is the prose authority it points at.

---

## 1. The operating rule

> Do not optimize for the smallest wedge. Optimize for the **smallest valuable, secure, testable, unblocked, reviewable** outcome.

Each major SLO stage carries a cybersecurity output:

| SLO stage | Security responsibility | Where it lives today |
|---|---|---|
| Idea | High-level risks, data-classification guess, trust-boundary hints | `/slo-ideate` |
| Research | Security source pack (standards, scopes, threat intel, prereqs) | `/slo-research` |
| Architect | Threat model (STRIDE, abuse cases, mitigations) | `/slo-architect` Step 3.5 (**shipped**) |
| Critique | Security assessment (class-elimination, variant analysis) | `/slo-critique` security persona (**shipped**) |
| Plan | Secure Value & Security Contract (§5B) | `/slo-plan` + v4 template |
| Execute | Proactive controls, Operator Readiness, Detected Work Ledger | `/slo-execute` |
| Verify | Security test matrix (Bundles A–F) as first-class evidence | `/slo-verify` Pass 4/5 (**shipped**) |
| Ship | Secure-release checklist, `ship_state`, residual-risk sign-off | `/slo-ship` |
| Retro | Disposition of every finding; upstream + reusable-rule loop | `/slo-retro` (**shipped lanes**) |

The envelope adds discipline, not capability. ~80% of the security machinery already ships (threat model, class-elimination critique, the verify matrix, nine security skills, lane-classified retro filing). This doc institutionalises the three genuinely-missing disciplines below.

---

## 2. External authority

The contract is grounded against recognised standards, **cited by name + edition** so a future renumber cannot silently change meaning:

- **NIST SSDF (SP 800-218 v1.1)** — the PO/PS/PW/RV outcome groups validate "every stage carries a security output"; **SP 800-218A** covers the AI/LLM lane.
- **OWASP Proactive Controls 2024** — cite controls by name (`C1 Implement Access Control`, `C4 Address Security from the Start`, `C10 Stop Server-Side Request Forgery`, …), **never by bare number** — OWASP renumbered C1–C10 between the 2018 and 2024 editions.
- **OWASP ASVS 5.0 / MASVS / API Security Top 10 (2023) / LLM Top 10 (2025)** — the verification standards the test bundles cite.
- **SLSA + SBOM (CycloneDX / SPDX)** — release-artifact provenance; conditional (see §6).

---

## 3. Operator Readiness Gate

Before a milestone runs, the agent must tell the human what action is required, so execution does not stall mid-run on a missing account, credential, or approval.

Each value-bearing/security-relevant milestone declares its prerequisites in §5B's **Security Definition of Ready**:

| Field | Meaning |
|---|---|
| Prerequisite | Cloud account / OAuth app / API key / test device / DNS / cert / approval |
| Owner | `human \| agent \| upstream` |
| Needed by | The milestone (M-N) it gates |
| Validation | An **executable proof** it is ready (e.g. "callback smoke passes"), not a self-asserted checkbox |
| Status | `ready \| partially_ready \| blocked` + `safe_to_continue_without_blockers: true \| false` |

**Fail-closed rule:** if `safe_to_continue_without_blockers: false`, `/slo-execute` MUST NOT start the milestone. If `true`, the runbook must name the degraded path and what is deferred. The milestone status becomes `blocked_by_operator` rather than a silent `in_progress`.

---

## 4. Detected Work Ledger

Every finding discovered mid-execution gets **exactly one disposition** and may never end as merely "observed". The ledger is a §5B table:

| ID | Finding | Severity | Disposition | Owner | Evidence/link | Due |
|---|---|---:|---|---|---|---|

### Dispositions reuse existing `/slo-retro` lanes — no third taxonomy

The five dispositions are a thin **routing** vocabulary. They introduce **no new `/slo-retro` lane verb**:

| Ledger disposition | Routes to (existing mechanism) |
|---|---|
| `fix_now` | carry-forward `micro` — fixed inside the current milestone (safe/local/in-allow-list only) |
| `file_github_issue` | `/slo-retro` lane `product` or `slo-process` (+ carry-forward `milestone`/`fresh-runbook`) |
| `upstream_feedback` | `/slo-retro` lane `upstream-OSS` (existing dedupe + per-session cap apply) |
| `operator_action` | the Operator Readiness Gate (§3); status `blocked_by_operator` |
| `accepted_risk` | the threat-model Residual-risks convention (owner + `review_by`); status `accepted_risk` |

`/slo-execute` refuses to mark a milestone `done` while any ledger row is undisposed. `/slo-retro` re-reads the ledger and files `file_github_issue`/`upstream_feedback` rows through its existing filing discipline (it does not re-implement filing or bypass the cap).

---

## 5. Honest exit states

The milestone-status vocabulary is **extended additively** — the existing `not_started | in_progress | blocked | done` keep working, plus:

- `human_review_required` — run complete, a human must review before close
- `blocked_by_operator` — stalled on an operator prerequisite (§3)
- `blocked_by_upstream` — stalled on an upstream dependency issue/PR
- `issue_filed` — work captured as a filed issue, intentionally not completed here
- `accepted_risk` — closed with a recorded residual-risk decision (owner + expiry)

**Fail-safe rule:** any consumer that does not recognise a status value treats it as **`blocked`** — never silently `done` (and never `not_started`). This is enforced in skill prose **and** in the published `sldo-common::runbook::MilestoneStatus` parser (from the M3 release onward), so `all_done()` can never report a runbook complete while an unknown/blocked row is unfinished.

---

## 6. Security test bundles (A–F)

Bundles are **selection inputs** to `/slo-verify`'s existing Pass 4/5 surface detection — not a new test runner. §5B's Security Test Plan references the bundle(s) a milestone's surface triggers; each test row resolves to `pass | not_applicable | waived_with_reason` (never blank).

| Bundle | Trigger surface | Cites | Resolved by |
|---|---|---|---|
| Bundle A | docs/planning only | — | security assessment + secrets scan |
| Bundle B | application code | OWASP ASVS 5.0 | Pass 4 SAST/SCA/secrets + authz/abuse |
| Bundle C | backend/API | OWASP API Security Top 10 (2023) | Pass 4 + `/slo-dast-tuner` |
| Bundle D | cloud/IaC/K8s | CIS/CSA + Hulumi CrossGuard | `/slo-cloud-threat-model` + IaC scan |
| Bundle E | AI/LLM/agent | OWASP LLM Top 10 (2025), MITRE ATLAS, NIST AI RMF | Pass 5 (gated on `ai_component`) |
| Bundle F | mobile/native/client | OWASP MASVS | Pass 4 + platform checks |

**SBOM/provenance is conditional.** SLSA + SBOM (CycloneDX/SPDX) apply only to milestones that build a **released artifact** (e.g. a crates.io publish or a release zip). For the common markdown/skill-contract milestone the Ship checklist resolves SBOM/provenance to `not_applicable` — it is never a hard gate for non-release work.

---

## 7. The `~~~text` fence rule (injection defense)

§5B fields are author-written runbook prose, so the injection surface that matters is where a **skill generates an artifact from those strings**. At those surfaces, every user-provided string is wrapped in a `~~~text` fence so Markdown/YAML/HTML metacharacters are literal, never interpreted (the load-bearing `/slo-architect` rule). The concrete generation surfaces this protects are:

- **`/slo-resume`** — quoting carry-forward / Detected-Work-Ledger snippets into its orientation output.
- **`/slo-ship`** — quoting Detected-Work-Ledger rows into a generated PR body.

This is `tm-secure-value-loop-abuse-1` (contract string injection). It is scoped to those generation surfaces, not over-claimed across inert author prose.

---

## 8. Ship state

`/slo-ship` records a closed `ship_state`: `shipped | human_review_required | blocked | canary_only | docs_only`, with `reason`, `rollback`, and `monitoring_links`. The secure-release checklist requires: tests + security scans complete; no critical/high untriaged finding; SBOM/provenance **when applicable**; least-privilege deploy creds; canary/staged rollout; monitoring/alerts for new failure modes; tested/documented rollback; residual risks with named owners + dates.

---

## 9. One-page agent prompt

Use this as the instruction block for agents working under the envelope:

~~~text
You are working under the SunLit Orchestra Secure Value Loop.

Do not optimize for the smallest technical slice. Optimize for the smallest
valuable, secure, testable, unblocked, reviewable outcome.

At each stage, produce the required cybersecurity artefact:
- Idea: high-level risks, data-classification guess, trust-boundary hints.
- Research: security source pack, vendor prereqs, standards, threat intel.
- Design: threat model (assets, actors, boundaries, abuse cases, mitigations).
- Critique: security assessment with blocking / non-blocking findings.
- Plan: §5B Secure Value & Security Contract — Value Wedge, Operator Readiness,
  Security Test Plan (Bundles A–F), Detected Work Ledger.
- Execute: proactive controls (OWASP 2024 by name), SecureLibraries/Hulumi,
  Operator Readiness Gate (fail closed), Detected Work Ledger (every finding
  disposed), small-fix policy.
- Verify: the bundle's SAST/SCA/secrets/IaC/container/DAST/authz/abuse/privacy/
  LLM tests as first-class evidence rows.
- Ship: secure-release checklist, ship_state, SBOM/provenance when applicable,
  monitoring, rollback, residual-risk owner.
- Retro: dispose every ledger row through existing /slo-retro lanes; upstream
  feedback; reusable SLO rules.

Anything discovered must be disposed as exactly one of:
fix_now | file_github_issue | operator_action | upstream_feedback | accepted_risk.
Never leave findings as merely observed.

Milestone status is one of: not_started | in_progress | blocked | done |
human_review_required | blocked_by_operator | blocked_by_upstream | issue_filed |
accepted_risk. An unknown status is treated as blocked, never as done.
~~~

---

## 10. GitHub labels

Two repo labels back the envelope (proposal §9 #9/#10). Create them once per repo (operator action — an external repo mutation, not run automatically):

~~~text
gh label create operator-action-required --description "Milestone blocked on a missing credential, registration, scope, approval, device, DNS, cloud access, or vendor ticket" --color FBCA04
gh label create security-review-required --description "Touches identity, secrets, PII, payment, cloud, AI agents, public/network boundaries, CI/CD, or infrastructure" --color D93F0B
~~~

- **`operator-action-required`** — applied to a milestone/issue in the `blocked_by_operator` state (ties to the Operator Readiness Gate, §3).
- **`security-review-required`** — applied to any milestone/issue whose surface matches the §5B security-relevant trigger list (ties to `/slo-plan`'s requirement and `/slo-retro` filing, M4).

## 11. Adoption criteria

The envelope is adopted when: every new value-bearing runbook has a §5B Value Wedge + Security Contract; every milestone has an Operator Readiness state; every discovered issue has a disposition; every security-relevant milestone has a threat model or a documented reason it is not required; every verification phase records security test results or explicit waivers; every launch has monitoring, rollback, and residual-risk ownership; and every retro can file security issues / upstream feedback through the existing lanes without widening the current runbook silently.
