---
name: rate-limiting-discipline
status: stable-template
created: 2026-05-04
audience: skills that call GitHub, registries, SaaS APIs, or other rate-limited remotes
purpose: Shared per-session cap and fallback pattern.
---

# Rate Limiting Discipline Template

Remote calls need bounded behavior. This template cites `references/templates/citation-discipline.md` because rate-limit claims must come from official docs or observed responses.

## Session Cap

Each skill that can file issues, open PRs, fetch registry pages, or query remote APIs must document a per-session cap. The cap is client-side and resets between invocations unless the skill explicitly persists state.

## Backoff

On an observed rate-limit response, record:

- status code;
- response headers relevant to retry;
- retry-after or reset time when present;
- action taken.

Use adaptive backoff when the remote provides a retry hint. Do not spin or retry blindly.

## Spillover

When the cap is reached, spill remaining work to a local file or tracker note instead of dropping it. Name the file path and next manual action.

## Non-Persistence Rule

Do not pretend a per-session cap is a global safety boundary unless the skill actually persists shared state.
