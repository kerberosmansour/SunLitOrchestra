# sldo-research

[![crates.io](https://img.shields.io/crates/v/sldo-research.svg)](https://crates.io/crates/sldo-research)
[![docs.rs](https://docs.rs/sldo-research/badge.svg)](https://docs.rs/sldo-research)
[![License: Apache-2.0 OR MIT](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](#license)

Optional Claude batch backend for the [SunLit Orchestra](https://github.com/kerberosmansour/SunLitOrchestra) `/slo-research` skill — produces sourced research dossiers ready to feed into `/slo-plan`.

## When to reach for this crate

You're using the SunLit Orchestra skill pack and you want `/slo-research` to produce **structured, citation-bearing research dossiers** with iterative deepening, instead of a single one-shot reply. `sldo-research` runs Claude as a batch backend with web search, multiple deepening passes, and a markdown-output convention that `sldo-plan` knows how to consume.

Use this if you want:

- Iterative research with **bounded deepening** (`--max-iterations`, default 3).
- **Bounded web searches** per run (`--max-searches`, default 5).
- A repeatable **markdown dossier** format (default output: `output/research-dossier.md`).
- **Repo-aware** research grounded in the contents of your project (`--repo-dir`).
- A configurable **rate-limit cooldown** between deepening passes so Claude API limits have time to recover.

Skip this crate if your hosting agent's built-in research is enough — `/slo-research` will use it without `sldo-research` installed.

## Install

```bash
cargo install sldo-research
```

You'll also need the `claude` CLI on `PATH` (the binary backend `sldo-research` shells out to).

## Quick start

```bash
# Run from a prompt file:
sldo-research path/to/prompt.md --repo-dir . --output output/dossier.md

# Or inline prompt:
sldo-research --prompt "Investigate Sigstore signing options for cargo crates" \
              --repo-dir . \
              --output output/dossier.md \
              --max-iterations 3 \
              --max-searches 5
```

The output is a structured markdown dossier with sections, citations, and a final summary — designed to be passed to `/slo-plan` as research context.

## CLI flags

| Flag | Default | Purpose |
|---|---|---|
| `<PROMPT_FILE>` | — | Path to a file containing the research prompt. |
| `--prompt <STRING>` | — | Inline research prompt (alternative to the prompt file). |
| `--repo-dir <DIR>` | — | Target repository to ground the research in. |
| `--output <PATH>` / `-o` | `output/research-dossier.md` | Output dossier path. |
| `--model <NAME>` / `-m` | `claude-opus-4-7` | Claude model to use. |
| `--max-iterations <N>` | `3` | Maximum research deepening iterations. |
| `--max-searches <N>` | `5` | Maximum web search invocations. |

## How it fits into `/slo-research`

`/slo-research` is host-native by default — the host agent (Claude Code, Codex, Copilot) does the research itself. `sldo-research` is the **optional batch backend** invoked when the skill author or user wants Claude-driven research with explicit deepening and bounded searches. The dossier it produces is the same shape `/slo-research` expects.

## Compatibility

- MSRV: 1.75
- macOS, Linux, Windows
- Requires the `claude` CLI on `PATH`

## Status

Stable for use with the SunLit Orchestra skill pack. Output schema may evolve as `/slo-plan` consumption rules change.

## Related crates

Part of the [SunLit Orchestra](https://github.com/kerberosmansour/SunLitOrchestra) workspace:

| Crate | Purpose |
|---|---|
| [`sldo-install`](https://crates.io/crates/sldo-install) | Installer CLI for the `/slo-*` skill pack. |
| [`sldo-common`](https://crates.io/crates/sldo-common) | Shared library used by this CLI and `sldo-install`. |

## Getting help

- **Questions, ideas, design discussions** — open a [GitHub Discussion](https://github.com/kerberosmansour/SunLitOrchestra/discussions).
- **Bug reports** — [GitHub Issues](https://github.com/kerberosmansour/SunLitOrchestra/issues).
- **Security issues** — please do **not** open a public issue. See [SECURITY.md](https://github.com/kerberosmansour/SunLitOrchestra/blob/main/SECURITY.md) for the responsible-disclosure process.

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](https://github.com/kerberosmansour/SunLitOrchestra/blob/main/CONTRIBUTING.md) and the [Code of Conduct](https://github.com/kerberosmansour/SunLitOrchestra/blob/main/CODE_OF_CONDUCT.md) before opening a PR.

## License

Dual-licensed under [Apache-2.0](https://github.com/kerberosmansour/SunLitOrchestra/blob/main/LICENSE-APACHE) or [MIT](https://github.com/kerberosmansour/SunLitOrchestra/blob/main/LICENSE-MIT) at your option.
