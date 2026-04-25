# Interfaces — sast-rulegen-skill-pack

Public surfaces downstream milestones (M2 extend-mode, M3 CI + dev-env, future Runbook B TypeScript) MUST keep stable. Each entry has a stability level: `stable` (frozen, breaking change requires `/slo-architect` re-run), `evolving` (may change with explicit migration in the runbook), `internal` (fair game, not consumed across skill / xtask / runbook boundaries).

## 1. xtask binary surface — `cargo xtask sast-verify`

**Stability:** `stable` (M2-M3 and Runbook B all shell out to this).

Cargo alias declared in `.cargo/config.toml`:

```toml
[alias]
xtask = "run --package sast-verify --"
```

Resolves anywhere in the workspace.

### Subcommands

| Subcommand | Args | Exit codes | Purpose |
|---|---|---|---|
| `validate <rule-path>` | path to rule YAML or directory | `0` valid; `2` bad YAML / unknown field; `3` unknown rule shape; `4` semgrep CLI not on PATH | Runs `semgrep --validate <rule-path>`; surfaces Semgrep's exit codes 5/7/4 mapped into stable xtask exit codes |
| `test <rule-path>` | path to rule YAML (paired `.rs` fixture must exist as `<rule-id>.rs` in same dir per Semgrep upstream convention) | `0` all `// ruleid:` lines fired AND no `// ok:` line fired; `2` mismatch; `4` semgrep CLI missing; `5` paired fixture missing | Runs `semgrep --validate` first then `semgrep --test`; refuses to run `--test` on an invalid rule per issue #10319 |
| `check-coverage <rule-path>` | path to rule YAML; reads `references/sast/variations/cwe-<XXX>.md` for N | `0` `pattern-either` has ≥ N arms; `2` below minimum; `3` rule lacks `pattern-either` (single-pattern rules exempt); `6` no CWE id in rule metadata | Parses the rule YAML with `serde_yaml` strict mode, counts `pattern-either` arms, asserts ≥ N where N is the per-CWE minimum from `references/sast/variations/cwe-<XXX>.md` |
| `check-clean <rule-path> [<clean-dir>]` | rule path + optional dir; **default: `xtasks/sast-verify/tests/fixtures/clean_subset/`** (per `/slo-critique` eng-1; the host crate's `src/` may legitimately contain a real bug the rule should fire on, which would self-poison the gate) | `0` zero matches in clean-dir; `2` ≥ 1 false positive; `4` semgrep CLI missing | Runs `semgrep --config <rule> <clean-dir>`; asserts zero matches. Host-`src/` scan is opt-in via `--clean-dir src/` for "find actual unfixed bugs" use cases |
| `gate <rule-path>` | rule path | `0` ALL of validate + test + check-coverage + check-clean passed; non-zero from first failing sub-step (exit codes propagate) | Composes the four above in order; this is the entry point `/slo-rulegen` and `/slo-ruleverify` shell out to before authorising any rule write |

### Common flags (all subcommands)

- `--semgrep-bin <path>` — override `semgrep` resolution; default is `which semgrep`. Required when M3 CI uses a containerised Semgrep.
- `--timeout-secs <N>` — passed through to `semgrep --timeout`. Default 30; minimum 5 (sub-5 disabled to avoid false negatives on slow CI).
- `--max-target-bytes <N>` — passed through to `semgrep --max-target-bytes`. Default 1_000_000 (1 MB).
- `--json` — emit structured JSON to stdout instead of human-readable. Used by skills to parse outcomes programmatically.

### Exit-code envelope

Exit codes 0–7 are owned by this binary. Exit codes ≥ 64 are reserved for unrecoverable internal errors (panic, signal) and are surfaced as `anyhow::Error` chains. Skills MUST NOT branch on exit codes ≥ 64; treat as "verifier crashed, surface to user."

## 2. Rule pack on-disk layout

**Stability:** `stable` (matches Semgrep upstream convention; portable to any consumer `semgrep ci`).

Generated rules land at:

```
<base>/<lang>/<rule-id>.yaml      # the rule
<base>/<lang>/<rule-id>.rs        # paired test fixture, lines tagged // ruleid: / // ok:
```

- `<base>` defaults to `.semgrep/` (relative to repo root). Override via `--target-dir`.
- `<lang>` is `rust` for v1; `typescript` lands in Runbook B.
- `<rule-id>` is kebab-case, prefixed with the CWE id for browse-ability (e.g. `cwe-755-panic-on-result-fn`, `cwe-416-uaf-after-vec-grow`).
- No alternative layouts permitted. No per-rule subdirectory. No `tests/` parallel tree. (Per Semgrep upstream convention: paired files share a basename; only the extension differs.)

## 3. Rule YAML manifest schema

**Stability:** `stable` (consumed by `check-coverage`, by skills for output formatting).

Each rule YAML carries the standard Semgrep rule body PLUS this metadata block:

```yaml
rules:
  - id: <kebab-rule-id>
    languages: [rust]
    severity: WARNING|ERROR|INFO
    message: <one-line description, ends with a period>
    metadata:
      cwe: "CWE-<NNN>: <CWE title>"
      category: security
      confidence: HIGH|MEDIUM|LOW
      source-of-bug-shape: <RUSTSEC-id> | clippy-<lint> | manual
      sldo-rulegen-version: <git-sha-of-skills/slo-rulegen/SKILL.md-at-emit>
      sldo-variation-template: <relative path to references/sast/variations/cwe-NNN.md>
    pattern-either:
      - pattern: ...
      - pattern: ...
      - ...
    pattern-not-inside:
      - pattern: |
          #[cfg(test)]
          mod tests {
            ...
          }
```

- `cwe` field is required and MUST start with `CWE-` and a valid id from `references/sast/cwe-map-rust.md`. Invalid ids fail `check-coverage`.
- `sldo-rulegen-version` is the provenance trail — re-running `/slo-rulegen` against an evolved skill yields a fresh value, letting `/slo-ruleverify` flag rules generated by older skill revisions for re-validation.
- `sldo-variation-template` is the link to the variation-template doc; `check-coverage` reads N from it.

## 4. Skill input contract — `/slo-rulegen` extend-mode

**Stability:** `stable` (M2 ships extend-mode against this contract; M3 CI invokes the same contract).

```
/slo-rulegen --extend
  --bug-summary <path-or-stdin>
  --fix-diff <path-or-stdin>
  --file-paths <comma-separated-list>
  [--cwe <CWE-id>]                 # optional override; default is auto-derived from RustSec/GHSA join on the advisory linked from cargo-audit
  [--target-dir <path>]            # default .semgrep/
  [--target-tier confidential|public]  # forces gitignore default; default auto-detected from repo metadata
```

Output contract: writes `(N variation snippets, 1 rule with N pattern-either arms, 1 paired fixture file with N // ruleid: annotations)` and runs `cargo xtask sast-verify gate` against the new rule. Skill MUST NOT write the rule files unless `gate` exits 0; on failure, skill prints the failed sub-step's exit code and reason and asks the user to course-correct.

## 5. Skill input contract — `/slo-ruleverify`

**Stability:** `stable`.

```
/slo-ruleverify
  [<rule-path-or-glob>]            # default: scan all rules under .semgrep/<lang>/
  [--strict]                       # treat WARNING-severity check failures as errors
```

Output: exit 0 if every rule passes `gate`; exit 1 with a structured report (per-rule pass/fail) otherwise. Skill prints the rule path, sub-step that failed, and the verifier's exit-code label.

## 6. Skill toolflag contract — `sldo-common::toolflags`

**Stability:** `stable`.

New module functions in `crates/sldo-common/src/toolflags.rs`:

- `rulegen_allow_flags() -> Vec<String>` — allow Bash, Read, Write, Edit. **DENIES WebFetch, WebSearch.**
- `rulegen_deny_flags() -> Vec<String>` — explicitly listing WebFetch, WebSearch (defense-in-depth; some Claude versions check both lists).
- `ruleverify_allow_flags() -> Vec<String>` — allow Bash, Read. **DENIES Write, Edit, WebFetch, WebSearch** (verify is read-only).
- `ruleverify_deny_flags() -> Vec<String>` — listed denials.

Removing the WebFetch/WebSearch denial requires re-running the threat model. Existing `plan_*`, `run_*`, `research_*` flag families are untouched.

## 7. References scaffolding — `references/sast/`

**Stability:** `evolving` for content (CWE map, variation files), `stable` for structure.

```
references/sast/
├── README.md                       # what this dir is, how skills consume it
├── AUTHORING.md                    # rule-authoring policy (Trail of Bits AGPL clean-room rule)
├── cwe-map-rust.md                 # the top-10 Rust CWE ranking with provenance
├── semgrep-rust-syntax.md          # which Semgrep primitives work for Rust in 2026 (smoke-test results)
├── manifest-schema.md              # exact YAML schema rules must satisfy
├── MIN-SEMGREP-VERSION.md          # pinned minimum Semgrep version
├── variations/
│   ├── cwe-755.md                  # variation enumeration for panic-on-Result-fn class
│   ├── cwe-416.md                  # use-after-free variations
│   ├── cwe-697.md                  # incorrect-comparison variations
│   ├── cwe-125.md                  # OOB read variations
│   ├── cwe-787.md                  # OOB write variations
│   ├── cwe-190.md                  # integer overflow variations
│   ├── cwe-295.md                  # cert validation variations
│   ├── cwe-672.md                  # operation on resource after expiration variations
│   ├── cwe-20.md                   # input validation variations
│   └── cwe-79.md                   # XSS in Rust webapp variations (user-pain-anchored)
└── prompts/
    ├── bootstrap.md                # the prompt /slo-rulegen reads to bootstrap the pack
    └── extend.md                   # the prompt /slo-rulegen --extend reads
```

Adding a new CWE to `cwe-map-rust.md` is `evolving`-stable: it's a content change, but the structure (variation file at `variations/cwe-<NNN>.md`, minimum-N declared in that file) is `stable`.

`prompts/bootstrap.md` and `prompts/extend.md` are the exact prompt bodies the skills reference; rewriting prompt content is `evolving`, but the file paths are `stable`.

## 8. CLAUDE.md baseline test-command append

**Stability:** `stable`.

[CLAUDE.md](../../CLAUDE.md)'s baseline test command MUST append `-p sast-verify` after M1:

```bash
cargo test -p sldo-common -p sldo-plan -p sldo-run -p sldo-research -p sldo-install -p sast-verify
```

`--workspace` remains parked because `sldo-tauri` is parked. The two are coupled — re-instituting `--workspace` requires un-parking Tauri AND adding `sast-verify` to the explicit list at the same time.

## 9. Internal — not part of the contract

These are explicitly NOT stable. Skills, milestones, and Runbook B MUST NOT branch on them.

- The exact `clap 4` derive shape inside `xtasks/sast-verify/src/main.rs`. Refactoring is permitted; the subcommand contract (Section 1) is what matters.
- Internal helper functions in the verifier crate.
- The exact wording of `references/sast/prompts/bootstrap.md` and `prompts/extend.md`. Path is stable; body is evolving.
- The exact set of inline `// ruleid:` and `// ok:` annotations within fixture files. Skills produce them per `references/sast/manifest-schema.md`; the upstream Semgrep `--test` runner consumes them; nothing else should parse them.
