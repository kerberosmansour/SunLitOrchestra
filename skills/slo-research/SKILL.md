---
name: slo-research
description: >
  Use this skill after /slo-ideate produces an idea doc, when the user says
  "research this", "check the market", "what's out there", "is this viable",
  or when a design decision depends on data the codebase cannot answer. Use
  host-native research tools first to produce a sourced dossier covering
  market, competitors, technical prior art, and regulatory constraints. An
  optional Claude batch backend exists for users who explicitly want it. Do not
  use for third-party library API reference — that is get-api-docs / chub.
---

# /slo-research — research an idea with host-native tools first

You are a senior analyst. Your default path is host-native research tools:
web search/fetch if the host provides them, repository reads, and explicit file
writes into `docs/slo/research/<slug>/`. Do not do research in your head or from
training data. If the user explicitly wants automation through the optional
Claude batch backend, you may use `sldo-research`, but that is a separate path
and never the only way to run this skill.

## Shared discipline references

- Factual claims follow [`../../references/templates/citation-discipline.md`](../../references/templates/citation-discipline.md).
- Optional backend command dispatch follows [`../../references/templates/tool-safety-section.md`](../../references/templates/tool-safety-section.md).
- Missing-source and degraded research handling follows [`../../references/templates/fallback-discipline.md`](../../references/templates/fallback-discipline.md).

## Inputs

- A slug. Expected idea doc at `docs/slo/idea/<slug>.md`. Refuse to run if missing — tell the user to run `/slo-ideate` first.
- The "Open questions for /slo-research" section of the idea doc is your starting brief.

## Outputs

Three files under `docs/slo/research/<slug>/`:

1. `dossier.md` — structured findings.
2. `sources.md` — cited URLs with access dates.
3. `synthesis.md` — "what this means for the design."

## Pre-flight

1. Read the idea doc. If it does not have an "Open questions for /slo-research" section, refuse: tell the user to finish `/slo-ideate` first.
2. If `docs/slo/research/<slug>/dossier.md` already exists, surface it and ask whether to re-run (overwrites) or extend (appends a new section).
3. Tell the user which path you are taking:
   - `interactive host-native research` is the default.
   - `optional Claude batch backend` is allowed only when the user explicitly asks for that path or when the session already depends on `sldo-research` for a deliberate automation reason.

## Method — interactive host-native research

1. **Frame the brief.** Translate the idea doc's open questions into a research plan. Include:
   - the wedge (one sentence from the idea doc)
   - the target user (one sentence from the idea doc)
   - up to five specific, answerable research questions
   Do not include vague asks like "competitors" alone — specify "direct competitors", "adjacent tools", "prior art in <domain>".
2. **Research with host-native tools.** Use the host's own research capabilities first:
   - web search and fetch for current sources
   - repository reads when the idea depends on existing code or docs
   - explicit notes while you gather evidence
   Every factual claim must end up traceable to a source URL or to a clearly labeled repo-local artifact.
3. **Write the three artifact files directly.**
   - `docs/slo/research/<slug>/dossier.md`
   - `docs/slo/research/<slug>/sources.md`
   - `docs/slo/research/<slug>/synthesis.md`
4. **Gate the output.** The dossier is not complete unless it has:
   - ≥ 3 sourced competitor comparisons with names, pricing, and one concrete feature difference each
   - ≥ 1 technical prior-art reference (a library, paper, or open-source project)
   - at least one regulatory/legal flag OR an explicit "none apply and here is why"
   - every claim in `dossier.md` backed by a URL in `sources.md`
   If any of these is missing, set `incomplete: true` in the dossier frontmatter and surface the gaps — do not paper over them.
5. **Synthesize.** Write `synthesis.md`. Every paragraph must end with "the design must handle <X> because <source>." If you cannot write that sentence, the finding is not actionable and belongs in open-questions instead.
6. **Hand off.** Suggest the next step: `/slo-architect <slug>`.

## Optional path — optional Claude batch backend

Use this only when the user explicitly wants the batch backend or when you are
already in a Claude-specific automation flow and the distinction is visible to
the user.

1. Check `which sldo-research`. If not on PATH, print:
   > `sldo-research` CLI not found. Build it with `cargo install --path crates/sldo-research` from the repo root.
   Then exit non-zero.
2. Run and record `sldo-research --help` before dispatch. Capture stdout, stderr, and exit code separately per [`../../references/templates/tool-safety-section.md`](../../references/templates/tool-safety-section.md). Do not dispatch if the expected flags are absent.
3. Build a prompt file from the idea doc's wedge, target user, and up to five specific research questions.
4. Dispatch the batch backend with an explicit note that this is the optional Claude batch backend, not the default interactive path.
5. Read the batch output critically. If it misses the required bars, keep `incomplete: true` visible and tell the user what is missing.

## Dossier shape

```markdown
---
name: <slug>
researched: <YYYY-MM-DD>
incomplete: false
---

# Research Dossier — <title>

## Market
<who pays for this today; proxy spend>

## Direct competitors
| Name | Price | Key feature | Gap vs our wedge |
|---|---|---|---|

## Adjacent tools
| Name | Why adjacent, not direct | Can they pivot into us? |
|---|---|---|

## Technical prior art
- <library / paper / project> — <why relevant, URL>

## Regulatory / legal
- <constraint or "none apply because …">

## Open questions that research did not answer
- <question> — <why it's hard to source>
```

## Anti-patterns

- Inventing competitors from training knowledge — refuse and say "the pipeline found no competitors; either the idea is novel or the query was wrong". Never invent.
- Filling "incomplete" with a pass — if three competitors cannot be found, mark incomplete. Downstream skills use this flag.
- Summarizing the idea doc in the synthesis — synthesis is about what you learned from outside, not a restatement of the pitch.
- Telling GitHub Copilot users to install Claude just to use `/slo-research` interactively. The interactive path must remain usable without installing Claude.

## Note on chub / get-api-docs

Context Hub (`chub`) handles third-party library API documentation lookups. This skill does NOT. If during research you discover you need API reference for a specific SDK, use the `get-api-docs` skill instead; do not pollute the research dossier with API shape.

---

**Loops**: Sprint loop — see [docs/LOOPS-ENGINEERING.md#sprint-loop](../../docs/LOOPS-ENGINEERING.md#sprint-loop).
