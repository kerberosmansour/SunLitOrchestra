# Lessons Learned - sec-libs Milestone 3

## What changed

- Added `skills/slo-sec-libs/references/capability-gap-schema.md`, the regex-validated record schema for default intake filings.
- Added `skills/slo-sec-libs/references/upstream-filing-discipline.md`, the argv-list, no-`--repo`, confirmation-gated filing discipline.
- Extended `skills/slo-sec-libs/SKILL.md` with `--file-gaps <m2-output.json> --intake-dir <path>` mode.
- Added `crates/sldo-install/tests/e2e_sec_libs_m3.rs` with 12 structural-contract tests.
- Updated the runbook tracker/evidence and the skill catalog.

## Design decisions and why

- **Default filing uses a local intake checkout.** M3 intentionally avoids `--repo`, so the destination is the local `slo-security-intake` origin URL shown to the user before filing.
- **No live issue was filed during implementation.** The M3 contract requires explicit per-issue confirmation. The implementation PR validates the path structurally and leaves the first real filing to a deliberate confirmed run.
- **The schema emits the canonical owner spelling.** User direction clarified that `kerberosmansour/SunLitSecureLibraries` is superseded by `kerberosmansour/SunLitSecurityLibraries`, so M3 rejects the legacy spelling even if older templates still mention it.
- **Target prose is not trusted data.** The issue body is built from normalized, regex-validated fields only; row context is one sanitized line, not copied Markdown.

## Test patterns that worked well

- Structural tests pin both the happy-path command shape and the forbidden shortcuts.
- Unicode and Markdown injection guards are tested through required prose, which keeps the contract visible to future maintainers.
- M1/M2 compatibility tests remain in the M3 suite so filing changes cannot erase the reader or matcher modes.

## Missing tests that should exist later

- M4 should add executable cap/rate-limit coverage when direct upstream filing is introduced.
- M5 dogfood should perform the first fully confirmed live filing against real unmatched output and record the resulting issue URL.
- A future standalone filer executable, if added, should get fixture-based tests for valid, rejected, declined, and filed cases.

## Rules for M4

- Keep M3's default destination unchanged.
- Add `--file-upstream` as an explicit gate; never infer third-party filing from owner names alone.
- Preserve per-issue confirmation even when the 40-issues/hr cap is added.
- Continue using canonical `SunLitSecurityLibraries` spelling everywhere.
