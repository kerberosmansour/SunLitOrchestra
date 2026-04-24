---
name: tla-sha-autopop
created: 2026-04-24
status: design-locked
tla_required: false
tla_reason: >
  No concurrency, no distributed state, no ordering guarantee. A single-process
  helper reads a file, fetches a URL, computes a hash, writes the file back.
  TLA+ would be theater here.
---

# Design — auto-populate `tools.toml` SHA-256

## System goal

A command-line helper, `sldo-tla-sha`, that reads `skills/slo-tla/tools.toml`, fetches every pinned URL whose `sha256 = "UNSET"`, computes SHA-256 locally, and prints a patch the human applies in a commit.

## Components

```
┌──────────────────────────────────────────────────────────────────┐
│                        sldo-tla-sha (new binary)                 │
│                                                                  │
│   Read tools.toml  ──►  For each [package] with UNSET:           │
│                              │                                   │
│                              ▼                                   │
│                         Fetch <url>  ──►  Compute SHA-256        │
│                                              │                   │
│                                              ▼                   │
│                         Print patch  ──►  Human commits          │
│                                                                  │
│   External: HTTPS to github.com release-assets CDN               │
└──────────────────────────────────────────────────────────────────┘
```

## Interfaces that must remain stable downstream

- `skills/slo-tla/tools.toml` shape: `[tlc]`, `[apalache]`, fields `version`, `url`, `sha256`. These are already stable — the helper must not change them.

## Stack decision

- Rust binary in the existing workspace (`crates/sldo-tla-sha/`). Reuses `reqwest` + `sha2` deps that M5 proposed. Consistent with the rest of SLO.

## Non-negotiables

- Helper prints a patch; does NOT write the file itself. Rationale: humans commit, not tools.
- On network failure, abort with a clear message; do not leave `tools.toml` partially updated.
- When cross-verifying against a `.sha256` sibling file is possible (future), the helper must NOT accept our local computation if the sibling disagrees.
