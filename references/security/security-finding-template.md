# Security finding template

Use this template for security findings that need more detail than the compact `/slo-critique` or `/slo-verify` tables can carry.

```markdown
### [SEVERITY] <short title>

| Field | Value |
|---|---|
| ID | `<prefix>-sec-<N>` |
| Source | `/slo-critique` security persona / `/slo-verify` Pass 4 / `/slo-sast` / `/slo-ruleverify` |
| Status | `open` / `fixed` / `accepted-risk` / `false-positive` |
| Confidence | `high` / `medium` / `low` |
| Location | `<file>:<line>` or `<component>` |
| Affected surface | `<API / CLI / skill / workflow / generated artifact / dependency>` |
| Data classification | `Public` / `Internal` / `Confidential` / `Restricted` / `N/A` |
| Threat-model row | `tm-<slug>-abuse-<N>` or `N/A - no existing row` |
| Bug class / CWE | `<local bug-class id>` / `CWE-<N>` or `N/A` |
| Standards mapping | `<OWASP / ASVS / LLM / OpenCRE reference>` or `N/A` |

#### Concrete scenario

Given `<actor and preconditions>`, when `<specific action>`, then `<specific bad outcome>`.

#### Evidence

- `<code path, command output, tool finding, or reproduction note>`

#### Impact

`<What the attacker or failure mode can achieve, and why it matters for this project.>`

#### Remediation

`<Specific change, test, or control that closes the finding.>`

#### Verification

`<Command, test, or review step that proves the remediation worked.>`
```

## Rules

- Do not file a finding that lacks a concrete scenario.
- Do not cite a broad category such as "OWASP A01" without naming the concrete surface.
- Prefer exact file and line evidence. If exact line evidence is impossible, name the component and explain why.
- `accepted-risk` requires a one-sentence residual-risk explanation and an owner.
- `false-positive` requires the evidence that disproves the issue.

