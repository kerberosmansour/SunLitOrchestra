# Security assessment summary template

Use this template when a review produces multiple security findings or combines outputs from several skills.

```markdown
# Security Assessment Summary - <target>

| Field | Value |
|---|---|
| Date | `<YYYY-MM-DD>` |
| Target | `<repo / feature / runbook / milestone / component>` |
| Scope | `<what was reviewed>` |
| Out of scope | `<what was not reviewed>` |
| Skills / tools used | `<skills and commands>` |

## Executive summary

`<Two or three sentences: scope, highest-risk issue, and recommended next action.>`

## Findings summary

| ID | Severity | Confidence | Title | Location | Threat row | CWE / standard | Status |
|---|---|---|---|---|---|---|---|
| `<id>` | `<severity>` | `<confidence>` | `<title>` | `<file:line>` | `<tm-row>` | `<CWE / OWASP / ASVS / OpenCRE>` | `<status>` |

## Findings detail

Use [security-finding-template.md](security-finding-template.md) for each finding.

## Coverage gaps

| Gap | Reason | Follow-up |
|---|---|---|
| `<what was not covered>` | `<why>` | `<issue / runbook / next command>` |

## Remediation order

1. `<Highest-risk or easiest risk-reduction step>`
2. `<Next step>`
3. `<Next step>`
```

## Rules

- Summaries must name scope and out-of-scope explicitly.
- Findings stay sorted by severity, then confidence, then exploitability.
- Coverage gaps are not findings unless there is a concrete exploit or failure scenario.

