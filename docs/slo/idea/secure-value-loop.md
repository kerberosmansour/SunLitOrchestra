---
name: secure-value-loop
created: 2026-06-02
status: ideation
tla_required: false
source_proposal: "~/Downloads/sunlit_orchestra_secure_value_loop(1).md"
---

# Secure Value Loop — a security/value envelope around the SLO loop

## The pain

Sherif drives AI agents through the SLO sprint loop (Think → Plan → Build → Review → Test → Ship → Reflect). The pack already embeds a great deal of security: `/slo-architect` emits a threat model, `/slo-critique` runs a class-elimination security persona, `/slo-verify` Pass 4/5 runs a security + LLM test matrix, `/slo-retro` files lane-classified issues, and nine dedicated security skills exist (`slo-sast`, `slo-dast-tuner`, `slo-nettacker`, `slo-rulegen`, `slo-ruleverify`, `slo-cloud-threat-model`, `slo-sec-libs`, `slo-kani`, `slo-tla`).

What it still does **not** do is make three disciplines *typed and unavoidable* in the runbook contract:

1. **Operator readiness** — a milestone can start, then stall mid-execution because a cloud account, OAuth app, API key, test device, or approval was never surfaced. There is no first-class "tell the human what they must provide before M-N starts" gate. Today readiness is implicit in the Definition of Done.
2. **Detected work** — work discovered mid-sprint (a too-broad OAuth scope, a missing upstream library API, a CrossGuard policy gap) lands in the lessons file as prose ("mistakes / root causes") with no enforced disposition. Findings can end as merely *observed*.
3. **Honest exit states** — a milestone's only terminal states are `not_started | in_progress | blocked | done`. There is no vocabulary for `human_review_required | blocked_by_operator | blocked_by_upstream | issue_filed | accepted_risk`, so a run that genuinely needs a human silently reads as "done" or "blocked".

The proposal also wants a **value-first gate** ("do not optimize for the smallest wedge — optimize for the smallest *valuable, secure, testable, unblocked, reviewable* outcome") and a single canonical reference doc + v4 template patch so every value-bearing runbook carries a Secure Value & Security Contract.

The delta is measurable: every runbook that touches identity, secrets, PII, cloud, AI, or a public boundary currently re-derives operator prereqs and finding-disposition by hand, and partial mocks / "operator assistance needed" surprises appear *after* a milestone starts.

## What is already shipped (do not rebuild)

- Threat model as a first-class `/slo-architect` artifact (`SECURITY.md` + `threat-model.md` + `threat-model.slo.json`, frozen `tm-<slug>-abuse-N` IDs, STRIDE, compliance mapping, `tla_required`/`kani_required`/`security_libs_required`/`ai_component` flags).
- `/slo-critique` security persona with class-elimination + variant analysis, concrete-scenario-only gate.
- `/slo-verify` Pass 4 (security: SAST/SCA/secrets/IaC/DAST via `/slo-dast-tuner`/privacy-PII) and Pass 5 (LLM, gated on `ai_component`).
- `/slo-retro` lane-classified issue filing (product / upstream-OSS / slo-process) with dedupe + per-session cap.
- v4 Contract Block already carries `Data classification`, `Proactive controls in play`, `Abuse acceptance scenarios`, `AI tolerance contract`, `Measurement deliverables`.
- Nine security skills + the threat-model producer runbook integrated at `/slo-architect` Step 3.5.

## The genuinely net-new wedge

A **Secure Value & Security Contract** template section + the three typed disciplines (Operator Readiness Gate, Detected Work Ledger, expanded handoff states) threaded into `/slo-plan` → `/slo-execute` → `/slo-verify` → `/slo-retro` (+ `/slo-ship`), plus a canonical `docs/SECURE-VALUE-LOOP.md` and the two LOOPS docs updated so every stage names its cybersecurity output. This is a **documentation + skill-prompt change to the SLO pack itself** — no new Rust crate; structural-contract tests under `xtasks/sast-verify/tests/` guard the skill frontmatter/output-path invariants.

## Three implementation approaches

- **Approach A — docs + template only.** Add `docs/SECURE-VALUE-LOOP.md`, patch the v4 template with the Secure Value & Security Contract section, patch the two LOOPS docs. ~3 days. Wedge proves the contract shape but nothing *enforces* it; skills keep behaving as today.
- **Approach B — template + skill-prompt enforcement (recommended).** A plus: patch `/slo-plan` to require the contract for value-bearing milestones, `/slo-execute` Global Entry to check Operator Readiness and open/update the Detected Work Ledger, `/slo-verify` to record security tests as first-class evidence rows, `/slo-retro` to dispose every ledger row. Add the expanded milestone status vocabulary as an *additive* enum. Structural-contract test extended. ~1 runbook (≤5 milestones).
- **Approach C — Rust-backed ledger/readiness validator (`sldo-*` crate).** B plus a binary that parses the runbook, validates ledger dispositions and readiness states deterministically, and gates CI. Biggest scope; forces LLM-native reasoning (is this finding `fix_now` or `fresh-runbook`?) into Rust where it is worse. Over-engineered for a markdown-discipline change.

## Recommendation

**Approach B.** The reasoning (classify a finding's disposition, judge whether a wedge is "too small", decide operator readiness) is LLM-native; the only deterministic surface worth a test is the *structural contract* (frontmatter, required rows, additive status enum), which the existing `xtasks/sast-verify/tests/` pattern already covers without a new crate. Backward compatibility is the hard constraint: every new field/section/state must be **additive and optional for legacy runbooks** (same posture as §5A Measurement Contract and §10 Carry-forward).

## Open questions for /slo-research

1. The proposal's `proactive_controls` list uses the **OWASP 2018** naming (`C1_define_security_requirements … C10_handle_errors`). OWASP reorganized to a different C1–C10 in 2024. Which version does the shipped threat model / Pass 4 cite today, and should the Secure Value Contract pin a version (or cite by name, not number) to avoid drift?
2. OpenAI **Symphony** supplies the handoff-state + bounded-concurrency model. Which of its exit states map cleanly onto an *additive* SLO milestone-status enum without breaking the existing `not_started|in_progress|blocked|done` consumers (`/slo-resume`, `/slo-execute` Step 1.5, the Milestone Tracker)?
3. **GStack** is cited as the source of the sprint chain SLO already runs — is there a canonical public source, or is it the author's shorthand? (Affects whether `sources.md` can cite it honestly.)
4. The Detected Work Ledger's dispositions (`fix_now | file_github_issue | operator_action | upstream_feedback | accepted_risk`) overlap `/slo-retro`'s existing lane vocabulary (`product | upstream-OSS | slo-process` and `micro | milestone | fresh-runbook`). Can the ledger reuse the existing vocabulary, or does it need its own — and how do they reconcile so we don't ship two competing taxonomies?
5. The security **test bundles A–F** (docs / app / backend-API / cloud-IaC / AI-LLM / mobile) — do they map onto the surface-detection logic `/slo-verify` Pass 4 already uses, or do they need to become an explicit, versioned bundle table the runbook references?
6. SBOM/provenance for "Ship" (proposal Stage 9) — SLSA + CycloneDX/SPDX. Which release artifacts in *this* repo (crates.io publishes, the release-zip workflow) actually warrant provenance, vs. "not applicable" for a markdown skill pack?
