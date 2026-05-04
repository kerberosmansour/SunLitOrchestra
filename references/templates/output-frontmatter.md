---
name: output-frontmatter
status: stable-template
created: 2026-05-04
audience: skills that write durable Markdown artifacts
purpose: Canonical base fields for artifact provenance and auditability.
---

# Output Frontmatter Template

This template is the cross-skill base. Domain schemas such as `references/biz/artifact-schema.md` may extend it but should not contradict it. It cites `references/templates/citation-discipline.md` for source provenance fields.

## Base Fields

```yaml
---
name: <kebab-slug>
created: <YYYY-MM-DD>
status: draft
skill: <skill-name>
source_refs:
  - <path-or-url>@<retrieved-date>
agent_version: <host-or-model-id>
agent_session_id: <opaque-id-without-secrets>
---
```

## Field Rules

- `name` is stable and path-safe.
- `created` uses local date in ISO format.
- `status` is one of `draft`, `in-progress`, `verified`, `complete`, or a domain-specific enum.
- `source_refs` names the authority files or URLs used for factual claims.
- `agent_session_id` must not contain user secrets or personal data.

## Extension Rule

Per-skill schemas may add fields such as `tier`, `mode`, `baseline_ref`, or `gates_evaluation`. They must document whether each field is required, optional, or conditional.
