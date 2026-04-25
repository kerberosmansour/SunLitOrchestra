# Semgrep Rust frontend — what works in 2026

Status: Rust GA in Semgrep 2026 with 40+ Pro rules, cross-function dataflow, taint mode. Below: the primitives the rule pack relies on, per the research dossier "Semgrep Rust frontend reality in 2026".

## Capability matrix

| Primitive | Status | Confidence | Use in this pack |
|---|---|---|---|
| `pattern: unsafe { ... }` | Works | HIGH | Used for unsafe-block review rules; existing `rust/lang/security/unsafe-usage.yml` precedent |
| `pattern-inside: unsafe { ... }` | **Smoke-test required (M1)** | LOW | If smoke confirms: use directly. If smoke fails: fall back to `pattern-inside: fn $F(...) { ... unsafe { ... } ... }` |
| `pattern-either` | Works | HIGH | Variation enumeration — primary structural primitive for the pack |
| `pattern-not-inside` | Works | HIGH | Excluding `#[cfg(test)] mod tests { ... }` from production-path rules |
| `pattern-not` | Works | HIGH | Carving out benign cases when a pattern is too broad |
| `metavariable-pattern` | Works | HIGH | Combining sub-patterns inside metavariable matches |
| `metavariable-regex` | Works | HIGH | Regex-based metavariable filtering (use sparingly — prefer structural) |
| `metavariable-type` on concrete types | Works | HIGH | Documented for Rust on Semgrep experiments page |
| `metavariable-type` on Rust generics + trait bounds | **Partial** | LOW | Open issues #10380, #11150. **Avoid for v1**; fall back to `metavariable-pattern` + regex |
| Taint source → sink (basic) | Works | HIGH | Used in Semgrep's own SCA reachability for Rust |
| Taint through `format!` / intermediate `let` | **Negative** | HIGH | Open issues #10757, #10900 — confirmed FNs. Avoid taint-mode rules that rely on this |
| Macro-arg taint propagation | Works | HIGH | Added July 2023; `foo!(&x)`, `foo!(*x)` propagate |
| Patterns through proc-macros (`#[axum::debug_handler]`, `#[tokio::main]`, `#[tracing::instrument]`) | **Negative** | MEDIUM | Issues #10471, #10362, #3600, #5221. Document the limitation; macro-decorated fn bodies will under-match |
| `semgrep --validate` non-zero exit on bad YAML | Works | HIGH | Exit 5 (bad config) / 7 (invalid rule) / 4 (bad pattern) |
| `semgrep --test` non-zero exit on assertion failure | Works (with caveat) | HIGH | PR #6070 made it return 1; **caveat #10319**: returns 0 on invalid rule itself — must run `--validate` first |
| `--json` structured output | Works | HIGH | Used by all xtask subcommands per sec-2 (never substring-match raw stdout) |

## M1 smoke-test results

This section is updated by the M1 implementation step that runs the smoke-test for `pattern-inside: unsafe { ... }`.

**Status as of 2026-04-25 (M1 in flight):** smoke-test pending. Until result is recorded:

- Unsafe-bound variations in `references/sast/variations/cwe-416.md` and `cwe-787.md` use the workaround `pattern-inside: fn $F(...) { ... unsafe { ... } ... }`.
- A direct `pattern-inside: unsafe { ... }` arm is permitted in any rule but the rule MUST also include the workaround as a fallback arm.

**Expected result format** (filled by smoke-test):

```
Smoke-test: `pattern-inside: unsafe { ... }`
Run: 2026-MM-DD with semgrep 1.X.Y on macOS arm64
Outcome: <CONFIRMED WORKING | NOT WORKING — see workaround>
Detail: <short prose; reference fixture file path; observed match count>
```

## What we explicitly do NOT use

- `mode: taint` for v1 rule pack — works but adds maintenance overhead and cross-file taint requires Pro engine. Reserve for variations that genuinely cannot be enumerated structurally.
- `metavariable-type` on generic types — partial support, fragile under proc-macro expansion. Use structural `pattern-inside fn $F(...) -> Result<$T1, $T2>` instead (Trail of Bits precedent shows this works without metavariable-type).
- Pro-engine-only rules — the pack must run under Semgrep CE (LGPL-2.1).

## CLI flags the xtask depends on

- `--validate <rule>` — schema check; exit 5/7/4
- `--test <rule>` — paired-fixture fire/silent; exit 0/1
- `--config <rule> <target>` — run rule against target dir; `--json` for structured output
- `--timeout <secs>` — per-file timeout (xtask passes 30s default, min 5s clamp)
- `--max-target-bytes <N>` — per-file size cap (xtask passes 1 MB default)
- `--json` — structured output mode (xtask requires this; never substring-matches raw stdout)
- `--version` — version assertion against `MIN-SEMGREP-VERSION.md`

## Open Semgrep issues we track

Pinned 2026-04-25 (subject to upstream closure):

- semgrep#10319 — `--test` returns 0 on invalid rule (workaround: validate first)
- semgrep#10380, #11150 — `metavariable-type` on Rust generics partial
- semgrep#10471, #10362, #3600, #5221 — patterns through proc-macros
- semgrep#10757, #10900 — taint through `format!` / intermediate `let`
- semgrep#2799 — `semgrep --test` invocation surface (resolved; pinned for context)

When any of these close upstream, this file should be updated and the corresponding constraint in `manifest-schema.md` relaxed.
