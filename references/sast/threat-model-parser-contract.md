# Threat-model parser contract

> The parse contract `/slo-sast` follows for extracting CWE references from `docs/slo/design/<slug>-threat-model.md`. Locked in M1 of the [scanner-orchestration runbook](../../docs/slo/completed/RUNBOOK-SCANNER-ORCHESTRATION.md). Cited by [`skills/slo-sast/SKILL.md`](../../skills/slo-sast/SKILL.md). Reference document — read by humans and by the agent driving `/slo-sast`.

## Regex

```
\bCWE-(\d+)\b
```

Case-sensitive. Matches the literal string `CWE-` followed by ≥ 1 ASCII digit. Word-boundary anchors prevent matches inside identifiers (`MyCWE-123Class` does not match — no `\b` between `y` and `C`; `(see CWE-89)` matches because `(` is a word boundary).

The capture group yields the integer-form CWE id.

## Output form

The parser emits long-form `"CWE-<integer>"` strings — e.g., a match against `CWE-89` emits `"CWE-89"`. Note: the canonical Semgrep registry tag form is `"CWE-89: Improper Neutralization of Special Elements..."`, but the M1 parser only emits the `"CWE-<integer>"` prefix. Downstream milestones (M2 onward) match this prefix against the registry's `metadata.cwe` field, which contains the full long form — prefix-match is sufficient because each registry rule's `metadata.cwe` is a single integer per entry.

Outputs are deduplicated and sorted ascending by integer value.

## Three exclusion regions

The parser MUST NOT extract CWE references from any of the following regions. CWE strings inside these regions are user content / non-prose and would bias rule selection without the threat-model author's intent.

### 1. HTML comments

```html
<!-- CWE-79 -->
<!-- This bug pattern is conceptually similar to CWE-89, though we don't fear it here. -->
<!--
multi-line
CWE-100 reference
inside the comment
-->
```

Standard Markdown HTML-comment syntax: `<!--` opens, `-->` closes, possibly spanning multiple lines. The entire region from `<!--` through `-->` is excluded — no CWE references picked up regardless of line position.

### 2. Fenced code blocks

````markdown
```
let cwe_to_avoid = "CWE-99";  // not authoritative — this is sample code
```

~~~yaml
metadata:
  cwe: CWE-101
~~~
````

GitHub-flavored Markdown supports both ```` ``` ```` and `~~~` opening fences. Per CommonMark, the opening fence must be at the start of a line (column 0, possibly with up to 3 spaces of leading whitespace). Indented code blocks (4-space indent at the start of a line) are also excluded.

The fence-language tag (` ```rust `, ` ```yaml `) does not affect exclusion — code in any language is excluded.

### 3. `~~~text` user-string fences (special case of #2)

```
~~~text
User-supplied paste. The user mentioned CWE-77 here in passing — this is THEIR
content, not the project's claimed CWE list.
~~~
```

This form is the [slo-security-embedding](../../docs/slo/design/slo-security-embedding-threat-model.md) fence rule for user-provided strings. By convention, anything inside `~~~text ... ~~~` is verbatim user content — the threat-model author chose to include it, but the CWE references inside are the user's words, not the project's classification. **Exclude regardless of integer values inside the fence.**

This is technically a subset of region #2 (any `~~~` fence excludes), but the explicit naming here defends the convention against future "let's preserve user-fence content" requests that would re-introduce the prompt-injection class.

## Rationale

Treating only the rendered Markdown prose body as authoritative — and leaving HTML comments, code fences, and user-string fences inert — eliminates the bug class enumerated as **`tm-scanner-orchestration-abuse-1`** in [`docs/slo/design/scanner-orchestration-threat-model.md`](../../docs/slo/design/scanner-orchestration-threat-model.md):

> A hostile or unwary contributor authoring (or editing) a threat-model file embeds CWE references inside HTML comments, fenced code blocks, or `~~~text` user-string fences hoping the skill picks them up — inflating the rule pack with irrelevant rules, or bypassing review by hiding CWE additions.

The scope rule defuses this **architecturally**, not via a runtime check. There is no list of "blessed" reviewers or trust labels to maintain — the parser simply ignores non-prose regions, and that's the entire defense.

## Stability

This contract is `stable` per [`docs/slo/design/scanner-orchestration-interfaces.md` §2](../../docs/slo/design/scanner-orchestration-interfaces.md). Changing the regex or any of the three exclusion regions requires:

1. Re-running `/slo-architect scanner-orchestration` to surface the design change.
2. A new runbook milestone with explicit migration tests for any existing threat-model files whose interpretation would change.
3. Updates to every consumer (`/slo-sast`, future `/slo-rulegen`, future audit-coverage skill).

## Out of scope for M1

- Full long-form CWE strings (`"CWE-89: Improper Neutralization..."`) — M1 emits only `"CWE-<integer>"`.
- Non-Markdown threat-model formats (OTM JSON, CycloneDX TM-BOM) — Markdown is v1 input per the architect overview.
- Stack detection, rule fetching, rule filtering — M2's domain.
- Length limits / streaming — M1 reads the file fully into memory; pathological-size files (>100 MiB) are out of scope (solo OSS maintainer threat models will not be that large).
