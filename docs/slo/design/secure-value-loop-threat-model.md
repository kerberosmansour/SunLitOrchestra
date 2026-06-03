---
name: secure-value-loop
generated: 2026-06-02
producer: slo-architect
compliance: [soc2, asvs]
---

# Threat Model — Secure Value Loop

This work ships **no runtime and no code that implements a security boundary** —
it is runbook-contract discipline (Markdown sections, an additive status enum,
skill-prompt edits, structural tests). The threat surface is therefore narrow and
of a kind the SLO pack has modelled before: prompt-injection through generated
artifacts, gate-gaming, and the meta-risk that a *security discipline can be
silently bypassed or give false assurance*. Conservative defaults are used where
the idea doc gave no explicit `## Top risks` block.

## Assets

- The integrity of the **runbook contract** (Operator Readiness rows, Detected
  Work Ledger dispositions, milestone-status values) as a truthful execution
  record.
- The **disposition guarantee** — that no finding ends as merely "observed".
- Author/agent **trust in the gate** — a readiness gate that lies (says ready
  when it isn't) is worse than none.
- User-provided strings flowing into generated artifacts (idea-doc/Top-risks text
  rendered into `docs/SECURE-VALUE-LOOP.md` examples, ledger rows, contract rows).

## Actors

- `legitimate author/agent` — runs the loop honestly.
- `unwary author / content-injector` — pastes crafted Markdown/YAML/HTML into a
  contract field.
- `author gaming the gate` — marks readiness `ready` or a ledger row disposed
  without doing the work, to make a milestone close.
- `legacy-runbook consumer` (`/slo-resume`, `/slo-execute` Step 1.5, Tracker
  parser) — must not be broken by the additive changes.

## Trust boundaries

- author/idea-doc text → generated `docs/SECURE-VALUE-LOOP.md` / contract rows.
- runbook Status cell → enum parsers (`/slo-resume`, `/slo-execute`).
- Detected Work Ledger row → `/slo-retro` issue-filing flow → GitHub.

## Entry points

- New v4-template sections (Value Wedge, Operator Readiness, Ledger, Security Test
  Plan) authored by `/slo-plan`.
- The additive milestone-status enum read by existing parsers.
- The ledger→`/slo-retro` disposition bridge.

## STRIDE sweep (per component)

| Component | Class | State | Control / reason |
|---|---|---|---|
| Generated `docs/SECURE-VALUE-LOOP.md` / contract-row string interpolation | Tampering | eliminated | User-provided strings wrapped in `~~~text` fences (the load-bearing `/slo-architect` rule); rendered as descriptive Markdown only, never selecting status/disposition/control fields |
| Generated artifacts | Elevation of privilege | eliminated | Generated text never chooses `id`/`status`/`disposition`/`classification`; those are author-controlled |
| Operator Readiness Gate | Spoofing | mitigated | `validation` column requires *proof* the prereq is ready (e.g. "callback smoke passes"), not a self-asserted checkbox; `/slo-execute` Global Entry re-reads it |
| Operator Readiness Gate | Elevation of privilege | mitigated | `safe_to_continue_without_blockers:false` makes `/slo-execute` refuse to start — the gate fails closed |
| Detected Work Ledger | Repudiation | mitigated | Every row carries Owner + Evidence/link + Due; `/slo-execute` refuses `done` while a row is undisposed |
| Detected Work Ledger | Information disclosure | mitigated | Ledger lives in the runbook (default git-tracked); secrets-in-evidence prevented by the existing secrets-scan lane + the no-secrets red line |
| Additive milestone-status enum | Tampering | mitigated | Closed enum; unknown value → parser fallback to `blocked` (fail-safe, never silent `done`); structural test asserts old four still parse |
| Ledger ↔ `/slo-retro` disposition bridge | Spoofing | mitigated | Reuses `/slo-retro` filing discipline (dedupe, fence, per-session cap); no new privileged path |
| `/slo-ship` secure-release checklist | Repudiation | mitigated | `ship_state` + residual-risk owner + monitoring links recorded; SBOM/provenance when applicable |
| Structural-contract test (`xtasks/sast-verify`) | Tampering | mitigated | Baseline/SHA changes are explicit and PR-reviewed; never waived |
| Whole envelope | Denial of service | N/A — no runtime, no network, no bounded resource at risk |
| Whole envelope | Information disclosure (meta) | residual | A determined author can still hand-author a thin/dishonest contract — the framework raises the cost of faking, it cannot enforce integrity (see residual risks) |

## Abuse cases

- **`tm-secure-value-loop-abuse-1`** — *Contract string injection.* A content-
  injector pastes Markdown/YAML/HTML metacharacters into a Value Wedge / ledger /
  readiness field to break out of the section or smuggle agent-readable
  instructions into a generated artifact. **Control:** all user strings rendered
  into generated security/threat artifacts are `~~~text`-fenced and never select
  control fields (reuse of the `/slo-architect` fence rule + the `/slo-resume`
  carry-forward fence rule).
- **`tm-secure-value-loop-abuse-2`** — *Readiness gate bypass.* An author marks
  Operator Readiness `ready` (or `safe_to_continue_without_blockers:true`) without
  the prerequisite actually being provisioned, so `/slo-execute` starts and stalls
  mid-run or proceeds on a degraded/mocked path. **Control:** the `validation`
  column requires an executable proof of readiness; `/slo-execute` Global Entry
  re-reads and, when degraded, must name the deferred path explicitly (proposal
  §5); status falls to `blocked_by_operator` rather than silent `in_progress`.
- **`tm-secure-value-loop-abuse-3`** — *Disposition laundering.* An author marks a
  real cross-boundary finding `fix_now` (or omits it) to avoid filing an issue or
  splitting scope, so security debt is silently absorbed. **Control:** `fix_now`
  is reserved for safe/local/in-allow-list findings (proposal §6 list);
  `/slo-execute` refuses `done` while a row is undisposed; `/slo-retro` re-reads
  the ledger and files `file_github_issue`/`upstream_feedback` rows through the
  existing lanes (dedupe + cap apply).
- **`tm-secure-value-loop-abuse-4`** — *Additive-enum break.* A new status value
  is read by a legacy parser that does not recognise it and is silently treated as
  `done`, hiding an unfinished/blocked milestone. **Control:** documented parser
  rule "unknown status → `blocked` semantics, never `done`"; structural test
  asserts the old four parse and the new values are listed in the template comment.

## Residual risks

| Risk | Exploit path | Compensating control | Accepted | Owner | Review by |
|---|---|---|---|---|---|
| Author can hand-author a thin/dishonest Secure Value Contract | The framework forces *shape* and cross-checks, not the team's integrity | `/slo-plan` + `/slo-verify` + `/slo-retro` raise the cost of faking; mitigation not elimination (`tm-secure-value-loop-abuse-3`) | yes | Sherif | 2026-09-02 |
| Envelope generates guidance; it does not enforce a target product's runtime security | SLO ships no security runtime; a target runbook could ignore recommended controls | Contract makes secure defaults the path of least resistance and routes infra/cloud/AI surfaces to the existing security skills; disclosed, not eliminated | yes | Sherif | 2026-09-02 |
| Bundle table drifts behind cited standards' editions (ASVS/MASVS/API/LLM Top 10) | A new edition renumbers/renames; the table goes stale (the exact OWASP-2018→2024 drift this work was created to fix) | Bundles cite **by name + edition year**; LOOPS doc names the review cadence | yes | Sherif | 2026-09-02 |

## Compliance mapping

| Control | SOC 2 | OWASP ASVS 5.0 |
|---|---|---|
| Operator-readiness fail-closed gate before execution | CC8.1 (change mgmt — readiness before change) | V1 Architecture / secure SDLC |
| Detected-work disposition + audit trail (owner/evidence/due) | CC7.2 / CC7.3 (incident/finding handling) | V7 Logging & error handling |
| Generated-artifact injection neutralisation (`~~~text` fence) | CC6.8 (unauthorized/malicious input) | V5 Validation, sanitization & encoding |
| Honest exit states (no silent `done`) | CC4.1 (monitoring — accurate status) | V1 SDLC / V7 |
| Residual-risk record with owner + expiry | CC3.2 (risk assessment) | V1.6 (security decisions documented) |

## AI-specific section

`N/A — ai_component: false.` This work introduces no new LLM/agent invocation
surface. The Bundle E (AI/LLM) lane it *documents* routes to the existing
`/slo-verify` Pass 5 (gated on a target runbook's own `ai_component: true`); this
envelope is not itself an AI component.
