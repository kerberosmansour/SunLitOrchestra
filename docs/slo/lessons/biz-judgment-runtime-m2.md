# Lessons — biz-judgment-runtime M2

## What worked

- **Branched assertion logic on `must_refuse`.** The harness has two paths: control / non-refusal expects an artifact with matching gates; refusal expects either no artifact + a refusal phrase, or a non-`mode: draft` artifact (a triage / prepare memo). The split keeps each path simple and the JUDGMENT REGRESSION message specific.
- **Refusal-phrase allowed list as `pub const REFUSAL_PHRASES`.** Easy to extend when a real claude response uses new wording — add a phrase, re-run. Single-word phrases like `"no"` deliberately excluded so neutral text can't accidentally match.
- **Conservative cost-cap test.** Computes a worst-case upper bound (`fixture_count × per_fixture_budget`) rather than parsing claude's per-call cost reporting. Stdlib-only, no JSON parser needed, and a real run will spend less than the bound — so a passing cap test really means safe.
- **Per-fixture `#[test]` (not table-driven loop).** Each fixture gets its own test function, so a single failure has a clear name in the output, and `cargo test fixture_slo_legal_tax_efficiency_pushback` runs just that one. Easier triage than a generic "all fixtures" loop.
- **Forwarder over deletion** for the legacy panic-stub. Renaming or removing `runtime_harness_invokes_claude_cli_per_fixture` would break any external doc/tooling that names it. The forwarder keeps the function signature, swaps the body for a one-line pointer.

## What I'd do differently

- **`global_cost_cap_enforced` is structural, not behavioural.** It asserts the worst-case bound is below the env-configured ceiling, but doesn't observe actual spend. Real spend tracking would require parsing `--output-format json`'s cost field; that's a real follow-up if a future fixture set grows past 30+ items where the worst-case bound becomes lossy.
- **Retry policy is exponential-ish (1s, 2s, 4s) but not jittered.** For 9 sequential fixtures across one process, jitter doesn't matter much. If we ever parallelise — don't, but if — add jitter.
- **`is_transient_error` heuristics on stderr substring matching.** Brittle if claude changes its error wording. Future: claude could emit a structured error JSON and we match on the `code` field.

## Pitfalls / things to remember

- **Don't broaden `REFUSAL_PHRASES`.** Each phrase must be specific enough that it can't appear in a non-refusal response. The temptation to add `"cannot"` (would match "I cannot tell you the price without more info" — not a refusal) is the kind of broadening that breaks fixtures down the line.
- **Tempdir cleanup is implicit via `TempDir::drop`.** If a test panics mid-run, cleanup still happens (Drop is run). But if the process is killed (e.g., `Ctrl-C` during a live run), tempdirs may leak under `/tmp` with the `biz-judgment-` prefix. Manual cleanup: `rm -rf /tmp/biz-judgment-*`.
- **The forwarder still has `#[ignore]`.** Don't remove the attribute — the function exists for compatibility, not to actually run anything. Removing `#[ignore]` would make `cargo test` print the forwarder's eprintlns on every default run, polluting output.

## Citations to combined critique

- Runbook A f6 — covered by `fixture_slo_legal_gdpr_*` (gate-4), `fixture_slo_legal_ir35_*` (gate-1), `fixture_slo_fundraise_aa_not_yet_applied` (SEIS/EIS), `fixture_slo_equity_cofounder_split_with_preferential_voting` (Abingdon Health line).
- B1+B2+C f5 — covered explicitly by `fixture_slo_legal_tax_efficiency_pushback` with the `JUDGMENT REGRESSION` failure message naming the load-bearing failure mode.
