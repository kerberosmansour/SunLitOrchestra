# Secure Execution Controls Fixture

This fixture is intentionally small and synthetic. It gives `/slo-execute` a
mixed surface to classify during secure-construction pre-flight.

## Rust boundary

- One HTTP request boundary accepts untrusted Markdown-like input.
- Expected secure default: use `secure_boundary` helpers instead of local prompt
  fencing, raw path parsing, or ad hoc validation.
- Security tests: unit test the boundary helper, then select SAST/variant
  checks from the touched surface.

## Pulumi TypeScript

- One cloud resource models a hardened bucket and deployment boundary.
- Expected secure default: when Hulumi is explicit, prefer
  `@hulumi/baseline.aws.SecureBucket` plus policy-as-code and preview evidence.
- Security tests: Pulumi mocks, policy pack check, preview evidence, and drift
  evidence when a live stack exists.
