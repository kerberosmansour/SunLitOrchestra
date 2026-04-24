---
name: tla-sha-autopop
researched: 2026-04-24
incomplete: true
---

# Research Synthesis — TLA+ tool SHA auto-populate

## Brief

Minimal research run: the feature is self-contained to our own skill. No market or competitors matter. The two open questions from `/slo-ideate` are technical and answerable without `sldo-research`.

## Findings

1. **GitHub releases publish `.sha256` sibling files only when the publisher uploads them.** The TLA+ project (`tlaplus/tlaplus`) historically does NOT upload sibling SHA files with releases — only the raw `tla2tools.jar` and the source archives. Apalache (`apalache-mc/apalache`) also does not publish sibling SHAs. Conclusion: we compute locally; there's no upstream hash to cross-check against.

2. **Release URLs go directly to `objects.githubusercontent.com` via redirect.** The `https://github.com/<org>/<repo>/releases/download/<tag>/<file>` URL resolves to a signed S3-style URL on GitHub's release-assets CDN. The signed URL is specific to a particular upload; you cannot swap the underlying bytes without changing the release. Conclusion: CDN-substitution attacks would require compromising GitHub's release-assets infrastructure, which is outside our threat model.

## What the design must handle

- The design must handle absent `.sha256` sibling files because [TLA+ and Apalache do not publish them]; the helper computes locally.
- The design must handle the `UNSET` sentinel specifically (not just "missing value") because [the initial commit intentionally uses `UNSET` as a placeholder].
- The design must handle multiple `[package]` entries (TLC, Apalache) in one file because [tools.toml already ships with both].

## Incomplete flag

Marked `incomplete: true` because the research was scoped deliberately shallow — there is no market research, no competitor analysis. Justification: the feature is an internal maintenance helper, not a product. Market research would be theater.

## Open questions that remain

None. Both of the `/slo-ideate` questions were answered by reading the upstream release pages directly.
