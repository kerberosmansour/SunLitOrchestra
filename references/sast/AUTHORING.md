# Rule-authoring policy — SAST rule-gen pack

## Trail of Bits AGPL clean-room rule

The Trail of Bits `semgrep-rules` repo (https://github.com/trailofbits/semgrep-rules) is **AGPL-3.0**. Copy-pasting any rule YAML from it would inherit AGPL into the consuming project.

**Policy: structural inspiration only — no YAML text copied.**

The single most-cited Trail of Bits precedent is `rs/panic-in-function-returning-result.yaml`, which establishes:

- The "one rule with N `pattern-either` arms" shape for variation enumeration.
- The use of `pattern-not-inside #[cfg(test)] mod tests { ... }` to exclude test-block panics.
- The CWE tag `CWE-755 — Improper Handling of Exceptional Conditions` for the panic-DoS class (instead of CWE-248).
- That `metavariable-type` is NOT required for Result-type discrimination — purely structural patterns work.

These four shape decisions are functional content (likely uncopyrightable), but the YAML *text* is AGPL. Authors of new rules in this pack:

1. **Read the Trail of Bits precedent for shape inspiration only.** Note the structural pattern; close the file.
2. **Author a fresh rule from the variation template** at `references/sast/variations/cwe-<NNN>.md`. The template declares the sink shapes and minimum arm count; the author writes the YAML from scratch.
3. **Cross-check the result against the precedent only at the structural level** ("does my rule cover the same fn-context shapes?"). Never compare YAML text side-by-side and copy a snippet.
4. **`/slo-critique` security persona spot-checks** new rules for AGPL-shaped textual similarity during review.

If a rule cannot be authored without textual reference to the AGPL source, that's a sign the variation template at `variations/cwe-<NNN>.md` needs more guidance — improve the template, then re-author.

## Rule style guide

### Required metadata block

Every rule MUST include this metadata. The xtask's `check-coverage` parses these fields strictly.

```yaml
metadata:
  cwe: "CWE-<NNN>: <CWE Title>"   # MUST start "CWE-" + numeric id; cited from references/sast/cwe-map-rust.md
  category: security
  confidence: HIGH | MEDIUM | LOW
  source-of-bug-shape: <RUSTSEC-id> | clippy-<lint> | manual
  sldo-rulegen-version: <git-sha-of-skills/slo-rulegen/SKILL.md-at-emit>
  sldo-variation-template: references/sast/variations/cwe-<NNN>.md
```

### Variation enumeration via `pattern-either`

Per the synthesis design rule, variation coverage is one rule with N `pattern-either` arms paired with one fixture file with N `// ruleid:` annotations. Splitting one logical rule into N rule files breaks the manifest schema and `check-coverage`'s minimum-N enforcement.

Authoring template:

```yaml
rules:
  - id: cwe-<NNN>-<short-name>
    languages: [rust]
    severity: WARNING
    message: <one-line description; ends with a period>
    metadata: { ... required block ... }
    pattern-either:
      - pattern: <variation 1 — sink shape from variations/cwe-<NNN>.md sink_shapes list>
      - pattern: <variation 2>
      - pattern: <variation 3>
      - pattern: <variation 4>
    pattern-not-inside: |
      #[cfg(test)]
      mod tests { ... }
```

### `// ruleid:` and `// ok:` annotations in fixtures

Each fixture file (`<rule-id>.rs`) places annotations on the line ABOVE the bad code or good code:

```rust
fn handler(body: &str) -> Result<User, Error> {
    // ruleid: cwe-755-panic-on-result-fn
    let user: User = serde_json::from_str(body).unwrap();
    Ok(user)
}

fn handler_safe(body: &str) -> Result<User, Error> {
    // ok: cwe-755-panic-on-result-fn
    let user: User = serde_json::from_str(body)?;
    Ok(user)
}
```

`semgrep --test` reads these; the fixture must contain at least one `// ruleid:` line per `pattern-either` arm.

### Avoiding false positives in `check-clean`

`cargo xtask sast-verify check-clean <rule>` defaults to scanning `xtasks/sast-verify/tests/fixtures/clean_subset/`. If your rule fires on a benign pattern present in that directory, the gate rejects it. Either:

- Tighten the pattern (probably the rule is too broad), or
- Carve out the benign case with `pattern-not-inside` / `pattern-not`.

NEVER work around a `check-clean` failure by removing the offending file from the clean subset. The clean subset is intentionally small and curated; widening it weakens the gate.

### What NOT to do

- Don't copy YAML text from `trailofbits/semgrep-rules` (AGPL).
- Don't author a rule whose `pattern-either` arms all cover the same sink shape — `check-coverage` count passes but `cwe_<NNN>_rule_covers_documented_variation_shapes` (eng-2 content coverage) fails.
- Don't use `metavariable-type` on Rust generics for v1 — partial support per Semgrep open issues; use structural `pattern-inside` instead.
- Don't use `pattern-inside: unsafe { ... }` until the M1 smoke-test in `references/sast/semgrep-rust-syntax.md` confirms the primitive works.
