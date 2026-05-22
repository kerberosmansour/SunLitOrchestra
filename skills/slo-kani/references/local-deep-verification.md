# Local Deep Verification — running Kani on the developer's laptop

v1 decision: **Kani proofs run locally**, not in CI. There is no nightly/PR Kani
CI job (deferred to a future iteration). Deep verification is a deliberate,
time-budgeted local step the developer runs before a release.

## Two tiers

### Quick tier (inner loop)

Small bounds, fast (seconds). Run constantly while iterating on a kernel or its
harness.

```bash
cargo kani --harness check_foo            # default small bounds
cargo check                               # compile the #[cfg(kani)] harnesses so they don't rot
```

- Arrays 8–16, vecs 0–2, `unwind` = iterations + 1.
- Goal: catch the obvious bug class immediately; keep the edit-prove loop tight.

### Deep tier (before release)

Larger bounds and the full harness set, run on the laptop **before** tagging a
release. Slower (minutes); not run on every edit.

```bash
cargo kani                                              # all harnesses
cargo kani --harness check_loop --unwind 17             # deeper bound
cargo kani -Z function-contracts --harness check_contract
cargo kani -Z function-contracts -Z stubbing --harness check_caller
```

- Escalate bounds deliberately (e.g. arrays 32) and record runtime.
- **The deep tier must run green before any release tag.** A build is not
  "release-ready" until the deep tier passes; record the verdict + bounds in
  `docs/slo/verify/<slug>-kani.md`.

## Pinned toolchain (reproducibility + supply chain)

Always run the version pinned in [`../tools.toml`](../tools.toml) — currently
`kani-verifier 0.67.0` — installed with `cargo install --locked`. Do not run
`latest`: the run/triage output parser is anchored to the pinned version, and a
pinned toolchain makes the deep-tier result reproducible. Confirm with
`cargo kani --version` before a release run.

## Why local, not CI (v1)

Kani proofs can be slow (the deep tier takes minutes; recursion-heavy kernels
more). Putting deep proofs on every PR would either block developers or get
deleted. Running them locally keeps the team in control of when the slow proofs
run and avoids standing up CI infrastructure before the skill has earned its
keep. CI automation (a quick-PR job + a scheduled deep job) is a documented
future enhancement, not a v1 deliverable.
