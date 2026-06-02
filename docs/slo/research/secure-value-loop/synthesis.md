---
name: secure-value-loop
researched: 2026-06-02
---

# Synthesis — what this means for the design

## The central finding: scope is contract-discipline, not capability

The proposal reads as a from-scratch security overhaul, but the repo evidence
says ~80% of its security *machinery* already ships (threat model in
`/slo-architect`, class-elimination critique, Pass 4/5 verify matrix, nine
security skills, lane-classified retro filing). The design must therefore be
**ruthlessly subtractive**: build only the three typed disciplines that are
genuinely absent, and wire them into existing skills rather than adding new
capability. The design must handle *de-duplication against shipped work* because
the grounding sweep found every scanner lane the proposal lists already has a
home skill (`slo-sast`, `slo-dast-tuner`, `slo-cloud-threat-model`, `slo-sec-libs`,
`slo-kani`/`slo-tla`).

## Net-new surface to design

The three absent disciplines are the whole job. The design must handle an
**Operator Readiness Gate** as a first-class, pre-execution milestone state
because the proposal's §5 + §11 make "every milestone has an Operator Readiness
state" an explicit adoption criterion and the grounding sweep confirmed the
concept is entirely absent today. The design must handle a **Detected Work
Ledger** with an enforced disposition on every row because the proposal's §6
forbids findings ending as merely "observed" and the sweep confirmed findings
currently land as unstructured lessons-file prose. The design must handle an
**additive milestone status vocabulary** (`human_review_required |
blocked_by_operator | blocked_by_upstream | issue_filed | accepted_risk`)
because the proposal (via Symphony) wants honest exit states and the sweep
confirmed the enum is `not_started|in_progress|blocked|done` today —
[Symphony SPEC.md](https://github.com/openai/symphony/blob/main/SPEC.md) shows the
handoff-state pattern is a proven model.

## Backward compatibility is the binding constraint

Every legacy runbook (and `/slo-resume`, `/slo-execute` Step 1.5, the Milestone
Tracker parser) must keep working. The design must handle additive-only changes
because the v4 template already established this exact posture twice — §5A
Measurement Contract and §10 Carry-forward are both "optional; legacy runbooks
without this section remain valid." The new Secure Value & Security Contract
section, the new status values, and the ledger must follow that precedent: new
fields default to optional/`N/A`, new status values *extend* the enum rather than
replace it, and the structural-contract test in `xtasks/sast-verify/tests/` must
assert the *additive* shape (old four values still parse) because that test
harness is the only deterministic gate the pack has.

## Reconcile the two disposition taxonomies — do not ship a third

The proposal's ledger dispositions (`fix_now | file_github_issue |
operator_action | upstream_feedback | accepted_risk`) overlap, but do not equal,
`/slo-retro`'s existing lanes (`product | upstream-OSS | slo-process`) and
carry-forward lanes (`micro | milestone | fresh-runbook`). The design must handle
an explicit mapping between the ledger dispositions and the existing lane
vocabulary because the sweep found both already in use and shipping a competing
third taxonomy would fracture `/slo-retro`'s already-tested filing flow. Concrete
direction: the ledger's `file_github_issue`/`upstream_feedback` rows feed
`/slo-retro`'s existing `product`/`upstream-OSS` lanes at retro time;
`operator_action` becomes the bridge to the Operator Readiness Gate;
`accepted_risk` reuses the threat-model Residual-risks table convention (owner +
expiry) already established in `slo-threat-model`.

## Pin OWASP control versions by name, not number

The proposal's `proactive_controls` block uses the **2018** OWASP names; OWASP
shipped a *renumbered* 2024 list (C4 is now "Address Security from the Start",
C10 is "Stop SSRF"). The design must handle citing proactive controls **by
name (and edition year), not by bare number**, because
[the 2024 list](https://top10proactive.owasp.org/archive/2024/the-top-10/)
proves bare "C5/C9" references silently change meaning across editions, and the
v4 template's existing Contract Block row ("OWASP Proactive Controls citations,
e.g., C1, C5, C9") is already exposed to this drift and should be tightened in
the same milestone.

## Test bundles map onto Pass 4, not a new engine

The proposal's Bundles A–F (docs / app / backend-API / cloud-IaC / AI-LLM /
mobile) should become a **referenceable table the runbook points at**, resolved
by the surface-detection `/slo-verify` Pass 4 already performs, plus Pass 5 for
LLM. The design must handle expressing bundles as *selection inputs to existing
passes* rather than a parallel test runner, because the verify-side matrix
(SAST/SCA/secrets/IaC/DAST/privacy + LLM-gated) already exists and a second
runner would duplicate it. The verification standards the bundles cite are stable
and named — ASVS 5.0 / MASVS / API Top 10 2023 / LLM Top 10 2025 — so the bundle
table can cite them directly ([MASVS](https://mas.owasp.org/MASVS/),
[API](https://owasp.org/API-Security/)).

## Ship-stage provenance is mostly "not applicable" here

SLSA/SBOM matter for *released artifacts*; this repo's artifacts are crates.io
publishes and a release-zip. The design must handle making provenance a
**conditional row that resolves to `not_applicable` for the common
markdown-skill milestone** because [SLSA/SBOM mandates](https://petronellatech.com/blog/from-sbom-to-slsa-securing-your-software-supply-chain/)
attach to third-party software distribution, and forcing SBOM on a docs change
would be exactly the kind of false-positive ceremony that makes a contract get
ignored. The `secure_definition_of_done` should keep SBOM/provenance as
`when applicable`, not unconditional.

## NIST SSDF gives the framing, not the structure

SSDF's PO/PS/PW/RV outcome groups validate the proposal's "every stage carries a
security output" thesis, and the finalised
[SP 800-218A GenAI profile](https://csrc.nist.gov/pubs/sp/800/218/a/final) backs
the `ai_component` lane. The design must handle citing SSDF as the *outcome
authority* in `docs/SECURE-VALUE-LOOP.md` (rather than inventing a rationale)
because SSDF is the recognised US baseline and grounds the contract against an
external standard instead of internal opinion.

## Recommended scope for /slo-architect → /slo-plan

Approach B (template + skill-prompt enforcement), one v4 runbook, ≤5 milestones,
no new crate. Provisional milestone shape to take into `/slo-architect`:

1. **Canonical doc + template patch** — `docs/SECURE-VALUE-LOOP.md`; v4 template
   gains the "Secure Value & Security Contract" section (Value Wedge, Security
   Definition of Ready / Operator Readiness, Detected Work Ledger, Security Test
   Plan referencing Bundles A–F), all additive/optional; tighten the proactive-
   controls row to name+edition.
2. **Status vocabulary + Operator Readiness Gate** — additive milestone-status
   enum; `/slo-execute` Global Entry checks readiness and refuses to start on
   `safe_to_continue_without_blockers: false`; structural-contract test extended.
3. **Detected Work Ledger in `/slo-execute` + reconciliation with `/slo-retro`** —
   open/update the ledger during execution; every row carries a disposition;
   retro disposes/files them through the *existing* lane vocabulary.
4. **`/slo-plan` + `/slo-verify` wiring** — `/slo-plan` requires the contract for
   value-bearing milestones; `/slo-verify` records the bundle's security tests as
   first-class evidence rows with pass/`not_applicable`/`waived_with_reason`.
5. **LOOPS docs + `/slo-ship` secure-release checklist + dogfood** — patch
   `docs/LOOPS-ENGINEERING.md` and `docs/LOOPS-BUSINESS.md` so each stage names
   its security output; add the Ship checklist (SBOM/provenance *when
   applicable*, canary, rollback, residual-risk owner, `ship_state`); dogfood the
   whole contract on one small real runbook to prove it is usable, not just
   declared.

`tla_required: false` — this is documentation + skill-prompt discipline with no
concurrency/distributed-state surface; the only deterministic obligation is the
structural-contract test, not a model-checked protocol.
