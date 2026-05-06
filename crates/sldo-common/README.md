# sldo-common

[![crates.io](https://img.shields.io/crates/v/sldo-common.svg)](https://crates.io/crates/sldo-common)
[![docs.rs](https://docs.rs/sldo-common/badge.svg)](https://docs.rs/sldo-common)
[![License: Apache-2.0 OR MIT](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](#license)

Shared library for the [SunLit Orchestra](https://github.com/kerberosmansour/SunLitOrchestra) (SLO) skill-pack tooling. The reusable plumbing under [`sldo-install`](https://crates.io/crates/sldo-install) and [`sldo-research`](https://crates.io/crates/sldo-research).

## When to reach for this crate

You're building a tool inside (or alongside) the SunLit Orchestra repo and need:

- A consistent **CLI logging vocabulary** (header / divider / info / success / warn) that matches the rest of the `sldo-*` toolchain.
- A **Claude CLI wrapper** for shelling out to `claude` with the right flags and capturing structured output.
- **Host detection** and host-skills-directory resolution for supported development hosts.
- **Git helpers** for the operations `sldo-install` and friends actually do (status checks, root discovery).
- **Preflight checks** before a tool starts touching the filesystem.
- **Runbook parsing helpers** for `/slo-plan`-shaped markdown.
- A **toolflags** helper for translating skill-defined CLI invariants into argv.

If you're a downstream consumer, you most likely want one of the binary crates that depend on this — not this one directly.

## Install

```toml
[dependencies]
sldo-common = "0.1"
```

## What's inside

| Module | Use it for |
|---|---|
| `claude_cli` | Wrapper for invoking the `claude` CLI with consistent flag handling and capture. |
| `color` | `header`, `divider`, `info`, `success`, `warn` — the SLO CLI logging vocabulary. |
| `detect` | Host-agent detection for supported development hosts. |
| `git` | Git operations used by `sldo-install` (root, status, file existence). |
| `logging` | `ensure_log_dir` and structured-log path helpers. |
| `preflight` | Common environment/preflight checks before a tool starts mutating state. |
| `runbook` | Markdown helpers for `/slo-plan` runbook parsing and edits. |
| `toolflags` | Translate skill-defined CLI invariants into argv vectors. |
| `version()` | Compile-time crate version string (`env!("CARGO_PKG_VERSION")`). |

## Quick example

```rust
use sldo_common::color::{header, info, success};
use sldo_common::version;

header(&format!("sldo-tool {}", version()));
info("starting work");
// ... do work ...
success("done");
```

## Compatibility

- MSRV: 1.75
- Pure Rust; depends on `anyhow` and the `colored` crate

## Status

Stable for use by `sldo-install` and `sldo-research`. Public API may evolve before 1.0 as new SLO tools land — pin to `0.1.x` when consuming.

## Related crates

Part of the [SunLit Orchestra](https://github.com/kerberosmansour/SunLitOrchestra) workspace:

| Crate | Purpose |
|---|---|
| [`sldo-install`](https://crates.io/crates/sldo-install) | Installer CLI for the SunLit Orchestra `/slo-*` skill pack. |
| [`sldo-research`](https://crates.io/crates/sldo-research) | Optional Claude batch backend for the `/slo-research` skill. |

## Getting help

- **Questions, ideas, design discussions** — open a [GitHub Discussion](https://github.com/kerberosmansour/SunLitOrchestra/discussions).
- **Bug reports** — [GitHub Issues](https://github.com/kerberosmansour/SunLitOrchestra/issues).
- **Security issues** — please do **not** open a public issue. See [SECURITY.md](https://github.com/kerberosmansour/SunLitOrchestra/blob/main/SECURITY.md) for the responsible-disclosure process.

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](https://github.com/kerberosmansour/SunLitOrchestra/blob/main/CONTRIBUTING.md) and the [Code of Conduct](https://github.com/kerberosmansour/SunLitOrchestra/blob/main/CODE_OF_CONDUCT.md) before opening a PR.

## License

Dual-licensed under [Apache-2.0](https://github.com/kerberosmansour/SunLitOrchestra/blob/main/LICENSE-APACHE) or [MIT](https://github.com/kerberosmansour/SunLitOrchestra/blob/main/LICENSE-MIT) at your option.
