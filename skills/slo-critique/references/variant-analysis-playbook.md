# Variant-analysis playbook — `/slo-critique`

When the security persona identifies a concrete instance of a bug class, the class-elimination framing demands the next question: **where else in this codebase could the same pattern exist?** That is variant analysis (Google PSC's canonical doctrine).

Three strategies scale from cheap (minutes) to deep (hours). Pick based on the class's parseability and the codebase's size.

## When variant analysis is N/A

- **Small codebase** (< 500 LOC total, excluding generated code and vendored dependencies). A single file / module can be read in full by the reviewer; grepping for siblings finds at most one or two, not enough to justify tool setup.
- **Class is already eliminated architecturally.** No variants possible — the class is impossible by construction. Record "class eliminated, no variants to hunt" in the finding and move on.
- **Class is out of scope for the runbook.** Variant analysis hunts within the milestone's allow-list plus the target repo. Hunting across unrelated systems is beyond the current runbook's scope.

In each of these, the finding records "variant-analysis: N/A — <reason>" explicitly. Silent omission of variant analysis is forbidden.

## Strategy 1 — ripgrep (fastest; structural simplicity)

Use when the bug class has an obvious textual signature: a function name, a format-string pattern, a missing-call pattern.

**Best for**: SQL injection via string interpolation (grep `format!(.*"SELECT|ORDER BY|WHERE.*{`), plaintext secret pattern (`password\s*=\s*"[^"]{6,}"`), shell-exec without arg splitting (`Command::new.*format!`), `unwrap()` on user-provided paths.

**How to run**:

```bash
# Find SQL interpolation candidates
rg -n --type rust '(format!|write!).*(SELECT|INSERT|UPDATE|DELETE|ORDER BY)' src/

# Find unwrap() directly on user-supplied paths
rg -n --type rust 'PathBuf::from\([^)]*\)\.unwrap' src/

# Find shell execution with string splicing
rg -n --type rust 'Command::new\(.*format!|Command::new\(.*&.*\.to_string' src/
```

**Worked example** — V4 SQL injection variant hunt:

> A finding identified SQL injection at `services/orders/src/sort.rs:42` (dynamic ORDER BY interpolation). Variant-analysis query:
>
> ```bash
> rg -n --type rust 'format!.*ORDER BY|format!.*" *(SELECT|UPDATE|DELETE)' services/
> ```
>
> Returns 3 additional sites: `services/inventory/src/report.rs:88`, `services/users/src/admin.rs:17`, `services/billing/src/query_builder.rs:134`. Each is a variant of the original class; the elimination pattern (`SqlIdentifier`) applies uniformly.

**Limits**: regex can't parse syntax; renames, macros, and generics may hide variants. Use as a first pass before ast-grep or semgrep.

## Strategy 2 — ast-grep (structural patterns; syntax-aware)

Use when the class has a syntactic structure but not a specific textual form: "all calls to a function with a specific argument shape", "all `match` arms missing a branch", "all `impl` blocks for a trait missing a method."

**Install**: `cargo install ast-grep` or `brew install ast-grep`. Native Rust, MIT-licensed, tree-sitter-based. SARIF output via `--json-compact-with-summary` since 0.40.0.

**Best for**: missing authorization checks (all handler signatures with `AuthContext` parameter vs. all handler signatures period), deserialization into a type without `deny_unknown_fields`, all `Command::new` invocations where the arg is a user-input-derived string.

**How to run**:

```bash
# Find all serde derive blocks missing deny_unknown_fields
ast-grep -p 'struct $NAME { $$$ }' --lang rust src/ \
  | ast-grep filter 'not #[serde(deny_unknown_fields)]'

# Find all Command::new calls with dynamic arg0
ast-grep -p 'Command::new($FUNC)' --lang rust src/
```

**Worked example** — missing `deny_unknown_fields` variant hunt:

> Finding identified mass-assignment risk on `CreateUserDto` (no `deny_unknown_fields` on the serde derive). Variant-analysis:
>
> ```bash
> ast-grep --pattern 'struct $NAME { $$$ }' --lang rust src/ | rg -L 'deny_unknown_fields'
> ```
>
> Surfaces 7 other DTOs without the attribute. Four are legitimate (internal-only), three expose POST endpoints — all three get `#[serde(deny_unknown_fields)]`. Class now eliminated at the workspace level.

**Limits**: rule ecosystem is smaller than Semgrep's; lacks taint-flow analysis; patterns are structural only.

## Strategy 3 — Semgrep (taint flow; cross-file)

Use when the class requires tracking data flow across functions: "user input reaching a subprocess invocation", "unvalidated input reaching a filesystem operation", "secret logged via `log::info!`."

**Install**: `pip install semgrep` or `brew install semgrep`. LGPL-2.1 core post-Dec 2024 relicense; pro rules are gated. For Rust cross-file, Semgrep's pro engine is required; CE handles intra-file analysis. Opengrep (LGPL fork, January 2025) is a hedge if commercial gating becomes a blocker.

**Best for**: SSRF taint (user input → HTTP client), path-traversal taint (user input → `fs::*`), credential-to-log taint (`SecretString` → log macros).

**How to run**:

```bash
# Use the auto config for quick wins
semgrep scan --config=auto --sarif --sarif-output=security-findings.sarif src/

# Write a custom rule for a specific taint path
cat > /tmp/rule.yaml <<'EOF'
rules:
  - id: ssrf-from-user-input
    message: "User input reaches reqwest without SafeUrl validation"
    severity: ERROR
    languages: [rust]
    patterns:
      - pattern-inside: |
          fn $F(...) { ... }
      - pattern: reqwest::get($URL)
      - pattern-not: reqwest::get($URL as SafeUrl)
EOF
semgrep scan --config=/tmp/rule.yaml src/
```

**Worked example** — SSRF variant hunt across services:

> Finding identified SSRF at `webhooks/src/outbound.rs:55` (user-supplied URL reaches `reqwest::get` without `SafeUrl`). Custom rule written; `semgrep scan` across the monorepo surfaces 2 additional sites in `integrations/src/*` and 1 in `admin/src/export.rs`. Three additional findings get `SafeUrl::try_from(..)?` prepended to the `reqwest::get` call; class eliminated.

**Limits**: SARIF output from Semgrep CE does not include every pro-rule finding; cross-file taint for Rust specifically is gated. Budget ~10 s for a medium repo on the auto config; hours for complex custom rules.

## Picking a strategy

| Signal | Strategy |
|---|---|
| Bug class has a textual signature; you can grep for it with < 30 min of effort | ripgrep |
| Bug class has a syntactic structure but not a regex; tree-sitter can match the shape | ast-grep |
| Bug class requires data-flow / taint tracking across calls | Semgrep (CE for intra-file; pro / Opengrep for cross-file) |
| Class is already eliminated architecturally | N/A — no variants possible |
| Codebase is too small to have meaningful variants (< 500 LOC) | N/A — small codebase exit |

## Integration with the finding

Every accepted security finding includes a **variant-analysis pointer** in one of two shapes:

1. **"Variants found: &lt;N&gt; sites; see &lt;file&gt;:&lt;line&gt;, &lt;file&gt;:&lt;line&gt;, ..."** — when variant analysis was run and surfaced siblings.
2. **"Variant analysis N/A — &lt;reason&gt;"** — when it was skipped per the rules above.

Silent omission (no variant-analysis line at all) is forbidden. The finding row schema in `skills/slo-critique/SKILL.md` does not call this out explicitly as a column; the security persona's prompt enforces it prose-side.

## Anti-patterns

- **Running variant analysis without first eliminating the specific instance.** The point is to find siblings of a known-bad pattern; hunting for a pattern that hasn't been identified is fishing.
- **Claiming "no variants" without running any tool.** Unless the small-codebase or class-eliminated exit applies, the reviewer runs at least ripgrep. "No variants found" must be defensible.
- **Accepting tool output as the full answer.** ripgrep regex misses renames; ast-grep patterns miss macro expansions; Semgrep pro rules miss what the LGPL CE can reach. Variant analysis is a surface-reduction technique, not a proof.
- **Using variant analysis to justify scope creep.** If the tool finds 50 variants across unrelated services, the response is to record them and scope them into separate milestones / runbooks — not to fix them all in the current one. The allow-list rule still governs.
