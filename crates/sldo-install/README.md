# sldo-install

[![crates.io](https://img.shields.io/crates/v/sldo-install.svg)](https://crates.io/crates/sldo-install)
[![docs.rs](https://docs.rs/sldo-install/badge.svg)](https://docs.rs/sldo-install)
[![License: Apache-2.0 OR MIT](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](#license)

Installer CLI for the [SunLit Orchestra](https://github.com/kerberosmansour/SunLitOrchestra) `/slo-*` skill pack — installs into supported host skill roots.

## When to reach for this crate

You want to use the SunLit Orchestra `/slo-*` skill pack inside a supported interactive development host, and you need to **link the skills under `skills/` in the repo into the host's skills directory** without copying or hand-editing config.

Key properties:

- **Idempotent** — running `sldo-install install` twice leaves the same state as running it once.
- **Reversible** — every change is recorded in a manifest; `uninstall` reverses each one.
- **Multi-host** — pick the target host per invocation.
- **Project-local or global** — install into your global `~/.claude/skills/` or a project-local `./.claude/skills/`.
- **Dry-run friendly** — `--dry-run` shows exactly what would be linked without writing.
- **Graphify-aware** — `sldo-install graphify` checks whether the Graphify CLI or a local source checkout is ready for SLO evidence-loop work, and prints macOS/Linux/Windows install commands without downloading anything itself.

## Install

Get the binary:

```bash
cargo install sldo-install
```

Or, if you've cloned the SunLit Orchestra repo, run it directly from the workspace:

```bash
cargo run -p sldo-install -- install
```

## Quick start

```bash
# Install into the global Claude Code skills directory (~/.claude/skills/).
sldo-install install

# Install into a project-local GitHub Copilot skills directory.
sldo-install --host github-copilot --local install

# These are SLO installer compatibility root paths; current host-native
# project skill roots may differ (for example `.github/skills` or
# `.agents/skills`).

# Show what would happen without making changes.
sldo-install --dry-run install

# See what's currently installed (according to the manifest).
sldo-install status

# Verify the on-disk state still matches the manifest.
sldo-install verify

# Reverse every change recorded in the manifest.
sldo-install uninstall

# Check Graphify readiness for graph-backed knowledge/security/troubleshooting workflows.
sldo-install --host codex graphify

# Print the Graphify install plan without requiring a skills/ directory.
sldo-install --host codex --local graphify --install-plan
```

## Subcommands

| Subcommand | What it does |
|---|---|
| `install` *(default)* | Links every skill subdirectory under `--skills-dir` into the host's skills directory. Records the changes in a manifest. |
| `uninstall` | Reverses every change recorded in the manifest. |
| `status` | Prints what's currently installed according to the manifest. |
| `verify` | Confirms that on-disk managed links match the manifest and the source skills (catches drift). |
| `graphify` | Checks Graphify CLI/source-checkout readiness and prints host-aware install instructions for Graphify. Does not install or download Graphify. |

## Common flags

| Flag | Purpose |
|---|---|
| `--host <HOST>` | Target host agent. Default: `claude-code`; run `sldo-install --help` for accepted values. |
| `--local` | Install into the project-local host skills directory (`./.claude/skills/` etc.) instead of the global host root. |
| `--skills-dir <DIR>` | Source directory containing skill subdirectories (each with a `SKILL.md`). Default: `./skills`. |
| `--force` | Overwrite existing managed links. |
| `--dry-run` | Show what would be done without writing any files. |

## Compatibility

- MSRV: 1.75
- macOS, Linux, Windows
- Hosts: Claude Code and GitHub Copilot, plus the other host roots supported by `sldo-install --help`
- Existing compatibility root paths include `~/.copilot/skills`, `./.copilot/skills`, `~/.codex/skills`, and `./.codex/skills`; they remain stable until an explicit migration is shipped.
- Graphify readiness checks work without a SunLit Orchestra checkout and print OS-specific commands. The recommended Graphify package is `graphifyy`; the command it installs is `graphify`.

## Status

Stable for the supported hosts.

## Related crates

Part of the [SunLit Orchestra](https://github.com/kerberosmansour/SunLitOrchestra) workspace:

| Crate | Purpose |
|---|---|
| [`sldo-common`](https://crates.io/crates/sldo-common) | Shared library used by this CLI and `sldo-research`. |
| [`sldo-research`](https://crates.io/crates/sldo-research) | Optional Claude batch backend for the `/slo-research` skill. |

## Getting help

- **Questions, ideas, design discussions** — open a [GitHub Discussion](https://github.com/kerberosmansour/SunLitOrchestra/discussions).
- **Bug reports** — [GitHub Issues](https://github.com/kerberosmansour/SunLitOrchestra/issues).
- **Security issues** — please do **not** open a public issue. See [SECURITY.md](https://github.com/kerberosmansour/SunLitOrchestra/blob/main/SECURITY.md) for the responsible-disclosure process.

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](https://github.com/kerberosmansour/SunLitOrchestra/blob/main/CONTRIBUTING.md) and the [Code of Conduct](https://github.com/kerberosmansour/SunLitOrchestra/blob/main/CODE_OF_CONDUCT.md) before opening a PR.

## License

Dual-licensed under [Apache-2.0](https://github.com/kerberosmansour/SunLitOrchestra/blob/main/LICENSE-APACHE) or [MIT](https://github.com/kerberosmansour/SunLitOrchestra/blob/main/LICENSE-MIT) at your option.
