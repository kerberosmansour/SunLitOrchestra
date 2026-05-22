# Completion Summary — kani Milestone 4

## Goal completed
- The failure bar is **met and documented**: four Rust kernels (off-by-one bounded loop, unsafe one-past-the-end pointer read, `u32` overflow, recursive `gcd` div-by-zero) each had a deliberate bug that Kani **caught** (red), a documented remediation that made it **green**, all at stated bounds, naive-first throughout. "More stable because of Kani" is now mechanically demonstrated.

## Files changed
- External: `~/Dev/GitHub/sunlit-kani-demo` (NEW repo, commit `959b23e`) — `Cargo.toml`, `src/lib.rs` (K1–K4 + harnesses), `README.md`, `.gitignore`.
- `docs/slo/verify/kani-verification-kani.md` (NEW — scope report with red→green evidence)
- `docs/slo/design/kani-verification-overview.md` (demo repo URL + commit)
- `README.md` (Formal verification demo note)
- `skills/slo-kani/tools.toml` (pin 0.56.0 → 0.67.0; M4 allow-list extension, user-confirmed)

## Tests added
- 5 Kani harnesses in the demo crate: `check_zero_prefix`, `check_read_byte`, `check_accumulate`, `check_gcd_contract`, `check_reduce_fraction`. Each verified green; each fixed kernel had its red recorded first.

## Runtime validations added
- Live `cargo kani` runs (0.67.0) — reds and greens captured in `docs/slo/verify/kani-verification-kani.md`. Report: `docs/slo/verify/kani-m4.md`.

## Compatibility checks performed
- SLO workspace baseline (`cargo test -p sast-verify`) unaffected — the demo is a separate repo, not a workspace member (toolchain isolation held, per the code-map dangerous-seam note).

## Documentation updated
- Overview (demo URL+commit), README (demo note), scope report (new).

## .gitignore changes
- Demo repo carries its own `.gitignore` (`/target`, Kani scratch). SLO repo `.gitignore` already had Kani patterns (M1).

## Test artifact cleanup verified
- SLO repo `git status` clean apart from intended files; demo's `target/` gitignored.

## Deferred follow-ups
- **User action:** run the provided `gh repo create … --public --push` command to publish `kerberosmansour/sunlit-kani-demo` (the agent is blocked from creating a public surface by the harness auto-classifier). Commit `959b23e` is final; the push does not change it.

## Known non-blocking limitations
- K4 `ensures` proves `*r != 0` (the div-by-zero-elimination property), not full GCD correctness — chosen for fast, reliable verification.
- All proofs are bounded (e.g. K1 `length<=8`); bounded ⇒ not unbounded, disclosed in the scope report.
- The public demo repo URL resolves only after the user runs the push command.
