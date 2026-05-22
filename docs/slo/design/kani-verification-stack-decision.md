# Stack Decision — kani-verification

## Chosen stack

- **Skill body:** pure Markdown `SKILL.md` + method-dispatch reference files under `skills/slo-kani/references/` + `skills/slo-kani/evals/` (mirror of `/slo-tla`'s structure). No new Rust binary — the skill drives an external subprocess.
- **Verification engine:** [Kani](https://github.com/model-checking/kani) via the `cargo kani` cargo subcommand, installed from the `kani-verifier` crate, **version-pinned** in `skills/slo-kani/tools.toml` (mirrors `slo-tla/tools.toml`'s pinned-artifact discipline).
- **Tool acquisition:** `cargo install --locked kani-verifier@<pinned>` followed by `cargo kani setup` (downloads the verification bundle). Prereq cascade checks `which cargo-kani` / `cargo kani --version` before any proof run.
- **Structural-contract test:** a Rust test in `xtasks/sast-verify/tests/` (the established home for skill structural tests) that asserts `/slo-kani` frontmatter shape, output-path safety, and reference-file presence — modeled on `sap_imp_m5_agents.rs`.
- **Demo crate (M4):** a **separate GitHub repo** (user-created) holding a small Rust crate with `#[cfg(kani)]`-gated harnesses and seeded bugs.

## Reason

The research report is unambiguous that Kani's unit of work is `cargo kani` over `#[cfg(kani)]`-gated parameterless `#[kani::proof]` harnesses, installed via `cargo install --locked kani-verifier` + `cargo kani setup`. The skill therefore must *drive* that toolchain, not reimplement it — exactly how `/slo-tla` drives `tla2tools.jar` rather than reimplementing TLC. Reusing `/slo-tla`'s proven structure (prereq cascade, pinned `tools.toml`, method-dispatch references, hard gates against overclaiming) is the lowest-risk path and keeps the two formal-methods skills consistent for users. A separate demo repo (per the user's decision) keeps Kani's nightly-toolchain and `cargo kani setup` bundle out of the main workspace's CI critical path while still proving the failure bar end-to-end.

## Rejected alternatives

- **A new `sldo-kani` Rust crate that shells out to `cargo kani` and parses output** — rejected: adds a maintained binary and a published-crate version-discipline burden ([[project_crates_io_published]]) for what is fundamentally agent-driven judgment (candidate scoring, harness authoring, counterexample interpretation). The 2026-04 cleanup deliberately removed the legacy CLIs in favor of skills as the canonical interface; a new CLI would reverse that.
- **Fold Kani guidance into the existing `/slo-tla` skill** — rejected: TLA+ is design-level (interleavings, protocol); Kani is code-level (bounded sequential kernels). Different suitability gates, different tools, different out-of-scope boundaries (TLA+ *is* concurrency; Kani *excludes* it). Conflating them would make both suitability gates muddier, not clearer.
- **Demo crate inside this workspace (`examples/` or a workspace member)** — rejected per user decision: Kani needs a specific nightly toolchain + a `cargo kani setup` bundle download; making it a workspace member risks coupling the main `cargo test` baseline to Kani's toolchain. A separate repo isolates that.

## Non-negotiables (downstream cannot change these without migration)

- `/slo-kani` is a **host-neutral** skill under `skills/slo-kani/` discovered by `discover_skills()` and installed by `sldo-install` on every host (same tier as `/slo-tla`). It is NOT a Claude-only `agents/` addition.
- Kani version is **pinned** in `skills/slo-kani/tools.toml`; the prereq cascade refuses to "try it anyway" on a missing/mismatched toolchain (same discipline as `slo-tla`'s SHA-checked jar).
- A green Kani run is reported **only** with its proof scope (bounds, preconditions, stubs, contracts, excluded features). No whole-system claims. No concurrency/interleaving claims.
- The `kani_required` frontmatter key is **additive** to `<slug>-overview.md`; default-when-absent is `false`, so existing design docs without the key remain valid.
