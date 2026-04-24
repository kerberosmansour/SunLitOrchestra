---
topic: a hand-written plan-ready research dossier for M7 E2E tests
generated_on: 2026-04-19 12:00:00 +0000
source_prompt_bytes: 100
generator: sldo-research
---

# Research Dossier

This dossier is a structured research artifact produced by `sldo-research`. It is intended as the `prompt_file` input to `sldo-plan`.

## Executive Summary

This dossier presents a synthesised summary of research into adding a feature-flag system to the target codebase. It draws on exploration of the user prompt, web searches for current library options, and deepening passes that resolved the most important open questions. The recommended path uses a small, embedded evaluator with a remote-config refresh hook, balancing operational simplicity with runtime flexibility.

## Topic Decomposition

The research broke the topic into eight sub-questions covering: the choice between a remote SaaS vs an embedded library, the supported targeting rules (user, tenant, environment), latency and cache strategy, observability hooks, rollout and kill-switch ergonomics, audit logging requirements, integration with existing config and secret stores, and how to test feature-flag-gated branches in CI.

## Key Findings

The dominant open-source feature-flag libraries are OpenFeature (a vendor-neutral SDK spec), Unleash, and GrowthBook. OpenFeature offers the longest-term portability story; Unleash and GrowthBook each ship a complete server + SDK pair. Embedding-only paths (no server) are feasible with file-backed config plus a refresh worker, and are the simplest to operate at small scale. Audit logging is consistently flagged as the weakest area across vendors and likely needs in-house instrumentation regardless of choice.

## Library & Tool Evaluations

OpenFeature (confidence: high) — vendor-neutral SDK; the project is CNCF-incubated with active maintenance; supports Rust via the `openfeature` crate; recommended when long-term portability outweighs ramp-up cost. Unleash (confidence: medium) — open-source server + Rust SDK; mature targeting DSL; requires hosting an additional service. GrowthBook (confidence: medium) — strong A/B test orientation; Rust SDK is maintained but smaller community; good fit when experimentation is the primary use case. File-backed embedded option (confidence: high) — simplest operationally; no new service; refresh worker re-reads config every 30s; recommended for the initial rollout.

## Architecture Options

Option A (recommended): embedded evaluator with file-backed config and a 30s refresh worker. Flags live in `flags.toml` checked into the repo; production overrides come from a secrets-managed JSON file. Pros: zero new infrastructure, easy to reason about, trivial CI integration. Cons: no real-time changes, no central UI. Option B: OpenFeature SDK with a hosted backend. Pros: standardised API, easy to swap backends later. Cons: requires choosing and operating a backend; longer integration timeline. Option C: full Unleash server + SDK. Pros: complete UX. Cons: heaviest operational burden; overkill for current scale.

## API & SDK Documentation

The OpenFeature Rust SDK is documented at the project's GitHub (openfeature/openfeature-rust). The `Client::get_boolean_value` API is the primary entry point and is stable as of 0.6. Unleash's Rust SDK (Unleash/unleash-client-rust) ships a polling client with a 15s default poll interval, configurable via `UnleashClientBuilder`. GrowthBook's Rust SDK is published as the `growthbook` crate and exposes a `GrowthBookClient::eval_feature` method. All three publish OpenAPI specs for their server APIs. File-backed evaluators have no external SDK — the implementation is a small in-house module reading TOML.

## Design Recommendations

(confidence: high) Start with the embedded file-backed evaluator (Option A) as the M1 deliverable. It removes infrastructure dependencies from the critical path and lets the team validate the targeting DSL without committing to a vendor. (confidence: high) Define an internal `FlagEvaluator` trait so that future swaps to OpenFeature (Option B) require changing only the construction call site. (confidence: medium) Add audit logging at the trait boundary: every `evaluate` call writes a structured log line with flag name, computed value, and targeting context — independent of which backend implements the trait. (confidence: medium) Deferred: building a UI for non-engineers to flip flags. The M1 deliverable expects engineers to edit `flags.toml` directly; a UI is an M3+ concern.

## Risks & Open Questions

The main open question is whether the targeting DSL needs to support percentage-based rollouts in M1 — current findings suggest no (binary on/off is sufficient for the first feature) but two stakeholders flagged it as desirable. A second risk is config drift between local development and production: needs verification that the secrets-managed override file format matches the in-repo TOML format exactly. A third risk is observability: none of the surveyed vendors ship audit logging out of the box, so in-house instrumentation is on the critical path for any compliance use case.

## References

- [OpenFeature project home](https://openfeature.dev)
- [OpenFeature Rust SDK](https://github.com/open-feature/rust-sdk)
- [Unleash open-source feature flags](https://www.getunleash.io/)
- [Unleash Rust client](https://github.com/Unleash/unleash-client-rust)
- [GrowthBook documentation](https://docs.growthbook.io/)
- [Feature toggle patterns by Pete Hodgson](https://martinfowler.com/articles/feature-toggles.html)
