---
name: slo-sast-methodology-m1-parser
source_skill: skills/slo-sast/SKILL.md
stage: M1
status: stable-reference
---

# /slo-sast Methodology M1 — Parser Scaffold

This is the M1 milestone of the [scanner-orchestration runbook](../../../docs/slo/completed/RUNBOOK-SCANNER-ORCHESTRATION.md): **parser-only**. M2–M5 progressively add stack detection, registry fetch + filter, file emission, manifest + preview-mode, and the re-derivation loop. Prior to M2 landing, this skill produces a CWE list to stdout and stops there.

## Method (M1 — parser scaffold)

The skill's M1 job is exactly one thing: extract CWE references from the threat-model file, per the parse contract in [`references/sast/threat-model-parser-contract.md`](../../../references/sast/threat-model-parser-contract.md).

### Threat-model parser scope rule

Apply regex `\bCWE-(\d+)\b` against the rendered Markdown body **only**. Exclude:

- **HTML comments** (`<!-- ... -->`, possibly multi-line). The entire region from `<!--` through `-->` is excluded.
- **Fenced code blocks** (` ``` ` or `~~~` at the start of a line, with up to 3 spaces of leading whitespace per CommonMark; closing fence at the start of a line). Indented code blocks (4-space indent) are also excluded.
- **`~~~text` user-string fences specifically** — these wrap user-provided content per the slo-security-embedding fence rule and must NOT influence rule selection. (This is technically a subset of fenced code blocks; explicit naming here defends the convention against future "let's preserve user-fence content" requests.)

This scope rule defuses **`tm-scanner-orchestration-abuse-1`** (a hostile or unwary contributor smuggling CWE references in non-prose regions to bias rule selection without the threat-model author noticing). The defense is architectural — non-prose regions are simply not parsed — not a runtime check.

### Process

1. Read the threat-model file fully into memory.
2. Walk the file line-by-line, tracking three exclusion-region states:
   - `in_html_comment: bool` — toggled on `<!--`, off on `-->`.
   - `in_code_fence: Option<&str>` — `Some("```"\)` or `Some("~~~")` when inside a fence; opening fence at line start (up to 3 spaces leading); closing fence matches the opening character sequence at line start.
   - `in_indented_code: bool` — heuristic; line starts with 4+ spaces and the prior line was blank or also indented.
3. For lines NOT inside any excluded region, apply regex `\bCWE-(\d+)\b` and capture the integer.
4. Deduplicate the captures.
5. Sort ascending by integer value.
6. Emit as a Python-style list literal of long-form strings: `["CWE-77", "CWE-78", "CWE-89"]`.

### Empty list behavior

If zero CWE references appear in prose:

- stdout: `[]`
- stderr: `"No CWE references found in threat-model prose. Default fallback rule selection lands in M2; until then, /slo-sast emits an empty list."`
- exit 0 (empty is a valid M1 result).

### Output format

stdout — JSON object envelope with `cwes_extracted`, `detected_stack`, `selected_rules`, `selection_strategy` (M2 contract). The CWE list in `cwes_extracted` carries the long-form `"CWE-N"` strings, ascending integer order. M1's bare-list format (`["CWE-77", "CWE-78", "CWE-89"]`) is superseded — the M1 E2E tests have been migrated to examine `cwes_extracted` field of the JSON envelope.

### Anti-patterns (M1 specific)

- **Treating CWE references inside HTML comments / code fences / user-string fences as authoritative.** The scope rule is non-negotiable. If a future requirement seems to need parsing comments, surface that as a fresh `/slo-architect` decision, not a code-level relaxation.
- **Emitting any artifact into the target repo.** M1 is parser-only; emission is M3. Do NOT create `.semgrep/`, do NOT create `.github/workflows/sast.yml`, do NOT touch any path beyond reading the threat-model file.
- **Inferring stack or selecting rules.** That's M2's domain.
- **Caching the parsed CWE list across invocations.** M1 reads, parses, prints, exits. No state survives.
- **Falling back to a default rule pack on empty parse.** That's M2's behavior; M1 just reports the empty list and notes the fallback is forthcoming.
- **Writing a partial list when the threat-model file is missing.** Exit non-zero, no stdout output. The user must run `/slo-architect` to produce the threat model.
- **Running `claude` / `git` / `gh` / `semgrep` subprocesses.** M1 is pure file-read + parse + print. Subprocess invocations land in M2 (`git`), M3 (none — emission is file writes), M4 (`semgrep --version`), M5 (`gh`).
