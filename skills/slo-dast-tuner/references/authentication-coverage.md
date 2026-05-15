---
name: slo-dast-tuner-authentication-coverage
source_skill: skills/slo-dast-tuner/SKILL.md
status: stable-reference
---

# Authentication Coverage

This reference follows the direction of ZAP's "Authentication Improvements" article (2025-07-03): authentication is scan-quality infrastructure, not a vulnerability rule.

## Auth First

For any authenticated app, record:

- auth mode: unauthenticated, API token/header, browser-based auth, browser steps, or client-side Zest auth
- credential source: secret manager, CI secret, local env, or manual-only
- logged-in verification signal
- logout URL/control patterns
- whether AJAX/client crawling runs inside an authenticated browser session

Do not place credentials, cookies, local storage, or screenshots containing secrets in committed files.

## Escalation Ladder

1. API token/header or basic login when the app is API-only.
2. Browser-backed auth with login URL and credentials for JavaScript-heavy apps.
3. Browser-backed auth with explicit steps when simple form detection fails.
4. Client-side Zest auth only when the flow is too custom for the previous modes.

## Coverage Failure States

Report `needs-human-input` or coverage failure when:

- login cannot be completed
- logged-in verification is absent or unstable
- the crawler logs out
- required tenant/role/fixture state is missing
- authenticated endpoints are in SARIF but scan config is unauthenticated

"No findings" is not meaningful until auth state is verified for the endpoints under test.

## Diagnostics

When auth fails, preserve diagnostics only in sensitive artifact locations: screenshots, HTTP traffic, browser storage, page elements, and session verification output. Redact before sharing outside the target team.
