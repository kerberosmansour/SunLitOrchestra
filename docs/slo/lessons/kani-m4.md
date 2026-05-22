# Lessons Learned — kani Milestone 4

## What changed
- A separate seeded-bug crate (`~/Dev/GitHub/sunlit-kani-demo`, commit `959b23e`) with four kernels (K1–K4), each demonstrated red→green against `kani-verifier 0.67.0`. SLO-repo artifacts: scope report `docs/slo/verify/kani-verification-kani.md`, overview demo-repo line, README "Formal verification demo" note, `tools.toml` pin bump 0.56.0→0.67.0.

## Design decisions and why
- **Bumped the pin to 0.67.0** (the installed, verified version). 0.56.0 was an unverified placeholder; the only honest pin is one verified against. This was the `tools.toml` "re-verify the demo on bump" step in action. Required an M4 allow-list extension (user-confirmed).
- **K4 used the contract path** (`requires`/`ensures`/`recursion` + `stub_verified`) rather than a defensive guard, because the runbook's whole K4 rationale is "recursion is expensive → contracts". The plain recursive `check_gcd` was killed for being slow — which *validated* that rationale empirically.
- **K2 fixed by a genuinely safe API** (`Option`), not by tightening the harness `assume` — the research report explicitly warns against the assume-tightening shortcut.

## Mistakes made
- First `gcd` draft had a `b==0` guard, which would have made `check_gcd` pass green with no red — defeating the naive-first requirement. Caught before recording; reworked to the genuinely-buggy contract version.
- `stub_verified` initially failed to compile: it needs `-Z stubbing` **in addition to** `-Z function-contracts`. Both flags required.

## Root causes
- A "defensive" guard can mask the very bug a demo is meant to catch — for a red→green demo, the pre-fix kernel must actually be buggy.
- Kani unstable features compose: `stub_verified` spans both `function-contracts` and `stubbing` gates.

## What was harder than expected
- Unbounded recursive `gcd` verification is genuinely expensive (had to stop a long-running plain `check_gcd`) — exactly why contracts exist.
- The harness auto-classifier blocks the agent from creating a public GitHub repo even with in-tool user approval; the terminal `gh` command must be run by the user.

## Naming conventions established
- Demo harnesses: `check_<fn>`; contract proof `check_<fn>_contract`; caller-reuse proof `check_<caller>` with `#[kani::stub_verified(<fn>)]`.
- Kani flags by kernel: plain harnesses default; contracts need `-Z function-contracts`; `stub_verified` callers need `-Z function-contracts -Z stubbing`.

## Test patterns that worked well
- Capture all reds first (one batch), then overwrite with fixed versions and capture greens — clean, ordered evidence.
- For K4's red, deleting just the `requires` line via `grep -v` then restoring from a backup gives a precise red without hand-editing.

## Missing tests that should exist now
- None. M5 (pairing + local deep-verify) references this demo's harnesses in the worked example.

## Rules for the next milestone
- M5's pairing-doc worked example must reference a real K-series harness from this demo (e.g. map a hypothetical TLA+ action to `gcd` → `check_gcd_contract`).
- The local-deep-verification reference should document the exact flag sets discovered here (`-Z function-contracts`, `-Z stubbing`) and the quick vs deep bound tiers.
- No CI (user decision); keep deep verification a documented local step.

## Template improvements suggested
- The v4 §5.8 sub-block now has a fully worked, executed example (this runbook's §5.2 + the demo) — strong reference for future Rust runbooks.

## filed_issues
- none filed automatically. One **open follow-up for the user**: run the `gh repo create` command to publish `sunlit-kani-demo` (command provided in chat + scope report). Not a tracked GitHub issue (it is a one-shot action, not backlog).
