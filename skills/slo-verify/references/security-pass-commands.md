# Pass 4 — security command reference for `/slo-verify`

Pass 4 runs supply-chain + variant-analysis + (conditional) DAST checks. This file names the commands. The skill's SKILL.md describes *when* Pass 4 runs; this file describes *what*.

## Core rules (apply to every command below)

- **Tool-optional.** If the named binary is not on PATH, Pass 4 emits a row `skipped — <tool> not installed (see <install-hint>)`, not a failure. Missing tools do not fail Pass 4.
- **Tool-error vs. finding.** Each command documents its exit-code contract. `exit 0` = clean; `exit 1` = finding; `exit ≥ 2` = tool error or unreachable advisory DB / network failure → maps to `skipped — <tool> unreachable, <reason>`, never to a finding. This is load-bearing for offline / air-gapped / flaky-network sessions: phantom CVE regression tests must not be auto-generated.
- **Polyglot rule.** When multiple stacks are detected (Cargo.toml + package.json, Cargo.toml + pyproject.toml, etc.), Pass 4 runs *all* applicable command sets. Each stack gets its own row in the Pass 4 section of the verification report. No arbitrary tiebreaker — the report surfaces everything.
- **DAST conditional.** DAST runs only when a runnable smoke / reference service is detected (e.g., `crates/secure_smoke_service/`, an OpenAPI spec at a conventional path, a `docker-compose.yml` exposing a service). On markdown-only / library-only targets, DAST is N/A with reason.
- **Interactive budget.** Pass 4 aims for ≤ 2 min total on a small milestone's changed files. If a command exceeds the budget, it is deferred to a nightly cadence (documented per-command).
- **No shell splicing.** All commands below use argument-typed invocation (e.g., `Command::new("cargo").arg("audit")`); milestone-derived strings go through `SafeCommandArg` or an explicit argv, never `format!` into a `sh -c ...` string. Attack class: V4 command injection via milestone name.
- **No fetching at verify time.** If a command needs an advisory DB (e.g., `cargo audit`), the host is expected to have a recent copy. Users on flaky networks should run `cargo audit --db <local-path>` or accept the "skipped — DB fetch failed" row.

## Rust stack

### `cargo audit`

Scans `Cargo.lock` against the RustSec advisory DB.

```
cargo audit
```

- Exit 0: no advisories.
- Exit 1: one or more advisories found. Report each in the verification section, open a regression test per advisory, hand to `/slo-execute` for fix.
- **Exit 2 or higher**: tool error or advisory DB fetch failure (network down, stale cache, etc.). Emit row `skipped — cargo audit unreachable (network / stale DB)`. Never mapped to a finding. Offline mitigations: `cargo audit --db <local-path>` or `cargo audit --stale`.
- Install hint: `cargo install cargo-audit`.

### `cargo deny check`

Verifies license compliance, banned crates, source registries, and git-source policy.

```
cargo deny check
```

- Exit 0: policy satisfied.
- Exit 1: policy violation. Reported same as `cargo audit`.
- Exit ≥ 2: tool error (missing `deny.toml`, config parse error). Emit `skipped — cargo deny config error <details>`, surface the config issue to the user, do not count as finding.
- Install hint: `cargo install cargo-deny && cargo deny init` (first run).

### `ast-grep scan`

Structural search across Rust source for known bug-class patterns.

```
ast-grep scan --rule-dirs skills/slo-verify/rules/rust --json-compact-with-summary
```

- Emits SARIF-compatible JSON; findings reported per-rule.
- Exit 0: no matches.
- Exit 1: matches found.
- Exit ≥ 2: tool error. Skipped.
- Install hint: `cargo install ast-grep` or `brew install ast-grep`. Version 0.40.0+ for SARIF output.

### `cargo clippy -- -D warnings` (optional, informational)

Security-adjacent lints as a supplementary signal. Not a Pass 4 gate on its own; surfaced in the report alongside the supply-chain / variant-analysis rows.

## Node / TypeScript stack

### `npm audit --json --audit-level=high`

```
npm audit --json --audit-level=high
```

- Exit 0: no advisories at or above `high`.
- Exit 1: advisories found.
- Exit ≥ 2: tool error / network. Skipped.
- No install hint needed — ships with `npm`.

### Semgrep

Covers Node / TS / JS cross-stack.

```
semgrep scan --config=auto --sarif --sarif-output=output/pass4-semgrep.sarif
```

- Exit 0: no findings.
- Exit 1: findings.
- Exit ≥ 2: tool error. Skipped.
- Install hint: `pip install semgrep` or `brew install semgrep`.

## Python stack

### `pip-audit`

```
pip-audit --format json
```

- Exit 0: no advisories.
- Exit 1: advisories found.
- Exit ≥ 2: tool error. Skipped.
- Install hint: `pip install pip-audit`.

## Go stack

### `govulncheck`

```
govulncheck ./...
```

- Exit 0: no vulns.
- Exit 3: vulns found.
- Exit ≥ 1 (other): tool error. Skipped.
- Install hint: `go install golang.org/x/vuln/cmd/govulncheck@latest`.

## Any stack — DAST (conditional, through zaprun)

DAST runs only when the target has authorization, in-scope URL(s), a runnable smoke/reference service, and enough route/OpenAPI/auth context to make the scan meaningful. Use `/slo-dast-tuner`, which operates ZAP through `zaprun` and the approved digest-pinned image. Pass 4 should record the `/slo-dast-tuner` command/evidence path, not teach direct scanner invocation.

Selector:

- HTTP route with OpenAPI or route inventory + smoke service: run `/slo-dast-tuner` and `zaprun` guided/full lanes as appropriate.
- Authenticated route: require configured auth and logged-in verification. An unauthenticated scan of authenticated attack surface is a coverage failure, never a clean result.
- Pure library / markdown-only / no smoke service: DAST is `N/A - no smoke service`; use unit, abuse, SAST, variant, and supply-chain checks.
- SPA / DOM-XSS: use the `/slo-dast-tuner` PTK/DOM-XSS lane only after confirming a PTK-capable image.

Artifact expectation: `zaprun` evidence such as guided map, summary, coverage, SARIF, observations, and auth diagnostics when applicable.

## Target-repo `.gitignore` snippet

Users can paste the following into the TARGET repo's `.gitignore` to ensure Pass 4 output artifacts do not leak into version control. **Do NOT add these patterns to SunLitOrchestra's own `.gitignore`** — SLO is not typically the target of Pass 4.

```
# Pass 4 security outputs (security-pass-commands.md)
output/pass4-*.sarif
output/zaprun-*
output/zap-report.*
.semgrep/
.ast-grep/
```

## Polyglot example

A target with Cargo.toml at root + a `package.json` under `ui/` runs:

- Rust row: `cargo audit` + `cargo deny check` + `ast-grep scan`
- Node row: `npm audit --json --audit-level=high` + Semgrep
- Any-stack: Semgrep (if not already counted in Node row, run once deduplicated)
- DAST (conditional): if a smoke service is detected in either stack, one `/slo-dast-tuner` / `zaprun` row.

Each row appears independently in the Pass 4 section; no tiebreaker, no merge.

## Bug-found flow (reused from passes 1–3)

Pass 4 does not invent a new flow. When a command returns a finding:

1. STOP — do not fix inline.
2. Write a regression test that reproduces the finding. Commit separately.
3. Hand the fix back to `/slo-execute` (or a human) — the same bug-found flow passes 1–3 use.
4. Re-run Pass 4; the finding should no longer appear.
5. Re-run passes 1–3 on the same milestone to confirm no regressions.

This is the existing `/slo-verify` discipline from the three-pass structure. Pass 4 extends the passes covered; the discipline is unchanged.
