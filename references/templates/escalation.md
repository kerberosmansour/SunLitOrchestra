---
name: escalation
status: stable-template
created: 2026-05-04
audience: skills that refuse, route, split, or defer work
purpose: Shared fail-loud and routing language for unsafe or underspecified requests.
---

# Escalation Template

Escalation is not failure; it is how the skill preserves correctness. This template cites `references/templates/citation-discipline.md` because routing decisions must name the predicate or missing authority.

## Decision States

- `proceed` when the contract is clear.
- `refuse` when the request violates a hard gate.
- `route` when another skill owns the next step.
- `split` when the work is too large for one ticket or milestone.
- `insufficient-info` when a high-risk field is unknown.

## Message Shape

```markdown
I cannot proceed as requested because <predicate or missing field>.
The risk is <concrete failure>.
Next action: <skill, human review, source lookup, or contract update>.
```

## False-Positive Triage

When a scanner or heuristic flags a likely false positive, record:

- the matched pattern;
- why it may be false positive;
- the override field or route;
- the reviewer who must accept the override.

Do not silently suppress the row. An auditable override is better than hidden triage.
