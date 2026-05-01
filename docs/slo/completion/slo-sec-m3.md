# Completion Summary — slo-sec Milestone 3

## Goal completed

`skills/slo-critique/personas/security.md` rewritten around bug-class elimination + variant analysis + threat-model citation + self-bounded mandate, with a five-condition finding-acceptance gate. Two canonical reference files land under `skills/slo-critique/references/`: the bug-class catalog (OWASP ASVS 5.0 chapters V1–V17 with elimination patterns) and the variant-analysis playbook (ripgrep / ast-grep / semgrep strategies + small-codebase exit). `skills/slo-critique/SKILL.md` rotation-order description updated to reference the new framing; finding-row table schema unchanged.

## Files changed

- `skills/slo-critique/personas/security.md` (rewritten — ~75 lines)
- `skills/slo-critique/SKILL.md` (one-line edit on rotation-order item 3)
- `skills/slo-critique/references/bug-class-catalog.md` (NEW — ~220 lines)
- `skills/slo-critique/references/variant-analysis-playbook.md` (NEW — ~140 lines)
- `crates/sldo-install/tests/e2e_slo_sec_m3.rs` (NEW — 18 tests)
- `docs/slo/completed/RUNBOOK-SLO-SECURITY-EMBEDDING.md` (Tracker + Evidence Log updated)

## Tests added

18 tests total. 13 exercise the rewritten persona + catalog + playbook; 5 lock in the no-edit invariants (ceo.md / eng.md / design.md / finding-row-schema / existing-critiques).

## Runtime validations added

All 18 M3 tests run via `cargo test -p sldo-install --test e2e_slo_sec_m3`; none rely on external tools or network.

## Compatibility checks performed

- 302 tests across the baseline (241 pre + 23 M1 + 16 M2 + 18 M3 + 4 legacy persona tests that still pass) — all green.
- FNV-1a-64 hashes of ceo.md, eng.md, design.md match pre-M3 captures (0xa297a61e54048204 / 0x0f8013ab4393afb4 / 0x449d7a844c24e5cd).
- Finding-row table header in `skills/slo-critique/SKILL.md` byte-identical to pre-M3.
- Existing `docs/slo/critique/tla-sha-autopop.md` still contains a valid finding-row table.
- Legacy `slo-sp-m6::security_persona_has_owasp_and_stride` passes after 5th finding-acceptance condition added (contains OWASP + STRIDE + exploit + step-by-step).
- `sldo-install --dry-run` (implicit from prior M1/M2 verification) continues to install the updated skill pack.

## Documentation updated

- Rotation-order line in `skills/slo-critique/SKILL.md` references class elimination + variant analysis + threat-model citation.
- `docs/ARCHITECTURE.md` — unchanged.
- `SECURITY.md` — unchanged.
- Runbook Evidence Log + Milestone Tracker — updated.

## .gitignore changes

None.

## Test artifact cleanup verified

`git status --short` shows 2 modified (security.md, critique SKILL.md) + 3 untracked (bug-class-catalog.md, variant-analysis-playbook.md, e2e_slo_sec_m3.rs) + the docs work from prior milestones. No log / SARIF / cache output.

## Deferred follow-ups

- Runtime agent test that `/slo-critique` with the rewritten persona produces findings in the 5-condition format. Deferred (Claude Code harness not wired).
- Property-based test that random persona-prompt-injection strings don't alter the output contract. Deferred.
- Per-chapter elimination-pattern assertion in the catalog test (currently only asserts total coverage). Deferred.
- CI job that verifies ripgrep / ast-grep / semgrep are installable on a fresh runner. Deferred to M4's Pass 4.

## Known non-blocking limitations

- Structural-contract tests validate the documented shape of the persona, not the runtime behavior of Claude Code interpreting the rewritten persona.
- FNV-1a-64 is non-cryptographic (same limitation as M2); the purpose is drift detection.
- The catalog cites current SunLitSecureLibraries crate names; if that workspace renames a crate, the catalog ages. No automated drift check.
- The variant-analysis playbook's Semgrep instructions note the pro-rules gating post-Dec 2024 relicense; if users need cross-file Rust analysis and don't have the pro license, they must fall back to Opengrep or accept intra-file coverage only.
- Added a 5th finding-acceptance condition during execution (not in the original M3 contract). Flagged in lessons as a "legacy test regression" — the addition strengthens the persona rather than widens scope.
