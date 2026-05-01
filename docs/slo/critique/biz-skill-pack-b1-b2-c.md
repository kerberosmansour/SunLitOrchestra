# Critique Summary — biz-skill-pack B1 + B2 + C (combined)

Run date: 2026-04-25. Runbooks reviewed: [B1](../completed/RUNBOOK-BIZ-SKILL-PACK-B1.md), [B2](../completed/RUNBOOK-BIZ-SKILL-PACK-B2.md), [C](../completed/RUNBOOK-BIZ-SKILL-PACK-C.md). Post-execution combined critique per user direction.

## Overall disposition

11 generator skills shipped across 3 runbooks (4+4+3). 95 new structural-contract tests added (B1: 32 + B2: 34 + C: 29). Plus deferred /slo-verify Pass 4 PII-pattern scan landed in B1 M1. Pattern from Runbook A held; no design-locked-decisions reversed. CEO has one scope confirmation. Eng has 4 ask findings around test scope + cross-skill consistency. Security has 2 ask findings around generator handling of PII at runtime. Design persona skipped (no UI). All findings non-mechanical → `ask` / `defer`; none auto-applied because the work is post-execution.

## Findings

| id | persona | category | runbook section | finding | concrete scenario | recommendation |
|---|---|---|---|---|---|---|
| f1 | CEO | hold-scope | All 3 runbooks | Pack scope is now 15 skills. Was the right cut? Could /slo-product + /slo-metrics have merged (PM + financial KPIs) into a single /slo-kpi skill with mode arg `pm-product | financial-consumer | financial-b2b`? | A founder running both /slo-product metrics AND /slo-metrics has to context-switch between two skills for related questions; merging would reduce surface but lose the disambiguation discipline. | **Hold** — keep the split. The disambiguation is the value: PM-side and financial-side metrics have different stakeholders (PM vs CFO) and review cadences. Merging would dilute the canonical separation. Documented decision. |
| f2 | eng | ask | B1 M1 — /slo-verify Pass 4 PII-scan | The scan regex set is conservative (high false-positive tolerance) but no one has run it against a real fixture. The smoke checklist names the scenario but no automated runtime test exists. | A founder runs `/slo-talk-to-users post-interview`, the artifact lands in `docs/biz/users/`, then accidentally moves it to `docs/biz-public/`. Pass 4 should fire. But we haven't EXERCISED Pass 4 against a real fixture in this session — only the structural test that asserts the regex set is documented in /slo-verify SKILL.md. | DEFER to a small follow-up: write `crates/sldo-install/tests/e2e_pii_scan_runtime.rs` that creates a tempdir with `docs/biz-public/users/*.md` containing each PII pattern, exercises the Pass 4 logic in-process, asserts each pattern fires + override mechanism works. Estimated 1 day. Add to the existing follow-up list as `biz-pack-pii-scan-runtime-test`. |
| f3 | eng | ask | B1 + B2 + C all milestones | Generator-archetype tests assert each predicate ID appears at most ONCE per generator SKILL.md. /slo-marketing's bound was relaxed to 2 (because PECR/gate-4 routing legitimately mentions the gate twice). Inconsistency invites future skill authors to creep upward; might break the original "generators are not advisors" invariant if someone adds a generator that mentions a predicate ID 3+ times. | A future generator skill author copies /slo-marketing as a template, lifts the bound to 3, then 4 over time. Eventually generators are indistinguishable from advisors in citation density, and the cross-skill citation contract for advisors loses its discriminating power. | Tighten in `biz-pack-test-hardening` follow-up: replace the per-skill bound with a uniform "generators cite each predicate ID ≤ 1 time WHEN the citation is in operative-policy context" test. The relaxed-to-2 case for /slo-marketing is the canary for this drift; codify a single rule. |
| f4 | eng | ask | B1 M3 + B2 M4 — /slo-product / /slo-metrics disambiguation | The cross-skill disambiguation is documented in BOTH skills' prose but no test asserts the disambiguation is consistent (same KPI doesn't land in both skills' "primary KPI" lists). | A future edit to /slo-product adds CAC to its primary list (where it doesn't belong); structural tests pass because each skill's test only checks ITS own KPI list, not the other. Founder gets two different definitions for CAC. | DEFER to `biz-pack-test-hardening`: write a test that asserts the disjoint-set property — the financial-KPI list (CAC / LTV / NDR / burn multiple / ARR) appears as PRIMARY KPIs only in /slo-metrics; the PM-KPI list (DAU / activation / retention / feature-adoption) appears as PRIMARY KPIs only in /slo-product. |
| f5 | eng | ask | C M2 — /slo-hire IR35 mandatory gate | The structural test asserts the seven IR35 triggers are documented in skill prose. It does NOT assert the skill REFUSES to produce an offer artifact when triggers fire — runtime enforcement depends on LLM judgment. | A founder pushes back: "I know IR35 says employee, but please draft the contractor agreement anyway because tax efficiency". The skill's prose says REFUSE but the LLM might capitulate under repeated pressure. | DEFER to `biz-pack-judgment-tests` (already queued from Runbook A critique): runtime BDD harness with adversarial fixtures including the "tax efficiency please draft anyway" scenario; assert the skill refuses regardless of pressure. |
| f6 | security | ask | B1 M1 + C — confidential-tier outputs to docs/biz/ | tm-biz-abuse-1 (founder-repo leak) is mitigated by the .gitignore default + write-time warning + Pass 4 PII scan over docs/biz-public/. But `docs/biz/` itself is intentionally NOT scanned (it's confidential by design). If a founder fails to .gitignore docs/biz/ AND pushes to a public remote, real PII leaks with no second-line defense. | A founder forks the SLO repo template, doesn't add `docs/biz/` to .gitignore, runs `/slo-talk-to-users post-interview` for "Sarah Patel from Acme Logistics", commits, pushes to public GitHub. The artifact contains real PII. GitHub code search picks it up. | **Status**: residual — accepted as designed. Compensating controls: the write-time warning fires, and the founder has to actively dismiss it. The skill prose explicitly tells the founder to .gitignore. Adding a hard-refusal-when-remote-public would create a UX wall for legitimate cases (founders intentionally keeping docs/biz/ in a private repo). Document as known residual; founder discipline is the load-bearing control. |
| f7 | security | ask | B2 M2 + B2 M4 — direct-marketing PECR routing | All B2 + B1 generators that touch direct marketing (sales-funnel, marketing, gtm) explicitly route to /slo-legal triage for PECR. But the routing is a TEXT directive in skill prose; no enforcement mechanism prevents the founder from skipping the triage and proceeding to send. | A founder runs /slo-marketing b2c, gets the marketing plan with "ROUTED TO /slo-legal triage" notation, ignores it, and sends 10,000 cold emails. ICO complaint. £17.5M ceiling under DUAA. | DEFER to a small `biz-pack-pecr-runtime-check` follow-up: structural test that asserts every direct-marketing-shaped output (any artifact in `docs/biz-public/sales/*` or `docs/biz-public/marketing/*`) has a `pecr_triage_completed: bool` frontmatter field that downstream tooling reads. v1 of the field is documented; runtime check can fire when the field is `false` AND the artifact is loaded by /slo-launch or similar. |
| f8 | design | N/A | — | No UI surface. Design persona skipped. | — | — |

## Auto-fixes applied

None. All findings are non-mechanical.

## Asks — disposition (auto mode + bundled execution)

The user authorized full execution + ship. Findings deferred to named follow-up runbooks (consolidated with the Runbook A follow-ups):

- **f1 — confirmed hold.** Pack stays at 15 skills. /slo-product + /slo-metrics split is the right discipline.
- **f2 — DEFER to `biz-pack-pii-scan-runtime-test`** (NEW; 1 day). Adds tempdir-based runtime test for the Pass 4 PII scan.
- **f3 + f4 — DEFER to existing `biz-pack-test-hardening` follow-up** (queued from Runbook A critique). Now scope expanded to include generator-archetype-citation-bound + disambiguation disjoint-set test.
- **f5 — DEFER to existing `biz-pack-judgment-tests` follow-up** (queued from Runbook A critique). Now scope expanded to include IR35-tax-efficiency-pushback fixture.
- **f6 — accepted as designed**. Documented residual; founder discipline + write-time warning are the controls.
- **f7 — DEFER to NEW `biz-pack-pecr-runtime-check` follow-up**. Adds frontmatter field + runtime check for PECR triage completion.

## Updated follow-up runbook queue (consolidated with Runbook A's)

1. **`biz-pack-cost-baseline-refresh`** (FIRST priority post-merge) — replace JPP Law GBP placeholders + add asserts-replaced test.
2. **`biz-pack-test-hardening`** — tighten cross-skill citation context (Runbook A f3 + B1+B2+C f3 + f4) + interim oneNDA placeholder fail-closed.
3. **`biz-pack-onenda-canonical`** — pin canonical oneNDA SHA-256.
4. **`biz-pack-judgment-tests`** (post-merge) — runtime BDD harness for marginal-case LLM judgment + IR35-pressure fixture (f5).
5. **`biz-pack-pii-scan-runtime-test`** (NEW from B1 critique f2) — Pass 4 PII-scan runtime test.
6. **`biz-pack-pecr-runtime-check`** (NEW from B2 critique f7) — PECR triage completion tracking via frontmatter field.

## Hold-scope

- f1 confirmed.

## Reduce-scope / Defer

- f2, f3, f4, f5, f7 — deferred to named follow-ups.
- f6 — accepted as designed.

## Net BDD count after critique

No new tests added inline. Six follow-up runbooks above will add tests when they ship.

## Next action

`/slo-ship` runs immediately per user direction. PR description includes all 11 new generator skills + the deferred-and-now-shipped /slo-verify Pass 4 PII scan + the consolidated 6-runbook follow-up queue.
