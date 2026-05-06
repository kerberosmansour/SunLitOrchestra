# Shared security reporting references

This directory holds shared output shapes for security findings and assessment summaries emitted by SunLit Orchestra skills.

Use these references when a skill produces security-relevant findings, especially:

- `/slo-critique` security persona findings.
- `/slo-verify` Pass 4 findings.
- `/slo-sast` coverage-gap or generated-SAST findings.
- `/slo-ruleverify` rule-gate failures that need human triage.

The goal is consistency: every security finding should be evidence-based, scenario-backed, mapped to the relevant threat-model row when one exists, and include a concrete remediation path. A generic framework category is not enough on its own.

## Files

- [security-finding-template.md](security-finding-template.md) — expanded single-finding shape for security rows that need more detail than a compact table cell can hold.
- [security-assessment-summary-template.md](security-assessment-summary-template.md) — optional roll-up shape for multi-finding security reviews.

## Minimum standard

Every security finding should answer:

- What component or file is affected?
- Which actor can trigger the problem?
- What action do they take?
- What bad outcome follows?
- Which evidence supports the finding?
- Which remediation closes it?
- Which threat-model row, CWE, OWASP, ASVS, or OpenCRE mapping applies, when known?

