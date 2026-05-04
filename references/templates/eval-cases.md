---
name: eval-cases
status: stable-template
created: 2026-05-04
audience: skills that maintain documented behavioral expectations
purpose: Shared eval-case shape for manual checks now and future runtime harnesses later.
---

# Eval Cases Template

Eval cases are durable expectations. They cite `references/templates/citation-discipline.md` when an expected behavior depends on a source-backed rule.

## File Frontmatter

```yaml
---
skill: slo-example
case: happy-path
category: happy-path
expected_behavior: <one sentence>
risk: low
---
```

## Required Categories

- `happy-path`
- `missing-context`
- `ambiguous-input`
- `adversarial`
- `outdated-information`
- `tool-failure`
- `high-risk-case`

Every category should carry at least two distinct examples unless the skill documents why that category does not apply.

## Body Shape

```markdown
## Input
~~~text
<literal user or tool input>
~~~

## Expected Behavior
<what the skill should do>

## Must Not
- <forbidden behavior>
```

## Harness Compatibility

Do not rely on hidden conversation state. A future runner should be able to load the case file and dispatch it with only the frontmatter plus body.
