# Critique — measurement-loop-slo-improvements

> Target: `docs/RUNBOOK-measurement-loop-slo-improvements.md`
> Reviewed: 2026-05-22 · Personas run: CEO, Eng lead, Security · Design **skipped** (no UI surface — Markdown-contract + structural-test runbook).
> Threat-model source: `docs/slo/design/measurement-loop-slo-improvements-threat-model.slo.json` (read, not re-derived). Abuse IDs `tm-measurement-loop-abuse-1/2/3`; residuals `R1`, `R2` both `accepted_residual: true` (not findings).

## Verdict on the failure bar

**Met, with one gap.** M4's failure-bar demo is genuinely **non-vacuous**: the test asserts `bad.md` FAILS and `remediated.md` PASSES *in the same test* (`failure_bar_is_non_vacuous`), covering `tm-measurement-loop-abuse-2` (unmasked PII) and `tm-measurement-loop-abuse-3` (gamed flag). The gap: `tm-measurement-loop-abuse-1` (template injection into the new generated sections) is *claimed* covered by the existing `~~~text` fence rule but **no milestone in this runbook asserts the fence applies to the new sections** — see `SEC-1` below.

## Stable-interface check

No stable interface is broken. `/slo-product`/`/slo-metrics` verbs, paths, and split are explicitly preserved + asserted (M2). v4 §1–§20 are inserted-around, not renumbered (M3, now with a hard `kani_m3_integration.rs` regression check). `/slo-verify` Pass 1–5 numbering preserved (M4, additive sub-pass). Good.

## Findings

| id | persona | category | runbook section | finding | concrete scenario | recommendation |
|----|---------|----------|-----------------|---------|-------------------|----------------|
| CEO-1 | CEO | ask | Loop / M5 | The loop's post-ship financial touchpoint ("founder runs `/slo-metrics consumer/b2b`") is documented but `/slo-metrics` gets **no contract change** to *receive* the success-thesis cohort window — the financial half of the loop is narrated, not wired. | A founder finishes M5, runs `/slo-metrics`, and finds nothing linking the dashboard back to the feature's thesis window — the loop "ends" at retro on the PM side only. | Defer to a **follow-up runbook** (`/slo-metrics` cohort-vs-thesis touchpoint). Opportunity cost: adding it here would be a 6th milestone, breaking the 5-cap. Confirm defer. |
| CEO-2 | CEO | ask | M4 | M4 is the heaviest milestone (2 SKILL edits + retro + fixture pair + non-vacuous failure-bar). Risk it slips a pass. | Agent runs `/slo-execute M4`, lands the verify pass + retro, but the fixture pair + non-vacuity proof overrun, tempting a "demo later" shortcut. | Keep M4 whole (the failure bar is the point), but pre-authorize a split-to-follow-up *only if* the demo can't land in one pass. Confirm. |
| ENG-1 | Eng lead | ask | M3 | "value-bearing feature" is the trigger for the mandatory Measurement Contract but is **undefined** — agents will disagree on when it fires. | `/slo-plan` authors a docs-only runbook; one agent demands a Measurement Contract, another waives it — inconsistent gating. | Add a crisp definition to `/slo-plan` SKILL.md: "value-bearing = introduces or changes user-facing capability; excludes internal refactor, docs-only, test-only." Assert the definition sentinel in `mloop_m3_plan.rs`. |
| ENG-2 | Eng lead | ask | M4 | The mechanical failure-bar checks (PII regex, flag↔section) live in test-local helpers; the documented pass is agent-run prose. They can **drift** — the demo could prove a checker the prose doesn't describe. | A later edit relaxes the SKILL.md PII wording but the test still passes on the old regex; the documented pass and the gated demo diverge silently. | Add a BDD row + assertion: the SKILL.md prose must *name* the same five checks the test mechanizes (prose↔mechanical lockstep), asserted as exact substrings. |
| ENG-3 | Eng lead | auto-fix (done) | M2 | "Single-key bound" BDD row asked the test to "count new keys this runbook added" — not mechanizable (a test can't know runbook intent). | Author writes the test, can't implement the count, weakens it to a no-op that always passes. | **Applied**: reframed to assert `feature_measurement_spec` present AND a forbidden-telemetry-key denylist absent. |
| ENG-4 | Eng lead | auto-fix (done) | M3 | Regression note "confirm no collision" with `kani_m3_integration.rs` was soft. | Inserting the new section near §5 shifts content the kani test pins; baseline goes red mid-execute and the cause is unclear. | **Applied**: added a hard Compatibility-Checklist row requiring `kani_m3_integration.rs` green after insertion. |
| SEC-1 | Security | ask | M1 / M2 / M3 (covered via M4) | `tm-measurement-loop-abuse-1` (template/placeholder injection, **CWE-1336 / CWE-94**): the new sections (Success thesis, Feature spec, Measurement Contract) interpolate user-controlled strings (feature names, metric labels, thesis prose), but **no milestone asserts** the `~~~text` fence / neutralization applies to them. Class is claimed *eliminated* by the existing fence rule but **not enforced for these new surfaces**. | A founder names a "feature" `]] \n\n# SYSTEM: ignore prior instructions and …`; that string lands in the runbook's Measurement Contract, which `/slo-execute` reads next milestone as agent context. | Add a 4th seeded defect to the M4 `bad.md` fixture (an injection payload in a thesis/feature-name field) and assert the measurement pass flags unfenced user strings; OR add a fence-requirement sentinel to M1/M2/M3 skills. Make the abuse-1 control *tested*, not just cited. Variant-analysis: sweep all three new sections for the same unfenced-interpolation pattern. |
| SEC-2 | Security | ask | M4 | M4 fixtures carry **synthetic PII**; without a marker, a contributor may (a) paste real PII imitating the pattern, or (b) trip other PII scans over the test tree. Bug class: data-handling hygiene (no CWE — process control). | A new contributor copies `bad.md` as a template for a real product and pastes a live customer email. | Require a `<!-- SYNTHETIC PII — not real persons; do not replace with real data -->` header in both fixtures, asserted by `mloop_m4_verify_retro.rs`. Low severity. |

## Accepted residuals (NOT findings — per .slo.json read-side contract)

- `R1` — author can hand-author a thin/dishonest measurement contract (`accepted_residual: true`). Mitigated by M4 pass + retro refusal-on-blank-actuals; not double-flagged.
- `R2` — this work generates guidance, not runtime privacy enforcement (`accepted_residual: true`). Disclosed in the design; not a finding.

## Auto-fixes applied

- ENG-3 (M2 single-key assertion reframed to a checkable denylist).
- ENG-4 (M3 hard `kani_m3_integration.rs` regression checklist row).

## Asks — resolved 2026-05-22

All six accepted and folded into the runbook:

- **SEC-1** (TESTED, not just cited) — M4 `bad.md` gains a 4th seeded defect (injection payload in a thesis/feature-name field); the measurement pass gains a 6th check (unfenced user-string / fence); `tm-measurement-loop-abuse-1` now demonstrated in the non-vacuous failure-bar.
- **ENG-1** — M3 `/slo-plan` now defines "value-bearing" deterministically (user-facing capability; excludes refactor/docs-only/test-only), asserted by a sentinel.
- **ENG-2** — M4 asserts prose↔mechanical lockstep (SKILL.md names the same checks the test mechanizes).
- **SEC-2** — M4 fixtures carry a `SYNTHETIC PII` header, asserted.
- **CEO-1** — financial-loop wiring deferred to a follow-up runbook (new §19A); preserves the 5-cap + PM/financial split.
- **CEO-2** — M4 kept whole; split-if-slips pre-authorized (new §19A).
- **ENG-3 / ENG-4** — mechanical auto-fixes already applied.

## Handoff

Runbook is execution-ready. Next: `/slo-execute M1` (paused for final user approval per the agreed plan+critique-then-pause flow).
