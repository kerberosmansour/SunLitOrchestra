# /slo-graphify

Graph-backed codebase investigation for private product repos.

Use it when you want Graphify to help an engineer or AI coding agent answer:

- Which files matter for this GitHub Issue?
- What route, worker, command, or module path explains this behavior?
- Which OpenGrep/Semgrep findings are real product risks?
- Which normal QA bugs are waiting to happen around retries, panics, input
  validation, or error handling?

The skill treats security as one lane, not the only lane. It also covers
knowledge discovery and troubleshooting.

Raw private evidence stays ignored and uncommitted. Tracked outputs should be
anonymized summaries only: counts, classes, generic path shapes, and next
actions.

Quick readiness check:

```bash
sldo-install graphify --install-plan
sldo-install graphify
```
