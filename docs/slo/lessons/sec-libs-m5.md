# Lessons Learned - sec-libs Milestone 5

## What changed

- Added `docs/sec-libs-dogfood-2026-05-06.md`, a real dogfood report against `slo-security-embedding` M3.
- Added `crates/sldo-install/tests/e2e_sec_libs_m5.rs` with structural-contract tests for the report, reader evidence, matched/unmatched/filed sections, confirmation-gated filing status, and M1-M4 compatibility.
- Updated the runbook tracker, M5 BDD wording, and M5 evidence to reflect the dogfood outcome.

## Design decisions and why

- **M5 picked one target after a shortlist.** The runbook said both "multiple already-shipped milestones" and "M5 picks ONE target milestone." The implementation followed the explicit out-of-scope boundary: shortlist three candidates, select the one with `security_libs_required: true`, and explain the rejection of the other two.
- **No live filing without confirmation.** M5 originally expected filed issue URLs, but M3 and M4 require explicit per-issue confirmation. The dogfood report records `deferred-pending-confirmation` rows rather than filing issues or inventing URLs.
- **The reader symlink guard paid off in real smoke.** A first run through `/tmp` failed because `/tmp` is a symlink on macOS. Rerunning through `/private/tmp` passed and preserved the M1 no-symlink-path guarantee.
- **Catalog-grounded matching stayed conservative.** `secure_boundary` and `security_core` are close to the target needs, but prompt-injection-boundary and variant-analysis-schema remain unmatched because the declarations do not advertise those exact capabilities.

## Test patterns that worked well

- The M5 test checks the report as an artifact, not hidden runtime state. That makes dogfood reviewable in normal PR diff form.
- Tests assert the declaration SHAs, schema SHA, commits, and catalog counts, so future report rewrites cannot silently drop provenance.
- The filed-section test accepts a truthful deferred status and pins the "No `gh issue create`" discipline.

## Missing tests that should exist later

- A future confirmed filing run should replace the deferred rows with real issue URLs and add a test that checks the URLs use `kerberosmansour/slo-security-intake` or an explicitly confirmed upstream destination.
- If a runtime matcher/filer executable lands later, it should consume the same report-shaped fixtures and verify matched/unmatched/filed records end to end.
- If declarations add an agent-prompt boundary component, the dogfood report should be refreshed so `gap-agent-prompt-boundary` becomes a matched recommendation.

## Rules for future follow-up

- Keep default filing pointed at `kerberosmansour/slo-security-intake` unless the user explicitly confirms direct upstream filing.
- Do not weaken the symlink refusal just because macOS makes `/tmp` convenient.
- Use `SunLitSecurityLibraries` as the canonical owner spelling.
- When a milestone asks for live issue URLs, first ask for per-issue confirmation; deferred status is the honest fallback.
